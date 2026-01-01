use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataItem {
    pub id: String,
    pub data_type: String,
    pub content: serde_json::Value,
    pub metadata: HashMap<String, String>,
}

pub struct DataStore {
    items: HashMap<String, DataItem>,
}

impl DataStore {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn store(&mut self, item: DataItem) -> Result<()> {
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    pub fn retrieve(&self, id: &str) -> Option<DataItem> {
        self.items.get(id).cloned()
    }

    pub fn list(&self, data_type: Option<&str>) -> Vec<DataItem> {
        self.items.values()
            .filter(|item| data_type.map_or(true, |t| item.data_type == t))
            .cloned()
            .collect()
    }
}