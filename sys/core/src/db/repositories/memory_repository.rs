//! Memory Repository
//!
//! T131: Implement Memory entity repository
//! §3.7: Total Memory Sovereignty
//! US3: Remember everything with instant recall

use crate::db::Connection;
use crate::error::{DatabaseError, NoaError, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Row};
use serde_json::{Map, Value};
use std::collections::HashSet;
use uuid::Uuid;

/// Memory entity representing stored interactions, decisions, learnings, and artifacts
#[derive(Debug, Clone)]
pub struct Memory {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub memory_type: MemoryType,
    pub content: String,
    pub metadata: Option<Map<String, Value>>,
    pub source_agent: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub tags: HashSet<String>,
    pub embedding_id: Option<Uuid>,
    pub checksum: String,
}

/// Memory type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    Interaction,
    Decision,
    Learning,
    Artifact,
}

impl MemoryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MemoryType::Interaction => "interaction",
            MemoryType::Decision => "decision",
            MemoryType::Learning => "learning",
            MemoryType::Artifact => "artifact",
        }
    }

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "interaction" => Ok(MemoryType::Interaction),
            "decision" => Ok(MemoryType::Decision),
            "learning" => Ok(MemoryType::Learning),
            "artifact" => Ok(MemoryType::Artifact),
            _ => Err(NoaError::Validation(crate::error::ValidationError::new(
                "type",
                format!("Invalid memory type: {}", s),
                "INVALID_TYPE",
            ))),
        }
    }
}

/// Memory repository for CRUD operations
pub struct MemoryRepository<'a> {
    conn: &'a Connection,
}

