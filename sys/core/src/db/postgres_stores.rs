//! PostgreSQL Store Implementations
//!
//! Implements the backend-agnostic traits for PostgreSQL with RuVector.
//! §3.2: PostgreSQL backend implementation
//!
//! # Architecture
//!
//! This module provides PostgreSQL implementations for all store traits defined
//! in the `backend` module. All stores use runtime queries (not compile-time macros)
//! to allow offline compilation without a database connection.
//!
//! # Vector Storage
//!
//! Uses RuVector extension for similarity search with configsurable dimensions.

use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::error::{Result, DatabaseError, NoaError, ValidationError};

/// Default embedding dimension for vector storage.
/// Matches nomic-embed-text (384) and many other small embedding models.
pub const DEFAULT_VECTOR_DIMENSION: usize = 384;
use super::backend::{
    DatabaseKind, DatabaseBackend, DatabaseHealth, HealthCheckResult,
    MemoryStore, ModelStore, TaskStore, AgentStore, AuditLogStore, VectorStore,
    VectorSearchResult,
};
use super::repository::PaginatedResult;
use super::repositories::{
    Memory, MemoryType,
    Model, ModelStatus, ModelType,
    Task, Agent, AgentLog,
};

/// PostgreSQL-based memory store.
pub struct PgMemoryStore {
    pool: PgPool,
}

impl PgMemoryStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MemoryStore for PgMemoryStore {
    async fn create(&self, memory: &Memory) -> Result<Uuid> {
        let tags: Vec<String> = memory.tags.iter().cloned().collect();
        let metadata_json = memory.metadata.as_ref()
            .and_then(|m| serde_json::to_value(m).ok());

        sqlx::query(
            r#"
            INSERT INTO memory (id, created_at, updated_at, memory_type, content, metadata,
                               source_agent, parent_id, tags, embedding_id, checksum)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(memory.id)
        .bind(memory.created_at)
        .bind(memory.updated_at)
        .bind(memory.memory_type.as_str())
        .bind(&memory.content)
        .bind(&metadata_json)
        .bind(memory.source_agent)
        .bind(memory.parent_id)
        .bind(&tags)
        .bind(memory.embedding_id)
        .bind(&memory.checksum)
        .execute(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "INSERT memory".to_string(),
            error: e.to_string(),
        }))?;

