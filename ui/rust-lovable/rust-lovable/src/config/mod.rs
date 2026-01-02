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

use ai_configs::AIConfig;
use compression_configs::CompressionConfig;
use ml_configs::MLConfig;
use monitoring_configs::MonitoringConfig;
use resource_configs::ResourceConfig;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Production,
    Testing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub timeout_seconds: u64,
    pub cors_origins: Vec<String>,
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub enabled: bool,
}

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
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                workers: None,
                timeout_seconds: 30,
                cors_origins: vec![
                    "http://localhost:3000".to_string(),
                    "http://127.0.0.1:3000".to_string(),
                ],
                rate_limit: RateLimitConfig {
                    requests_per_minute: 60,
                    burst_size: 10,
                    enabled: true,
                },
            },
            ai: AIConfig::default(),
            resources: ResourceConfig::default(),
            ml: MLConfig::default(),
            compression: CompressionConfig::default(),
            monitoring: MonitoringConfig::default(),
            security: SecurityConfig {
                jwt_secret: "dev-secret-change-in-production".to_string(),
                api_key_encryption_key: "dev-encryption-key-change-in-production".to_string(),
                max_file_size_mb: 100,
                allowed_file_types: vec![
                    "rs".to_string(),
                    "toml".to_string(),
                    "json".to_string(),
                    "js".to_string(),
                    "ts".to_string(),
                    "jsx".to_string(),
                    "tsx".to_string(),
                    "html".to_string(),
                    "css".to_string(),
                    "scss".to_string(),
                    "py".to_string(),
                    "md".to_string(),
                    "yaml".to_string(),
                    "yml".to_string(),
                ],
                sandbox_enabled: true,
                sandbox_memory_limit_mb: 512,
                sandbox_timeout_seconds: 300,
            },
            features: FeatureFlags {
                enable_streaming: true,
                enable_compression: true,
                enable_caching: true,
                enable_monitoring: true,
                enable_ml_features: true,
                enable_distributed_mode: false,
                enable_real_time_collaboration: true,
                enable_advanced_analytics: true,
            },
        }
    }
}

impl AppConfig {
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn load_from_env() -> Result<Self> {
        let mut config = AppConfig::default();

        // Override from environment variables
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

        if let Ok(openai_key) = std::env::var("OPENAI_API_KEY") {
            config.ai.providers.openai.api_key = Some(openai_key);
        }

        if let Ok(anthropic_key) = std::env::var("ANTHROPIC_API_KEY") {
            config.ai.providers.anthropic.api_key = Some(anthropic_key);
        }

        Ok(config)
    }

    pub fn is_development(&self) -> bool {
        matches!(self.environment, Environment::Development)
    }

    pub fn is_production(&self) -> bool {
        matches!(self.environment, Environment::Production)
    }

    pub fn get_config_directory() -> PathBuf {
        if let Ok(config_dir) = std::env::var("RUST_LOVABLE_CONFIG_DIR") {
            PathBuf::from(config_dir)
        } else {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("rust-lovable")
        }
    }

    pub fn get_data_directory() -> PathBuf {
        if let Ok(data_dir) = std::env::var("RUST_LOVABLE_DATA_DIR") {
            PathBuf::from(data_dir)
        } else {
            dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("rust-lovable")
        }
    }

    pub fn get_cache_directory() -> PathBuf {
        if let Ok(cache_dir) = std::env::var("RUST_LOVABLE_CACHE_DIR") {
            PathBuf::from(cache_dir)
        } else {
            dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("rust-lovable")
        }
    }
}
