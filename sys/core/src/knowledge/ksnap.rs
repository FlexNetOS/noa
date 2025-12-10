use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSnapshot {
    pub id: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub payload: serde_json::Value,
}

/// KSNAP_CAP: Snapshot and restore for knowledge capsules.
pub struct SnapshotStore {
    root: PathBuf,
    snapshots: HashMap<String, KnowledgeSnapshot>,
}

impl SnapshotStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            snapshots: HashMap::new(),
        }
    }

    pub fn create(&mut self, description: impl Into<String>, payload: serde_json::Value) -> Result<KnowledgeSnapshot> {
        fs::create_dir_all(&self.root)?;
        let id = uuid::Uuid::new_v4().to_string();
        let snapshot = KnowledgeSnapshot {
            id: id.clone(),
            description: description.into(),
            created_at: Utc::now(),
            payload,
        };

        let path = self.root.join(format!("{}.json", id));
        fs::write(&path, serde_json::to_vec_pretty(&snapshot)?)?;
        self.snapshots.insert(id.clone(), snapshot.clone());
        Ok(snapshot)
    }

    pub fn restore(&self, id: &str) -> Option<KnowledgeSnapshot> {
        self.snapshots.get(id).cloned()
    }

    pub fn list(&self) -> Vec<KnowledgeSnapshot> {
        self.snapshots.values().cloned().collect()
    }
}

impl Default for SnapshotStore {
    fn default() -> Self {
        Self::new(PathBuf::from("data/knowledge/snapshots"))
    }
}
