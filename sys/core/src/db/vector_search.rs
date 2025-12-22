//! Vector Search Integration
//!
//! T133: Integrate sqlite-vss for vector search
//! §3.7: Total Memory Sovereignty
//! US3: Semantic search with <500ms latency

use crate::db::Connection;
use crate::error::{DatabaseError, NoaError, Result};
use uuid::Uuid;

/// Vector search result
#[derive(Debug, Clone)]
pub struct VectorSearchResult {
    pub id: Uuid,
    pub distance: f32,
    pub score: f32, // 1.0 - distance (for cosine similarity)
}

/// Vector search configuration
#[derive(Debug, Clone)]
pub struct VectorSearchConfig {
    pub model: String,
    pub dimensions: u32,
    pub distance_metric: String, // 'cosine', 'euclidean', 'dot'
    pub ef_construction: u32,
    pub ef_search: u32,
    pub m: u32,
}

impl Default for VectorSearchConfig {
    fn default() -> Self {
        Self {
            model: "all-MiniLM-L6-v2".to_string(),
            dimensions: 384,
            distance_metric: "cosine".to_string(),
            ef_construction: 200,
            ef_search: 100,
            m: 16,
        }
    }
}

/// Vector search integration for sqlite-vss
pub struct VectorSearch<'a> {
    conn: &'a Connection,
    config: VectorSearchConfig,
}

impl<'a> VectorSearch<'a> {
    /// Create a new vector search instance
    pub fn new(conn: &'a Connection) -> Result<Self> {
        let config = Self::load_config(conn)?;
        Ok(Self { conn, config })
    }

    /// Load vector search configuration from database
    fn load_config(conn: &Connection) -> Result<VectorSearchConfig> {
        let model: String = conn
            .query_row(
                "SELECT value FROM vss_config WHERE key = 'model'",
                [],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| "all-MiniLM-L6-v2".to_string());

        let dimensions: u32 = conn
            .query_row(
                "SELECT value FROM vss_config WHERE key = 'dimensions'",
                [],
                |row| {
                    let s: String = row.get(0)?;
                    s.parse().map_err(|_| {
                        rusqlite::Error::InvalidColumnType(
                            0,
                            "integer".to_string(),
                            rusqlite::types::Type::Text,
                        )
                    })
                },
            )
            .unwrap_or(384);

        let distance_metric: String = conn
            .query_row(
                "SELECT value FROM vss_config WHERE key = 'distance_metric'",
                [],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| "cosine".to_string());

        Ok(VectorSearchConfig {
            model,
            dimensions,
            distance_metric,
            ef_construction: 200,
            ef_search: 100,
            m: 16,
        })
    }

    /// Search for similar vectors in memory embeddings
    ///
    /// # Arguments
    /// * `query_vector` - Query vector (384-dim for MiniLM)
    /// * `limit` - Maximum number of results
    /// * `threshold` - Minimum similarity score (0.0-1.0)
    pub fn search_memory(
        &self,
        query_vector: &[f32],
        limit: u32,
        threshold: f32,
    ) -> Result<Vec<VectorSearchResult>> {
        if query_vector.len() != self.config.dimensions as usize {
            return Err(NoaError::Validation(crate::error::ValidationError::new(
                "query_vector",
                format!(
                    "Vector dimension mismatch: expected {}, got {}",
                    self.config.dimensions,
                    query_vector.len()
                ),
                "DIMENSION_MISMATCH",
            )));
        }

        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT
                    e.id,
                    e.vector,
                    vm.memory_id
                FROM embedding e
                JOIN vss_memory_map vm ON e.id = vm.embedding_id
                WHERE e.source_type = 'memory'
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM embedding".to_string(),
                    error: e.to_string(),
                })
            })?;

        let rows = stmt
            .query_map([], |row| {
                let id_str: String = row.get(0)?;
                let vector_bytes: Vec<u8> = row.get(1)?;
                let memory_id_str: String = row.get(2)?;

                // Deserialize vector
                let vector: Vec<f32> = vector_bytes
                    .chunks_exact(4)
                    .map(|chunk| {
                        let bytes: [u8; 4] = chunk.try_into().unwrap();
                        f32::from_le_bytes(bytes)
                    })
                    .collect();

                Ok((id_str, vector, memory_id_str))
            })
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM embedding".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut results = Vec::new();
        for row in rows {
            let (id_str, vector, memory_id_str) = row?;
            let id = Uuid::parse_str(&memory_id_str)
                .or_else(|_| Uuid::parse_str(&id_str))
                .unwrap_or_else(|_| Uuid::nil());

            // Compute cosine similarity
            let similarity = cosine_similarity(query_vector, &vector);
            let distance = 1.0 - similarity;

            if similarity >= threshold {
                results.push(VectorSearchResult {
                    id,
                    distance,
                    score: similarity,
                });
            }
        }

        // Sort by score (descending) and limit
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit as usize);

        Ok(results)
    }

    /// Get configuration
    pub fn config(&self) -> &VectorSearchConfig {
        &self.config
    }
}

/// Compute cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 0.0).abs() < 0.001);
    }
}