        Ok(memory.id)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Memory>> {
        let row = sqlx::query(
            r#"
            SELECT id, created_at, updated_at, memory_type, content, metadata,
                   source_agent, parent_id, tags, embedding_id, checksum
            FROM memory
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT memory".to_string(),
            error: e.to_string(),
        }))?;

        match row {
            Some(r) => {
                let memory_type_str: String = r.get("memory_type");
                let memory_type = MemoryType::from_str(&memory_type_str)
                    .unwrap_or(MemoryType::Interaction);
                let metadata: Option<serde_json::Value> = r.get("metadata");
                let metadata = metadata.and_then(|v| serde_json::from_value(v).ok());
                let tags_vec: Option<Vec<String>> = r.get("tags");
                let tags = tags_vec.unwrap_or_default().into_iter().collect();

                Ok(Some(Memory {
                    id: r.get("id"),
                    created_at: r.get("created_at"),
                    updated_at: r.get("updated_at"),
                    memory_type,
                    content: r.get("content"),
                    metadata,
                    source_agent: r.get("source_agent"),
                    parent_id: r.get("parent_id"),
                    tags,
                    embedding_id: r.get("embedding_id"),
                    checksum: r.get("checksum"),
                }))
            }
            None => Ok(None),
        }
    }

    async fn update(&self, memory: &Memory) -> Result<()> {
        let tags: Vec<String> = memory.tags.iter().cloned().collect();
        let metadata_json = memory.metadata.as_ref()
            .and_then(|m| serde_json::to_value(m).ok());

        let result = sqlx::query(
            r#"
            UPDATE memory SET
                updated_at = $2,
                memory_type = $3,
                content = $4,
                metadata = $5,
                source_agent = $6,
                parent_id = $7,
                tags = $8,
                embedding_id = $9,
                checksum = $10
            WHERE id = $1
            "#,
        )
        .bind(memory.id)
        .bind(memory.updated_at)
        .bind(memory.memory_type.as_str())
        .bind(&memory.content)
        .bind(&metadata_json)
        .bind(memory.source_agent)
        .bind(memory.parent_id)
        .bind(&tags)
        .bind(memory.embedding_id)
        .bind(&memory.checksum)
        .execute(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "UPDATE memory".to_string(),
            error: e.to_string(),
        }))?;

        if result.rows_affected() == 0 {
            return Err(NoaError::NotFound {
                resource: "memory".to_string(),
                id: memory.id.to_string(),
            });
        }

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM memory WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "DELETE memory".to_string(),
                error: e.to_string(),
            }))?;

        Ok(result.rows_affected() > 0)
    }

    async fn list(&self, offset: u64, limit: u64) -> Result<PaginatedResult<Memory>> {
        let rows = sqlx::query(
            r#"
            SELECT id, created_at, updated_at, memory_type, content, metadata,
                   source_agent, parent_id, tags, embedding_id, checksum
            FROM memory
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT memories".to_string(),
            error: e.to_string(),
        }))?;

        let total = self.count().await?;
        let items = rows.into_iter().map(|r| {
            let memory_type_str: String = r.get("memory_type");
            let memory_type = MemoryType::from_str(&memory_type_str)
                .unwrap_or(MemoryType::Interaction);
            let metadata: Option<serde_json::Value> = r.get("metadata");
            let metadata = metadata.and_then(|v| serde_json::from_value(v).ok());
            let tags_vec: Option<Vec<String>> = r.get("tags");
            let tags = tags_vec.unwrap_or_default().into_iter().collect();

            Memory {
                id: r.get("id"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                memory_type,
                content: r.get("content"),
                metadata,
                source_agent: r.get("source_agent"),
                parent_id: r.get("parent_id"),
                tags,
                embedding_id: r.get("embedding_id"),
                checksum: r.get("checksum"),
            }
        }).collect();

        Ok(PaginatedResult::new(items, total, offset, limit))
    }

    async fn find_by_type(&self, memory_type: MemoryType, offset: u64, limit: u64) -> Result<PaginatedResult<Memory>> {
        let rows = sqlx::query(
            r#"
            SELECT id, created_at, updated_at, memory_type, content, metadata,
                   source_agent, parent_id, tags, embedding_id, checksum
            FROM memory
            WHERE memory_type = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(memory_type.as_str())
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT memories by type".to_string(),
            error: e.to_string(),
        }))?;

        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM memory WHERE memory_type = $1",
        )
        .bind(memory_type.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "COUNT memories".to_string(),
            error: e.to_string(),
        }))?;

        let items = rows.into_iter().map(|r| {
            let memory_type_str: String = r.get("memory_type");
            let mtype = MemoryType::from_str(&memory_type_str).unwrap_or(MemoryType::Interaction);
            let metadata: Option<serde_json::Value> = r.get("metadata");
            let metadata = metadata.and_then(|v| serde_json::from_value(v).ok());
            let tags_vec: Option<Vec<String>> = r.get("tags");
            let tags = tags_vec.unwrap_or_default().into_iter().collect();

            Memory {
                id: r.get("id"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                memory_type: mtype,
                content: r.get("content"),
                metadata,
                source_agent: r.get("source_agent"),
                parent_id: r.get("parent_id"),
                tags,
                embedding_id: r.get("embedding_id"),
                checksum: r.get("checksum"),
            }
        }).collect();

        Ok(PaginatedResult::new(items, count.0 as u64, offset, limit))
    }

    async fn search(&self, query: &str, limit: u64) -> Result<Vec<Memory>> {
        // Full-text search using pg_trgm
        let like_pattern = format!("%{}%", query);
        let rows = sqlx::query(
            r#"
            SELECT id, created_at, updated_at, memory_type, content, metadata,
                   source_agent, parent_id, tags, embedding_id, checksum
            FROM memory
            WHERE content ILIKE $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(&like_pattern)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "search memories".to_string(),
            error: e.to_string(),
        }))?;

        Ok(rows.into_iter().map(|r| {
            let memory_type_str: String = r.get("memory_type");
            let memory_type = MemoryType::from_str(&memory_type_str).unwrap_or(MemoryType::Interaction);
            let metadata: Option<serde_json::Value> = r.get("metadata");
            let metadata = metadata.and_then(|v| serde_json::from_value(v).ok());
            let tags_vec: Option<Vec<String>> = r.get("tags");
            let tags = tags_vec.unwrap_or_default().into_iter().collect();

            Memory {
                id: r.get("id"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                memory_type,
                content: r.get("content"),
                metadata,
                source_agent: r.get("source_agent"),
                parent_id: r.get("parent_id"),
                tags,
                embedding_id: r.get("embedding_id"),
                checksum: r.get("checksum"),
            }
        }).collect())
    }

    async fn count(&self) -> Result<u64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM memory")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "COUNT memories".to_string(),
                error: e.to_string(),
            }))?;

        Ok(count.0 as u64)
    }
}

/// PostgreSQL-based model store.
pub struct PgModelStore {
    pool: PgPool,
}

