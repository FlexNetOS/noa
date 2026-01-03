//! Knowledge Node Repository
//!
//! T163: Implement KnowledgeNode repository
//! §3.4: Digest Everything Pipeline
//! US4: Digest Everything Pipeline

use crate::db::Connection;
use crate::error::{DatabaseError, NoaError, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Row};
use serde_json::{Map, Value};
use uuid::Uuid;

/// Knowledge node entity
#[derive(Debug, Clone)]
pub struct KnowledgeNode {
    pub id: Uuid,
    pub node_type: KnowledgeNodeType,
    pub name: String,
    pub qualified_name: Option<String>,
    pub description: Option<String>,
    pub source_digest: Option<Uuid>,
    pub location: Option<Map<String, Value>>,
    pub properties: Option<Map<String, Value>>,
    pub embedding_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Knowledge node type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowledgeNodeType {
    Function,
    Class,
    Module,
    File,
    Repo,
    Concept,
}

impl KnowledgeNodeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            KnowledgeNodeType::Function => "function",
            KnowledgeNodeType::Class => "class",
            KnowledgeNodeType::Module => "module",
            KnowledgeNodeType::File => "file",
            KnowledgeNodeType::Repo => "repo",
            KnowledgeNodeType::Concept => "concept",
        }
    }

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "function" => Ok(KnowledgeNodeType::Function),
            "class" => Ok(KnowledgeNodeType::Class),
            "module" => Ok(KnowledgeNodeType::Module),
            "file" => Ok(KnowledgeNodeType::File),
            "repo" => Ok(KnowledgeNodeType::Repo),
            "concept" => Ok(KnowledgeNodeType::Concept),
            _ => Err(NoaError::Validation(crate::error::ValidationError::new(
                "type",
                format!("Invalid knowledge node type: {}", s),
                "INVALID_TYPE",
            ))),
        }
    }
}

/// Knowledge node repository
pub struct KnowledgeNodeRepository {
    conn: Connection,
}

