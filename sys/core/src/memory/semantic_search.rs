//! Semantic Search
//!
//! T136: Implement semantic search with HNSW index
//! §3.7: Total Memory Sovereignty

use crate::db::vector_search::VectorSearch;
use crate::error::Result;
use uuid::Uuid;

/// Semantic search engine using HNSW index
pub struct SemanticSearch {
    vector_search: VectorSearch,
}

impl SemanticSearch {
    /// Create a new semantic search instance
    pub fn new(vector_search: VectorSearch) -> Self {
        Self { vector_search }
    }

    /// Search for similar memories by query text
    ///
    /// # Arguments
    /// * `query_text` - Text query to search for
    /// * `query_vector` - Pre-computed query vector (if available)
    /// * `limit` - Maximum number of results
    /// * `threshold` - Minimum similarity score
    pub async fn search(
        &self,
        query_text: Option<&str>,
        query_vector: Option<&[f32]>,
        limit: u32,
        threshold: f32,
    ) -> Result<Vec<SearchResult>> {
        // If query_vector is provided, use it directly
        // Otherwise, we would need to generate it from query_text
        // For now, we'll require query_vector to be provided
        let vector = query_vector.ok_or_else(|| {
            crate::error::NoaError::Validation(crate::error::ValidationError::new(
                "query_vector",
                "Query vector is required for semantic search".to_string(),
                "MISSING_QUERY_VECTOR",
            ))
        })?;

        // Perform vector search
        let results = self
            .vector_search
            .search_memory(vector, limit, threshold)?;

        // Convert to SearchResult
        Ok(results
            .into_iter()
            .map(|r| SearchResult {
                memory_id: r.id,
                score: r.score,
                distance: r.distance,
            })
            .collect())
    }
}

/// Semantic search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub memory_id: Uuid,
    pub score: f32,
    pub distance: f32,
}

