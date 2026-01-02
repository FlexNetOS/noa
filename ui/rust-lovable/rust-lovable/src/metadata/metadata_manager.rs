use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::metadata::{EntityType, MetadataQuery, MetadataValidationResult, RichMetadata};

pub struct MetadataManager {
    metadata_store: Arc<RwLock<HashMap<String, RichMetadata>>>,
    entity_index: Arc<RwLock<HashMap<EntityType, Vec<String>>>>,
    relationship_index: Arc<RwLock<HashMap<String, Vec<String>>>>,
    schema_registry: Arc<RwLock<SchemaRegistry>>,
    lineage_tracker: Arc<RwLock<LineageTracker>>,
    provenance_tracker: Arc<RwLock<ProvenanceTracker>>,
}

pub struct SchemaRegistry {
    schemas: HashMap<String, crate::metadata::schemas::SchemaDefinition>,
    version_history: HashMap<String, Vec<crate::metadata::schemas::SchemaDefinition>>,
}

pub struct LineageTracker {
    upstream_graph: HashMap<String, Vec<String>>,
    downstream_graph: HashMap<String, Vec<String>>,
    impact_cache: HashMap<String, crate::metadata::ImpactAnalysis>,
}

pub struct ProvenanceTracker {
    transformation_chains: HashMap<String, Vec<crate::metadata::TransformationStep>>,
    source_mapping: HashMap<String, String>,
}

