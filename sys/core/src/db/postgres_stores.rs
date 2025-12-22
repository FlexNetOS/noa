//! PostgreSQL Store Implementations
//!
//! Implements the backend-agnostic traits for PostgreSQL with RuVector.
//! §3.2: PostgreSQL backend implementation

use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::error::{Result, DatabaseError, NoaError};
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
            .map(|m| serde_json::to_value(m).ok())
            .flatten();

        sqlx::query!(
            r#"
            INSERT INTO memory (id, created_at, updated_at, memory_type, content, metadata,
                               source_agent, parent_id, tags, embedding_id, checksum)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            memory.id,
            memory.created_at,
            memory.updated_at,
            memory.memory_type.as_str(),
            memory.content,
            metadata_json,
            memory.source_agent,
            memory.parent_id,
            &tags,
            memory.embedding_id,
            memory.checksum,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "INSERT memory".to_string(),
            error: e.to_string(),
        }))?;

        Ok(memory.id)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Memory>> {
        let row = sqlx::query!(
            r#"
            SELECT id, created_at, updated_at, memory_type, content, metadata,
                   source_agent, parent_id, tags, embedding_id, checksum
            FROM memory
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT memory".to_string(),
            error: e.to_string(),
        }))?;

        match row {
            Some(r) => {
                let memory_type = MemoryType::from_str(&r.memory_type)
                    .unwrap_or(MemoryType::Interaction);
                let metadata = r.metadata.map(|v| {
                    serde_json::from_value(v).ok()
                }).flatten();
                let tags = r.tags.unwrap_or_default().into_iter().collect();

                Ok(Some(Memory {
                    id: r.id,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                    memory_type,
                    content: r.content,
                    metadata,
                    source_agent: r.source_agent,
                    parent_id: r.parent_id,
                    tags,
                    embedding_id: r.embedding_id,
                    checksum: r.checksum,
                }))
            }
            None => Ok(None),
        }
    }

    async fn update(&self, memory: &Memory) -> Result<()> {
        let tags: Vec<String> = memory.tags.iter().cloned().collect();
        let metadata_json = memory.metadata.as_ref()
            .map(|m| serde_json::to_value(m).ok())
            .flatten();

        let result = sqlx::query!(
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
            memory.id,
            memory.updated_at,
            memory.memory_type.as_str(),
            memory.content,
            metadata_json,
            memory.source_agent,
            memory.parent_id,
            &tags,
            memory.embedding_id,
            memory.checksum,
        )
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
        let result = sqlx::query!("DELETE FROM memory WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "DELETE memory".to_string(),
                error: e.to_string(),
            }))?;

        Ok(result.rows_affected() > 0)
    }

    async fn list(&self, offset: u64, limit: u64) -> Result<PaginatedResult<Memory>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, created_at, updated_at, memory_type, content, metadata,
                   source_agent, parent_id, tags, embedding_id, checksum
            FROM memory
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset as i64,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT memories".to_string(),
            error: e.to_string(),
        }))?;

        let total = self.count().await?;
        let items = rows.into_iter().map(|r| {
            let memory_type = MemoryType::from_str(&r.memory_type)
                .unwrap_or(MemoryType::Interaction);
            let metadata = r.metadata.map(|v| serde_json::from_value(v).ok()).flatten();
            let tags = r.tags.unwrap_or_default().into_iter().collect();

            Memory {
                id: r.id,
                created_at: r.created_at,
                updated_at: r.updated_at,
                memory_type,
                content: r.content,
                metadata,
                source_agent: r.source_agent,
                parent_id: r.parent_id,
                tags,
                embedding_id: r.embedding_id,
                checksum: r.checksum,
            }
        }).collect();

        Ok(PaginatedResult::new(items, total, offset, limit))
    }

    async fn find_by_type(&self, memory_type: MemoryType, offset: u64, limit: u64) -> Result<PaginatedResult<Memory>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, created_at, updated_at, memory_type, content, metadata,
                   source_agent, parent_id, tags, embedding_id, checksum
            FROM memory
            WHERE memory_type = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            memory_type.as_str(),
            limit as i64,
            offset as i64,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT memories by type".to_string(),
            error: e.to_string(),
        }))?;

        let count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM memory WHERE memory_type = $1",
            memory_type.as_str(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "COUNT memories".to_string(),
            error: e.to_string(),
        }))?
        .unwrap_or(0);

        let items = rows.into_iter().map(|r| {
            let mtype = MemoryType::from_str(&r.memory_type).unwrap_or(MemoryType::Interaction);
            let metadata = r.metadata.map(|v| serde_json::from_value(v).ok()).flatten();
            let tags = r.tags.unwrap_or_default().into_iter().collect();

            Memory {
                id: r.id,
                created_at: r.created_at,
                updated_at: r.updated_at,
                memory_type: mtype,
                content: r.content,
                metadata,
                source_agent: r.source_agent,
                parent_id: r.parent_id,
                tags,
                embedding_id: r.embedding_id,
                checksum: r.checksum,
            }
        }).collect();

        Ok(PaginatedResult::new(items, count as u64, offset, limit))
    }

    async fn search(&self, query: &str, limit: u64) -> Result<Vec<Memory>> {
        // Full-text search using pg_trgm
        let like_pattern = format!("%{}%", query);
        let rows = sqlx::query!(
            r#"
            SELECT id, created_at, updated_at, memory_type, content, metadata,
                   source_agent, parent_id, tags, embedding_id, checksum
            FROM memory
            WHERE content ILIKE $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            like_pattern,
            limit as i64,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "search memories".to_string(),
            error: e.to_string(),
        }))?;

        Ok(rows.into_iter().map(|r| {
            let memory_type = MemoryType::from_str(&r.memory_type).unwrap_or(MemoryType::Interaction);
            let metadata = r.metadata.map(|v| serde_json::from_value(v).ok()).flatten();
            let tags = r.tags.unwrap_or_default().into_iter().collect();

            Memory {
                id: r.id,
                created_at: r.created_at,
                updated_at: r.updated_at,
                memory_type,
                content: r.content,
                metadata,
                source_agent: r.source_agent,
                parent_id: r.parent_id,
                tags,
                embedding_id: r.embedding_id,
                checksum: r.checksum,
            }
        }).collect())
    }

    async fn count(&self) -> Result<u64> {
        let count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM memory")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "COUNT memories".to_string(),
                error: e.to_string(),
            }))?
            .unwrap_or(0);

        Ok(count as u64)
    }
}

