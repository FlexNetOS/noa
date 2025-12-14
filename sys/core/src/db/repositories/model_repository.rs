//! Model Repository
//!
//! T103: Implement Model entity repository
//! §3.2: Local-First & Offline-Capable
//! US2: Model management for neural runtime

use crate::db::Connection;
use crate::error::{DatabaseError, NoaError, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Row};
use serde_json::Value as JsonValue;
use uuid::Uuid;

/// Model entity representing registered AI models
#[derive(Debug, Clone)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub model_type: ModelType,
    pub provider: String,
    pub path: Option<String>,
    pub uri: Option<String>,
    pub size_bytes: Option<i64>,
    pub parameters: Option<String>,
    pub context_length: Option<i32>,
    pub license: Option<String>,
    pub config: JsonValue,
    pub status: ModelStatus,
    pub metrics: Option<JsonValue>,
}

/// Model type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    LLM,
    Embedding,
    Vision,
    Audio,
}

impl ModelType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModelType::LLM => "llm",
            ModelType::Embedding => "embedding",
            ModelType::Vision => "vision",
            ModelType::Audio => "audio",
        }
    }

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "llm" => Ok(ModelType::LLM),
            "embedding" => Ok(ModelType::Embedding),
            "vision" => Ok(ModelType::Vision),
            "audio" => Ok(ModelType::Audio),
            _ => Err(NoaError::Validation(crate::error::ValidationError::new(
                "model_type",
                format!("Invalid model type: {}", s),
                "INVALID_MODEL_TYPE",
            ))),
        }
    }
}

/// Model status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelStatus {
    Available,
    Downloading,
    Loading,
    Loaded,
    Error,
}

impl ModelStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModelStatus::Available => "available",
            ModelStatus::Downloading => "downloading",
            ModelStatus::Loading => "loading",
            ModelStatus::Loaded => "loaded",
            ModelStatus::Error => "error",
        }
    }

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "available" => Ok(ModelStatus::Available),
            "downloading" => Ok(ModelStatus::Downloading),
            "loading" => Ok(ModelStatus::Loading),
            "loaded" => Ok(ModelStatus::Loaded),
            "error" => Ok(ModelStatus::Error),
            _ => Err(NoaError::Validation(crate::error::ValidationError::new(
                "status",
                format!("Invalid model status: {}", s),
                "INVALID_MODEL_STATUS",
            ))),
        }
    }
}

/// Model repository for CRUD operations
pub struct ModelRepository {
    conn: Connection,
}

impl ModelRepository {
    /// Create a new model repository
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Create a new model entry
    pub fn create(&self, model: &Model) -> Result<Uuid> {
        let config_json = serde_json::to_string(&model.config).map_err(|e| {
            DatabaseError::QueryFailed {
                query: "create model".to_string(),
                error: format!("Failed to serialize config: {}", e),
            }
        })?;

        let metrics_json = model.metrics.as_ref().map(|m| {
            serde_json::to_string(m).map_err(|e| {
                DatabaseError::QueryFailed {
                    query: "create model".to_string(),
                    error: format!("Failed to serialize metrics: {}", e),
                }
            })
        }).transpose()?;

        self.conn
            .execute(
                r#"
                INSERT INTO model (
                    id, name, type, provider, path, uri, size_bytes,
                    parameters, context_length, license, config, status, metrics
                )
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
                "#,
                params![
                    model.id.to_string(),
                    model.name,
                    model.model_type.as_str(),
                    model.provider,
                    model.path,
                    model.uri,
                    model.size_bytes,
                    model.parameters,
                    model.context_length,
                    model.license,
                    config_json,
                    model.status.as_str(),
                    metrics_json,
                ],
            )
            .map_err(|e| {
                DatabaseError::QueryFailed {
                    query: "create model".to_string(),
                    error: e.to_string(),
                }
            })?;

        Ok(model.id)
    }

