//! Daemon configuration.

use anyhow::Result;
use std::path::PathBuf;

/// Daemon configuration.
#[derive(Debug, Clone)]
pub struct DaemonConfig {
    /// HTTP/gRPC port.
    pub port: u16,
    
    /// Data directory for state storage.
    pub data_dir: PathBuf,
    
    /// P2P configuration.
    pub p2p: P2pConfig,
    
    /// State sync configuration.
    pub state: StateConfig,
}

/// P2P network configuration.
#[derive(Debug, Clone)]
pub struct P2pConfig {
    /// Enable P2P networking.
    pub enabled: bool,
    
    /// P2P port (0 for random).
    pub port: u16,
    
    /// Bootstrap peers.
    pub bootstrap_peers: Vec<String>,
    
    /// Enable mDNS discovery.
    pub mdns: bool,
}

/// State synchronization configuration.
#[derive(Debug, Clone)]
pub struct StateConfig {
    /// Enable state sync.
    pub enabled: bool,
    
    /// Sync interval in seconds.
    pub sync_interval_secs: u64,
    
    /// Maximum state size in bytes.
    pub max_state_size: usize,
}

impl DaemonConfig {
    /// Create new configuration.
    pub fn new(port: u16, data_dir: Option<PathBuf>) -> Result<Self> {
        let data_dir = data_dir.unwrap_or_else(|| {
            directories::ProjectDirs::from("com", "flexnetos", "noa")
                .map(|dirs| dirs.data_dir().to_path_buf())
                .unwrap_or_else(|| PathBuf::from(".noa"))
        });
        
        // Ensure data directory exists
        std::fs::create_dir_all(&data_dir)?;
        
        Ok(Self {
            port,
            data_dir,
            p2p: P2pConfig {
                enabled: true,
                port: 0, // Random port
                bootstrap_peers: vec![],
                mdns: true,
            },
            state: StateConfig {
                enabled: true,
                sync_interval_secs: 5,
                max_state_size: 100 * 1024 * 1024, // 100MB
            },
        })
    }
    
    /// Get state file path.
    pub fn state_path(&self) -> PathBuf {
        self.data_dir.join("state.json")
    }
    
    /// Get peer ID file path.
    pub fn peer_id_path(&self) -> PathBuf {
        self.data_dir.join("peer_id.key")
    }
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self::new(9999, None).unwrap_or(Self {
            port: 9999,
            data_dir: PathBuf::from(".noa"),
            p2p: P2pConfig {
                enabled: true,
                port: 0,
                bootstrap_peers: vec![],
                mdns: true,
            },
            state: StateConfig {
                enabled: true,
                sync_interval_secs: 5,
                max_state_size: 100 * 1024 * 1024,
            },
        })
    }
}
