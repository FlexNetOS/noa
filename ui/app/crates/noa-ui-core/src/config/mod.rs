//! configsuration module for NOA UI
//!
//! Provides centralized configsuration management for the UI application.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub mod ai_configss;
pub mod compression_configss;
pub mod ml_configss;
pub mod monitoring_configss;
pub mod resource_configss;

pub use ai_configss::*;
pub use compression_configss::*;
pub use ml_configss::*;
pub use monitoring_configss::*;
pub use resource_configss::*;

/// Main application configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appconfigs {
    pub version: String,
    pub environment: Environment,
    pub server: Serverconfigs,
    pub ai: AIconfigs,
    pub resources: Resourceconfigs,
    pub ml: MLconfigs,
    pub compression: Compressionconfigs,
    pub monitoring: Monitoringconfigs,
    pub security: Securityconfigs,
    pub features: FeatureFlags,
}

/// Runtime environment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Production,
    Testing,
}

/// Server configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Serverconfigs {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub timeout_seconds: u64,
    pub cors_origins: Vec<String>,
    pub rate_limit: RateLimitconfigs,
}

/// Rate limiting configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitconfigs {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub enabled: bool,
}

/// Security configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Securityconfigs {
    pub jwt_secret: String,
    pub api_key_encryption_key: String,
    pub max_file_size_mb: u64,
    pub allowed_file_types: Vec<String>,
    pub sandbox_enabled: bool,
    pub sandbox_memory_limit_mb: u64,
    pub sandbox_timeout_seconds: u64,
}

/// Feature flags for conditional functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub enable_streaming: bool,
    pub enable_compression: bool,
    pub enable_caching: bool,
    pub enable_monitoring: bool,
    pub enable_ml_features: bool,
    pub enable_distributed_mode: bool,
    pub enable_real_time_collaboration: bool,
    pub enable_advanced_analytics: bool,
}

impl Default for Appconfigs {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: Environment::Development,
            server: Serverconfigs::default(),
            ai: AIconfigs::default(),
            resources: Resourceconfigs::default(),
            ml: MLconfigs::default(),
            compression: Compressionconfigs::default(),
            monitoring: Monitoringconfigs::default(),
            security: Securityconfigs::default(),
            features: FeatureFlags::default(),
        }
    }
}

impl Default for Serverconfigs {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: None,
            timeout_seconds: 30,
            cors_origins: vec![
                "http://localhost:3000".to_string(),
                "http://127.0.0.1:3000".to_string(),
            ],
            rate_limit: RateLimitconfigs::default(),
        }
    }
}

impl Default for RateLimitconfigs {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            burst_size: 10,
            enabled: true,
        }
    }
}

impl Default for Securityconfigs {
    fn default() -> Self {
        Self {
            jwt_secret: "dev-secret-change-in-production".to_string(),
            api_key_encryption_key: "dev-encryption-key-change-in-production".to_string(),
            max_file_size_mb: 100,
            allowed_file_types: vec![
                "rs", "toml", "json", "js", "ts", "jsx", "tsx",
                "html", "css", "scss", "py", "md", "yaml", "yml",
            ].into_iter().map(String::from).collect(),
            sandbox_enabled: true,
            sandbox_memory_limit_mb: 512,
            sandbox_timeout_seconds: 300,
        }
    }
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            enable_streaming: true,
            enable_compression: true,
            enable_caching: true,
            enable_monitoring: true,
            enable_ml_features: true,
            enable_distributed_mode: false,
            enable_real_time_collaboration: true,
            enable_advanced_analytics: true,
        }
    }
}

impl Appconfigs {
    /// Load configsuration from a file
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let configs: Appconfigs = serde_json::from_str(&content)?;
        Ok(configs)
    }

    /// Save configsuration to a file
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Load configsuration with environment variable overrides
    pub fn load_from_env() -> Result<Self> {
        let mut configs = Appconfigs::default();

        if let Ok(port) = std::env::var("PORT") {
            configs.server.port = port.parse()?;
        }

        if let Ok(env) = std::env::var("ENVIRONMENT") {
            configs.environment = match env.as_str() {
                "production" => Environment::Production,
                "staging" => Environment::Staging,
                "testing" => Environment::Testing,
                _ => Environment::Development,
            };
        }

        Ok(configs)
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        matches!(self.environment, Environment::Development)
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        matches!(self.environment, Environment::Production)
    }

    /// Get configsuration directory path
    pub fn get_configs_directory() -> PathBuf {
        if let Ok(configs_dir) = std::env::var("NOA_configs_DIR") {
            PathBuf::from(configs_dir)
        } else {
            dirs::configs_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("noa")
        }
    }

    /// Get data directory path
    pub fn get_data_directory() -> PathBuf {
        if let Ok(data_dir) = std::env::var("NOA_DATA_DIR") {
            PathBuf::from(data_dir)
        } else {
            dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("noa")
        }
    }

    /// Get cache directory path
    pub fn get_cache_directory() -> PathBuf {
        if let Ok(cache_dir) = std::env::var("NOA_CACHE_DIR") {
            PathBuf::from(cache_dir)
        } else {
            dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("noa")
        }
    }
}