impl PgModelStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ModelStore for PgModelStore {
    async fn list(&self) -> Result<Vec<Model>> {
        let rows = sqlx::query(
            r#"
            SELECT id, created_at, name, provider, kind, metadata
            FROM model
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT models".to_string(),
            error: e.to_string(),
        }))?;

        Ok(rows.into_iter().map(|r| {
            let id: Uuid = r.get("id");
            let name: String = r.get("name");
            let provider: Option<String> = r.get("provider");
            let kind: Option<String> = r.get("kind");
            let metadata: Option<serde_json::Value> = r.get("metadata");
            
            let model_type = kind.as_deref()
                .map(|k| match k {
                    "llm" => ModelType::LLM,
                    "embedding" => ModelType::Embedding,
                    "vision" => ModelType::Vision,
                    "audio" => ModelType::Audio,
                    _ => ModelType::LLM,
                })
                .unwrap_or(ModelType::LLM);

            Model {
                id,
                name,
                model_type,
                provider: provider.unwrap_or_default(),
                path: None,
                uri: None,
                size_bytes: None,
                parameters: None,
                context_length: None,
                license: None,
                configs: metadata.unwrap_or(serde_json::json!({})),
                status: ModelStatus::Available,
                metrics: None,
            }
        }).collect())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Model>> {
        let row = sqlx::query(
            r#"
            SELECT id, created_at, name, provider, kind, metadata
            FROM model
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT model".to_string(),
            error: e.to_string(),
        }))?;

        Ok(row.map(|r| {
            let id: Uuid = r.get("id");
            let name: String = r.get("name");
            let provider: Option<String> = r.get("provider");
            let kind: Option<String> = r.get("kind");
            let metadata: Option<serde_json::Value> = r.get("metadata");
            
            let model_type = kind.as_deref()
                .map(|k| match k {
                    "llm" => ModelType::LLM,
                    "embedding" => ModelType::Embedding,
                    "vision" => ModelType::Vision,
                    "audio" => ModelType::Audio,
                    _ => ModelType::LLM,
                })
                .unwrap_or(ModelType::LLM);

            Model {
                id,
                name,
                model_type,
                provider: provider.unwrap_or_default(),
                path: None,
                uri: None,
                size_bytes: None,
                parameters: None,
                context_length: None,
                license: None,
                configs: metadata.unwrap_or(serde_json::json!({})),
                status: ModelStatus::Available,
                metrics: None,
            }
        }))
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Model>> {
        let row = sqlx::query(
            r#"
            SELECT id, created_at, name, provider, kind, metadata
            FROM model
            WHERE name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT model by name".to_string(),
            error: e.to_string(),
        }))?;

        Ok(row.map(|r| {
            let id: Uuid = r.get("id");
            let model_name: String = r.get("name");
            let provider: Option<String> = r.get("provider");
            let kind: Option<String> = r.get("kind");
            let metadata: Option<serde_json::Value> = r.get("metadata");
            
            let model_type = kind.as_deref()
                .map(|k| match k {
                    "llm" => ModelType::LLM,
                    "embedding" => ModelType::Embedding,
                    "vision" => ModelType::Vision,
                    "audio" => ModelType::Audio,
                    _ => ModelType::LLM,
                })
                .unwrap_or(ModelType::LLM);

            Model {
                id,
                name: model_name,
                model_type,
                provider: provider.unwrap_or_default(),
                path: None,
                uri: None,
                size_bytes: None,
                parameters: None,
                context_length: None,
                license: None,
                configs: metadata.unwrap_or(serde_json::json!({})),
                status: ModelStatus::Available,
                metrics: None,
            }
        }))
    }

    async fn create(&self, model: &Model) -> Result<Uuid> {
        sqlx::query(
            r#"
            INSERT INTO model (id, name, provider, kind, metadata)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(model.id)
        .bind(&model.name)
        .bind(&model.provider)
        .bind(model.model_type.as_str())
        .bind(&model.configs)
        .execute(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "INSERT model".to_string(),
            error: e.to_string(),
        }))?;

        Ok(model.id)
    }

    async fn update_status(&self, id: &Uuid, status: ModelStatus) -> Result<()> {
        // Status stored in metadata for PG schema compatibility
        let status_json = serde_json::json!({ "status": status.as_str() });
        let result = sqlx::query(
            r#"
            UPDATE model SET metadata = COALESCE(metadata, '{}'::jsonb) || $2
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(&status_json)
        .execute(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "UPDATE model status".to_string(),
            error: e.to_string(),
        }))?;

        if result.rows_affected() == 0 {
            return Err(NoaError::NotFound {
                resource: "model".to_string(),
                id: id.to_string(),
            });
        }

        Ok(())
    }

    async fn get_status(&self, id: &Uuid) -> Result<Option<ModelStatus>> {
        let row: Option<(Option<serde_json::Value>,)> = sqlx::query_as(
            "SELECT metadata FROM model WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT model status".to_string(),
            error: e.to_string(),
        }))?;

        Ok(row.and_then(|(metadata,)| {
            metadata.and_then(|m| {
                m.get("status").and_then(|s| s.as_str()).map(|s| {
                    match s {
                        "available" => ModelStatus::Available,
                        "downloading" => ModelStatus::Downloading,
                        "loading" => ModelStatus::Loading,
                        "loaded" => ModelStatus::Loaded,
                        "error" => ModelStatus::Error,
                        _ => ModelStatus::Available,
                    }
                })
            })
        }))
    }

    async fn count_by_status(&self, status: ModelStatus) -> Result<u64> {
        // Count by status in metadata using JSONB operator
        let status_str = status.as_str();
        let count: (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*) FROM model WHERE metadata->>'status' = $1"#,
        )
        .bind(status_str)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "COUNT models by status".to_string(),
            error: e.to_string(),
        }))?;

        Ok(count.0 as u64)
    }
}

