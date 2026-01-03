//! Configuration module for NOA UI
//!
//! Provides centralized configuration management for the UI application.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub mod ai_configs;
pub mod compression_configs;
pub mod ml_configs;
pub mod monitoring_configs;
pub mod resource_configs;

pub use ai_configs::*;
pub use compression_configs::*;
pub use ml_configs::*;
pub use monitoring_configs::*;
pub use resource_configs::*;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub version: String,
    pub environment: Environment,
    pub server: ServerConfig,
    pub ai: AIConfig,
    pub resources: ResourceConfig,
    pub ml: MLConfig,
    pub compression: CompressionConfig,
    pub monitoring: MonitoringConfig,
    pub security: SecurityConfig,
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

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub timeout_seconds: u64,
    pub cors_origins: Vec<String>,
    pub rate_limit: RateLimitConfig,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub enabled: bool,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
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

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: Environment::Development,
            server: ServerConfig::default(),
            ai: AIConfig::default(),
            resources: ResourceConfig::default(),
            ml: MLConfig::default(),
            compression: CompressionConfig::default(),
            monitoring: MonitoringConfig::default(),
            security: SecurityConfig::default(),
            features: FeatureFlags::default(),
        }
    }
}

impl Default for ServerConfig {
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
            rate_limit: RateLimitConfig::default(),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            burst_size: 10,
            enabled: true,
        }
    }
}

impl Default for SecurityConfig {
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

impl AppConfig {
    /// Load configuration from a file
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a file
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Load configuration with environment variable overrides
    pub fn load_from_env() -> Result<Self> {
        let mut config = AppConfig::default();

        if let Ok(port) = std::env::var("PORT") {
            config.server.port = port.parse()?;
        }

        if let Ok(env) = std::env::var("ENVIRONMENT") {
            config.environment = match env.as_str() {
                "production" => Environment::Production,
                "staging" => Environment::Staging,
                "testing" => Environment::Testing,
                _ => Environment::Development,
            };
        }

        Ok(config)
    }

    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        matches!(self.environment, Environment::Development)
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        matches!(self.environment, Environment::Production)
    }

    /// Get configuration directory path
    pub fn get_config_directory() -> PathBuf {
        if let Ok(config_dir) = std::env::var("NOA_CONFIG_DIR") {
            PathBuf::from(config_dir)
        } else {
            dirs::config_dir()
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
