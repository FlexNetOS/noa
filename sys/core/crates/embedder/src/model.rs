//! Embedding model types

use serde::{Deserialize, Serialize};

/// Supported embedding models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EmbeddingModel {
    /// All-MiniLM-L6-v2 (384 dimensions)
    AllMiniLmL6V2,
    /// BGE-Small-EN-v1.5 (384 dimensions)
    BgeSmallEnV15,
    /// E5-Small-v2 (384 dimensions)
    E5SmallV2,
}

impl Default for EmbeddingModel {
    fn default() -> Self {
        Self::AllMiniLmL6V2
    }
}

impl EmbeddingModel {
    /// Get the dimension count for this model
    pub fn dimensions(&self) -> usize {
        match self {
            Self::AllMiniLmL6V2 => 384,
            Self::BgeSmallEnV15 => 384,
            Self::E5SmallV2 => 384,
        }
    }

    /// Get the model name for fastembed
    pub fn model_name(&self) -> &'static str {
        match self {
            Self::AllMiniLmL6V2 => "sentence-transformers/all-MiniLM-L6-v2",
            Self::BgeSmallEnV15 => "BAAI/bge-small-en-v1.5",
            Self::E5SmallV2 => "intfloat/e5-small-v2",
        }
    }
}

/// Embedding result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    /// The text that was embedded
    pub text: String,
    /// The embedding vector
    pub vector: Vec<f32>,
    /// The model used
    pub model: EmbeddingModel,
}

