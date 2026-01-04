//! NOA configsuration Module
//!
//! Provides configsuration loading from JSON/YAML files with environment variable expansion.
//! §3.2: configsuration management
//! FR-001: Self-contained operation within noa_root

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::error::{configsError, NoaError, Result};

mod loader;
mod validator;

pub mod raw_access;
pub mod query;
pub mod access;
pub mod watch;
pub mod merge_map;

pub use loader::configsLoader;
pub use validator::configsValidator;

/// Core NOA configsuration
#[derive(Debug, Clone)]
pub struct Noaconfigs {
    /// Root directory for NOA installation
    pub noa_root: PathBuf,

    /// Instance name
    pub instance_name: String,

    /// Environment (development, staging, production)
    pub environment: Environment,

    /// Database configsuration
    pub database: Databaseconfigs,

    /// Logging configsuration
    pub logging: Loggingconfigs,

    /// Provider configsuration
    pub providers: Providerconfigs,

    /// Feature flags
    pub feature_flags: HashMap<String, bool>,

    /// Raw configsuration values for extension
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl std::str::FromStr for Environment {
    type Err = configsError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Environment::Development),
            "staging" | "stage" => Ok(Environment::Staging),
            "production" | "prod" => Ok(Environment::Production),
            _ => Err(configsError::InvalidValue {
                field: "environment".to_string(),
                value: s.to_string(),
                expected: "development, staging, or production".to_string(),
            }),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Development
    }
}

#[derive(Debug, Clone)]
pub struct Databaseconfigs {
    pub driver: String,
    /// Connection URL (used for PostgreSQL; optional for SQLite)
    pub url: Option<String>,
    pub path: PathBuf,
    pub max_connections: u32,
    pub settings: HashMap<String, String>,
}

impl Default for Databaseconfigs {
    fn default() -> Self {
        Self {
            driver: "sqlite".to_string(),
            url: None,
            path: PathBuf::from("data/noa.db"),
            max_connections: 10,
            settings: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Loggingconfigs {
    pub level: LogLevel,
    pub format: LogFormat,
    pub output: PathBuf,
    pub rotate: bool,
    pub max_size_mb: u64,
    pub max_files: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

impl std::str::FromStr for LogLevel {
    type Err = configsError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" | "warning" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(configsError::InvalidValue {
                field: "log_level".to_string(),
                value: s.to_string(),
                expected: "trace, debug, info, warn, or error".to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    Json,
    Text,
    Pretty,
}

impl Default for LogFormat {
    fn default() -> Self {
        LogFormat::Json
    }
}

impl Default for Loggingconfigs {
    fn default() -> Self {
        Self {
            level: LogLevel::default(),
            format: LogFormat::default(),
            output: PathBuf::from("logs/noa.log"),
            rotate: true,
            max_size_mb: 100,
            max_files: 10,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Providerconfigs {
    pub priority: Vec<String>,
    pub providers: HashMap<String, ProviderSettings>,
}

impl Default for Providerconfigs {
    fn default() -> Self {
        Self {
            priority: vec!["local".to_string(), "hybrid".to_string(), "cloud".to_string()],
            providers: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProviderSettings {
    pub enabled: bool,
    pub priority: u32,
    pub provider_type: String,
    pub configs_path: PathBuf,
}

impl Noaconfigs {
    /// Load configsuration from the default locations
    pub fn load() -> Result<Self> {
        let noa_root = Self::detect_noa_root()?;
        Self::load_from_root(&noa_root)
    }

    /// Load configsuration from a specific NOA root
    pub fn load_from_root(noa_root: &Path) -> Result<Self> {
        let loader = configsLoader::new(noa_root);
        loader.load()
    }

    /// Detect NOA_ROOT from environment or current directory
    fn detect_noa_root() -> Result<PathBuf> {
        // Check NOA_ROOT environment variable
        if let Ok(root) = std::env::var("NOA_ROOT") {
            let path = PathBuf::from(&root);
            if path.exists() {
                return Ok(path);
            }
        }

        // Check for .noa-env marker file in current directory and parents
        let mut current = std::env::current_dir().map_err(|e| {
            NoaError::Internal {
                message: "Failed to get current directory".to_string(),
                source: Some(Box::new(e)),
            }
        })?;

        loop {
            let marker = current.join(".noa-env");
            if marker.exists() {
                return Ok(current);
            }

            if !current.pop() {
                break;
            }
        }

        Err(configsError::MissingRequired(
            "NOA_ROOT environment variable or .noa-env marker file".to_string(),
        ).into())
    }

    /// Get a configsuration value by path (dot-separated)
    pub fn get(&self, path: &str) -> Option<&serde_json::Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &self.raw;

        for part in parts {
            current = current.get(part)?;
        }

        Some(current)
    }

    /// Check if a feature flag is enabled
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        self.feature_flags.get(feature).copied().unwrap_or(false)
    }
}

/// Expand environment variables in a string
/// Supports ${VAR} and ${VAR:-default} syntax
pub fn expand_env_vars(input: &str) -> String {
    let mut result = input.to_string();
    let re = regex::Regex::new(r"\$\{([A-Z_][A-Z0-9_]*)(?::-([^}]*))?\}").unwrap();

    for cap in re.captures_iter(input) {
        let full_match = cap.get(0).unwrap().as_str();
        let var_name = cap.get(1).unwrap().as_str();
        let default_value = cap.get(2).map(|m| m.as_str()).unwrap_or("");

        let replacement = std::env::var(var_name).unwrap_or_else(|_| default_value.to_string());
        result = result.replace(full_match, &replacement);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_parsing() {
        assert_eq!("development".parse::<Environment>().unwrap(), Environment::Development);
        assert_eq!("prod".parse::<Environment>().unwrap(), Environment::Production);
    }

    #[test]
    fn test_log_level_parsing() {
        assert_eq!("info".parse::<LogLevel>().unwrap(), LogLevel::Info);
        assert_eq!("warning".parse::<LogLevel>().unwrap(), LogLevel::Warn);
    }

    #[test]
    fn test_expand_env_vars() {
        std::env::set_var("TEST_VAR", "test_value");
        assert_eq!(expand_env_vars("${TEST_VAR}"), "test_value");
        assert_eq!(expand_env_vars("${UNDEFINED_VAR:-default}"), "default");
        std::env::remove_var("TEST_VAR");
    }
}

