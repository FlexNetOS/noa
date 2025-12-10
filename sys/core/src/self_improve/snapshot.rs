use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotDescriptor {
    pub id: String,
    pub label: String,
    pub created_at: DateTime<Utc>,
    pub path: PathBuf,
}

/// Persists lightweight state prior to modification for rollback.
pub struct SnapshotManager {
    root: PathBuf,
}

impl SnapshotManager {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn create(&self, label: impl Into<String>, state: serde_json::Value) -> Result<SnapshotDescriptor> {
        fs::create_dir_all(&self.root)?;
        let id = uuid::Uuid::new_v4().to_string();
        let path = self.root.join(format!("{}.json", id));
        fs::write(&path, serde_json::to_vec_pretty(&state)?)?;

        Ok(SnapshotDescriptor {
            id,
            label: label.into(),
            created_at: Utc::now(),
            path,
        })
    }

    pub fn load(&self, id: &str) -> Result<Option<serde_json::Value>> {
        let path = self.root.join(format!("{}.json", id));
        if path.exists() {
            let data = fs::read(&path)?;
            let value = serde_json::from_slice(&data)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

impl Default for SnapshotManager {
    fn default() -> Self {
        Self::new(PathBuf::from("data/self_improve/snapshots"))
    }
}