/// PostgreSQL-based task store.
pub struct PgTaskStore {
    pool: PgPool,
}

impl PgTaskStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskStore for PgTaskStore {
    async fn create(&self, task: &Task) -> Result<Uuid> {
        let id = Uuid::new_v4();
        let metadata = task.payload.as_ref().map(|p| serde_json::json!({ "payload": p }));
        
        sqlx::query(
            r#"
            INSERT INTO task (id, title, description, status, priority, metadata)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(id)
        .bind(&task.title)
        .bind(None::<String>) // description
        .bind(&task.status)
        .bind(None::<i32>) // priority
        .bind(&metadata)
        .execute(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "INSERT task".to_string(),
            error: e.to_string(),
        }))?;

        Ok(id)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Task>> {
        let row = sqlx::query(
            r#"
            SELECT id, title, status, metadata
            FROM task
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT task".to_string(),
            error: e.to_string(),
        }))?;

        Ok(row.map(|r| {
            let metadata: Option<serde_json::Value> = r.get("metadata");
            Task {
                id: r.get("id"),
                agent_id: None,
                title: r.get("title"),
                status: r.get("status"),
                payload: metadata.and_then(|m| m.get("payload").and_then(|p| p.as_str().map(String::from))),
            }
        }))
    }

    async fn update(&self, task: &Task) -> Result<()> {
        let metadata = task.payload.as_ref().map(|p| serde_json::json!({ "payload": p }));
        
        let result = sqlx::query(
            r#"
            UPDATE task SET
                title = $2,
                status = $3,
                metadata = $4
            WHERE id = $1
            "#,
        )
        .bind(task.id)
        .bind(&task.title)
        .bind(&task.status)
        .bind(&metadata)
        .execute(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "UPDATE task".to_string(),
            error: e.to_string(),
        }))?;

        if result.rows_affected() == 0 {
            return Err(NoaError::NotFound {
                resource: "task".to_string(),
                id: task.id.to_string(),
            });
        }

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM task WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "DELETE task".to_string(),
                error: e.to_string(),
            }))?;

        Ok(result.rows_affected() > 0)
    }

    async fn list(&self, offset: u64, limit: u64) -> Result<PaginatedResult<Task>> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, status, metadata
            FROM task
            ORDER BY updated_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT tasks".to_string(),
            error: e.to_string(),
        }))?;

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM task")
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        let items = rows.into_iter().map(|r| {
            let metadata: Option<serde_json::Value> = r.get("metadata");
            Task {
                id: r.get("id"),
                agent_id: None,
                title: r.get("title"),
                status: r.get("status"),
                payload: metadata.and_then(|m| m.get("payload").and_then(|p| p.as_str().map(String::from))),
            }
        }).collect();

        Ok(PaginatedResult::new(items, count as u64, offset, limit))
    }

    async fn find_by_status(&self, status: &str, offset: u64, limit: u64) -> Result<PaginatedResult<Task>> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, status, metadata
            FROM task
            WHERE status = $1
            ORDER BY updated_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(status)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT tasks by status".to_string(),
            error: e.to_string(),
        }))?;

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM task WHERE status = $1")
            .bind(status)
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        let items = rows.into_iter().map(|r| {
            let metadata: Option<serde_json::Value> = r.get("metadata");
            Task {
                id: r.get("id"),
                agent_id: None,
                title: r.get("title"),
                status: r.get("status"),
                payload: metadata.and_then(|m| m.get("payload").and_then(|p| p.as_str().map(String::from))),
            }
        }).collect();

        Ok(PaginatedResult::new(items, count as u64, offset, limit))
    }

    async fn count(&self) -> Result<u64> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM task")
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        Ok(count as u64)
    }
}

/// PostgreSQL-based agent store.
pub struct PgAgentStore {
    pool: PgPool,
}

impl PgAgentStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AgentStore for PgAgentStore {
    async fn create(&self, agent: &Agent) -> Result<Uuid> {
        let id = Uuid::new_v4();
        let metadata = serde_json::json!({
            "description": agent.description,
            "status": agent.status,
        });
        
        sqlx::query(
            r#"
            INSERT INTO agent (id, name, metadata)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(id)
        .bind(&agent.name)
        .bind(&metadata)
        .execute(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "INSERT agent".to_string(),
            error: e.to_string(),
        }))?;

