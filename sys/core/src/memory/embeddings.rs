//! Embedding Generation
//!
//! T134: Implement embedding generation with Candle
//! T138: Implement batch embedding requests
//! §3.7: Total Memory Sovereignty

use crate::error::Result;
use crate::memory::cache::EmbeddingCache;
use crate::memory::embedding_model::EmbeddingModel;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Embedding generator using Candle models
pub struct EmbeddingGenerator {
    model: Arc<RwLock<EmbeddingModel>>,
    cache: Arc<EmbeddingCache>,
}

impl EmbeddingGenerator {
    /// Create a new embedding generator
    pub async fn new(model_name: &str) -> Result<Self> {
        let model = EmbeddingModel::load(model_name).await?;
        let cache = EmbeddingCache::new();

        Ok(Self {
            model: Arc::new(RwLock::new(model)),
            cache: Arc::new(cache),
        })
    }

    /// Generate embedding for a single text
    pub async fn generate(&self, text: &str) -> Result<Vec<f32>> {
        // Check cache first
        if let Some(cached) = self.cache.get(text) {
            return Ok(cached);
        }

        // Generate embedding
        let model = self.model.read().await;
        let embedding = model.embed(text).await?;

        // Cache the result
        self.cache.put(text, embedding.clone());

        Ok(embedding)
    }

    /// Generate embeddings for multiple texts (batch processing)
    pub async fn generate_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let mut results = Vec::with_capacity(texts.len());
        let mut uncached_texts = Vec::new();
        let mut uncached_indices = Vec::new();

        // Check cache for all texts
        for (idx, text) in texts.iter().enumerate() {
            if let Some(cached) = self.cache.get(text) {
                results.push((idx, cached));
            } else {
                uncached_texts.push(text.clone());
                uncached_indices.push(idx);
            }
        }

        // Generate embeddings for uncached texts
        if !uncached_texts.is_empty() {
            let model = self.model.read().await;
            let embeddings = model.embed_batch(&uncached_texts).await?;

            // Store results and cache them
            for (i, embedding) in embeddings.iter().enumerate() {
                let original_idx = uncached_indices[i];
                let text = &uncached_texts[i];

                self.cache.put(text, embedding.clone());
                results.push((original_idx, embedding.clone()));
            }
        }

        // Sort by original index and return embeddings only
        results.sort_by_key(|(idx, _)| *idx);
        Ok(results.into_iter().map(|(_, emb)| emb).collect())
    }

    /// Get model dimensions
    pub async fn dimensions(&self) -> usize {
        let model = self.model.read().await;
        model.dimensions()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_embedding_generator() {
        // Note: This test would require a model to be loaded
        // For now, we'll just test the structure
        let generator = EmbeddingGenerator::new("all-MiniLM-L6-v2")
            .await
            .expect("Failed to create generator");

        assert_eq!(generator.dimensions().await, 384);
    }
}
