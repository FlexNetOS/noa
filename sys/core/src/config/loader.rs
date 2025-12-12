//! Configuration Loader
//!
//! Loads configuration from JSON and YAML files with validation.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::{
    expand_env_vars, DatabaseConfig, LogFormat, LoggingConfig, NoaConfig,
    ProviderConfig, ProviderSettings,
};
use crate::error::{ConfigError, Result};

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
        self.merge_provider_config(&mut raw)?;
        self.merge_shared_resources_config(&mut raw)?;

        // Build the configuration
        self.build_config(raw)
    }

    /// Load a single configuration file
    fn load_file(&self, path: &Path) -> Result<serde_json::Value> {
        let content = fs::read_to_string(path)
            .map_err(|_| ConfigError::FileNotFound(path.display().to_string()))?;

        let expanded = expand_env_vars(&content);

        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        match extension {
            "yaml" | "yml" => serde_yaml::from_str(&expanded).map_err(|e| {
                ConfigError::ParseError {
                    path: path.display().to_string(),
                    error: e.to_string(),
                }
                .into()
            }),
            "json" => serde_json::from_str(&expanded).map_err(|e| {
                ConfigError::ParseError {
                    path: path.display().to_string(),
                    error: e.to_string(),
                }
                .into()
            }),
            _ => Err(ConfigError::ParseError {
                path: path.display().to_string(),
                error: "Unsupported file format".to_string(),
            }
            .into()),
        }
    }

    /// Merge provider configuration
    fn merge_provider_config(&self, raw: &mut serde_json::Value) -> Result<()> {
        let providers_path = self.noa_root.join("config/ai-providers.json");
        if providers_path.exists() {
            let providers: serde_json::Value = self.load_file(&providers_path)?;
            if let serde_json::Value::Object(ref mut map) = raw {
                map.insert("providers".to_string(), providers);
            }
        }
        Ok(())
    }

    /// Merge shared resources configuration
    fn merge_shared_resources_config(&self, raw: &mut serde_json::Value) -> Result<()> {
        let shared_path = self.noa_root.join("config/shared-resources.json");
        if shared_path.exists() {
            let shared: serde_json::Value = self.load_file(&shared_path)?;
            if let serde_json::Value::Object(ref mut map) = raw {
                map.insert("shared_resources".to_string(), shared);
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

        let path_str = db
            .and_then(|v| v.get("primary"))
            .and_then(|v| v.get("path"))
            .and_then(|v| v.as_str())
            .unwrap_or("${NOA_ROOT}/data/noa.db");

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

        let format =
            match log.and_then(|v| v.get("format")).and_then(|v| v.as_str()).unwrap_or("json") {
                "text" => LogFormat::Text,
                "pretty" => LogFormat::Pretty,
                _ => LogFormat::Json,
            };

        let output_str = log
            .and_then(|v| v.get("output"))
            .and_then(|v| v.as_str())
            .unwrap_or("${NOA_ROOT}/logs/noa.log");

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
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_else(|| {
                vec![
                    "local".to_string(),
                    "hybrid".to_string(),
                    "cloud".to_string(),
                ]
            });

        let mut providers = HashMap::new();

        if let Some(providers_obj) =
            prov.and_then(|v| v.get("providers")).and_then(|v| v.as_object())
        {
            for (name, settings) in providers_obj {
                let enabled = settings.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
                let priority_val =
                    settings.get("priority").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                let provider_type =
                    settings.get("type").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
                let config_path_str =
                    settings.get("configPath").and_then(|v| v.as_str()).unwrap_or("");
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

        Ok(ProviderConfig {
            priority,
            providers,
        })
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

        // Fallback: load feature flags from config/features.json
        if flags.is_empty() {
            let feature_path = self.noa_root.join("config/features.json");
            if feature_path.exists() {
                let features = self.load_file(&feature_path)?;
                collect_feature_flags_from_value(&features, &mut flags);
            }
        }

        if flags.is_empty() {
            // Minimal defaults to avoid empty configuration
            flags.insert("connectors.enabled".to_string(), true);
            flags.insert("connectors.offline_cache".to_string(), true);
        }

        Ok(flags)
    }
}

fn collect_feature_flags_from_value(value: &serde_json::Value, flags: &mut HashMap<String, bool>) {
    if let Some(arr) = value.get("feature_flags").and_then(|v| v.as_array()) {
        for flag in arr {
            if let (Some(name), Some(enabled)) = (
                flag.get("name").and_then(|v| v.as_str()),
                flag.get("enabled").and_then(|v| v.as_bool()),
            ) {
                flags.insert(name.to_string(), enabled);
            }
        }
    }

    if let Some(connectors) = value.get("connectors").and_then(|v| v.as_object()) {
        for (name, enabled) in connectors {
            if let Some(enabled_bool) = enabled.as_bool() {
                flags.insert(format!("connectors.{}", name), enabled_bool);
            }
        }
    }

    if let Some(providers) = value.get("providers").and_then(|v| v.as_object()) {
        for (name, enabled) in providers {
            if let Some(enabled_bool) = enabled.as_bool() {
                flags.insert(format!("providers.{}", name), enabled_bool);
            }
        }
    }
}