        Ok(id)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Agent>> {
        let row = sqlx::query("SELECT id, name, metadata FROM agent WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT agent".to_string(),
                error: e.to_string(),
            }))?;

        Ok(row.map(|r| {
            let metadata: Option<serde_json::Value> = r.get("metadata");
            Agent {
                id: r.get("id"),
                name: r.get("name"),
                description: metadata.as_ref()
                    .and_then(|m| m.get("description").and_then(|d| d.as_str().map(String::from))),
                status: metadata.as_ref()
                    .and_then(|m| m.get("status").and_then(|s| s.as_str()))
                    .unwrap_or("active")
                    .to_string(),
            }
        }))
    }

    async fn list(&self) -> Result<Vec<Agent>> {
        let rows = sqlx::query("SELECT id, name, metadata FROM agent ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT agents".to_string(),
                error: e.to_string(),
            }))?;

        Ok(rows.into_iter().map(|r| {
            let metadata: Option<serde_json::Value> = r.get("metadata");
            Agent {
                id: r.get("id"),
                name: r.get("name"),
                description: metadata.as_ref()
                    .and_then(|m| m.get("description").and_then(|d| d.as_str().map(String::from))),
                status: metadata.as_ref()
                    .and_then(|m| m.get("status").and_then(|s| s.as_str()))
                    .unwrap_or("active")
                    .to_string(),
            }
        }).collect())
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Agent>> {
        let row = sqlx::query("SELECT id, name, metadata FROM agent WHERE name = $1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT agent by name".to_string(),
                error: e.to_string(),
            }))?;

        Ok(row.map(|r| {
            let metadata: Option<serde_json::Value> = r.get("metadata");
            Agent {
                id: r.get("id"),
                name: r.get("name"),
                description: metadata.as_ref()
                    .and_then(|m| m.get("description").and_then(|d| d.as_str().map(String::from))),
                status: metadata.as_ref()
                    .and_then(|m| m.get("status").and_then(|s| s.as_str()))
                    .unwrap_or("active")
                    .to_string(),
            }
        }))
    }

    async fn update(&self, agent: &Agent) -> Result<()> {
        let metadata = serde_json::json!({
            "description": agent.description,
            "status": agent.status,
        });
        
        let result = sqlx::query(
            r#"
            UPDATE agent SET
                name = $2,
                metadata = $3
            WHERE id = $1
            "#,
        )
        .bind(agent.id)
        .bind(&agent.name)
        .bind(&metadata)
        .execute(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "UPDATE agent".to_string(),
            error: e.to_string(),
        }))?;

        if result.rows_affected() == 0 {
            return Err(NoaError::NotFound {
                resource: "agent".to_string(),
                id: agent.id.to_string(),
            });
        }

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM agent WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "DELETE agent".to_string(),
                error: e.to_string(),
            }))?;

        Ok(result.rows_affected() > 0)
    }
}

/// PostgreSQL-based audit log store.
pub struct PgAuditLogStore {
    pool: PgPool,
}

impl PgAuditLogStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditLogStore for PgAuditLogStore {
    async fn create(&self, log: &AgentLog) -> Result<Uuid> {
        let id = Uuid::new_v4();
        let fields_json = log.fields.as_ref().map(|f| serde_json::json!({ "data": f }));
        
        sqlx::query(
            r#"
            INSERT INTO agent_log (id, agent_id, level, message, fields)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(id)
        .bind(log.agent_id)
        .bind(&log.level)
        .bind(&log.message)
        .bind(&fields_json)
        .execute(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "INSERT agent_log".to_string(),
            error: e.to_string(),
        }))?;

        Ok(id)
    }

    async fn find_by_agent(&self, agent_id: &Uuid, offset: u64, limit: u64) -> Result<PaginatedResult<AgentLog>> {
        let rows = sqlx::query(
            r#"
            SELECT id, agent_id, level, message, fields, created_at
            FROM agent_log
            WHERE agent_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(agent_id)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT agent_logs".to_string(),
            error: e.to_string(),
        }))?;

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_log WHERE agent_id = $1")
            .bind(agent_id)
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        let items = rows.into_iter().map(|r| {
            let fields: Option<serde_json::Value> = r.get("fields");
            let agent_id_val: Option<Uuid> = r.get("agent_id");
            AgentLog {
                id: r.get("id"),
                agent_id: agent_id_val.unwrap_or_else(Uuid::nil),
                level: r.get("level"),
                message: r.get("message"),
                fields: fields.and_then(|f| f.get("data").and_then(|d| d.as_str().map(String::from))),
            }
        }).collect();

        Ok(PaginatedResult::new(items, count as u64, offset, limit))
    }

    async fn find_by_level(&self, level: &str, offset: u64, limit: u64) -> Result<PaginatedResult<AgentLog>> {
        let rows = sqlx::query(
            r#"
            SELECT id, agent_id, level, message, fields, created_at
            FROM agent_log
            WHERE level = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(level)
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT agent_logs by level".to_string(),
            error: e.to_string(),
        }))?;

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_log WHERE level = $1")
            .bind(level)
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        let items = rows.into_iter().map(|r| {
            let fields: Option<serde_json::Value> = r.get("fields");
            let agent_id_val: Option<Uuid> = r.get("agent_id");
            AgentLog {
                id: r.get("id"),
                agent_id: agent_id_val.unwrap_or_else(Uuid::nil),
                level: r.get("level"),
                message: r.get("message"),
                fields: fields.and_then(|f| f.get("data").and_then(|d| d.as_str().map(String::from))),
            }
        }).collect();

        Ok(PaginatedResult::new(items, count as u64, offset, limit))
    }

    async fn list_recent(&self, limit: u64) -> Result<Vec<AgentLog>> {
        let rows = sqlx::query(
            r#"
            SELECT id, agent_id, level, message, fields, created_at
            FROM agent_log
            ORDER BY created_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT recent agent_logs".to_string(),
            error: e.to_string(),
        }))?;

        Ok(rows.into_iter().map(|r| {
            let fields: Option<serde_json::Value> = r.get("fields");
            let agent_id_val: Option<Uuid> = r.get("agent_id");
            AgentLog {
                id: r.get("id"),
                agent_id: agent_id_val.unwrap_or_else(Uuid::nil),
                level: r.get("level"),
                message: r.get("message"),
                fields: fields.and_then(|f| f.get("data").and_then(|d| d.as_str().map(String::from))),
            }
        }).collect())
    }

    async fn count_by_agent(&self, agent_id: &Uuid) -> Result<u64> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_log WHERE agent_id = $1")
            .bind(agent_id)
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        Ok(count as u64)
    }
}