    /// Find model by ID
    pub fn find_by_id(&self, id: &Uuid) -> Result<Option<Model>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, name, type, provider, path, uri, size_bytes,
                       parameters, context_length, license, config, status, metrics
                FROM model
                WHERE id = ?1
                "#,
            )
            .map_err(|e| DatabaseError::QueryFailed {
                query: "find_by_id".to_string(),
                error: e.to_string(),
            })?;

        let result = stmt
            .query_row(params![id.to_string()], |row| self.row_to_model(row))
            .optional()
            .map_err(|e| DatabaseError::QueryFailed {
                query: "find_by_id".to_string(),
                error: e.to_string(),
            })?;

        Ok(result)
    }

    /// Find model by name
    pub fn find_by_name(&self, name: &str) -> Result<Option<Model>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, name, type, provider, path, uri, size_bytes,
                       parameters, context_length, license, config, status, metrics
                FROM model
                WHERE name = ?1
                "#,
            )
            .map_err(|e| DatabaseError::QueryFailed {
                query: "find_by_name".to_string(),
                error: e.to_string(),
            })?;

        let result = stmt
            .query_row(params![name], |row| self.row_to_model(row))
            .optional()
            .map_err(|e| DatabaseError::QueryFailed {
                query: "find_by_name".to_string(),
                error: e.to_string(),
            })?;

        Ok(result)
    }

    /// Find all models
    pub fn find_all(&self) -> Result<Vec<Model>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, name, type, provider, path, uri, size_bytes,
                       parameters, context_length, license, config, status, metrics
                FROM model
                ORDER BY name
                "#,
            )
            .map_err(|e| DatabaseError::QueryFailed {
                query: "find_all".to_string(),
                error: e.to_string(),
            })?;

        let models = stmt
            .query_map([], |row| self.row_to_model(row))
            .map_err(|e| DatabaseError::QueryFailed {
                query: "find_all".to_string(),
                error: e.to_string(),
            })?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| DatabaseError::QueryFailed {
                query: "find_all".to_string(),
                error: e.to_string(),
            })?;

        Ok(models)
    }

    /// Find models by status
    pub fn find_by_status(&self, status: ModelStatus) -> Result<Vec<Model>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, name, type, provider, path, uri, size_bytes,
                       parameters, context_length, license, config, status, metrics
                FROM model
                WHERE status = ?1
                ORDER BY name
                "#,
            )
            .map_err(|e| DatabaseError::QueryFailed {
                query: "find_by_status".to_string(),
                error: e.to_string(),
            })?;

        let models = stmt
            .query_map(params![status.as_str()], |row| self.row_to_model(row))
            .map_err(|e| DatabaseError::QueryFailed {
                query: "find_by_status".to_string(),
                error: e.to_string(),
            })?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| DatabaseError::QueryFailed {
                query: "find_by_status".to_string(),
                error: e.to_string(),
            })?;

        Ok(models)
    }

    /// Update model
    pub fn update(&self, model: &Model) -> Result<()> {
        let config_json = serde_json::to_string(&model.config).map_err(|e| {
            DatabaseError::QueryFailed {
                query: "update model".to_string(),
                error: format!("Failed to serialize config: {}", e),
            }
        })?;

        let metrics_json = model.metrics.as_ref().map(|m| {
            serde_json::to_string(m).map_err(|e| {
                DatabaseError::QueryFailed {
                    query: "update model".to_string(),
                    error: format!("Failed to serialize metrics: {}", e),
                }
            })
        }).transpose()?;

        let rows_affected = self
            .conn
            .execute(
                r#"
                UPDATE model
                SET name = ?2, type = ?3, provider = ?4, path = ?5, uri = ?6,
                    size_bytes = ?7, parameters = ?8, context_length = ?9,
                    license = ?10, config = ?11, status = ?12, metrics = ?13
                WHERE id = ?1
                "#,
                params![
                    model.id.to_string(),
                    model.name,
                    model.model_type.as_str(),
                    model.provider,
                    model.path,
                    model.uri,
                    model.size_bytes,
                    model.parameters,
                    model.context_length,
                    model.license,
                    config_json,
                    model.status.as_str(),
                    metrics_json,
                ],
            )
            .map_err(|e| DatabaseError::QueryFailed {
                query: "update model".to_string(),
                error: e.to_string(),
            })?;

        if rows_affected == 0 {
            return Err(NoaError::NotFound {
                resource: "Model".to_string(),
                id: model.id.to_string(),
            });
        }

        Ok(())
    }

    /// Update model status
    pub fn update_status(&self, id: &Uuid, status: ModelStatus) -> Result<()> {
        let rows_affected = self
            .conn
            .execute(
                "UPDATE model SET status = ?1 WHERE id = ?2",
                params![status.as_str(), id.to_string()],
            )
            .map_err(|e| DatabaseError::QueryFailed {
                query: "update_status".to_string(),
                error: e.to_string(),
            })?;

        if rows_affected == 0 {
            return Err(NoaError::NotFound {
                resource: "Model".to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }

    /// Delete model by ID
    pub fn delete(&self, id: &Uuid) -> Result<bool> {
        let rows_affected = self
            .conn
            .execute("DELETE FROM model WHERE id = ?1", params![id.to_string()])
            .map_err(|e| DatabaseError::QueryFailed {
                query: "delete model".to_string(),
                error: e.to_string(),
            })?;

        Ok(rows_affected > 0)
    }

    /// Count all models
    pub fn count(&self) -> Result<u64> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM model", [], |row| row.get(0))
            .map_err(|e| DatabaseError::QueryFailed {
                query: "count models".to_string(),
                error: e.to_string(),
            })?;

        Ok(count as u64)
    }

    /// Convert database row to Model entity
    fn row_to_model(&self, row: &Row) -> rusqlite::Result<Model> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|e| {
            rusqlite::Error::InvalidColumnType(0, "uuid".to_string(), rusqlite::types::Type::Text)
        })?;

        let model_type_str: String = row.get(2)?;
        let model_type = ModelType::from_str(&model_type_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(2, "model_type".to_string(), rusqlite::types::Type::Text)
        })?;

        let status_str: String = row.get(11)?;
        let status = ModelStatus::from_str(&status_str).map_err(|_| {
            rusqlite::Error::InvalidColumnType(11, "status".to_string(), rusqlite::types::Type::Text)
        })?;

        let config_str: String = row.get(10)?;
        let config: JsonValue = serde_json::from_str(&config_str).unwrap_or(JsonValue::Object(serde_json::Map::new()));

        let metrics: Option<JsonValue> = row.get::<_, Option<String>>(12)?
            .map(|s| serde_json::from_str(&s).unwrap_or(JsonValue::Null))
            .filter(|v| !v.is_null());

        Ok(Model {
            id,
            name: row.get(1)?,
            model_type,
            provider: row.get(3)?,
            path: row.get(4)?,
            uri: row.get(5)?,
            size_bytes: row.get(6)?,
            parameters: row.get(7)?,
            context_length: row.get(8)?,
            license: row.get(9)?,
            config,
            status,
            metrics,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_database;
    use tempfile::tempdir;

    #[test]
    fn test_model_repository_crud() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = init_database(&db_path).unwrap();

        // Create model table (simplified for test)
        conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS model (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                type TEXT NOT NULL,
                provider TEXT NOT NULL,
                path TEXT,
                uri TEXT,
                size_bytes INTEGER,
                parameters TEXT,
                context_length INTEGER,
                license TEXT,
                config TEXT NOT NULL,
                status TEXT NOT NULL,
                metrics TEXT
            )
            "#,
            [],
        ).unwrap();

        let repo = ModelRepository::new(conn);

        let model = Model {
            id: Uuid::new_v4(),
            name: "test-model".to_string(),
            model_type: ModelType::LLM,
            provider: "llama.cpp".to_string(),
            path: Some("/path/to/model.gguf".to_string()),
            uri: None,
            size_bytes: Some(1000000),
            parameters: Some("7B".to_string()),
            context_length: Some(2048),
            license: Some("MIT".to_string()),
            config: serde_json::json!({}),
            status: ModelStatus::Available,
            metrics: None,
        };

        let id = repo.create(&model).unwrap();
        assert_eq!(id, model.id);

        let found = repo.find_by_id(&id).unwrap().unwrap();
        assert_eq!(found.name, "test-model");
        assert_eq!(found.status, ModelStatus::Available);

        repo.update_status(&id, ModelStatus::Loaded).unwrap();
        let updated = repo.find_by_id(&id).unwrap().unwrap();
        assert_eq!(updated.status, ModelStatus::Loaded);

        assert!(repo.delete(&id).unwrap());
        assert!(repo.find_by_id(&id).unwrap().is_none());
    }
}

