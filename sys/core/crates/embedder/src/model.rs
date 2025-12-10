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
    /// Llama-based embedding (T112)
    Llama,
    /// Mistral-based embedding (T112)
    Mistral,
    /// Qwen-based embedding (T112)
    Qwen,
    /// BGE-Large-EN-v1.5 (1024 dimensions) (T112)
    BgeLargeEnV15,
    /// E5-Large-v2 (1024 dimensions) (T112)
    E5LargeV2,
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
            Self::Llama => 4096, // Typical for Llama embeddings
            Self::Mistral => 4096,
            Self::Qwen => 4096,
            Self::BgeLargeEnV15 => 1024,
            Self::E5LargeV2 => 1024,
        }
    }

    /// Get the model name for fastembed or Candle
    pub fn model_name(&self) -> &'static str {
        match self {
            Self::AllMiniLmL6V2 => "sentence-transformers/all-MiniLM-L6-v2",
            Self::BgeSmallEnV15 => "BAAI/bge-small-en-v1.5",
            Self::E5SmallV2 => "intfloat/e5-small-v2",
            Self::Llama => "meta-llama/Llama-2-7b-hf",
            Self::Mistral => "mistralai/Mistral-7B-v0.1",
            Self::Qwen => "Qwen/Qwen-7B",
            Self::BgeLargeEnV15 => "BAAI/bge-large-en-v1.5",
            Self::E5LargeV2 => "intfloat/e5-large-v2",
        }
    }

    /// Check if model uses Candle (vs fastembed)
    pub fn uses_candle(&self) -> bool {
        matches!(
            self,
            Self::Llama | Self::Mistral | Self::Qwen | Self::BgeLargeEnV15 | Self::E5LargeV2
        )
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