/// PostgreSQL-based vector store using RuVector.
///
/// Supports configsurable vector dimensions and uses parameterized queries
/// for safety against SQL injection attacks.
pub struct PgVectorStore {
    pool: PgPool,
    dimension: usize,
}

impl PgVectorStore {
    /// Create a new vector store with the default dimension (384).
    pub fn new(pool: PgPool) -> Self {
        Self::with_dimension(pool, DEFAULT_VECTOR_DIMENSION)
    }

    /// Create a new vector store with a custom dimension.
    pub fn with_dimension(pool: PgPool, dimension: usize) -> Self {
        Self { pool, dimension }
    }

    /// Format a vector as a string for RuVector.
    /// Note: RuVector requires vector literals in the query since it doesn't
    /// support parameterized vector values. Values are sanitized (floats only).
    fn format_vector(vector: &[f32]) -> String {
        format!("[{}]", vector.iter()
            .map(|v| {
                // Ensure we only output valid float representations
                if v.is_finite() {
                    format!("{}", v)
                } else {
                    "0.0".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(","))
    }
}

#[async_trait]
impl VectorStore for PgVectorStore {
    async fn store_embedding(
        &self,
        id: Uuid,
        source_type: &str,
        source_id: Uuid,
        model_id: Option<Uuid>,
        vector: &[f32],
    ) -> Result<()> {
        // Validate vector dimension
        if vector.len() != self.dimension {
            return Err(NoaError::Validation(ValidationError::new(
                "vector",
                format!("dimension mismatch: expected {}, got {}", self.dimension, vector.len()),
                "VECTOR_DIMENSION_MISMATCH",
            )));
        }

        // Convert f32 vector to the format RuVector expects
        let vector_str = Self::format_vector(vector);

        // Note: RuVector doesn't support parameterized vector values,
        // so we must include the vector in the query string.
        // The format_vector function sanitizes the values.
        let query = format!(
            r#"
            INSERT INTO embedding (id, source_type, source_id, model_id, vector)
            VALUES ($1, $2, $3, $4, '{}'::ruvector({}))
            "#,
            vector_str,
            self.dimension,
        );

        sqlx::query(&query)
            .bind(id)
            .bind(source_type)
            .bind(source_id)
            .bind(model_id)
            .execute(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "INSERT embedding".to_string(),
                error: e.to_string(),
            }))?;

        Ok(())
    }

    async fn find_similar(&self, query_vector: &[f32], limit: usize) -> Result<Vec<VectorSearchResult>> {
        // Validate vector dimension
        if query_vector.len() != self.dimension {
            return Err(NoaError::Validation(ValidationError::new(
                "query_vector",
                format!("dimension mismatch: expected {}, got {}", self.dimension, query_vector.len()),
                "VECTOR_DIMENSION_MISMATCH",
            )));
        }

        let vector_str = Self::format_vector(query_vector);

        let query = format!(
            r#"
            SELECT id, source_type, source_id, (vector <-> '{}'::ruvector({})) as distance
            FROM embedding
            ORDER BY vector <-> '{}'::ruvector({})
            LIMIT $1
            "#,
            vector_str, self.dimension, vector_str, self.dimension,
        );

        let rows = sqlx::query(&query)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "find similar".to_string(),
            error: e.to_string(),
        }))?;

        let mut results = Vec::new();
        for row in rows {
            use sqlx::Row;
            results.push(VectorSearchResult {
                id: row.get("id"),
                source_type: row.get("source_type"),
                source_id: row.get("source_id"),
                distance: row.get::<f64, _>("distance"),
            });
        }

        Ok(results)
    }

    async fn find_similar_memories(&self, query_vector: &[f32], limit: usize) -> Result<Vec<(Uuid, f64)>> {
        let results = self.find_similar(query_vector, limit).await?;
        Ok(results.into_iter()
            .filter(|r| r.source_type == "memory")
            .map(|r| (r.source_id, r.distance))
            .collect())
    }

    async fn find_similar_knowledge(&self, query_vector: &[f32], limit: usize) -> Result<Vec<(Uuid, f64)>> {
        let results = self.find_similar(query_vector, limit).await?;
        Ok(results.into_iter()
            .filter(|r| r.source_type == "knowledge_node")
            .map(|r| (r.source_id, r.distance))
            .collect())
    }

    async fn delete_by_source(&self, source_type: &str, source_id: &Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM embedding WHERE source_type = $1 AND source_id = $2")
            .bind(source_type)
            .bind(source_id)
            .execute(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "DELETE embedding".to_string(),
                error: e.to_string(),
            }))?;

        Ok(result.rows_affected() > 0)
    }
}

