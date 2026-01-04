//! State management for the daemon.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::configs::Daemonconfigs;

/// State manager for CRDT-based state synchronization.
pub struct StateManager {
    configs: Daemonconfigs,
    state: Arc<RwLock<DaemonState>>,
}

/// Daemon state (persisted).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DaemonState {
    /// Version for optimistic concurrency.
    pub version: u64,
    
    /// Connected peers.
    pub peers: HashMap<String, PeerInfo>,
    
    /// Shared documents.
    pub documents: HashMap<String, DocumentInfo>,
    
    /// Agent states.
    pub agents: HashMap<String, AgentState>,
    
    /// Last sync timestamp.
    pub last_sync: Option<u64>,
}

/// Peer information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: String,
    pub address: String,
    pub connected_at: u64,
    pub last_seen: u64,
}

/// Document information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentInfo {
    pub id: String,
    pub name: String,
    pub size: usize,
    pub modified_at: u64,
}

/// Agent state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub id: String,
    pub name: String,
    pub status: AgentStatus,
    pub last_heartbeat: u64,
}

/// Agent status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    Idle,
    Running,
    Paused,
    Error,
}

impl StateManager {
    /// Create new state manager.
    pub fn new(configs: &Daemonconfigs) -> Result<Self> {
        let state = Self::load_state(configs)?;
        
        Ok(Self {
            configs: configs.clone(),
            state: Arc::new(RwLock::new(state)),
        })
    }
    
    /// Load state from disk.
    fn load_state(configs: &Daemonconfigs) -> Result<DaemonState> {
        let path = configs.state_path();
        
        if path.exists() {
            let data = std::fs::read_to_string(&path)?;
            let state: DaemonState = serde_json::from_str(&data)?;
            info!("Loaded state from {:?} (version {})", path, state.version);
            Ok(state)
        } else {
            debug!("No existing state, starting fresh");
            Ok(DaemonState::default())
        }
    }
    
    /// Save state to disk.
    pub async fn save(&self) -> Result<()> {
        let state = self.state.read().await;
        let path = self.configs.state_path();
        let data = serde_json::to_string_pretty(&*state)?;
        std::fs::write(&path, data)?;
        debug!("Saved state to {:?} (version {})", path, state.version);
        Ok(())
    }
    
    /// Get current state.
    pub async fn get_state(&self) -> DaemonState {
        self.state.read().await.clone()
    }
    
    /// Update state with a function.
    pub async fn update<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce(&mut DaemonState),
    {
        let mut state = self.state.write().await;
        f(&mut state);
        state.version += 1;
        
        // Auto-save on update
        let path = self.configs.state_path();
        let data = serde_json::to_string_pretty(&*state)?;
        std::fs::write(&path, data)?;
        
        Ok(())
    }
    
    /// Add a peer.
    pub async fn add_peer(&self, id: String, address: String) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.update(|state| {
            state.peers.insert(id.clone(), PeerInfo {
                id,
                address,
                connected_at: now,
                last_seen: now,
            });
        }).await
    }
    
    /// Remove a peer.
    #[allow(dead_code)]
    pub async fn remove_peer(&self, id: &str) -> Result<()> {
        self.update(|state| {
            state.peers.remove(id);
        }).await
    }
    
    /// Get peer count.
    #[allow(dead_code)]
    pub async fn peer_count(&self) -> usize {
        self.state.read().await.peers.len()
    }
    
    /// Register an agent.
    pub async fn register_agent(&self, id: String, name: String) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.update(|state| {
            state.agents.insert(id.clone(), AgentState {
                id,
                name,
                status: AgentStatus::Idle,
                last_heartbeat: now,
            });
        }).await
    }
    
    /// Update agent status.
    pub async fn update_agent_status(&self, id: &str, status: AgentStatus) -> Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        self.update(|state| {
            if let Some(agent) = state.agents.get_mut(id) {
                agent.status = status;
                agent.last_heartbeat = now;
            }
        }).await
    }
}

impl Clone for StateManager {
    fn clone(&self) -> Self {
        Self {
            configs: self.configs.clone(),
            state: Arc::clone(&self.state),
        }
    }
}
