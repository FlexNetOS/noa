//! Digest Source Repository
//!
//! T162: Implement DigestSource repository
//! §3.4: Digest Everything Pipeline
//! US4: Digest Everything Pipeline

use crate::db::Connection;
use crate::error::{DatabaseError, NoaError, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Row};
use serde_json::{Map, Value};
use uuid::Uuid;

/// Digest source entity
#[derive(Debug, Clone)]
pub struct DigestSource {
    pub id: Uuid,
    pub source_type: DigestSourceType,
    pub uri: String,
    pub name: String,
    pub status: DigestStatus,
    pub last_digest: Option<DateTime<Utc>>,
    pub version: Option<String>,
    pub profile: Option<Map<String, Value>>,
    pub sbom: Option<Map<String, Value>>,
    pub security_report: Option<Map<String, Value>>,
    pub stats: Option<Map<String, Value>>,
}

/// Digest source type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigestSourceType {
    Repository,
    File,
    Api,
    Document,
}

impl DigestSourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DigestSourceType::Repository => "repository",
            DigestSourceType::File => "file",
            DigestSourceType::Api => "api",
            DigestSourceType::Document => "document",
        }
    }

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "repository" => Ok(DigestSourceType::Repository),
            "file" => Ok(DigestSourceType::File),
            "api" => Ok(DigestSourceType::Api),
            "document" => Ok(DigestSourceType::Document),
            _ => Err(NoaError::Validation(crate::error::ValidationError::new(
                "type",
                format!("Invalid digest source type: {}", s),
                "INVALID_TYPE",
            ))),
        }
    }
}

/// Digest status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigestStatus {
    Pending,
    Fetching,
    Parsing,
    Analyzing,
    Complete,
    Failed,
}

impl DigestStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DigestStatus::Pending => "pending",
            DigestStatus::Fetching => "fetching",
            DigestStatus::Parsing => "parsing",
            DigestStatus::Analyzing => "analyzing",
            DigestStatus::Complete => "complete",
            DigestStatus::Failed => "failed",
        }
    }

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "pending" => Ok(DigestStatus::Pending),
            "fetching" => Ok(DigestStatus::Fetching),
            "parsing" => Ok(DigestStatus::Parsing),
            "analyzing" => Ok(DigestStatus::Analyzing),
            "complete" => Ok(DigestStatus::Complete),
            "failed" => Ok(DigestStatus::Failed),
            _ => Err(NoaError::Validation(crate::error::ValidationError::new(
                "status",
                format!("Invalid digest status: {}", s),
                "INVALID_STATUS",
            ))),
        }
    }
}

/// Digest source repository
pub struct DigestRepository {
    conn: Connection,
}