/// PostgreSQL health checker.
pub struct PgHealth {
    pool: PgPool,
}

impl PgHealth {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DatabaseHealth for PgHealth {
    fn kind(&self) -> DatabaseKind {
        DatabaseKind::Postgres
    }

    async fn check_connection(&self) -> Result<HealthCheckResult> {
        match sqlx::query_scalar::<_, i64>("SELECT 1")
            .fetch_one(&self.pool)
            .await
        {
            Ok(_) => Ok(HealthCheckResult::healthy()),
            Err(e) => Ok(HealthCheckResult::unhealthy(format!("Connection failed: {}", e))),
        }
    }

    async fn check_integrity(&self) -> Result<HealthCheckResult> {
        // Check RuVector extension
        match sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM pg_extension WHERE extname = 'ruvector')"
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(true) => Ok(HealthCheckResult::healthy()),
            Ok(false) => Ok(HealthCheckResult::unhealthy("RuVector extension not installed")),
            Err(e) => Ok(HealthCheckResult::unhealthy(format!("Extension check failed: {}", e))),
        }
    }

    async fn check_schema(&self) -> Result<HealthCheckResult> {
        let required_tables = ["memory", "model", "task", "agent", "embedding"];

        for table in &required_tables {
            let exists: bool = sqlx::query_scalar(&format!(
                "SELECT EXISTS(SELECT 1 FROM information_schema.tables WHERE table_name = '{}')",
                table
            ))
            .fetch_one(&self.pool)
            .await
            .unwrap_or(false);

            if !exists {
                return Ok(HealthCheckResult::unhealthy(format!("Missing table: {}", table)));
            }
        }

        Ok(HealthCheckResult::healthy())
    }

    async fn get_migration_version(&self) -> Result<Option<String>> {
        let version: Option<String> = sqlx::query_scalar(
            "SELECT version FROM _sqlx_migrations ORDER BY installed_on DESC LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten();

        Ok(version)
    }

    async fn get_stats(&self) -> Result<serde_json::Value> {
        let db_size: i64 = sqlx::query_scalar("SELECT pg_database_size(current_database())")
            .fetch_one(&self.pool)
            .await
            .unwrap_or(0);

        let table_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public'"
        )
        .fetch_one(&self.pool)
        .await
        .unwrap_or(0);

        Ok(serde_json::json!({
            "database_size_bytes": db_size,
            "table_count": table_count,
        }))
    }
}

/// The complete PostgreSQL database backend.
///
/// Provides a unified interface to all PostgreSQL-based stores.
/// Supports configsurable vector dimensions for embedding storage.
pub struct PgBackend {
    pool: PgPool,
    memories: Arc<PgMemoryStore>,
    models: Arc<PgModelStore>,
    tasks: Arc<PgTaskStore>,
    agents: Arc<PgAgentStore>,
    audit_logs: Arc<PgAuditLogStore>,
    vectors: Arc<PgVectorStore>,
    health: Arc<PgHealth>,
}

impl PgBackend {
    /// Create a new PostgreSQL backend with default settings.
    ///
    /// Uses the default vector dimension (384) for embedding storage.
    pub fn new(pool: PgPool) -> Self {
        Self::with_vector_dimension(pool, DEFAULT_VECTOR_DIMENSION)
    }

    /// Create a new PostgreSQL backend with custom vector dimension.
    ///
    /// # Arguments
    /// * `pool` - The PostgreSQL connection pool
    /// * `vector_dimension` - The dimension of vectors for embedding storage
    ///
    /// # Common dimensions
    /// - 384: nomic-embed-text, all-MiniLM-L6-v2
    /// - 768: all-mpnet-base-v2, e5-base
    /// - 1024: e5-large
    /// - 1536: text-embedding-ada-002 (OpenAI)
    /// - 3072: text-embedding-3-large (OpenAI)
    pub fn with_vector_dimension(pool: PgPool, vector_dimension: usize) -> Self {
        Self {
            memories: Arc::new(PgMemoryStore::new(pool.clone())),
            models: Arc::new(PgModelStore::new(pool.clone())),
            tasks: Arc::new(PgTaskStore::new(pool.clone())),
            agents: Arc::new(PgAgentStore::new(pool.clone())),
            audit_logs: Arc::new(PgAuditLogStore::new(pool.clone())),
            vectors: Arc::new(PgVectorStore::with_dimension(pool.clone(), vector_dimension)),
            health: Arc::new(PgHealth::new(pool.clone())),
            pool,
        }
    }