impl MetadataManager {
    pub fn new() -> Self {
        Self {
            metadata_store: Arc::new(RwLock::new(HashMap::new())),
            entity_index: Arc::new(RwLock::new(HashMap::new())),
            relationship_index: Arc::new(RwLock::new(HashMap::new())),
            schema_registry: Arc::new(RwLock::new(SchemaRegistry::new())),
            lineage_tracker: Arc::new(RwLock::new(LineageTracker::new())),
            provenance_tracker: Arc::new(RwLock::new(ProvenanceTracker::new())),
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        // Register built-in schemas
        self.register_builtin_schemas().await?;
        Ok(())
    }

    pub async fn cleanup(&self) -> Result<()> {
        // Persist metadata to storage
        self.persist_metadata().await?;
        Ok(())
    }

    pub async fn create_metadata(
        &self,
        entity_type: EntityType,
        created_by: String,
    ) -> Result<String> {
        let metadata = RichMetadata::new(entity_type, created_by);
        let id = metadata.id.clone();

        // Store metadata
        self.metadata_store
            .write()
            .await
            .insert(id.clone(), metadata);

        // Update entity index
        self.entity_index
            .write()
            .await
            .entry(entity_type)
            .or_default()
            .push(id.clone());

        Ok(id)
    }

    pub async fn get_metadata(&self, id: &str) -> Option<RichMetadata> {
        self.metadata_store.read().await.get(id).cloned()
    }

    pub async fn update_metadata(&self, id: &str, updates: MetadataUpdates) -> Result<()> {
        let mut store = self.metadata_store.write().await;

        if let Some(metadata) = store.get_mut(id) {
            metadata.updated_at = chrono::Utc::now();

            // Apply updates based on type
            if let Some(quality) = updates.quality_metrics {
                metadata.update_quality_metrics(quality);
            }

            if let Some(relationships) = updates.relationships {
                for relationship in relationships {
                    metadata.add_relationship(relationship);
                }
            }

            Ok(())
        } else {
            Err(anyhow::anyhow!("Metadata not found: {}", id))
        }
    }

    pub async fn delete_metadata(&self, id: &str) -> Result<()> {
        let mut store = self.metadata_store.write().await;
        let mut entity_index = self.entity_index.write().await;
        let mut relationship_index = self.relationship_index.write().await;

        if let Some(metadata) = store.remove(id) {
            // Remove from entity index
            if let Some(entity_ids) = entity_index.get_mut(&metadata.entity_type) {
                entity_ids.retain(|entity_id| entity_id != id);
            }

            // Remove from relationship index
            relationship_index.remove(id);

            Ok(())
        } else {
            Err(anyhow::anyhow!("Metadata not found: {}", id))
        }
    }

    pub async fn query_metadata(&self, query: MetadataQuery) -> Vec<RichMetadata> {
        let store = self.metadata_store.read().await;
        let mut results = Vec::new();

        // Start with all metadata if no entity type specified
        let candidate_ids = if let Some(entity_type) = &query.entity_type {
            self.entity_index
                .read()
                .await
                .get(entity_type)
                .cloned()
                .unwrap_or_default()
        } else {
            store.keys().cloned().collect()
        };

        // Apply filters
        for id in candidate_ids {
            if let Some(metadata) = store.get(&id) {
                if self.matches_query(metadata, &query) {
                    results.push(metadata.clone());
                }
            }
        }

        results
    }

    pub async fn validate_metadata(&self, id: &str) -> Result<MetadataValidationResult> {
        let store = self.metadata_store.read().await;

        if let Some(metadata) = store.get(id) {
            let mut result = metadata.validate();

            // Add schema-specific validation
            let schema_registry = self.schema_registry.read().await;
            for schema in &metadata.schemas {
                if let Some(schema_def) = schema_registry.get_schema(&schema.name) {
                    // Validate against schema
                    // Implementation would depend on specific validation requirements
                }
            }

            Ok(result)
        } else {
            Err(anyhow::anyhow!("Metadata not found: {}", id))
        }
    }

    pub async fn add_schema(
        &self,
        id: &str,
        schema: crate::metadata::schemas::SchemaDefinition,
    ) -> Result<()> {
        let mut store = self.metadata_store.write().await;
        let mut schema_registry = self.schema_registry.write().await;

        if let Some(metadata) = store.get_mut(id) {
            metadata.add_schema(schema.clone());
            schema_registry.register_schema(schema)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Metadata not found: {}", id))
        }
    }

    pub async fn add_data_table(
        &self,
        id: &str,
        table: crate::metadata::data_tables::DataTable,
    ) -> Result<()> {
        let mut store = self.metadata_store.write().await;

        if let Some(metadata) = store.get_mut(id) {
            metadata.add_data_table(table);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Metadata not found: {}", id))
        }
    }

    pub async fn track_lineage(
        &self,
        entity_id: &str,
        upstream_ids: Vec<String>,
        downstream_ids: Vec<String>,
    ) -> Result<()> {
        let mut lineage_tracker = self.lineage_tracker.write().await;
        lineage_tracker.track_lineage(entity_id, upstream_ids, downstream_ids);
        Ok(())
    }

    pub async fn get_impact_analysis(
        &self,
        entity_id: &str,
    ) -> Option<crate::metadata::ImpactAnalysis> {
        let mut lineage_tracker = self.lineage_tracker.write().await;
        lineage_tracker.get_impact_analysis(entity_id)
    }

    pub async fn get_entity_statistics(&self, entity_type: EntityType) -> EntityStatistics {
        let entity_index = self.entity_index.read().await;
        let store = self.metadata_store.read().await;

        let entity_ids = entity_index.get(&entity_type).cloned().unwrap_or_default();
        let mut total_quality = 0.0;
        let mut count = 0;

        for id in &entity_ids {
            if let Some(metadata) = store.get(id) {
                total_quality += metadata.quality.overall_score;
                count += 1;
            }
        }

        EntityStatistics {
            total_entities: entity_ids.len(),
            average_quality: if count > 0 {
                total_quality / count as f64
            } else {
                0.0
            },
            entity_ids,
        }
    }

    fn matches_query(&self, metadata: &RichMetadata, query: &MetadataQuery) -> bool {
        // Check schema name filter
        if let Some(schema_name) = &query.schema_name {
            if !metadata.schemas.iter().any(|s| &s.name == schema_name) {
                return false;
            }
        }

        // Check tags filter
        if !query.tags.is_empty() {
            // For simplicity, check if any tag matches
            // In a real implementation, this would be more sophisticated
            let has_matching_tag = metadata
                .data_tables
                .iter()
                .flat_map(|t| &t.metadata.tags)
                .chain(
                    metadata
                        .relationships
                        .iter()
                        .flat_map(|r| r.properties.keys()),
                )
                .any(|tag| query.tags.contains(tag));

            if !has_matching_tag {
                return false;
            }
        }

        // Check date range filter
        if let Some(date_range) = &query.date_range {
            if metadata.created_at < date_range.start || metadata.created_at > date_range.end {
                return false;
            }
        }

        // Check quality threshold
        if let Some(threshold) = query.quality_threshold {
            if metadata.quality.overall_score < threshold {
                return false;
            }
        }

        // Check relationship filters
        for relationship_query in &query.relationships {
            let has_matching_relationship = metadata.relationships.iter().any(|r| {
                r.target_id == relationship_query.target_id
                    && relationship_query
                        .relationship_type
                        .as_ref()
                        .map_or(true, |t| &r.relationship_type == t)
            });

            if !has_matching_relationship {
                return false;
            }
        }

        // Check provenance filter
        if let Some(provenance_query) = &query.provenance {
            if let Some(created_by) = &provenance_query.created_by {
                if &metadata.provenance.created_by != created_by {
                    return false;
                }
            }

            if let Some(source_system) = &provenance_query.source_system {
                if &metadata.provenance.source_system != source_system {
                    return false;
                }
            }
        }

        true
    }

    async fn register_builtin_schemas(&self) -> Result<()> {
        let mut schema_registry = self.schema_registry.write().await;

        // Register built-in schemas
        let prompt_schema = crate::metadata::schemas::create_prompt_schema();
        let embedding_schema = crate::metadata::schemas::create_embedding_schema();
        let skill_schema = crate::metadata::schemas::create_skill_schema();

        schema_registry.register_schema(prompt_schema)?;
        schema_registry.register_schema(embedding_schema)?;
        schema_registry.register_schema(skill_schema)?;

        Ok(())
    }

    async fn persist_metadata(&self) -> Result<()> {
        // Implementation would persist metadata to configured storage backend
        Ok(())
    }
}

pub struct MetadataUpdates {
    pub quality_metrics: Option<crate::metadata::QualityMetrics>,
    pub relationships: Option<Vec<crate::metadata::Relationship>>,
    pub compliance: Option<crate::metadata::ComplianceInfo>,
}

#[derive(Debug, Clone)]
pub struct EntityStatistics {
    pub total_entities: usize,
    pub average_quality: f64,
    pub entity_ids: Vec<String>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
            version_history: HashMap::new(),
        }
    }

    pub fn register_schema(
        &mut self,
        schema: crate::metadata::schemas::SchemaDefinition,
    ) -> Result<()> {
        let key = format!("{}:{}", schema.name, schema.version);

        // Store schema
        self.schemas.insert(key.clone(), schema.clone());

        // Update version history
        self.version_history
            .entry(schema.name.clone())
            .or_default()
            .push(schema);

        Ok(())
    }

    pub fn get_schema(&self, name: &str) -> Option<crate::metadata::schemas::SchemaDefinition> {
        // Return the latest version
        self.version_history
            .get(name)
            .and_then(|versions| versions.last().cloned())
    }
}

