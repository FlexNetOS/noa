//! State management using loro CRDT.

use noa_hive_config::Config;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tracing::{debug, info};

/// Manages state synchronization using loro CRDT.
pub struct StateManager {
    /// Active state rooms.
    rooms: Arc<RwLock<HashMap<String, StateRoom>>>,
    /// Configuration.
    config: Config,
}

impl StateManager {
    /// Create a new state manager.
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
            config: config.clone(),
        })
    }

    /// Join or create a state room.
    pub fn join_room(&self, room_id: &str) -> anyhow::Result<()> {
        let mut rooms = self.rooms.write().unwrap();
        if !rooms.contains_key(room_id) {
            info!(room_id = %room_id, "Creating state room");
            rooms.insert(room_id.to_string(), StateRoom::new(room_id));
        }
        Ok(())
    }

    /// Leave a state room.
    pub fn leave_room(&self, room_id: &str) -> anyhow::Result<()> {
        let mut rooms = self.rooms.write().unwrap();
        rooms.remove(room_id);
        info!(room_id = %room_id, "Left state room");
        Ok(())
    }

    /// Get the state for a room.
    pub fn get_state(&self, room_id: &str) -> Option<Vec<u8>> {
        let rooms = self.rooms.read().unwrap();
        rooms.get(room_id).map(|room| room.export())
    }

    /// Apply a CRDT operation to a room.
    pub fn apply_op(&self, room_id: &str, op: &[u8]) -> anyhow::Result<()> {
        let rooms = self.rooms.read().unwrap();
        if let Some(room) = rooms.get(room_id) {
            room.apply(op)?;
        }
        Ok(())
    }
}

/// A state room backed by loro CRDT.
pub struct StateRoom {
    id: String,
    // In a real implementation, this would be a loro::LoroDoc
    data: RwLock<Vec<u8>>,
}

impl StateRoom {
    /// Create a new state room.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            data: RwLock::new(Vec::new()),
        }
    }

    /// Export the current state as bytes.
    pub fn export(&self) -> Vec<u8> {
        self.data.read().unwrap().clone()
    }

    /// Apply a CRDT operation.
    pub fn apply(&self, op: &[u8]) -> anyhow::Result<()> {
        // In a real implementation, this would apply a loro operation
        debug!(room_id = %self.id, op_size = op.len(), "Applying CRDT operation");
        Ok(())
    }
}
