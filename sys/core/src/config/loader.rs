//! Configuration Loader
//!
//! Loads configuration from JSON and YAML files with validation.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::merge_map::{MergeStrategy, CORE_MERGE_SPECS};
use crate::error::{ConfigError, Result};
use super::{
    expand_env_vars, DatabaseConfig, Environment, LogFormat, LogLevel, LoggingConfig,
    NoaConfig, ProviderConfig, ProviderSettings,
};

/// Configuration loader for NOA
pub struct ConfigLoader {
    noa_root: PathBuf,
}

impl ConfigLoader {
    pub fn new(noa_root: &Path) -> Self {
        Self {
            noa_root: noa_root.to_path_buf(),
        }
    }

    /// Load configuration from standard locations
    pub fn load(&self) -> Result<NoaConfig> {
        // Try to load from noa-instance.yaml, then config/noa.yaml, then defaults
        let config_paths = [
            self.noa_root.join("config/noa-instance.yaml"),
            self.noa_root.join("config/noa.yaml"),
            self.noa_root.join("config/noa.json"),
        ];

        let mut raw = serde_json::Value::Object(serde_json::Map::new());

        for path in &config_paths {
            if path.exists() {
                raw = self.load_file(path)?;
                break;
            }
        }

        // Load additional configuration files and merge
        self.merge_from_map(&mut raw)?;

        // Build the configuration
        self.build_config(raw)
    }

    /// Load a single configuration file
    fn load_file(&self, path: &Path) -> Result<serde_json::Value> {
        let content = fs::read_to_string(path).map_err(|_| {
            ConfigError::FileNotFound(path.display().to_string())
        })?;

        let expanded = expand_env_vars(&content);

        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        match extension {
            "yaml" | "yml" => {
                serde_yaml::from_str(&expanded).map_err(|e| {
                    ConfigError::ParseError {
                        path: path.display().to_string(),
                        error: e.to_string(),
                    }.into()
                })
            }
            "json" => {
                serde_json::from_str(&expanded).map_err(|e| {
                    ConfigError::ParseError {
                        path: path.display().to_string(),
                        error: e.to_string(),
                    }.into()
                })
            }
            _ => Err(ConfigError::ParseError {
                path: path.display().to_string(),
                error: "Unsupported file format".to_string(),
            }.into()),
        }
    }

    fn merge_from_map(&self, raw: &mut serde_json::Value) -> Result<()> {
        let serde_json::Value::Object(ref mut map) = raw else {
            return Ok(());
        };

        for spec in CORE_MERGE_SPECS {
            let path = spec.full_path(&self.noa_root);
            if !path.exists() {
                continue;
            }

            let value = self.load_file(&path)?;
            match spec.strategy {
                MergeStrategy::Namespaced => {
                    map.insert(spec.raw_key.to_string(), value);
                }
            }
        }

        Ok(())
    }

    /// Build NoaConfig from raw JSON
    fn build_config(&self, raw: serde_json::Value) -> Result<NoaConfig> {
        let instance_name = raw
            .get("instance")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("noa-local")
            .to_string();

        let environment = raw
            .get("instance")
            .and_then(|v| v.get("environment"))
            .and_then(|v| v.as_str())
            .unwrap_or("development")
            .parse()
            .unwrap_or_default();

        let database = self.build_database_config(&raw)?;
        let logging = self.build_logging_config(&raw)?;
        let providers = self.build_provider_config(&raw)?;
        let feature_flags = self.build_feature_flags(&raw)?;

        Ok(NoaConfig {
            noa_root: self.noa_root.clone(),
            instance_name,
            environment,
            database,
            logging,
            providers,
            feature_flags,
            raw,
        })
    }

