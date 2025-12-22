//! Semantic Search
//!
//! T136: Implement semantic search with HNSW index
//! §3.7: Total Memory Sovereignty

use crate::db::vector_search::VectorSearch;
use crate::error::Result;
use uuid::Uuid;

/// Semantic search engine using HNSW index
pub struct SemanticSearch<'a> {
    vector_search: VectorSearch<'a>,
}

impl<'a> SemanticSearch<'a> {
    /// Create a new semantic search instance
    pub fn new(vector_search: VectorSearch<'a>) -> Self {
        Self { vector_search }
    }

    /// Search for similar memories by pre-computed query vector.
    ///
    /// Embedding generation is owned by higher-level services (e.g. `SearchService`) so
    /// this layer stays focused on vector index lookups.
    ///
    /// # Arguments
    /// * `query_vector` - Query vector to search for
    /// * `limit` - Maximum number of results
    /// * `threshold` - Minimum similarity score
    pub async fn search(
        &self,
        query_vector: &[f32],
        limit: u32,
        threshold: f32,
    ) -> Result<Vec<SearchResult>> {

        // Perform vector search
        let results = self.vector_search.search_memory(query_vector, limit, threshold)?;

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

