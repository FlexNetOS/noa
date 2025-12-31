use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    pub id: String,
    pub vector: Vec<f32>,
    pub metadata: EmbeddingMetadata,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

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

#[derive(Debug, Clone)]
pub struct CachedEmbedding {
    pub embedding: Embedding,
    pub last_accessed: Instant,
    pub access_count: u64,
    pub compressed_size: usize,
}

pub struct EmbeddingCache {
    embeddings: HashMap<String, CachedEmbedding>,
    source_index: HashMap<String, Vec<String>>,
    model_index: HashMap<String, Vec<String>>,
    max_size: usize,
    ttl: Duration,
    compression_enabled: bool,
}

impl EmbeddingCache {
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

    pub async fn initialize(&mut self) -> Result<()> {
        // Initialize embedding cache
        Ok(())
    }

    pub async fn cleanup(&mut self) -> Result<()> {
        self.remove_expired();
        Ok(())
    }

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

    pub fn get_embedding(&mut self, id: &str) -> Option<Embedding> {
        if let Some(cached) = self.embeddings.get_mut(id) {
            cached.last_accessed = Instant::now();
            cached.access_count += 1;
            Some(cached.embedding.clone())
        } else {
            None
        }
    }

    pub fn get_embeddings_by_source(&mut self, source_type: &str, source_id: &str) -> Vec<Embedding> {
        let key = format!("{}:{}", source_type, source_id);
        if let Some(ids) = self.source_index.get(&key) {
            ids.iter()
                .filter_map(|id| self.get_embedding(id))
                .collect()
        } else {
            vec![]
        }
    }

    pub fn get_embeddings_by_model(&mut self, model: &str) -> Vec<Embedding> {
        if let Some(ids) = self.model_index.get(model) {
            ids.iter()
                .filter_map(|id| self.get_embedding(id))
                .collect()
        } else {
            vec![]
        }
    }

    pub fn similarity_search(&self, query_vector: &[f32], top_k: usize, min_similarity: f32) -> Vec<SearchResult> {
        let mut results = Vec::new();
        
        for cached in self.embeddings.values() {
            let similarity = self.cosine_similarity(query_vector, &cached.embedding.vector);
            if similarity >= min_similarity {
                results.push(SearchResult {
                    embedding: cached.embedding.clone(),
                    similarity,
                    access_count: cached.access_count,
                });
            }
        }
        
        // Sort by similarity descending
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        
        // Return top k
        results.into_iter().take(top_k).collect()
    }

    pub fn remove_embedding(&mut self, id: &str) -> Result<()> {
        if let Some(cached) = self.embeddings.remove(id) {
            // Remove from indices
            self.update_indices(&cached.embedding, false);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Embedding not found"))
        }
    }

    pub fn get_stats(&self) -> EmbeddingCacheStats {
        let total_embeddings = self.embeddings.len();
        let total_vectors: usize = self.embeddings.values()
            .map(|cached| cached.embedding.vector.len())
            .sum();
        let total_memory: usize = self.embeddings.values()
            .map(|cached| cached.compressed_size)
            .sum();
        
        EmbeddingCacheStats {
            total_embeddings,
            total_vectors,
            total_memory_bytes: total_memory,
            models: self.model_index.len(),
            sources: self.source_index.len(),
            compression_ratio: if self.compression_enabled {
                let original_size: usize = self.embeddings.values()
                    .map(|cached| cached.embedding.vector.len() * std::mem::size_of::<f32>())
                    .sum();
                original_size as f64 / total_memory as f64
            } else {
                1.0
            },
        }
    }

    fn update_indices(&mut self, embedding: &Embedding, add: bool) {
        let id = &embedding.id;
        
        // Source index
        let source_key = format!("{}:{}", embedding.metadata.source_type, embedding.metadata.source_id);
        if add {
            self.source_index
                .entry(source_key)
                .or_default()
                .push(id.clone());
        } else {
            if let Some(index) = self.source_index.get_mut(&source_key) {
                index.retain(|embedding_id| embedding_id != id);
            }
        }
        
        // Model index
        if add {
            self.model_index
                .entry(embedding.metadata.model.clone())
                .or_default()
                .push(id.clone());
        } else {
            if let Some(index) = self.model_index.get_mut(&embedding.metadata.model) {
                index.retain(|embedding_id| embedding_id != id);
            }
        }
    }

    fn remove_expired(&mut self) {
        let now = Instant::now();
        let expired_ids: Vec<String> = self.embeddings
            .iter()
            .filter(|(_, cached)| now.duration_since(cached.last_accessed) > self.ttl)
            .map(|(id, _)| id.clone())
            .collect();
        
        for id in expired_ids {
            let _ = self.remove_embedding(&id);
        }
    }

    fn evict_if_needed(&mut self) {
        if self.embeddings.len() > self.max_size {
            // Find the least recently used embedding
            let lru_id = self.embeddings
                .iter()
                .min_by_key(|(_, cached)| cached.last_accessed)
                .map(|(id, _)| id.clone());
            
            if let Some(id) = lru_id {
                let _ = self.remove_embedding(&id);
            }
        }
    }

    fn compress_embedding(&self, embedding: &Embedding) -> Vec<u8> {
        // Simple quantization for demonstration
        // In production, use proper compression algorithms
        let quantized: Vec<u8> = embedding.vector.iter()
            .map(|&val| ((val + 1.0) * 127.5).clamp(0.0, 255.0) as u8)
            .collect();
        quantized
    }

    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }
        
        let dot_product: f32 = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|&x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|&x| x * x).sum::<f32>().sqrt();
        
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub embedding: Embedding,
    pub similarity: f32,
    pub access_count: u64,
}

#[derive(Debug, Clone)]
pub struct EmbeddingCacheStats {
    pub total_embeddings: usize,
    pub total_vectors: usize,
    pub total_memory_bytes: usize,
    pub models: usize,
    pub sources: usize,
    pub compression_ratio: f64,
}

impl Embedding {
    pub fn new(
        vector: Vec<f32>,
        source_type: String,
        source_id: String,
        model: String,
        tags: Vec<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            vector,
            metadata: EmbeddingMetadata {
                source_type,
                source_id,
                model: model.clone(),
                dimensions: vector.len(),
                token_count: 0,
                language: None,
                tags,
            },
            created_at: chrono::Utc::now(),
        }
    }
}