impl<'a> MemoryRepository<'a> {
    /// Create a new memory repository
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Create a new memory entry
    pub fn create(&self, memory: &Memory) -> Result<Uuid> {
        let tags_json = serde_json::to_string(&memory.tags.iter().collect::<Vec<_>>())
            .map_err(|e| NoaError::Serialization(format!("Failed to serialize tags: {}", e)))?;

        let metadata_json = memory
            .metadata
            .as_ref()
            .map(|m| serde_json::to_string(m))
            .transpose()
            .map_err(|e| NoaError::Serialization(format!("Failed to serialize metadata: {}", e)))?;

        self.conn
            .execute(
                r#"
                INSERT INTO memory (
                    id, created_at, updated_at, type, content, metadata,
                    source_agent, parent_id, tags, embedding_id, checksum
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                "#,
                params![
                    memory.id.to_string(),
                    memory.created_at.to_rfc3339(),
                    memory.updated_at.to_rfc3339(),
                    memory.memory_type.as_str(),
                    memory.content,
                    metadata_json,
                    memory.source_agent.map(|id| id.to_string()),
                    memory.parent_id.map(|id| id.to_string()),
                    tags_json,
                    memory.embedding_id.map(|id| id.to_string()),
                    memory.checksum,
                ],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "INSERT INTO memory".to_string(),
                    error: e.to_string(),
                })
            })?;

        Ok(memory.id)
    }

    /// Find memory by ID
    pub fn find_by_id(&self, id: &Uuid) -> Result<Option<Memory>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, created_at, updated_at, type, content, metadata,
                       source_agent, parent_id, tags, embedding_id, checksum
                FROM memory
                WHERE id = ?1
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM memory".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut rows = stmt
            .query_map(params![id.to_string()], |row| self.row_to_memory(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM memory".to_string(),
                    error: e.to_string(),
                })
            })?;

        match rows.next() {
            Some(Ok(memory)) => Ok(Some(memory)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT FROM memory".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// Update an existing memory entry
    pub fn update(&self, memory: &Memory) -> Result<()> {
        let tags_json = serde_json::to_string(&memory.tags.iter().collect::<Vec<_>>())
            .map_err(|e| NoaError::Serialization(format!("Failed to serialize tags: {}", e)))?;

        let metadata_json = memory
            .metadata
            .as_ref()
            .map(|m| serde_json::to_string(m))
            .transpose()
            .map_err(|e| NoaError::Serialization(format!("Failed to serialize metadata: {}", e)))?;

        let rows_affected = self
            .conn
            .execute(
                r#"
                UPDATE memory SET
                    updated_at = ?1,
                    type = ?2,
                    content = ?3,
                    metadata = ?4,
                    source_agent = ?5,
                    parent_id = ?6,
                    tags = ?7,
                    embedding_id = ?8,
                    checksum = ?9
                WHERE id = ?10
                "#,
                params![
                    memory.updated_at.to_rfc3339(),
                    memory.memory_type.as_str(),
                    memory.content,
                    metadata_json,
                    memory.source_agent.map(|id| id.to_string()),
                    memory.parent_id.map(|id| id.to_string()),
                    tags_json,
                    memory.embedding_id.map(|id| id.to_string()),
                    memory.checksum,
                    memory.id.to_string(),
                ],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "UPDATE memory".to_string(),
                    error: e.to_string(),
                })
            })?;

        if rows_affected == 0 {
            return Err(NoaError::NotFound {
                resource: "memory".to_string(),
                id: memory.id.to_string(),
            });
        }

        Ok(())
    }

    /// Delete memory by ID
    pub fn delete(&self, id: &Uuid) -> Result<bool> {
        let rows_affected = self
            .conn
            .execute("DELETE FROM memory WHERE id = ?1", params![id.to_string()])
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "DELETE FROM memory".to_string(),
                    error: e.to_string(),
                })
            })?;

        Ok(rows_affected > 0)
    }

    /// Check if memory exists
    pub fn exists(&self, id: &Uuid) -> Result<bool> {
        let count: i64 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM memory WHERE id = ?1",
                params![id.to_string()],
                |row| row.get(0),
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT COUNT FROM memory".to_string(),
                    error: e.to_string(),
                })
            })?;

        Ok(count > 0)
    }

    /// List memories with pagination
    pub fn list(&self, offset: u64, limit: u64) -> Result<Vec<Memory>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, created_at, updated_at, type, content, metadata,
                       source_agent, parent_id, tags, embedding_id, checksum
                FROM memory
                ORDER BY created_at DESC
                LIMIT ?1 OFFSET ?2
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM memory".to_string(),
                    error: e.to_string(),
                })
            })?;

        let rows = stmt
            .query_map(params![limit as i64, offset as i64], |row| {
                self.row_to_memory(row)
            })
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM memory".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut memories = Vec::new();
        for row in rows {
            memories.push(row?);
        }

        Ok(memories)
    }

    /// Find memories by type
    pub fn find_by_type(&self, memory_type: MemoryType) -> Result<Vec<Memory>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, created_at, updated_at, type, content, metadata,
                       source_agent, parent_id, tags, embedding_id, checksum
                FROM memory
                WHERE type = ?1
                ORDER BY created_at DESC
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM memory".to_string(),
                    error: e.to_string(),
                })
            })?;

        let rows = stmt
            .query_map(params![memory_type.as_str()], |row| self.row_to_memory(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM memory".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut memories = Vec::new();
        for row in rows {
            memories.push(row?);
        }

        Ok(memories)
    }

    /// Count total memories
    pub fn count(&self) -> Result<u64> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM memory", [], |row| row.get(0))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT COUNT FROM memory".to_string(),
                    error: e.to_string(),
                })
            })?;

        Ok(count as u64)
    }

    /// Convert database row to Memory entity
    fn row_to_memory(&self, row: &Row) -> rusqlite::Result<Memory> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|e| {
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

        let updated_at_str: String = row.get(2)?;
        let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    2,
                    "timestamp".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?
            .with_timezone(&Utc);

        let type_str: String = row.get(3)?;
        let memory_type = MemoryType::from_str(&type_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(3, "type".to_string(), rusqlite::types::Type::Text)
        })?;

        let content: String = row.get(4)?;
        let metadata_str: Option<String> = row.get(5)?;
        let metadata = metadata_str
            .map(|s| serde_json::from_str::<Map<String, Value>>(&s))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    5,
                    "json".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?;

        let source_agent_str: Option<String> = row.get(6)?;
        let source_agent = source_agent_str
            .map(|s| Uuid::parse_str(&s))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    6,
                    "uuid".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?;

        let parent_id_str: Option<String> = row.get(7)?;
        let parent_id = parent_id_str
            .map(|s| Uuid::parse_str(&s))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    7,
                    "uuid".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?;

        let tags_str: String = row.get(8)?;
        let tags_vec: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
        let tags = tags_vec.into_iter().collect();

        let embedding_id_str: Option<String> = row.get(9)?;
        let embedding_id = embedding_id_str
            .map(|s| Uuid::parse_str(&s))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    9,
                    "uuid".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?;

        let checksum: String = row.get(10)?;

        Ok(Memory {
            id,
            created_at,
            updated_at,
            memory_type,
            content,
            metadata,
            source_agent,
            parent_id,
            tags,
            embedding_id,
            checksum,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_memory_repository() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = crate::db::init_database(&db_path).unwrap();

        // Create tables
        conn.execute_batch(
            r#"
            CREATE TABLE memory (
                id TEXT PRIMARY KEY,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL,
                type TEXT NOT NULL,
                content TEXT NOT NULL,
                metadata TEXT,
                source_agent TEXT,
                parent_id TEXT,
                tags TEXT,
                embedding_id TEXT,
                checksum TEXT NOT NULL
            );
            "#,
        )
        .unwrap();

        let repo = MemoryRepository::new(&conn);

        let memory = Memory {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            memory_type: MemoryType::Interaction,
            content: "Test content".to_string(),
            metadata: None,
            source_agent: None,
            parent_id: None,
            tags: HashSet::new(),
            embedding_id: None,
            checksum: "test_checksum".to_string(),
        };

        // Create
        let id = repo.create(&memory).unwrap();
        assert_eq!(id, memory.id);

        // Find
        let found = repo.find_by_id(&memory.id).unwrap().unwrap();
        assert_eq!(found.content, "Test content");

        // Update
        let mut updated = memory.clone();
        updated.content = "Updated content".to_string();
        repo.update(&updated).unwrap();

        let found = repo.find_by_id(&memory.id).unwrap().unwrap();
        assert_eq!(found.content, "Updated content");

        // Delete
        assert!(repo.delete(&memory.id).unwrap());
        assert!(repo.find_by_id(&memory.id).unwrap().is_none());
    }
}

