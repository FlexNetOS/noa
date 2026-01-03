//! Knowledge Edge Repository
//!
//! T164: Implement KnowledgeEdge repository
//! §3.4: Digest Everything Pipeline
//! US4: Digest Everything Pipeline

use crate::db::Connection;
use crate::error::{DatabaseError, NoaError, Result};
use rusqlite::{params, Row};
use serde_json::{Map, Value};
use uuid::Uuid;

/// Knowledge edge entity
#[derive(Debug, Clone)]
pub struct KnowledgeEdge {
    pub id: Uuid,
    pub source_node: Uuid,
    pub target_node: Uuid,
    pub relationship: RelationshipType,
    pub weight: f64,
    pub properties: Option<Map<String, Value>>,
}

/// Relationship type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationshipType {
    Calls,
    Imports,
    Extends,
    Implements,
    Contains,
    References,
}

impl RelationshipType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationshipType::Calls => "calls",
            RelationshipType::Imports => "imports",
            RelationshipType::Extends => "extends",
            RelationshipType::Implements => "implements",
            RelationshipType::Contains => "contains",
            RelationshipType::References => "references",
        }
    }

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "calls" => Ok(RelationshipType::Calls),
            "imports" => Ok(RelationshipType::Imports),
            "extends" => Ok(RelationshipType::Extends),
            "implements" => Ok(RelationshipType::Implements),
            "contains" => Ok(RelationshipType::Contains),
            "references" => Ok(RelationshipType::References),
            _ => Err(NoaError::Validation(crate::error::ValidationError::new(
                "relationship",
                format!("Invalid relationship type: {}", s),
                "INVALID_RELATIONSHIP",
            ))),
        }
    }
}

/// Knowledge edge repository
pub struct KnowledgeEdgeRepository {
    conn: Connection,
}

