use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::error::{NoaError, Result};

/// VHDX snapshot descriptor.
#[derive(Debug, Clone)]
pub struct VhdxSnapshot {
    pub id: String,
    pub parent_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub description: String,
}

/// Snapshot/rollback manager for VHDX stacks.
pub struct VhdxSnapshotManager {
    snapshots: HashMap<String, VhdxSnapshot>,
}

impl VhdxSnapshotManager {
    pub fn new() -> Self {
        Self {
            snapshots: HashMap::new(),
        }
    }

    pub fn create_snapshot(&mut self, parent_id: Option<String>, description: impl Into<String>) -> VhdxSnapshot {
        let snap = VhdxSnapshot {
            id: uuid::Uuid::new_v4().to_string(),
            parent_id,
            created_at: Utc::now(),
            description: description.into(),
        };
        self.snapshots.insert(snap.id.clone(), snap.clone());
        snap
    }

    pub fn rollback(&self, id: &str) -> Result<VhdxSnapshot> {
        self.snapshots
            .get(id)
            .cloned()
            .ok_or_else(|| NoaError::NotFound {
                resource: "vhdx_snapshot".to_string(),
                id: id.to_string(),
            })
    }

    pub fn list(&self) -> Vec<VhdxSnapshot> {
        self.snapshots.values().cloned().collect()
    }
}

impl Default for VhdxSnapshotManager {
    fn default() -> Self {
        Self::new()
    }
}
