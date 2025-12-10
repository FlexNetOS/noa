//! Embedder service

use crate::model::{Embedding, EmbeddingModel};
use noa_common::{NoaError, Result};

/// Embedding service for generating text embeddings
pub struct EmbedderService {
    model: EmbeddingModel,
}

impl EmbedderService {
    /// Create a new embedder service
    pub fn new(model: EmbeddingModel) -> Self {
        Self { model }
    }

    /// Generate embeddings for texts
    pub async fn embed(&self, texts: &[String]) -> Result<Vec<Embedding>> {
        // Placeholder implementation
        // Will use fastembed for actual embedding generation
        let embeddings = texts
            .iter()
            .map(|text| Embedding {
                text: text.clone(),
                vector: vec![0.0; self.model.dimensions()],
                model: self.model,
            })
            .collect();

        Ok(embeddings)
    }

    /// Generate embedding for a single text
    pub async fn embed_single(&self, text: &str) -> Result<Embedding> {
        let texts = vec![text.to_string()];
        let mut embeddings = self.embed(&texts).await?;
        embeddings
            .pop()
            .ok_or_else(|| NoaError::Embedding("Failed to generate embedding".into()))
    }

    /// Get the current model
    pub fn model(&self) -> EmbeddingModel {
        self.model
    }
}

impl Default for EmbedderService {
    fn default() -> Self {
        Self::new(EmbeddingModel::default())
    }
}
