//! Database Backend Abstraction
//!
//! Provides a backend-agnostic interface for database operations.
//! Routes don't need to know SQLite vs Postgres - they work with trait objects.
//!
//! §3.2: Database backend abstraction layer

use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::Result;
use super::repositories::{Memory, MemoryType, Model, ModelStatus, Task, Agent, AgentLog};
use super::repository::PaginatedResult;

/// Database backend enumeration for health checks and introspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseKind {
    Sqlite,
    Postgres,
}

impl DatabaseKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            DatabaseKind::Sqlite => "sqlite",
            DatabaseKind::Postgres => "postgresql",
        }
    }
}

/// Backend-agnostic memory repository trait.
#[async_trait]
pub trait MemoryStore: Send + Sync {
    /// Create a new memory entry.
    async fn create(&self, memory: &Memory) -> Result<Uuid>;

    /// Find memory by ID.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Memory>>;

    /// Update an existing memory entry.
    async fn update(&self, memory: &Memory) -> Result<()>;

    /// Delete memory by ID.
    async fn delete(&self, id: &Uuid) -> Result<bool>;

    /// List memories with pagination.
    async fn list(&self, offset: u64, limit: u64) -> Result<PaginatedResult<Memory>>;

    /// Find memories by type.
    async fn find_by_type(&self, memory_type: MemoryType, offset: u64, limit: u64) -> Result<PaginatedResult<Memory>>;

    /// Search memories by content (full-text).
    async fn search(&self, query: &str, limit: u64) -> Result<Vec<Memory>>;

    /// Count total memories.
    async fn count(&self) -> Result<u64>;
}

/// Backend-agnostic model repository trait.
#[async_trait]
pub trait ModelStore: Send + Sync {
    /// List all models.
    async fn list(&self) -> Result<Vec<Model>>;

    /// Find model by ID.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Model>>;

    /// Find model by name.
    async fn find_by_name(&self, name: &str) -> Result<Option<Model>>;

    /// Create a new model entry (ingest).
    async fn create(&self, model: &Model) -> Result<Uuid>;

    /// Update model status.
    async fn update_status(&self, id: &Uuid, status: ModelStatus) -> Result<()>;

    /// Get model status.
    async fn get_status(&self, id: &Uuid) -> Result<Option<ModelStatus>>;

    /// Count models by status.
    async fn count_by_status(&self, status: ModelStatus) -> Result<u64>;
}

/// Backend-agnostic task repository trait.
#[async_trait]
pub trait TaskStore: Send + Sync {
    /// Create a new task.
    async fn create(&self, task: &Task) -> Result<Uuid>;

    /// Find task by ID.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Task>>;

    /// Update an existing task.
    async fn update(&self, task: &Task) -> Result<()>;

    /// Delete task by ID.
    async fn delete(&self, id: &Uuid) -> Result<bool>;

    /// List tasks with pagination.
    async fn list(&self, offset: u64, limit: u64) -> Result<PaginatedResult<Task>>;

    /// Find tasks by status.
    async fn find_by_status(&self, status: &str, offset: u64, limit: u64) -> Result<PaginatedResult<Task>>;

    /// Count total tasks.
    async fn count(&self) -> Result<u64>;
}

/// Backend-agnostic agent repository trait.
#[async_trait]
pub trait AgentStore: Send + Sync {
    /// Create a new agent.
    async fn create(&self, agent: &Agent) -> Result<Uuid>;

    /// Find agent by ID.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Agent>>;

    /// List all agents.
    async fn list(&self) -> Result<Vec<Agent>>;

    /// Update an agent.
    async fn update(&self, agent: &Agent) -> Result<()>;

    /// Delete an agent by ID.
    async fn delete(&self, id: &Uuid) -> Result<bool>;
}

/// Backend-agnostic audit log (agent log) repository trait.
#[async_trait]
pub trait AuditLogStore: Send + Sync {
    /// Create a new log entry.
    async fn create(&self, log: &AgentLog) -> Result<Uuid>;

    /// Find logs by agent ID.
    async fn find_by_agent(&self, agent_id: &Uuid, offset: u64, limit: u64) -> Result<PaginatedResult<AgentLog>>;

    /// Find logs by level.
    async fn find_by_level(&self, level: &str, offset: u64, limit: u64) -> Result<PaginatedResult<AgentLog>>;

    /// List recent logs.
    async fn list_recent(&self, limit: u64) -> Result<Vec<AgentLog>>;

    /// Count logs by agent.
    async fn count_by_agent(&self, agent_id: &Uuid) -> Result<u64>;
}

/// Vector search result.
#[derive(Debug, Clone)]
pub struct VectorSearchResult {
    pub id: Uuid,
    pub source_type: String,
    pub source_id: Uuid,
    pub distance: f64,
}

