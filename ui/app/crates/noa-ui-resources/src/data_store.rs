//! Data store for general data storage

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A data item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataItem {
    pub id: String,
    pub data_type: String,
    pub content: serde_json::Value,
    pub metadata: HashMap<String, String>,
}

/// General data store
pub struct DataStore {
    items: HashMap<String, DataItem>,
}

impl DataStore {
    /// Create a new DataStore
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Initialize the store
    pub async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// Cleanup resources
    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Store an item
    pub fn store(&mut self, item: DataItem) -> Result<()> {
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve an item by ID
    pub fn retrieve(&self, id: &str) -> Option<DataItem> {
        self.items.get(id).cloned()
    }

    /// List items, optionally filtered by type
    pub fn list(&self, data_type: Option<&str>) -> Vec<DataItem> {
        self.items
            .values()
            .filter(|item| data_type.map_or(true, |t| item.data_type == t))
            .cloned()
            .collect()
    }

    /// Remove an item
    pub fn remove(&mut self, id: &str) -> Option<DataItem> {
        self.items.remove(id)
    }
}

impl Default for DataStore {
    fn default() -> Self {
        Self::new()
    }
}