    fn build_database_config(&self, raw: &serde_json::Value) -> Result<DatabaseConfig> {
        let db = raw.get("database");

        let driver = db
            .and_then(|v| v.get("primary"))
            .and_then(|v| v.get("driver"))
            .and_then(|v| v.as_str())
            .unwrap_or("sqlite")
            .to_string();

        let url_str = db
            .and_then(|v| v.get("primary"))
            .and_then(|v| v.get("url").or_else(|| v.get("connection_string")))
            .and_then(|v| v.as_str());

        let mut url = url_str
            .map(expand_env_vars)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        // Allow URL to come purely from environment if not present in config.
        // This is especially common for local/dev PostgreSQL setups.
        if url.is_none() {
            url = std::env::var("DATABASE_URL")
                .ok()
                .or_else(|| std::env::var("DB_CONNECTION_STRING").ok());
        }

        let path_str = db
            .and_then(|v| v.get("primary"))
            .and_then(|v| v.get("path"))
            .and_then(|v| v.as_str())
            .unwrap_or("data/noa.db");

        let path = PathBuf::from(expand_env_vars(path_str));

        let max_connections = db
            .and_then(|v| v.get("primary"))
            .and_then(|v| v.get("max_connections"))
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as u32;

        let mut settings = HashMap::new();
        if let Some(settings_obj) = db
            .and_then(|v| v.get("primary"))
            .and_then(|v| v.get("settings"))
            .and_then(|v| v.as_object())
        {
            for (k, v) in settings_obj {
                if let Some(s) = v.as_str() {
                    settings.insert(k.clone(), s.to_string());
                } else {
                    settings.insert(k.clone(), v.to_string());
                }
            }
        }

        Ok(DatabaseConfig {
            driver,
            url,
            path,
            max_connections,
            settings,
        })
    }

    fn build_logging_config(&self, raw: &serde_json::Value) -> Result<LoggingConfig> {
        let log = raw.get("logging");

        let level = log
            .and_then(|v| v.get("level"))
            .and_then(|v| v.as_str())
            .unwrap_or("info")
            .parse()
            .unwrap_or_default();

        let format = match log
            .and_then(|v| v.get("format"))
            .and_then(|v| v.as_str())
            .unwrap_or("json")
        {
            "text" => LogFormat::Text,
            "pretty" => LogFormat::Pretty,
            _ => LogFormat::Json,
        };

        let output_str = log
            .and_then(|v| v.get("output"))
            .and_then(|v| v.as_str())
            .unwrap_or("logs/noa.log");

        let output = PathBuf::from(expand_env_vars(output_str));

        let rotate = log
            .and_then(|v| v.get("rotate"))
            .and_then(|v| v.get("enabled"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let max_size_mb = log
            .and_then(|v| v.get("rotate"))
            .and_then(|v| v.get("max_size_mb"))
            .and_then(|v| v.as_u64())
            .unwrap_or(100);

        let max_files = log
            .and_then(|v| v.get("rotate"))
            .and_then(|v| v.get("max_files"))
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as u32;

        Ok(LoggingConfig {
            level,
            format,
            output,
            rotate,
            max_size_mb,
            max_files,
        })
    }

    fn build_provider_config(&self, raw: &serde_json::Value) -> Result<ProviderConfig> {
        let prov = raw.get("providers");

        let priority = prov
            .and_then(|v| v.get("providerPriority"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_else(|| vec!["local".to_string(), "hybrid".to_string(), "cloud".to_string()]);

        let mut providers = HashMap::new();

        if let Some(providers_obj) = prov.and_then(|v| v.get("providers")).and_then(|v| v.as_object())
        {
            for (name, settings) in providers_obj {
                let enabled = settings.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
                let priority_val = settings.get("priority").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                let provider_type = settings
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                let config_path_str = settings
                    .get("configPath")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let config_path = PathBuf::from(expand_env_vars(config_path_str));

                providers.insert(
                    name.clone(),
                    ProviderSettings {
                        enabled,
                        priority: priority_val,
                        provider_type,
                        config_path,
                    },
                );
            }
        }

        Ok(ProviderConfig { priority, providers })
    }

    fn build_feature_flags(&self, raw: &serde_json::Value) -> Result<HashMap<String, bool>> {
        let mut flags = HashMap::new();

        if let Some(flags_array) = raw.get("feature_flags").and_then(|v| v.as_array()) {
            for flag in flags_array {
                if let (Some(name), Some(enabled)) = (
                    flag.get("name").and_then(|v| v.as_str()),
                    flag.get("enabled").and_then(|v| v.as_bool()),
                ) {
                    flags.insert(name.to_string(), enabled);
                }
            }
        }

        Ok(flags)
    }
}

