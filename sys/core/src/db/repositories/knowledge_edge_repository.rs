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
        let properties_json =
            edge.properties.as_ref().map(|p| serde_json::to_string(p)).transpose().map_err(
                |e| NoaError::Serialization(format!("Failed to serialize properties: {}", e)),
            )?;

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
            .query_map(params![source_node.to_string()], |row| {
                self.row_to_edge(row)
            })
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
            .query_map(params![target_node.to_string()], |row| {
                self.row_to_edge(row)
            })
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
            rusqlite::Error::InvalidColumnType(1, "uuid".to_string(), rusqlite::types::Type::Text)
        })?;

        let target_node_str: String = row.get(2)?;
        let target_node = Uuid::parse_str(&target_node_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(2, "uuid".to_string(), rusqlite::types::Type::Text)
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
                rusqlite::Error::InvalidColumnType(
                    5,
                    "json".to_string(),
                    rusqlite::types::Type::Text,
                )
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
