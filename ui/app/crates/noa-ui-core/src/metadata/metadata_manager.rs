//! Metadata manager for coordinating metadata operations

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{EntityType, RichMetadata, Relationship};

/// Metadata manager for coordinating metadata operations
pub struct MetadataManager {
    metadata_store: Arc<RwLock<HashMap<String, RichMetadata>>>,
    entity_index: Arc<RwLock<HashMap<EntityType, Vec<String>>>>,
    relationship_index: Arc<RwLock<HashMap<String, Vec<Relationship>>>>,
}

impl MetadataManager {
    /// Create a new MetadataManager
    pub fn new() -> Self {
        Self {
            metadata_store: Arc::new(RwLock::new(HashMap::new())),
            entity_index: Arc::new(RwLock::new(HashMap::new())),
            relationship_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize the manager
    pub async fn initialize(&self) -> Result<()> {
        Ok(())
    }

    /// Store metadata for an entity
    pub async fn store_metadata(&self, metadata: RichMetadata) -> Result<()> {
        let id = metadata.id.clone();
        let entity_type = metadata.entity_type;
        let relationships = metadata.relationships.clone();

        // Store the metadata
        {
            let mut store = self.metadata_store.write().await;
            store.insert(id.clone(), metadata);
        }

        // Update entity index
        {
            let mut index = self.entity_index.write().await;
            index.entry(entity_type).or_default().push(id.clone());
        }

        // Update relationship index
        {
            let mut rel_index = self.relationship_index.write().await;
            rel_index.insert(id, relationships);
        }

        Ok(())
    }

    /// Retrieve metadata by ID
    pub async fn get_metadata(&self, id: &str) -> Option<RichMetadata> {
        let store = self.metadata_store.read().await;
        store.get(id).cloned()
    }

    /// Get all metadata for an entity type
    pub async fn get_by_type(&self, entity_type: EntityType) -> Vec<RichMetadata> {
        let index = self.entity_index.read().await;
        let store = self.metadata_store.read().await;

        index
            .get(&entity_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| store.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get relationships for an entity
    pub async fn get_relationships(&self, entity_id: &str) -> Vec<Relationship> {
        let rel_index = self.relationship_index.read().await;
        rel_index.get(entity_id).cloned().unwrap_or_default()
    }

    /// Update metadata
    pub async fn update_metadata(&self, id: &str, metadata: RichMetadata) -> Result<()> {
        let mut store = self.metadata_store.write().await;
        if store.contains_key(id) {
            store.insert(id.to_string(), metadata);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Metadata not found: {}", id))
        }
    }

    /// Delete metadata
    pub async fn delete_metadata(&self, id: &str) -> Option<RichMetadata> {
        let mut store = self.metadata_store.write().await;
        let metadata = store.remove(id)?;

        // Update entity index
        {
            let mut index = self.entity_index.write().await;
            if let Some(ids) = index.get_mut(&metadata.entity_type) {
                ids.retain(|i| i != id);
            }
        }

        // Remove relationships
        {
            let mut rel_index = self.relationship_index.write().await;
            rel_index.remove(id);
        }

        Some(metadata)
    }

    /// Search metadata by query
    pub async fn search(&self, query: &str) -> Vec<RichMetadata> {
        let store = self.metadata_store.read().await;
        let query_lower = query.to_lowercase();

        store
            .values()
            .filter(|m| m.id.to_lowercase().contains(&query_lower))
            .cloned()
            .collect()
    }

    /// Get lineage upstream entities
    pub async fn get_upstream(&self, entity_id: &str) -> Vec<RichMetadata> {
        let store = self.metadata_store.read().await;
        
        if let Some(metadata) = store.get(entity_id) {
            metadata
                .lineage
                .upstream_entities
                .iter()
                .filter_map(|id| store.get(id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get lineage downstream entities
    pub async fn get_downstream(&self, entity_id: &str) -> Vec<RichMetadata> {
        let store = self.metadata_store.read().await;
        
        if let Some(metadata) = store.get(entity_id) {
            metadata
                .lineage
                .downstream_entities
                .iter()
                .filter_map(|id| store.get(id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Count entities by type
    pub async fn count_by_type(&self) -> HashMap<EntityType, usize> {
        let index = self.entity_index.read().await;
        index.iter().map(|(k, v)| (*k, v.len())).collect()
    }
}

impl Default for MetadataManager {
    fn default() -> Self {
        Self::new()
    }
}