impl MemoryType {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "interaction" => Ok(MemoryType::Interaction),
            "decision" => Ok(MemoryType::Decision),
            "learning" => Ok(MemoryType::Learning),
            "artifact" => Ok(MemoryType::Artifact),
            _ => Err(NoaError::Validation(crate::error::ValidationError::new(
                "memory_type",
                format!("Invalid memory type: {}", s),
                "INVALID_TYPE",
            ))),
        }
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
        let rows = sqlx::query!(
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
            let model_type = r.kind.as_deref()
                .map(|k| match k {
                    "llm" => ModelType::LLM,
                    "embedding" => ModelType::Embedding,
                    "vision" => ModelType::Vision,
                    "audio" => ModelType::Audio,
                    _ => ModelType::LLM,
                })
                .unwrap_or(ModelType::LLM);

            Model {
                id: r.id,
                name: r.name,
                model_type,
                provider: r.provider.unwrap_or_default(),
                path: None,
                uri: None,
                size_bytes: None,
                parameters: None,
                context_length: None,
                license: None,
                config: r.metadata.unwrap_or(serde_json::json!({})),
                status: ModelStatus::Available,
                metrics: None,
            }
        }).collect())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Model>> {
        let row = sqlx::query!(
            r#"
            SELECT id, created_at, name, provider, kind, metadata
            FROM model
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT model".to_string(),
            error: e.to_string(),
        }))?;

        Ok(row.map(|r| {
            let model_type = r.kind.as_deref()
                .map(|k| match k {
                    "llm" => ModelType::LLM,
                    "embedding" => ModelType::Embedding,
                    "vision" => ModelType::Vision,
                    "audio" => ModelType::Audio,
                    _ => ModelType::LLM,
                })
                .unwrap_or(ModelType::LLM);

            Model {
                id: r.id,
                name: r.name,
                model_type,
                provider: r.provider.unwrap_or_default(),
                path: None,
                uri: None,
                size_bytes: None,
                parameters: None,
                context_length: None,
                license: None,
                config: r.metadata.unwrap_or(serde_json::json!({})),
                status: ModelStatus::Available,
                metrics: None,
            }
        }))
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Model>> {
        let row = sqlx::query!(
            r#"
            SELECT id, created_at, name, provider, kind, metadata
            FROM model
            WHERE name = $1
            "#,
            name,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT model by name".to_string(),
            error: e.to_string(),
        }))?;

        Ok(row.map(|r| {
            let model_type = r.kind.as_deref()
                .map(|k| match k {
                    "llm" => ModelType::LLM,
                    "embedding" => ModelType::Embedding,
                    _ => ModelType::LLM,
                })
                .unwrap_or(ModelType::LLM);

            Model {
                id: r.id,
                name: r.name,
                model_type,
                provider: r.provider.unwrap_or_default(),
                path: None,
                uri: None,
                size_bytes: None,
                parameters: None,
                context_length: None,
                license: None,
                config: r.metadata.unwrap_or(serde_json::json!({})),
                status: ModelStatus::Available,
                metrics: None,
            }
        }))
    }

    async fn create(&self, model: &Model) -> Result<Uuid> {
        sqlx::query!(
            r#"
            INSERT INTO model (id, name, provider, kind, metadata)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            model.id,
            model.name,
            model.provider,
            model.model_type.as_str(),
            model.config,
        )
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
        let result = sqlx::query!(
            r#"
            UPDATE model SET metadata = metadata || $2
            WHERE id = $1
            "#,
            id,
            status_json,
        )
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
        let row = sqlx::query!(
            "SELECT metadata FROM model WHERE id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT model status".to_string(),
            error: e.to_string(),
        }))?;

        Ok(row.and_then(|r| {
            r.metadata.and_then(|m| {
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
        // Count by status in metadata
        let status_pattern = format!("%\"status\":\"{}\"%", status.as_str());
        let count: i64 = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM model WHERE metadata::text LIKE $1"#,
            status_pattern,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "COUNT models by status".to_string(),
            error: e.to_string(),
        }))?
        .unwrap_or(0);

        Ok(count as u64)
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
        sqlx::query!(
            r#"
            INSERT INTO task (id, title, description, status, priority, metadata)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            id,
            task.title,
            None::<String>, // description
            task.status,
            None::<i32>, // priority
            task.payload.as_ref().map(|p| serde_json::json!({ "payload": p })),
        )
        .execute(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "INSERT task".to_string(),
            error: e.to_string(),
        }))?;

        Ok(id)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Task>> {
        let row = sqlx::query!(
            r#"
            SELECT id, title, status, metadata
            FROM task
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT task".to_string(),
            error: e.to_string(),
        }))?;

        Ok(row.map(|r| Task {
            id: 0, // Compatibility - UUID stored separately
            agent_id: None,
            title: r.title,
            status: r.status,
            payload: r.metadata.and_then(|m| m.get("payload").and_then(|p| p.as_str().map(String::from))),
        }))
    }

    async fn update(&self, task: &Task) -> Result<()> {
        // This is a simplified update - full implementation would need UUID field
        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM task WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "DELETE task".to_string(),
                error: e.to_string(),
            }))?;

        Ok(result.rows_affected() > 0)
    }

    async fn list(&self, offset: u64, limit: u64) -> Result<PaginatedResult<Task>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, title, status, metadata
            FROM task
            ORDER BY updated_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset as i64,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT tasks".to_string(),
            error: e.to_string(),
        }))?;

        let count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM task")
            .fetch_one(&self.pool)
            .await
            .unwrap()
            .unwrap_or(0);

        let items = rows.into_iter().map(|r| Task {
            id: 0,
            agent_id: None,
            title: r.title,
            status: r.status,
            payload: r.metadata.and_then(|m| m.get("payload").and_then(|p| p.as_str().map(String::from))),
        }).collect();

        Ok(PaginatedResult::new(items, count as u64, offset, limit))
    }

    async fn find_by_status(&self, status: &str, offset: u64, limit: u64) -> Result<PaginatedResult<Task>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, title, status, metadata
            FROM task
            WHERE status = $1
            ORDER BY updated_at DESC
            LIMIT $2 OFFSET $3
            "#,
            status,
            limit as i64,
            offset as i64,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT tasks by status".to_string(),
            error: e.to_string(),
        }))?;

        let count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM task WHERE status = $1", status)
            .fetch_one(&self.pool)
            .await
            .unwrap()
            .unwrap_or(0);

        let items = rows.into_iter().map(|r| Task {
            id: 0,
            agent_id: None,
            title: r.title,
            status: r.status,
            payload: r.metadata.and_then(|m| m.get("payload").and_then(|p| p.as_str().map(String::from))),
        }).collect();

        Ok(PaginatedResult::new(items, count as u64, offset, limit))
    }

    async fn count(&self) -> Result<u64> {
        let count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM task")
            .fetch_one(&self.pool)
            .await
            .unwrap()
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
        sqlx::query!(
            r#"
            INSERT INTO agent (id, name, metadata)
            VALUES ($1, $2, $3)
            "#,
            id,
            agent.name,
            serde_json::json!({
                "description": agent.description,
                "status": agent.status,
            }),
        )
        .execute(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "INSERT agent".to_string(),
            error: e.to_string(),
        }))?;

        Ok(id)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Agent>> {
        let row = sqlx::query!(
            "SELECT id, name, metadata FROM agent WHERE id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT agent".to_string(),
            error: e.to_string(),
        }))?;

        Ok(row.map(|r| Agent {
            id: 0,
            name: r.name,
            description: r.metadata.as_ref()
                .and_then(|m| m.get("description").and_then(|d| d.as_str().map(String::from))),
            status: r.metadata.as_ref()
                .and_then(|m| m.get("status").and_then(|s| s.as_str()))
                .unwrap_or("active")
                .to_string(),
        }))
    }

    async fn list(&self) -> Result<Vec<Agent>> {
        let rows = sqlx::query!("SELECT id, name, metadata FROM agent ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT agents".to_string(),
                error: e.to_string(),
            }))?;

        Ok(rows.into_iter().map(|r| Agent {
            id: 0,
            name: r.name,
            description: r.metadata.as_ref()
                .and_then(|m| m.get("description").and_then(|d| d.as_str().map(String::from))),
            status: r.metadata.as_ref()
                .and_then(|m| m.get("status").and_then(|s| s.as_str()))
                .unwrap_or("active")
                .to_string(),
        }).collect())
    }

    async fn update(&self, agent: &Agent) -> Result<()> {
        // Simplified - would need UUID mapping
        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<bool> {
        let result = sqlx::query!("DELETE FROM agent WHERE id = $1", id)
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
        sqlx::query!(
            r#"
            INSERT INTO agent_log (id, agent_id, level, message, fields)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            id,
            None::<Uuid>, // agent_id from i64 not directly compatible
            log.level,
            log.message,
            log.fields.as_ref().map(|f| serde_json::json!({ "data": f })),
        )
        .execute(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "INSERT agent_log".to_string(),
            error: e.to_string(),
        }))?;

        Ok(id)
    }

    async fn find_by_agent(&self, agent_id: &Uuid, offset: u64, limit: u64) -> Result<PaginatedResult<AgentLog>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, agent_id, level, message, fields, created_at
            FROM agent_log
            WHERE agent_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            agent_id,
            limit as i64,
            offset as i64,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT agent_logs".to_string(),
            error: e.to_string(),
        }))?;

        let count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM agent_log WHERE agent_id = $1",
            agent_id,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
        .unwrap_or(0);

        let items = rows.into_iter().map(|r| AgentLog {
            id: 0,
            agent_id: 0,
            level: r.level,
            message: r.message,
            fields: r.fields.and_then(|f| f.get("data").and_then(|d| d.as_str().map(String::from))),
        }).collect();

        Ok(PaginatedResult::new(items, count as u64, offset, limit))
    }

    async fn find_by_level(&self, level: &str, offset: u64, limit: u64) -> Result<PaginatedResult<AgentLog>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, agent_id, level, message, fields, created_at
            FROM agent_log
            WHERE level = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            level,
            limit as i64,
            offset as i64,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT agent_logs by level".to_string(),
            error: e.to_string(),
        }))?;

        let count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM agent_log WHERE level = $1",
            level,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
        .unwrap_or(0);

        let items = rows.into_iter().map(|r| AgentLog {
            id: 0,
            agent_id: 0,
            level: r.level,
            message: r.message,
            fields: r.fields.and_then(|f| f.get("data").and_then(|d| d.as_str().map(String::from))),
        }).collect();

        Ok(PaginatedResult::new(items, count as u64, offset, limit))
    }

    async fn list_recent(&self, limit: u64) -> Result<Vec<AgentLog>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, agent_id, level, message, fields, created_at
            FROM agent_log
            ORDER BY created_at DESC
            LIMIT $1
            "#,
            limit as i64,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| NoaError::Database(DatabaseError::QueryFailed {
            query: "SELECT recent agent_logs".to_string(),
            error: e.to_string(),
        }))?;

        Ok(rows.into_iter().map(|r| AgentLog {
            id: 0,
            agent_id: 0,
            level: r.level,
            message: r.message,
            fields: r.fields.and_then(|f| f.get("data").and_then(|d| d.as_str().map(String::from))),
        }).collect())
    }

    async fn count_by_agent(&self, agent_id: &Uuid) -> Result<u64> {
        let count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM agent_log WHERE agent_id = $1",
            agent_id,
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
        .unwrap_or(0);

        Ok(count as u64)
    }
}