impl DigestRepository {
    /// Create a new digest repository
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Create a new digest source
    pub fn create(&self, source: &DigestSource) -> Result<Uuid> {
        let profile_json = source
            .profile
            .as_ref()
            .map(|p| serde_json::to_string(p))
            .transpose()
            .map_err(|e| NoaError::Serialization(format!("Failed to serialize profile: {}", e)))?;

        let sbom_json = source
            .sbom
            .as_ref()
            .map(|s| serde_json::to_string(s))
            .transpose()
            .map_err(|e| NoaError::Serialization(format!("Failed to serialize SBOM: {}", e)))?;

        let security_report_json = source
            .security_report
            .as_ref()
            .map(|s| serde_json::to_string(s))
            .transpose()
            .map_err(|e| {
                NoaError::Serialization(format!("Failed to serialize security report: {}", e))
            })?;

        let stats_json = source
            .stats
            .as_ref()
            .map(|s| serde_json::to_string(s))
            .transpose()
            .map_err(|e| NoaError::Serialization(format!("Failed to serialize stats: {}", e)))?;

        self.conn
            .execute(
                r#"
                INSERT INTO digest_source (
                    id, type, uri, name, status, last_digest, version,
                    profile, sbom, security_report, stats
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                "#,
                params![
                    source.id.to_string(),
                    source.source_type.as_str(),
                    source.uri,
                    source.name,
                    source.status.as_str(),
                    source.last_digest.map(|d| d.to_rfc3339()),
                    source.version,
                    profile_json,
                    sbom_json,
                    security_report_json,
                    stats_json,
                ],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "INSERT INTO digest_source".to_string(),
                    error: e.to_string(),
                })
            })?;

        Ok(source.id)
    }

    /// Find digest source by ID
    pub fn find_by_id(&self, id: &Uuid) -> Result<Option<DigestSource>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, type, uri, name, status, last_digest, version,
                       profile, sbom, security_report, stats
                FROM digest_source
                WHERE id = ?1
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM digest_source".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut rows = stmt
            .query_map(params![id.to_string()], |row| self.row_to_source(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM digest_source".to_string(),
                    error: e.to_string(),
                })
            })?;

        match rows.next() {
            Some(Ok(source)) => Ok(Some(source)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT FROM digest_source".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// Find digest source by URI
    pub fn find_by_uri(&self, uri: &str) -> Result<Option<DigestSource>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, type, uri, name, status, last_digest, version,
                       profile, sbom, security_report, stats
                FROM digest_source
                WHERE uri = ?1
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM digest_source".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut rows = stmt
            .query_map(params![uri], |row| self.row_to_source(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM digest_source".to_string(),
                    error: e.to_string(),
                })
            })?;

        match rows.next() {
            Some(Ok(source)) => Ok(Some(source)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT FROM digest_source".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// Update an existing digest source
    pub fn update(&self, source: &DigestSource) -> Result<()> {
        let profile_json = source
            .profile
            .as_ref()
            .map(|p| serde_json::to_string(p))
            .transpose()
            .map_err(|e| NoaError::Serialization(format!("Failed to serialize profile: {}", e)))?;

        let sbom_json = source
            .sbom
            .as_ref()
            .map(|s| serde_json::to_string(s))
            .transpose()
            .map_err(|e| NoaError::Serialization(format!("Failed to serialize SBOM: {}", e)))?;

        let security_report_json = source
            .security_report
            .as_ref()
            .map(|s| serde_json::to_string(s))
            .transpose()
            .map_err(|e| {
                NoaError::Serialization(format!("Failed to serialize security report: {}", e))
            })?;

        let stats_json = source
            .stats
            .as_ref()
            .map(|s| serde_json::to_string(s))
            .transpose()
            .map_err(|e| NoaError::Serialization(format!("Failed to serialize stats: {}", e)))?;

        let rows_affected = self
            .conn
            .execute(
                r#"
                UPDATE digest_source SET
                    type = ?1,
                    uri = ?2,
                    name = ?3,
                    status = ?4,
                    last_digest = ?5,
                    version = ?6,
                    profile = ?7,
                    sbom = ?8,
                    security_report = ?9,
                    stats = ?10
                WHERE id = ?11
                "#,
                params![
                    source.source_type.as_str(),
                    source.uri,
                    source.name,
                    source.status.as_str(),
                    source.last_digest.map(|d| d.to_rfc3339()),
                    source.version,
                    profile_json,
                    sbom_json,
                    security_report_json,
                    stats_json,
                    source.id.to_string(),
                ],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "UPDATE digest_source".to_string(),
                    error: e.to_string(),
                })
            })?;

        if rows_affected == 0 {
            return Err(NoaError::NotFound {
                resource: "digest_source".to_string(),
                id: source.id.to_string(),
            });
        }

        Ok(())
    }

    /// List all digest sources with pagination
    pub fn list(&self, offset: u64, limit: u64) -> Result<Vec<DigestSource>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, type, uri, name, status, last_digest, version,
                       profile, sbom, security_report, stats
                FROM digest_source
                ORDER BY last_digest DESC NULLS LAST
                LIMIT ?1 OFFSET ?2
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM digest_source".to_string(),
                    error: e.to_string(),
                })
            })?;

        let rows = stmt
            .query_map(params![limit as i64, offset as i64], |row| {
                self.row_to_source(row)
            })
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM digest_source".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut sources = Vec::new();
        for row in rows {
            sources.push(row?);
        }

        Ok(sources)
    }

    /// Convert database row to DigestSource entity
    fn row_to_source(&self, row: &Row) -> rusqlite::Result<DigestSource> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(0, "uuid".to_string(), rusqlite::types::Type::Text)
        })?;

        let type_str: String = row.get(1)?;
        let source_type = DigestSourceType::from_str(&type_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(1, "type".to_string(), rusqlite::types::Type::Text)
        })?;

        let uri: String = row.get(2)?;
        let name: String = row.get(3)?;

        let status_str: String = row.get(4)?;
        let status = DigestStatus::from_str(&status_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(4, "status".to_string(), rusqlite::types::Type::Text)
        })?;

        let last_digest_str: Option<String> = row.get(5)?;
        let last_digest = last_digest_str
            .map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .map_err(|_| {
                        rusqlite::Error::InvalidColumnType(
                            5,
                            "timestamp".to_string(),
                            rusqlite::types::Type::Text,
                        )
                    })?
                    .with_timezone(&Utc)
            })
            .transpose()?;

        let version: Option<String> = row.get(6)?;

        let profile_str: Option<String> = row.get(7)?;
        let profile = profile_str
            .map(|s| serde_json::from_str::<Map<String, Value>>(&s))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(7, "json".to_string(), rusqlite::types::Type::Text)
            })?;

        let sbom_str: Option<String> = row.get(8)?;
        let sbom = sbom_str
            .map(|s| serde_json::from_str::<Map<String, Value>>(&s))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(8, "json".to_string(), rusqlite::types::Type::Text)
            })?;

        let security_report_str: Option<String> = row.get(9)?;
        let security_report = security_report_str
            .map(|s| serde_json::from_str::<Map<String, Value>>(&s))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(
                    9,
                    "json".to_string(),
                    rusqlite::types::Type::Text,
                )
            })?;

        let stats_str: Option<String> = row.get(10)?;
        let stats = stats_str
            .map(|s| serde_json::from_str::<Map<String, Value>>(&s))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(10, "json".to_string(), rusqlite::types::Type::Text)
            })?;

        Ok(DigestSource {
            id,
            source_type,
            uri,
            name,
            status,
            last_digest,
            version,
            profile,
            sbom,
            security_report,
            stats,
        })
    }
}