impl KnowledgeNodeRepository {
    /// Create a new knowledge node repository
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Create a new knowledge node
    pub fn create(&self, node: &KnowledgeNode) -> Result<Uuid> {
        let location_json = node
            .location
            .as_ref()
            .map(|l| serde_json::to_string(l))
            .transpose()
            .map_err(|e| {
                NoaError::Serialization(format!("Failed to serialize location: {}", e))
            })?;

        let properties_json = node
            .properties
            .as_ref()
            .map(|p| serde_json::to_string(p))
            .transpose()
            .map_err(|e| {
                NoaError::Serialization(format!("Failed to serialize properties: {}", e))
            })?;

        self.conn
            .execute(
                r#"
                INSERT INTO knowledge_node (
                    id, node_type, name, qualified_name, description, source_digest,
                    location, properties, embedding_id, created_at
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                "#,
                params![
                    node.id.to_string(),
                    node.node_type.as_str(),
                    node.name,
                    node.qualified_name,
                    node.description,
                    node.source_digest.map(|id| id.to_string()),
                    location_json,
                    properties_json,
                    node.embedding_id.map(|id| id.to_string()),
                    node.created_at.to_rfc3339(),
                ],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "INSERT INTO knowledge_node".to_string(),
                    error: e.to_string(),
                })
            })?;

        Ok(node.id)
    }

    /// Find knowledge node by ID
    pub fn find_by_id(&self, id: &Uuid) -> Result<Option<KnowledgeNode>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, node_type, name, qualified_name, description, source_digest,
                       location, properties, embedding_id, created_at
                FROM knowledge_node
                WHERE id = ?1
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_node".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut rows = stmt
            .query_map(params![id.to_string()], |row| self.row_to_node(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_node".to_string(),
                    error: e.to_string(),
                })
            })?;

        match rows.next() {
            Some(Ok(node)) => Ok(Some(node)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT FROM knowledge_node".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// Find nodes by type
    pub fn find_by_type(&self, node_type: KnowledgeNodeType) -> Result<Vec<KnowledgeNode>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, node_type, name, qualified_name, description, source_digest,
                       location, properties, embedding_id, created_at
                FROM knowledge_node
                WHERE node_type = ?1
                ORDER BY created_at DESC
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_node".to_string(),
                    error: e.to_string(),
                })
            })?;

        let rows = stmt
            .query_map(params![node_type.as_str()], |row| self.row_to_node(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_node".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut nodes = Vec::new();
        for row in rows {
            nodes.push(row?);
        }

        Ok(nodes)
    }

    /// Find nodes by source digest
    pub fn find_by_source_digest(&self, source_digest: &Uuid) -> Result<Vec<KnowledgeNode>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, node_type, name, qualified_name, description, source_digest,
                       location, properties, embedding_id, created_at
                FROM knowledge_node
                WHERE source_digest = ?1
                ORDER BY created_at DESC
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_node".to_string(),
                    error: e.to_string(),
                })
            })?;

        let rows = stmt
            .query_map(params![source_digest.to_string()], |row| self.row_to_node(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_node".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut nodes = Vec::new();
        for row in rows {
            nodes.push(row?);
        }

        Ok(nodes)
    }

    /// Convert database row to KnowledgeNode entity
    fn row_to_node(&self, row: &Row) -> rusqlite::Result<KnowledgeNode> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "uuid".to_string(), rusqlite::types::Type::Text)
        })?;

        let type_str: String = row.get(1)?;
        let node_type = KnowledgeNodeType::from_str(&type_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(1, "type".to_string(), rusqlite::types::Type::Text)
        })?;

        let name: String = row.get(2)?;
        let qualified_name: Option<String> = row.get(3)?;
        let description: Option<String> = row.get(4)?;

        let source_digest_str: Option<String> = row.get(5)?;
        let source_digest = source_digest_str
            .map(|s| Uuid::parse_str(&s))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    5,
                    "uuid".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?;

        let location_str: Option<String> = row.get(6)?;
        let location = location_str
            .map(|s| serde_json::from_str::<Map<String, Value>>(&s))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(6, "json".to_string(), rusqlite::types::Type::Text)
            })?;

        let properties_str: Option<String> = row.get(7)?;
        let properties = properties_str
            .map(|s| serde_json::from_str::<Map<String, Value>>(&s))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(7, "json".to_string(), rusqlite::types::Type::Text)
            })?;

        let embedding_id_str: Option<String> = row.get(8)?;
        let embedding_id = embedding_id_str
            .map(|s| Uuid::parse_str(&s))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(8, "uuid".to_string(), rusqlite::types::Type::Text)
            })?;

        let created_at_str: String = row.get(9)?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    9,
                    "timestamp".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?
            .with_timezone(&Utc);

        Ok(KnowledgeNode {
            id,
            node_type,
            name,
            qualified_name,
            description,
            source_digest,
            location,
            properties,
            embedding_id,
            created_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn setup_test_db() -> Connection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = Connection::open(&db_path).unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS knowledge_node (
                id TEXT PRIMARY KEY,
                node_type TEXT NOT NULL,
                name TEXT NOT NULL,
                qualified_name TEXT,
                description TEXT,
                source_digest TEXT,
                location TEXT,
                properties TEXT,
                embedding_id TEXT,
                created_at TEXT NOT NULL
            );
            "#,
        ).unwrap();
        std::mem::forget(dir);
        conn
    }

    #[test]
    fn test_knowledge_node_create_and_find() {
        let conn = setup_test_db();
        let repo = KnowledgeNodeRepository::new(conn);

        let node = KnowledgeNode {
            id: Uuid::new_v4(),
            node_type: KnowledgeNodeType::Function,
            name: "process_data".to_string(),
            qualified_name: Some("app::process_data".to_string()),
            description: Some("Processes input data".to_string()),
            source_digest: None,
            location: None,
            properties: None,
            embedding_id: None,
            created_at: Utc::now(),
        };

        let id = repo.create(&node).unwrap();
        assert_eq!(id, node.id);

        let found = repo.find_by_id(&node.id).unwrap().unwrap();
        assert_eq!(found.name, "process_data");
        assert_eq!(found.node_type, KnowledgeNodeType::Function);
    }

    #[test]
    fn test_knowledge_node_types() {
        assert_eq!(KnowledgeNodeType::Function.as_str(), "function");
        assert_eq!(KnowledgeNodeType::Class.as_str(), "class");
        assert_eq!(KnowledgeNodeType::Module.as_str(), "module");
        assert_eq!(KnowledgeNodeType::File.as_str(), "file");
        assert_eq!(KnowledgeNodeType::Repo.as_str(), "repo");
        assert_eq!(KnowledgeNodeType::Concept.as_str(), "concept");
        
        assert!(matches!(KnowledgeNodeType::from_str("function"), Ok(KnowledgeNodeType::Function)));
        assert!(KnowledgeNodeType::from_str("invalid").is_err());
    }

    #[test]
    fn test_knowledge_node_find_nonexistent() {
        let conn = setup_test_db();
        let repo = KnowledgeNodeRepository::new(conn);

        let result = repo.find_by_id(&Uuid::new_v4()).unwrap();
        assert!(result.is_none());
    }
}

