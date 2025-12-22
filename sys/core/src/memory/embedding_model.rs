//! Embedding Model Loader
//!
//! T135: Create embedding model loader (MiniLM-384 or BGE/E5)
//! §3.7: Total Memory Sovereignty

use crate::error::{NoaError, Result};
use std::collections::HashMap;

/// Embedding model for generating vector embeddings
pub struct EmbeddingModel {
    name: String,
    dimensions: usize,
    // Note: In a real implementation, this would hold the actual model
    // For now, we'll use a placeholder that can be replaced with Candle models
    _model_data: HashMap<String, String>,
}

impl EmbeddingModel {
    /// Load an embedding model by name
    pub async fn load(name: &str) -> Result<Self> {
        // Map model names to dimensions
        let dimensions = match name {
            "all-MiniLM-L6-v2" => 384,
            "BGE-small-en-v1.5" => 384,
            "BGE-base-en-v1.5" => 768,
            "BGE-large-en-v1.5" => 1024,
            "E5-small-v2" => 384,
            "E5-base-v2" => 768,
            "E5-large-v2" => 1024,
            _ => {
                return Err(NoaError::Validation(crate::error::ValidationError::new(
                    "model_name",
                    format!("Unknown model: {}", name),
                    "UNKNOWN_MODEL",
                )));
            }
        };

        // TODO: Load actual model using Candle
        // For now, return a placeholder
        Ok(Self {
            name: name.to_string(),
            dimensions,
            _model_data: HashMap::new(),
        })
    }

    /// Generate embedding for a single text
    pub async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        Ok(self.hashing_embed(text))
    }

    /// Generate embeddings for multiple texts (batch)
    pub async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        // NOTE: This is intentionally implemented without external model downloads.
        // It provides a stable, deterministic embedding suitable for local semantic search.
        Ok(texts.iter().map(|t| self.hashing_embed(t)).collect())
    }

    /// Deterministic, local embedding using the hashing trick.
    ///
    /// This is a practical, dependency-light embedding that:
    /// - avoids network/model downloads during `load()`
    /// - produces non-zero vectors for non-empty inputs
    /// - supports cosine similarity (L2-normalized)
    ///
    /// It is *not* intended to be a drop-in replacement for transformer embeddings.
    /// When Candle/fastembed integration is enabled, this can become a fallback.
    fn hashing_embed(&self, text: &str) -> Vec<f32> {
        let dims = self.dimensions.max(1);
        let mut v = vec![0.0f32; dims];

        let input = text.trim();
        if input.is_empty() {
            return v;
        }

        // Tokenize: simple whitespace split. (We keep it simple and fast.)
        // Each token is hashed into a bucket with a sign bit.
        for token in input.split_whitespace() {
            let token = token.trim();
            if token.is_empty() {
                continue;
            }

            // Include model name to avoid accidental collisions across model choices.
            let mut h = blake3::Hasher::new();
            h.update(self.name.as_bytes());
            h.update(b"\0");
            h.update(token.as_bytes());
            let out = h.finalize();
            let b = out.as_bytes();

            let idx = u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize % dims;
            let sign = if (b[4] & 1) == 0 { 1.0f32 } else { -1.0f32 };

            // Light weighting by token length to reduce the impact of extremely short tokens.
            let len = token.chars().count().max(1) as f32;
            let weight = (1.0f32 + len.ln()).min(3.0);

            v[idx] += sign * weight;
        }

        // L2 normalize for cosine similarity.
        let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in &mut v {
                *x /= norm;
            }
        }

        v
    }

    /// Get model dimensions
    pub fn dimensions(&self) -> usize {
        self.dimensions
    }

    /// Get model name
    pub fn name(&self) -> &str {
        &self.name
    }
}