/// Backend-agnostic vector search trait.
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Store an embedding.
    async fn store_embedding(
        &self,
        id: Uuid,
        source_type: &str,
        source_id: Uuid,
        model_id: Option<Uuid>,
        vector: &[f32],
    ) -> Result<()>;

    /// Find similar vectors (nearest neighbor search).
    async fn find_similar(&self, query_vector: &[f32], limit: usize) -> Result<Vec<VectorSearchResult>>;

    /// Find similar memories.
    async fn find_similar_memories(&self, query_vector: &[f32], limit: usize) -> Result<Vec<(Uuid, f64)>>;

    /// Find similar knowledge nodes.
    async fn find_similar_knowledge(&self, query_vector: &[f32], limit: usize) -> Result<Vec<(Uuid, f64)>>;

    /// Delete embedding by source.
    async fn delete_by_source(&self, source_type: &str, source_id: &Uuid) -> Result<bool>;
}

/// Health check result for a database component.
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    pub healthy: bool,
    pub message: Option<String>,
    pub details: Option<serde_json::Value>,
}

impl HealthCheckResult {
    pub fn healthy() -> Self {
        Self {
            healthy: true,
            message: None,
            details: None,
        }
    }

    pub fn healthy_with_details(details: serde_json::Value) -> Self {
        Self {
            healthy: true,
            message: None,
            details: Some(details),
        }
    }

    pub fn unhealthy(message: impl Into<String>) -> Self {
        Self {
            healthy: false,
            message: Some(message.into()),
            details: None,
        }
    }
}

/// Backend-agnostic health check trait.
#[async_trait]
pub trait DatabaseHealth: Send + Sync {
    /// Get the database kind.
    fn kind(&self) -> DatabaseKind;

    /// Basic connectivity check (SELECT 1).
    async fn check_connection(&self) -> Result<HealthCheckResult>;

    /// Check database integrity (SQLite: PRAGMA integrity_check, PG: extension check).
    async fn check_integrity(&self) -> Result<HealthCheckResult>;

    /// Check required tables exist.
    async fn check_schema(&self) -> Result<HealthCheckResult>;

    /// Check migration version.
    async fn get_migration_version(&self) -> Result<Option<String>>;

    /// Get database statistics.
    async fn get_stats(&self) -> Result<serde_json::Value>;

    /// Full health check combining all checks.
    async fn full_health_check(&self) -> Result<HealthCheckResult> {
        // Connection check
        let conn_check = self.check_connection().await?;
        if !conn_check.healthy {
            return Ok(conn_check);
        }

        // Schema check
        let schema_check = self.check_schema().await?;
        if !schema_check.healthy {
            return Ok(schema_check);
        }

        // Integrity check (optional, can be slow)
        let integrity_check = self.check_integrity().await?;
        if !integrity_check.healthy {
            return Ok(integrity_check);
        }

        // Get stats for details
        let stats = self.get_stats().await.unwrap_or(serde_json::json!({}));
        let migration = self.get_migration_version().await.unwrap_or(None);

        Ok(HealthCheckResult::healthy_with_details(serde_json::json!({
            "kind": self.kind().as_str(),
            "migration_version": migration,
            "stats": stats,
        })))
    }
}

/// The unified database backend trait.
/// Combines all repository traits into one interface.
pub trait DatabaseBackend: Send + Sync {
    /// Get the database kind.
    fn kind(&self) -> DatabaseKind;

    /// Get the memory store.
    fn memories(&self) -> Arc<dyn MemoryStore>;

    /// Get the model store.
    fn models(&self) -> Arc<dyn ModelStore>;

    /// Get the task store.
    fn tasks(&self) -> Arc<dyn TaskStore>;

    /// Get the agent store.
    fn agents(&self) -> Arc<dyn AgentStore>;

    /// Get the audit log store.
    fn audit_logs(&self) -> Arc<dyn AuditLogStore>;

    /// Get the vector store.
    fn vectors(&self) -> Arc<dyn VectorStore>;

    /// Get the health checker.
    fn health(&self) -> Arc<dyn DatabaseHealth>;
}

/// A thread-safe handle to a database backend.
pub type DatabaseHandle = Arc<dyn DatabaseBackend>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_kind() {
        assert_eq!(DatabaseKind::Sqlite.as_str(), "sqlite");
        assert_eq!(DatabaseKind::Postgres.as_str(), "postgresql");
    }

    #[test]
    fn test_health_check_result() {
        let healthy = HealthCheckResult::healthy();
        assert!(healthy.healthy);
        assert!(healthy.message.is_none());

        let unhealthy = HealthCheckResult::unhealthy("Connection failed");
        assert!(!unhealthy.healthy);
        assert_eq!(unhealthy.message.as_deref(), Some("Connection failed"));
    }
}
