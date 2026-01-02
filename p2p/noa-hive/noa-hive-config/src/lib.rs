//! Configuration management for NOA-Hive.
//!
//! Provides configuration loading from TOML files with environment variable
//! expansion and sensible defaults.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration for the NOA-Hive daemon.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Network configuration.
    pub network: NetworkConfig,
    /// Storage configuration.
    pub storage: StorageConfig,
    /// gRPC server configuration.
    pub grpc: GrpcConfig,
    /// loro CRDT configuration.
    pub loro: LoroConfig,
    /// iroh blob storage configuration.
    pub iroh: IrohConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: NetworkConfig::default(),
            storage: StorageConfig::default(),
            grpc: GrpcConfig::default(),
            loro: LoroConfig::default(),
            iroh: IrohConfig::default(),
        }
    }
}

/// Network configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct NetworkConfig {
    /// Addresses to listen on.
    pub listen_addrs: Vec<String>,
    /// Bootstrap peers for initial connection.
    pub bootstrap_peers: Vec<String>,
    /// Enable mDNS for local discovery.
    pub enable_mdns: bool,
    /// Enable relay for NAT traversal.
    pub enable_relay: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_addrs: vec![
                "/ip4/0.0.0.0/tcp/0".to_string(),
                "/ip4/0.0.0.0/udp/0/quic-v1".to_string(),
            ],
            bootstrap_peers: vec![],
            enable_mdns: true,
            enable_relay: true,
        }
    }
}

/// Storage configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct StorageConfig {
    /// Data directory for persistent storage.
    pub data_dir: PathBuf,
    /// State database filename.
    pub state_db: String,
    /// Identity key file.
    pub identity_file: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("noa")
            .join("hive");

        Self {
            data_dir,
            state_db: "state.db".to_string(),
            identity_file: "identity.key".to_string(),
        }
    }
}

/// gRPC server configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct GrpcConfig {
    /// Address to listen on.
    pub listen_addr: String,
    /// Enable reflection for debugging.
    pub enable_reflection: bool,
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:50051".to_string(),
            enable_reflection: true,
        }
    }
}

/// loro CRDT configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LoroConfig {
    /// Enable loro state synchronization.
    pub enable: bool,
    /// Sync interval in milliseconds.
    pub sync_interval_ms: u64,
}

impl Default for LoroConfig {
    fn default() -> Self {
        Self {
            enable: true,
            sync_interval_ms: 100,
        }
    }
}

/// iroh blob storage configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct IrohConfig {
    /// Enable iroh blob storage.
    pub enable: bool,
    /// Blob storage directory.
    pub blob_store: PathBuf,
}

impl Default for IrohConfig {
    fn default() -> Self {
        let blob_store = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("noa")
            .join("blobs");

        Self {
            enable: true,
            blob_store,
        }
    }
}

impl Config {
    /// Load configuration from a TOML file.
    pub fn load(path: &std::path::Path) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Load configuration from the default location.
    pub fn load_default() -> Result<Self, ConfigError> {
        let config_path = Self::default_config_path();
        if config_path.exists() {
            Self::load(&config_path)
        } else {
            Ok(Config::default())
        }
    }

    /// Get the default configuration file path.
    pub fn default_config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("noa")
            .join("hive.toml")
    }

    /// Save configuration to a TOML file.
    pub fn save(&self, path: &std::path::Path) -> Result<(), ConfigError> {
        let content = toml::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Configuration errors.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
}