/// PostgreSQL-based vector store using RuVector.
pub struct PgVectorStore {
    pool: PgPool,
}

impl PgVectorStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
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
        // Convert f32 vector to the format RuVector expects
        // RuVector uses a special binary format or array syntax
        let vector_str = format!("[{}]", vector.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(","));

        sqlx::query(&format!(
            r#"
            INSERT INTO embedding (id, source_type, source_id, model_id, vector)
            VALUES ($1, $2, $3, $4, '{}'::ruvector(384))
            "#,
            vector_str,
        ))
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
        let vector_str = format!("[{}]", query_vector.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(","));

        let rows = sqlx::query(&format!(
            r#"
            SELECT id, source_type, source_id, (vector <-> '{}'::ruvector(384)) as distance
            FROM embedding
            ORDER BY vector <-> '{}'::ruvector(384)
            LIMIT $1
            "#,
            vector_str, vector_str,
        ))
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
        let result = sqlx::query!(
            "DELETE FROM embedding WHERE source_type = $1 AND source_id = $2",
            source_type,
            source_id,
        )
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
    pub fn new(pool: PgPool) -> Self {
        Self {
            memories: Arc::new(PgMemoryStore::new(pool.clone())),
            models: Arc::new(PgModelStore::new(pool.clone())),
            tasks: Arc::new(PgTaskStore::new(pool.clone())),
            agents: Arc::new(PgAgentStore::new(pool.clone())),
            audit_logs: Arc::new(PgAuditLogStore::new(pool.clone())),
            vectors: Arc::new(PgVectorStore::new(pool.clone())),
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