    /// Get the raw pool for legacy code.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

impl DatabaseBackend for PgBackend {
    fn kind(&self) -> DatabaseKind {
        DatabaseKind::Postgres
    }

    fn memories(&self) -> Arc<dyn MemoryStore> {
        self.memories.clone()
    }

    fn models(&self) -> Arc<dyn ModelStore> {
        self.models.clone()
    }

    fn tasks(&self) -> Arc<dyn TaskStore> {
        self.tasks.clone()
    }

    fn agents(&self) -> Arc<dyn AgentStore> {
        self.agents.clone()
    }

    fn audit_logs(&self) -> Arc<dyn AuditLogStore> {
        self.audit_logs.clone()
    }

    fn vectors(&self) -> Arc<dyn VectorStore> {
        self.vectors.clone()
    }

    fn health(&self) -> Arc<dyn DatabaseHealth> {
        self.health.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that Task struct construction works with UUID
    #[test]
    fn test_task_struct_uuid() {
        let id = Uuid::new_v4();
        let task = Task {
            id,
            agent_id: Some(Uuid::new_v4()),
            title: "Test Task".to_string(),
            status: "pending".to_string(),
            payload: Some("test payload".to_string()),
        };
        assert_eq!(task.id, id);
        assert!(task.agent_id.is_some());
    }

    /// Test that Agent struct construction works with UUID
    #[test]
    fn test_agent_struct_uuid() {
        let id = Uuid::new_v4();
        let agent = Agent {
            id,
            name: "Test Agent".to_string(),
            description: Some("A test agent".to_string()),
            status: "active".to_string(),
        };
        assert_eq!(agent.id, id);
    }

    /// Test that AgentLog struct construction works with UUID
    #[test]
    fn test_agentlog_struct_uuid() {
        let id = Uuid::new_v4();
        let agent_id = Uuid::new_v4();
        let log = AgentLog {
            id,
            agent_id,
            message: "Test log message".to_string(),
            level: "INFO".to_string(),
            fields: Some("extra data".to_string()),
        };
        assert_eq!(log.id, id);
        assert_eq!(log.agent_id, agent_id);
        assert!(log.fields.is_some());
    }

    /// Test vector formatting
    #[test]
    fn test_vector_formatting() {
        let vector = vec![0.1f32, 0.2, 0.3, -0.5, 1.0];
        let formatted = PgVectorStore::format_vector(&vector);
        assert!(formatted.starts_with('['));
        assert!(formatted.ends_with(']'));
        assert!(formatted.contains("0.1"));
        assert!(formatted.contains("-0.5"));
    }

    /// Test vector formatting handles NaN and Inf
    #[test]
    fn test_vector_formatting_edge_cases() {
        let vector = vec![f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 0.5];
        let formatted = PgVectorStore::format_vector(&vector);
        // NaN and infinities should be replaced with 0.0
        assert!(formatted.contains("0.0"));
        assert!(formatted.contains("0.5"));
        // Should not contain actual NaN or Inf strings
        assert!(!formatted.to_lowercase().contains("nan"));
        assert!(!formatted.to_lowercase().contains("inf"));
    }

    /// Test default vector dimension constant
    #[test]
    fn test_default_vector_dimension() {
        assert_eq!(DEFAULT_VECTOR_DIMENSION, 384);
    }

    /// Test PgBackend construction (requires actual database connection)
    /// This is a placeholder for integration tests with testcontainers
    #[tokio::test]
    #[ignore = "Requires PostgreSQL database - run with testcontainers"]
    async fn test_pg_backend_integration() {
        // This test requires a running PostgreSQL instance.
        // To run: cargo test --features full -- --ignored
        //
        // Example with testcontainers:
        // ```
        // use testcontainers::{clients::Cli, images::postgres::Postgres};
        // let docker = Cli::default();
        // let pg = docker.run(Postgres::default());
        // let pool = PgPool::connect(&format!(
        //     "postgres://postgres:postgres@localhost:{}/postgres",
        //     pg.get_host_port_ipv4(5432)
        // )).await.unwrap();
        // let backend = PgBackend::new(pool);
        // // Run tests...
        // ```
    }

    /// Placeholder for Task CRUD integration test
    #[tokio::test]
    #[ignore = "Requires PostgreSQL database - run with testcontainers"]
    async fn test_task_store_crud() {
        // Integration test for TaskStore CRUD operations
        // Requires testcontainers setup
    }

    /// Placeholder for Agent CRUD integration test
    #[tokio::test]
    #[ignore = "Requires PostgreSQL database - run with testcontainers"]
    async fn test_agent_store_crud() {
        // Integration test for AgentStore CRUD operations
        // Requires testcontainers setup
    }

    /// Placeholder for AuditLog CRUD integration test
    #[tokio::test]
    #[ignore = "Requires PostgreSQL database - run with testcontainers"]
    async fn test_auditlog_store_crud() {
        // Integration test for AuditLogStore CRUD operations
        // Requires testcontainers setup
    }
}
