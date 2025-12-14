//! Search Service
//!
//! T144: Implement SearchService with semantic + keyword search
//! §3.7: Total Memory Sovereignty
//! US3: Search results in <500ms

use crate::db::repositories::MemoryRepository;
use crate::db::vector_search::VectorSearch;
use crate::error::{NoaError, Result};
use crate::memory::semantic_search::SemanticSearch;
use crate::memory::embeddings::EmbeddingGenerator;
use uuid::Uuid;

/// Search service for semantic and keyword search
pub struct SearchService {
    memory_repo: MemoryRepository,
    semantic_search: SemanticSearch,
    embedding_generator: Option<EmbeddingGenerator>,
}

impl SearchService {
    /// Create a new search service
    pub fn new(memory_repo: MemoryRepository, vector_search: VectorSearch) -> Self {
        let semantic_search = SemanticSearch::new(vector_search);
        Self {
            memory_repo,
            semantic_search,
            embedding_generator: None,
        }
    }

    /// Create search service with embedding generator
    pub async fn with_embeddings(
        memory_repo: MemoryRepository,
        vector_search: VectorSearch,
        model_name: &str,
    ) -> Result<Self> {
        let generator = EmbeddingGenerator::new(model_name).await?;
        let semantic_search = SemanticSearch::new(vector_search);
        Ok(Self {
            memory_repo,
            semantic_search,
            embedding_generator: Some(generator),
        })
    }

    /// Perform semantic search
    pub async fn search_semantic(
        &self,
        query: &str,
        limit: u32,
        threshold: f32,
    ) -> Result<Vec<SearchResult>> {
        // Generate query embedding
        let query_vector = if let Some(ref generator) = self.embedding_generator {
            generator.generate(query).await?
        } else {
            return Err(NoaError::Internal {
                message: "Embedding generator not available".to_string(),
                source: None,
            });
        };

        // Perform semantic search
        let results = self
            .semantic_search
            .search(None, Some(&query_vector), limit, threshold)
            .await?;

        // Fetch full memory records
        let mut search_results = Vec::new();
        for result in results {
            if let Some(memory) = self.memory_repo.find_by_id(&result.memory_id)? {
                search_results.push(SearchResult {
                    memory,
                    score: result.score,
                    distance: result.distance,
                });
            }
        }

        Ok(search_results)
    }

    /// Perform keyword search
    pub fn search_keyword(
        &self,
        query: &str,
        limit: u32,
    ) -> Result<Vec<SearchResult>> {
        // Simple keyword search in content and tags
        // In a real implementation, this would use full-text search
        let all_memories = self.memory_repo.list(0, (limit * 2) as u64)?; // Get more to filter

        let query_lower = query.to_lowercase();
        let mut results: Vec<_> = all_memories
            .into_iter()
            .filter(|m| {
                m.content.to_lowercase().contains(&query_lower)
                    || m.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .take(limit as usize)
            .map(|memory| SearchResult {
                memory,
                score: 1.0, // Keyword matches get full score
                distance: 0.0,
            })
            .collect();

        // Sort by created_at (most recent first)
        results.sort_by(|a, b| b.memory.created_at.cmp(&a.memory.created_at));

        Ok(results)
    }

    /// Perform hybrid search (semantic + keyword)
    pub async fn search_hybrid(
        &self,
        query: &str,
        limit: u32,
        semantic_threshold: f32,
    ) -> Result<Vec<SearchResult>> {
        // Get results from both methods
        let semantic_results = self
            .search_semantic(query, limit, semantic_threshold)
            .await
            .unwrap_or_default();

        let keyword_results = self.search_keyword(query, limit).unwrap_or_default();

        // Combine and deduplicate by memory ID
        let mut combined: std::collections::HashMap<Uuid, SearchResult> =
            std::collections::HashMap::new();

        for result in semantic_results {
            combined.insert(result.memory.id, result);
        }

        for result in keyword_results {
            combined
                .entry(result.memory.id)
                .and_modify(|r| {
                    // Boost score if found in both
                    r.score = (r.score + 1.0).min(2.0);
                })
                .or_insert(result);
        }

        // Sort by score and limit
        let mut results: Vec<_> = combined.into_values().collect();
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit as usize);

        Ok(results)
    }
}

/// Search result with memory and score
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub memory: crate::db::repositories::Memory,
    pub score: f32,
    pub distance: f32,
}