impl LineageTracker {
    pub fn new() -> Self {
        Self {
            upstream_graph: HashMap::new(),
            downstream_graph: HashMap::new(),
            impact_cache: HashMap::new(),
        }
    }

    pub fn track_lineage(
        &mut self,
        entity_id: &str,
        upstream_ids: Vec<String>,
        downstream_ids: Vec<String>,
    ) {
        // Update upstream graph
        self.upstream_graph
            .insert(entity_id.to_string(), upstream_ids.clone());

        // Update downstream graph
        for upstream_id in &upstream_ids {
            self.downstream_graph
                .entry(upstream_id.clone())
                .or_default()
                .push(entity_id.to_string());
        }

        // Add downstream entities
        self.downstream_graph
            .insert(entity_id.to_string(), downstream_ids);

        // Clear impact cache for affected entities
        self.impact_cache.remove(entity_id);
        for upstream_id in &upstream_ids {
            self.impact_cache.remove(upstream_id);
        }
    }

    pub fn get_impact_analysis(
        &mut self,
        entity_id: &str,
    ) -> Option<crate::metadata::ImpactAnalysis> {
        if let Some(cached) = self.impact_cache.get(entity_id) {
            return Some(cached.clone());
        }

        // Calculate impact analysis
        let affected_entities = self.get_affected_entities(entity_id);
        let impact_score = self.calculate_impact_score(entity_id);
        let critical_path = self.find_critical_path(entity_id);

        let analysis = crate::metadata::ImpactAnalysis {
            affected_entities,
            impact_score,
            critical_path,
        };

        self.impact_cache
            .insert(entity_id.to_string(), analysis.clone());
        Some(analysis)
    }

    fn get_affected_entities(&self, entity_id: &str) -> Vec<String> {
        let mut affected = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();

        queue.push_back(entity_id.to_string());
        visited.insert(entity_id.to_string());

        while let Some(current) = queue.pop_front() {
            if let Some(downstream) = self.downstream_graph.get(&current) {
                for entity in downstream {
                    if !visited.contains(entity) {
                        visited.insert(entity.clone());
                        queue.push_back(entity.clone());
                        affected.push(entity.clone());
                    }
                }
            }
        }

        affected
    }

    fn calculate_impact_score(&self, entity_id: &str) -> f64 {
        // Simple impact score based on number of downstream dependencies
        let affected_count = self.get_affected_entities(entity_id).len() as f64;
        (affected_count / 10.0).min(1.0) // Normalize to 0-1
    }

    fn find_critical_path(&self, entity_id: &str) -> Vec<String> {
        // Find the longest dependency chain
        let mut longest_path = Vec::new();
        let mut current_path = vec![entity_id.to_string()];

        self.find_longest_path(entity_id, &mut current_path, &mut longest_path);

        longest_path
    }

    fn find_longest_path(
        &self,
        entity_id: &str,
        current_path: &mut Vec<String>,
        longest_path: &mut Vec<String>,
    ) {
        if let Some(downstream) = self.downstream_graph.get(entity_id) {
            for entity in downstream {
                current_path.push(entity.clone());

                if current_path.len() > longest_path.len() {
                    longest_path.clone_from(current_path);
                }

                self.find_longest_path(entity, current_path, longest_path);

                current_path.pop();
            }
        }
    }
}

impl ProvenanceTracker {
    pub fn new() -> Self {
        Self {
            transformation_chains: HashMap::new(),
            source_mapping: HashMap::new(),
        }
    }

    pub fn track_transformation(
        &mut self,
        entity_id: String,
        steps: Vec<crate::metadata::TransformationStep>,
    ) {
        self.transformation_chains.insert(entity_id, steps);
    }

    pub fn map_source(&mut self, entity_id: String, source_id: String) {
        self.source_mapping.insert(entity_id, source_id);
    }
}
