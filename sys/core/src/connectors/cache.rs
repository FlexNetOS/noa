use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::error::{NoaError, Result};

use super::ConnectorState;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    state: ConnectorState,
    cached_at: DateTime<Utc>,
}

/// Simple disk-backed cache for connector state and metadata
pub struct ConnectorCache {
    path: PathBuf,
    entries: std::sync::Mutex<HashMap<String, CacheEntry>>,
}

impl ConnectorCache {
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let path = path.unwrap_or_else(|| PathBuf::from("cache/connectors/cache.json"));
        let entries = if path.exists() {
            let content = fs::read_to_string(&path).map_err(NoaError::from)?;
            serde_json::from_str::<HashMap<String, CacheEntry>>(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };

        Ok(Self {
            path,
            entries: std::sync::Mutex::new(entries),
        })
    }

    pub fn get(&self, name: &str, max_age: Duration) -> Option<ConnectorState> {
        let guard = self.entries.lock().ok()?;
        guard.get(name).and_then(|entry| {
            if Utc::now() - entry.cached_at <= max_age {
                Some(entry.state.clone())
            } else {
                None
            }
        })
    }

    pub fn store(&self, state: &ConnectorState) -> Result<()> {
        let mut guard = self.entries.lock().map_err(|_| NoaError::Internal {
            message: "Connector cache poisoned".to_string(),
            source: None,
        })?;

        guard.insert(
            state.name.clone(),
            CacheEntry {
                state: state.clone(),
                cached_at: Utc::now(),
            },
        );
        drop(guard);
        self.persist()
    }

    fn persist(&self) -> Result<()> {
        let guard = self.entries.lock().map_err(|_| NoaError::Internal {
            message: "Connector cache poisoned".to_string(),
            source: None,
        })?;

        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(NoaError::from)?;
        }

        let serialized = serde_json::to_string_pretty(&*guard)
            .map_err(|e| NoaError::Serialization(e.to_string()))?;
        fs::write(&self.path, serialized).map_err(NoaError::from)
    }
}
