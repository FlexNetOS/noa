//! configsuration management for NOA-Hive.
//!
//! Provides configsuration loading from TOML files with environment variable
//! expansion and sensible defaults.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configsuration for the NOA-Hive daemon.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct configs {
    /// Network configsuration.
    pub network: Networkconfigs,
    /// Storage configsuration.
    pub storage: Storageconfigs,
    /// gRPC server configsuration.
    pub grpc: Grpcconfigs,
    /// loro CRDT configsuration.
    pub loro: Loroconfigs,
    /// iroh blob storage configsuration.
    pub iroh: Irohconfigs,
}

impl Default for configs {
    fn default() -> Self {
        Self {
            network: Networkconfigs::default(),
            storage: Storageconfigs::default(),
            grpc: Grpcconfigs::default(),
            loro: Loroconfigs::default(),
            iroh: Irohconfigs::default(),
        }
    }
}

/// Network configsuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Networkconfigs {
    /// Addresses to listen on.
    pub listen_addrs: Vec<String>,
    /// Bootstrap peers for initial connection.
    pub bootstrap_peers: Vec<String>,
    /// Enable mDNS for local discovery.
    pub enable_mdns: bool,
    /// Enable relay for NAT traversal.
    pub enable_relay: bool,
}

impl Default for Networkconfigs {
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

/// Storage configsuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Storageconfigs {
    /// Data directory for persistent storage.
    pub data_dir: PathBuf,
    /// State database filename.
    pub state_db: String,
    /// Identity key file.
    pub identity_file: String,
}

impl Default for Storageconfigs {
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

/// gRPC server configsuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Grpcconfigs {
    /// Address to listen on.
    pub listen_addr: String,
    /// Enable reflection for debugging.
    pub enable_reflection: bool,
}

impl Default for Grpcconfigs {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:50051".to_string(),
            enable_reflection: true,
        }
    }
}

/// loro CRDT configsuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Loroconfigs {
    /// Enable loro state synchronization.
    pub enable: bool,
    /// Sync interval in milliseconds.
    pub sync_interval_ms: u64,
}

impl Default for Loroconfigs {
    fn default() -> Self {
        Self {
            enable: true,
            sync_interval_ms: 100,
        }
    }
}

/// iroh blob storage configsuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Irohconfigs {
    /// Enable iroh blob storage.
    pub enable: bool,
    /// Blob storage directory.
    pub blob_store: PathBuf,
}

impl Default for Irohconfigs {
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

impl configs {
    /// Load configsuration from a TOML file.
    pub fn load(path: &std::path::Path) -> Result<Self, configsError> {
        let content = std::fs::read_to_string(path)?;
        let configs: configs = toml::from_str(&content)?;
        Ok(configs)
    }

    /// Load configsuration from the default location.
    pub fn load_default() -> Result<Self, configsError> {
        let configs_path = Self::default_configs_path();
        if configs_path.exists() {
            Self::load(&configs_path)
        } else {
            Ok(configs::default())
        }
    }

    /// Get the default configsuration file path.
    pub fn default_configs_path() -> PathBuf {
        dirs::configs_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("noa")
            .join("hive.toml")
    }

    /// Save configsuration to a TOML file.
    pub fn save(&self, path: &std::path::Path) -> Result<(), configsError> {
        let content = toml::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// configsuration errors.
#[derive(Debug, thiserror::Error)]
pub enum configsError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
}
