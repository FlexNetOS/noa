//! Embedding Repository
//!
//! T132: Implement Embedding entity repository
//! §3.7: Total Memory Sovereignty
//! US3: Vector embeddings for semantic search

use crate::db::Connection;
use crate::error::{DatabaseError, NoaError, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Row};
use uuid::Uuid;

/// Embedding entity representing vector embeddings
#[derive(Debug, Clone)]
pub struct Embedding {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub vector: Vec<f32>, // 384-dim for MiniLM-L6-v2
    pub model: String,
    pub source_type: String, // 'memory', 'node', 'document'
    pub source_id: Uuid,
}

/// Embedding repository for CRUD operations
pub struct EmbeddingRepository {
    conn: Connection,
}

impl EmbeddingRepository {
    /// Create a new embedding repository
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Create a new embedding entry
    pub fn create(&self, embedding: &Embedding) -> Result<Uuid> {
        // Serialize vector as BLOB (f32 array)
        let vector_bytes: Vec<u8> =
            embedding.vector.iter().flat_map(|f| f.to_le_bytes().to_vec()).collect();

        self.conn
            .execute(
                r#"
                INSERT INTO embedding (id, created_at, vector, model, source_type, source_id)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
                params![
                    embedding.id.to_string(),
                    embedding.created_at.to_rfc3339(),
                    vector_bytes,
                    embedding.model,
                    embedding.source_type,
                    embedding.source_id.to_string(),
                ],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "INSERT INTO embedding".to_string(),
                    error: e.to_string(),
                })
            })?;

        Ok(embedding.id)
    }

    /// Find embedding by ID
    pub fn find_by_id(&self, id: &Uuid) -> Result<Option<Embedding>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, created_at, vector, model, source_type, source_id
                FROM embedding
                WHERE id = ?1
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM embedding".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut rows = stmt
            .query_map(params![id.to_string()], |row| self.row_to_embedding(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM embedding".to_string(),
                    error: e.to_string(),
                })
            })?;

        match rows.next() {
            Some(Ok(embedding)) => Ok(Some(embedding)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT FROM embedding".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// Find embeddings by source
    pub fn find_by_source(&self, source_type: &str, source_id: &Uuid) -> Result<Vec<Embedding>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, created_at, vector, model, source_type, source_id
                FROM embedding
                WHERE source_type = ?1 AND source_id = ?2
                ORDER BY created_at DESC
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM embedding".to_string(),
                    error: e.to_string(),
                })
            })?;

        let rows = stmt
            .query_map(params![source_type, source_id.to_string()], |row| {
                self.row_to_embedding(row)
            })
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM embedding".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut embeddings = Vec::new();
        for row in rows {
            embeddings.push(row?);
        }

        Ok(embeddings)
    }

    /// Delete embedding by ID
    pub fn delete(&self, id: &Uuid) -> Result<bool> {
        let rows_affected = self
            .conn
            .execute(
                "DELETE FROM embedding WHERE id = ?1",
                params![id.to_string()],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "DELETE FROM embedding".to_string(),
                    error: e.to_string(),
                })
            })?;

        Ok(rows_affected > 0)
    }

    /// Check if embedding exists
    pub fn exists(&self, id: &Uuid) -> Result<bool> {
        let count: i64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM embedding WHERE id = ?1",
                params![id.to_string()],
                |row| row.get(0),
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT COUNT FROM embedding".to_string(),
                    error: e.to_string(),
                })
            })?;

        Ok(count > 0)
    }

    /// Count total embeddings
    pub fn count(&self) -> Result<u64> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM embedding", [], |row| row.get(0))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT COUNT FROM embedding".to_string(),
                    error: e.to_string(),
                })
            })?;

        Ok(count as u64)
    }

    /// Convert database row to Embedding entity
    fn row_to_embedding(&self, row: &Row) -> rusqlite::Result<Embedding> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "uuid".to_string(), rusqlite::types::Type::Text)
        })?;

        let created_at_str: String = row.get(1)?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    1,
                    "timestamp".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?
            .with_timezone(&Utc);

        let vector_bytes: Vec<u8> = row.get(2)?;
        // Deserialize f32 array from bytes (little-endian)
        let vector: Vec<f32> = vector_bytes
            .chunks_exact(4)
            .map(|chunk| {
                let bytes: [u8; 4] = chunk.try_into().unwrap();
                f32::from_le_bytes(bytes)
            })
            .collect();

        let model: String = row.get(3)?;
        let source_type: String = row.get(4)?;

        let source_id_str: String = row.get(5)?;
        let source_id = Uuid::parse_str(&source_id_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(5, "uuid".to_string(), rusqlite::types::Type::Text)
        })?;

        Ok(Embedding {
            id,
            created_at,
            vector,
            model,
            source_type,
            source_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_embedding_repository() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = crate::db::init_database(&db_path).unwrap();

        // Create tables
        conn.execute_batch(
            r#"
            CREATE TABLE embedding (
                id TEXT PRIMARY KEY,
                created_at TIMESTAMP NOT NULL,
                vector BLOB NOT NULL,
                model TEXT NOT NULL,
                source_type TEXT NOT NULL,
                source_id TEXT NOT NULL
            );
            "#,
        )
        .unwrap();

        let repo = EmbeddingRepository::new(conn);

        let embedding = Embedding {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            vector: vec![0.1, 0.2, 0.3, 0.4],
            model: "all-MiniLM-L6-v2".to_string(),
            source_type: "memory".to_string(),
            source_id: Uuid::new_v4(),
        };

        // Create
        let id = repo.create(&embedding).unwrap();
        assert_eq!(id, embedding.id);

        // Find
        let found = repo.find_by_id(&embedding.id).unwrap().unwrap();
        assert_eq!(found.vector.len(), 4);
        assert_eq!(found.model, "all-MiniLM-L6-v2");

        // Delete
        assert!(repo.delete(&embedding.id).unwrap());
        assert!(repo.find_by_id(&embedding.id).unwrap().is_none());
    }
}
