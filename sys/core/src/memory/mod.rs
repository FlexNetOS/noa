//! Memory Module
//!
//! Phase 5: User Story 3 - Total Memory Sovereignty
//! §3.7: Total Memory Sovereignty
//! US3: Remember everything with instant recall

pub mod cache;
pub mod embedding_model;
pub mod embeddings;
pub mod semantic_search;

pub use cache::EmbeddingCache;
pub use embedding_model::EmbeddingModel;
pub use embeddings::EmbeddingGenerator;
pub use semantic_search::SemanticSearch;

// Re-export MemoryType from memory_repository
pub use crate::db::repositories::memory_repository::MemoryType;
