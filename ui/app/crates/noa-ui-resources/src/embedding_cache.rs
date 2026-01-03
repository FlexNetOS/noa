//! Embedding cache for vector embeddings

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// An embedding vector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    pub id: String,
    pub vector: Vec<f32>,
    pub metadata: EmbeddingMetadata,
    pub created_at: DateTime<Utc>,
}

/// Metadata for an embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingMetadata {
    pub source_type: String,
    pub source_id: String,
    pub model: String,
    pub dimensions: usize,
    pub token_count: usize,
    pub language: Option<String>,
    pub tags: Vec<String>,
}

/// A cached embedding with access tracking
#[derive(Debug, Clone)]
pub struct CachedEmbedding {
    pub embedding: Embedding,
    pub last_accessed: Instant,
    pub access_count: u64,
    pub compressed_size: usize,
}

/// Cache for embeddings
pub struct EmbeddingCache {
    embeddings: HashMap<String, CachedEmbedding>,
    source_index: HashMap<String, Vec<String>>,
    model_index: HashMap<String, Vec<String>>,
    max_size: usize,
    ttl: Duration,
    compression_enabled: bool,
}

impl EmbeddingCache {
    /// Create a new EmbeddingCache
    pub fn new() -> Self {
        Self {
            embeddings: HashMap::new(),
            source_index: HashMap::new(),
            model_index: HashMap::new(),
            max_size: 100000,
            ttl: Duration::from_secs(7200), // 2 hours
            compression_enabled: true,
        }
    }

    /// Initialize the cache
    pub async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// Cleanup expired entries
    pub async fn cleanup(&mut self) -> Result<()> {
        self.remove_expired();
        Ok(())
    }

    /// Add an embedding to the cache
    pub fn add_embedding(&mut self, embedding: Embedding) -> Result<()> {
        let id = embedding.id.clone();
        let compressed_size = if self.compression_enabled {
            self.compress_embedding(&embedding).len()
        } else {
            embedding.vector.len() * std::mem::size_of::<f32>()
        };

        let cached_embedding = CachedEmbedding {
            embedding: embedding.clone(),
            last_accessed: Instant::now(),
            access_count: 0,
            compressed_size,
        };

        // Update indices
        self.update_indices(&embedding, true);

        // Add to cache
        self.embeddings.insert(id, cached_embedding);

        // Evict if necessary
        self.evict_if_needed();

        Ok(())
    }

    /// Get an embedding by ID
    pub fn get_embedding(&mut self, id: &str) -> Option<Embedding> {
        if let Some(cached) = self.embeddings.get_mut(id) {
            cached.last_accessed = Instant::now();
            cached.access_count += 1;
            Some(cached.embedding.clone())
        } else {
            None
        }
    }

    /// Get embeddings by source
    pub fn get_by_source(&self, source_id: &str) -> Vec<Embedding> {
        self.source_index
            .get(source_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.embeddings.get(id).map(|c| c.embedding.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get embeddings by model
    pub fn get_by_model(&self, model: &str) -> Vec<Embedding> {
        self.model_index
            .get(model)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.embeddings.get(id).map(|c| c.embedding.clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Remove an embedding
    pub fn remove_embedding(&mut self, id: &str) -> Option<Embedding> {
        if let Some(cached) = self.embeddings.remove(id) {
            self.update_indices(&cached.embedding, false);
            Some(cached.embedding)
        } else {
            None
        }
    }

    fn compress_embedding(&self, embedding: &Embedding) -> Vec<u8> {
        // Simple placeholder for compression
        let bytes: Vec<u8> = embedding
            .vector
            .iter()
            .flat_map(|f| f.to_le_bytes())
            .collect();
        bytes
    }

    fn update_indices(&mut self, embedding: &Embedding, add: bool) {
        let id = &embedding.id;

        if add {
            self.source_index
                .entry(embedding.metadata.source_id.clone())
                .or_default()
                .push(id.clone());

            self.model_index
                .entry(embedding.metadata.model.clone())
                .or_default()
                .push(id.clone());
        } else {
            if let Some(ids) = self.source_index.get_mut(&embedding.metadata.source_id) {
                ids.retain(|i| i != id);
            }
            if let Some(ids) = self.model_index.get_mut(&embedding.metadata.model) {
                ids.retain(|i| i != id);
            }
        }
    }

    fn remove_expired(&mut self) {
        let now = Instant::now();
        let expired: Vec<_> = self
            .embeddings
            .iter()
            .filter(|(_, cached)| now.duration_since(cached.last_accessed) > self.ttl)
            .map(|(id, _)| id.clone())
            .collect();

        for id in expired {
            self.remove_embedding(&id);
        }
    }

    fn evict_if_needed(&mut self) {
        while self.embeddings.len() > self.max_size {
            // Find least recently used
            if let Some(lru_id) = self
                .embeddings
                .iter()
                .min_by_key(|(_, cached)| cached.last_accessed)
                .map(|(id, _)| id.clone())
            {
                self.remove_embedding(&lru_id);
            } else {
                break;
            }
        }
    }
}

impl Default for EmbeddingCache {
    fn default() -> Self {
        Self::new()
    }
}
