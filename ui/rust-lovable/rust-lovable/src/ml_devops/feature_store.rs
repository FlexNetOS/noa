use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureGroup {
    pub id: String,
    pub name: String,
    pub description: String,
    pub entity: String,
    pub features: Vec<Feature>,
    pub online_store_config: OnlineStoreConfig,
    pub offline_store_config: OfflineStoreConfig,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub data_type: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineStoreConfig {
    pub store_type: String,
    pub connection_string: String,
    pub ttl_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineStoreConfig {
    pub store_type: String,
    pub location: String,
    pub format: String,
}

pub struct FeatureStore {
    feature_groups: HashMap<String, FeatureGroup>,
    online_store: HashMap<String, HashMap<String, HashMap<String, serde_json::Value>>>,
}

impl FeatureStore {
    pub fn new() -> Self {
        Self {
            feature_groups: HashMap::new(),
            online_store: HashMap::new(),
        }
    }
    
    pub async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }
    
    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
    
    pub async fn create_feature_group(&mut self, group: FeatureGroup) -> Result<String> {
        let id = group.id.clone();
        self.feature_groups.insert(id.clone(), group);
        Ok(id)
    }
    
    pub async fn get_feature_group(&self, group_id: &str) -> Option<FeatureGroup> {
        self.feature_groups.get(group_id).cloned()
    }
    
    pub async fn ingest_features(&mut self, group_id: &str, features: Vec<HashMap<String, serde_json::Value>>) -> Result<()> {
        if let Some(group) = self.feature_groups.get(group_id) {
            // Store features in online store
            for feature_row in features {
                if let Some(entity_id) = feature_row.get(&group.entity) {
                    let entity_str = entity_id.to_string();
                    
                    let group_store = self.online_store
                        .entry(group_id.to_string())
                        .or_default();
                    
                    group_store.insert(entity_str, feature_row);
                }
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Feature group not found"))
        }
    }
    
    pub async fn get_online_features(&self, group_id: &str, entity_id: &str) -> Option<HashMap<String, serde_json::Value>> {
        self.online_store
            .get(group_id)
            .and_then(|group| group.get(entity_id))
            .cloned()
    }
    
    pub async fn get_feature_group_count(&self) -> usize {
        self.feature_groups.len()
    }
}