impl KnowledgeEdgeRepository {
    /// Create a new knowledge edge repository
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Create a new knowledge edge
    pub fn create(&self, edge: &KnowledgeEdge) -> Result<Uuid> {
        let properties_json = edge
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
                INSERT INTO knowledge_edge (
                    id, source_node, target_node, relationship, weight, properties
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
                params![
                    edge.id.to_string(),
                    edge.source_node.to_string(),
                    edge.target_node.to_string(),
                    edge.relationship.as_str(),
                    edge.weight,
                    properties_json,
                ],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "INSERT INTO knowledge_edge".to_string(),
                    error: e.to_string(),
                })
            })?;

        Ok(edge.id)
    }

    /// Find knowledge edge by ID
    pub fn find_by_id(&self, id: &Uuid) -> Result<Option<KnowledgeEdge>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, source_node, target_node, relationship, weight, properties
                FROM knowledge_edge
                WHERE id = ?1
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_edge".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut rows = stmt
            .query_map(params![id.to_string()], |row| self.row_to_edge(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_edge".to_string(),
                    error: e.to_string(),
                })
            })?;

        match rows.next() {
            Some(Ok(edge)) => Ok(Some(edge)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT FROM knowledge_edge".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// Find edges by source node
    pub fn find_by_source_node(&self, source_node: &Uuid) -> Result<Vec<KnowledgeEdge>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, source_node, target_node, relationship, weight, properties
                FROM knowledge_edge
                WHERE source_node = ?1
                ORDER BY weight DESC
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_edge".to_string(),
                    error: e.to_string(),
                })
            })?;

        let rows = stmt
            .query_map(params![source_node.to_string()], |row| self.row_to_edge(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_edge".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut edges = Vec::new();
        for row in rows {
            edges.push(row?);
        }

        Ok(edges)
    }

    /// Find edges by target node
    pub fn find_by_target_node(&self, target_node: &Uuid) -> Result<Vec<KnowledgeEdge>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, source_node, target_node, relationship, weight, weight, properties
                FROM knowledge_edge
                WHERE target_node = ?1
                ORDER BY weight DESC
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_edge".to_string(),
                    error: e.to_string(),
                })
            })?;

        let rows = stmt
            .query_map(params![target_node.to_string()], |row| self.row_to_edge(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_edge".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut edges = Vec::new();
        for row in rows {
            edges.push(row?);
        }

        Ok(edges)
    }

    /// Find edges by relationship type
    pub fn find_by_relationship(
        &self,
        relationship: RelationshipType,
    ) -> Result<Vec<KnowledgeEdge>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, source_node, target_node, relationship, weight, properties
                FROM knowledge_edge
                WHERE relationship = ?1
                ORDER BY weight DESC
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_edge".to_string(),
                    error: e.to_string(),
                })
            })?;

        let rows = stmt
            .query_map(params![relationship.as_str()], |row| self.row_to_edge(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM knowledge_edge".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut edges = Vec::new();
        for row in rows {
            edges.push(row?);
        }

        Ok(edges)
    }

    /// Convert database row to KnowledgeEdge entity
    fn row_to_edge(&self, row: &Row) -> rusqlite::Result<KnowledgeEdge> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "uuid".to_string(), rusqlite::types::Type::Text)
        })?;

        let source_node_str: String = row.get(1)?;
        let source_node = Uuid::parse_str(&source_node_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(
                1,
                "uuid".to_string(),
                rusqlite::types::Type::Text,
            )
        })?;

        let target_node_str: String = row.get(2)?;
        let target_node = Uuid::parse_str(&target_node_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(
                2,
                "uuid".to_string(),
                rusqlite::types::Type::Text,
            )
        })?;

        let relationship_str: String = row.get(3)?;
        let relationship = RelationshipType::from_str(&relationship_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(
                3,
                "relationship".to_string(),
                rusqlite::types::Type::Text,
            )
        })?;

        let weight: f64 = row.get(4)?;

        let properties_str: Option<String> = row.get(5)?;
        let properties = properties_str
            .map(|s| serde_json::from_str::<Map<String, Value>>(&s))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(5, "json".to_string(), rusqlite::types::Type::Text)
            })?;

        Ok(KnowledgeEdge {
            id,
            source_node,
            target_node,
            relationship,
            weight,
            properties,
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
            CREATE TABLE IF NOT EXISTS knowledge_edge (
                id TEXT PRIMARY KEY,
                source_node TEXT NOT NULL,
                target_node TEXT NOT NULL,
                relationship TEXT NOT NULL,
                weight REAL NOT NULL,
                properties TEXT
            );
            "#,
        ).unwrap();
        std::mem::forget(dir);
        conn
    }

    #[test]
    fn test_knowledge_edge_create_and_find() {
        let conn = setup_test_db();
        let repo = KnowledgeEdgeRepository::new(conn);

        let source = Uuid::new_v4();
        let target = Uuid::new_v4();
        
        let edge = KnowledgeEdge {
            id: Uuid::new_v4(),
            source_node: source,
            target_node: target,
            relationship: RelationshipType::Calls,
            weight: 1.0,
            properties: None,
        };

        let id = repo.create(&edge).unwrap();
        assert_eq!(id, edge.id);

        let found = repo.find_by_id(&edge.id).unwrap().unwrap();
        assert_eq!(found.source_node, source);
        assert_eq!(found.target_node, target);
        assert_eq!(found.relationship, RelationshipType::Calls);
    }

    #[test]
    fn test_relationship_types() {
        assert_eq!(RelationshipType::Calls.as_str(), "calls");
        assert_eq!(RelationshipType::Imports.as_str(), "imports");
        assert_eq!(RelationshipType::Extends.as_str(), "extends");
        assert_eq!(RelationshipType::Implements.as_str(), "implements");
        assert_eq!(RelationshipType::Contains.as_str(), "contains");
        assert_eq!(RelationshipType::References.as_str(), "references");
        
        assert!(matches!(RelationshipType::from_str("calls"), Ok(RelationshipType::Calls)));
        assert!(RelationshipType::from_str("invalid").is_err());
    }

    #[test]
    fn test_knowledge_edge_find_by_source() {
        let conn = setup_test_db();
        let repo = KnowledgeEdgeRepository::new(conn);

        let source = Uuid::new_v4();
        
        // Create multiple edges from same source
        for i in 0..3 {
            let edge = KnowledgeEdge {
                id: Uuid::new_v4(),
                source_node: source,
                target_node: Uuid::new_v4(),
                relationship: RelationshipType::References,
                weight: 1.0 - (i as f64 * 0.1),
                properties: None,
            };
            repo.create(&edge).unwrap();
        }

        let edges = repo.find_by_source_node(&source).unwrap();
        assert_eq!(edges.len(), 3);
    }
}

