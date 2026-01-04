//! SQLite Store Implementations
//!
//! Wraps the existing SQLite repositories to implement the backend-agnostic traits.
//! This module provides SQLite implementations of all backend store traits,
//! enabling local-first operation with full offline capability.
//!
//! # Architecture
//!
//! Each store wraps the corresponding repository pattern implementation and
//! adapts it to the async trait interface expected by the backend abstraction.
//!
//! # Features
//!
//! - **Local-first**: All operations work without network connectivity
//! - **Memory-efficient**: Uses connection pooling for concurrent access
//! - **Portable**: SQLite database can be easily backed up or moved
//!
//! §3.2: SQLite backend implementation
//! §3.3: Local-First & Offline-Capable

use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::error::{Result, DatabaseError, NoaError, ValidationError};
use super::backend::{
    DatabaseKind, DatabaseBackend, DatabaseHealth, HealthCheckResult,
    MemoryStore, ModelStore, TaskStore, AgentStore, AuditLogStore, VectorStore,
    VectorSearchResult,
};
use super::repository::PaginatedResult;
use super::repositories::{
    Memory, MemoryType, MemoryRepository,
    Model, ModelStatus, ModelRepository,
    Task, TaskRepository,
    Agent, AgentRepository,
    AgentLog, AgentLogRepository,
    EmbeddingRepository,
};
use super::{ConnectionPool, check_integrity, get_stats};

/// Default vector dimension for embedding storage.
///
/// This matches the output dimension of common embedding models:
/// - nomic-embed-text: 384 dimensions
/// - all-MiniLM-L6-v2: 384 dimensions
///
/// For other models, use `SqliteVectorStore::with_dimension()` or
/// `SqliteBackend::with_vector_dimension()`.
pub const DEFAULT_VECTOR_DIMENSION: usize = 384;

/// SQLite-based memory store.
pub struct SqliteMemoryStore {
    pool: Arc<ConnectionPool>,
}

impl SqliteMemoryStore {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MemoryStore for SqliteMemoryStore {
    async fn create(&self, memory: &Memory) -> Result<Uuid> {
        let conn = self.pool.get()?;
        let repo = MemoryRepository::new(&conn);
        repo.create(memory)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Memory>> {
        let conn = self.pool.get()?;
        let repo = MemoryRepository::new(&conn);
        repo.find_by_id(id)
    }

    async fn update(&self, memory: &Memory) -> Result<()> {
        let conn = self.pool.get()?;
        let repo = MemoryRepository::new(&conn);
        repo.update(memory)
    }

    async fn delete(&self, id: &Uuid) -> Result<bool> {
        let conn = self.pool.get()?;
        let repo = MemoryRepository::new(&conn);
        repo.delete(id)
    }

    async fn list(&self, offset: u64, limit: u64) -> Result<PaginatedResult<Memory>> {
        let conn = self.pool.get()?;
        let repo = MemoryRepository::new(&conn);
        let items = repo.list(offset, limit)?;
        let total = repo.count()?;
        Ok(PaginatedResult::new(items, total, offset, limit))
    }

    async fn find_by_type(&self, memory_type: MemoryType, offset: u64, limit: u64) -> Result<PaginatedResult<Memory>> {
        let conn = self.pool.get()?;
        let repo = MemoryRepository::new(&conn);
        let items = repo.find_by_type(memory_type)?;
        let total = items.len() as u64;
        // Apply pagination manually since the underlying repo doesn't support it
        let start = offset as usize;
        let end = std::cmp::min(start + limit as usize, items.len());
        let paginated = if start < items.len() {
            items[start..end].to_vec()
        } else {
            vec![]
        };
        Ok(PaginatedResult::new(paginated, total, offset, limit))
    }

    async fn search(&self, query: &str, limit: u64) -> Result<Vec<Memory>> {
        // TODO: Implement proper search (SQLite FTS5, or vector search).
        // Keep parameters referenced to avoid unused warnings in strict builds.
        let _ = (query, limit);
        Ok(Vec::new())
    }

    async fn count(&self) -> Result<u64> {
        let conn = self.pool.get()?;
        let repo = MemoryRepository::new(&conn);
        repo.count()
    }
}

/// SQLite-based model store.
pub struct SqliteModelStore {
    pool: Arc<ConnectionPool>,
}

impl SqliteModelStore {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ModelStore for SqliteModelStore {
    async fn list(&self) -> Result<Vec<Model>> {
        let conn = self.pool.get()?;
        let repo = ModelRepository::new(&conn);
        repo.list()
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Model>> {
        let conn = self.pool.get()?;
        let repo = ModelRepository::new(&conn);
        repo.find_by_id(id)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Model>> {
        let conn = self.pool.get()?;
        let repo = ModelRepository::new(&conn);
        repo.find_by_name(name)
    }

    async fn create(&self, model: &Model) -> Result<Uuid> {
        let conn = self.pool.get()?;
        let repo = ModelRepository::new(&conn);
        repo.create(model)
    }

    async fn update_status(&self, id: &Uuid, status: ModelStatus) -> Result<()> {
        let conn = self.pool.get()?;
        let repo = ModelRepository::new(&conn);
        repo.update_status(id, status)
    }

    async fn get_status(&self, id: &Uuid) -> Result<Option<ModelStatus>> {
        let conn = self.pool.get()?;
        let repo = ModelRepository::new(&conn);
        match repo.find_by_id(id)? {
            Some(model) => Ok(Some(model.status)),
            None => Ok(None),
        }
    }

    async fn count_by_status(&self, status: ModelStatus) -> Result<u64> {
        let conn = self.pool.get()?;
        let repo = ModelRepository::new(&conn);
        repo.count_by_status(status)
    }
}

/// SQLite-based task store.
pub struct SqliteTaskStore {
    pool: Arc<ConnectionPool>,
}

impl SqliteTaskStore {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskStore for SqliteTaskStore {
    async fn create(&self, task: &Task) -> Result<Uuid> {
        let conn = self.pool.get()?;
        let repo = TaskRepository::new(&conn);
        repo.create(task)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Task>> {
        let conn = self.pool.get()?;
        let repo = TaskRepository::new(&conn);
        repo.find_by_id(id)
    }

    async fn update(&self, task: &Task) -> Result<()> {
        let conn = self.pool.get()?;
        let repo = TaskRepository::new(&conn);
        repo.update(task)
    }

    async fn delete(&self, id: &Uuid) -> Result<bool> {
        let conn = self.pool.get()?;
        let repo = TaskRepository::new(&conn);
        repo.delete(id)
    }

    async fn list(&self, offset: u64, limit: u64) -> Result<PaginatedResult<Task>> {
        let conn = self.pool.get()?;
        let repo = TaskRepository::new(&conn);
        let items = repo.list(offset, limit)?;
        let total = repo.count()?;
        Ok(PaginatedResult::new(items, total, offset, limit))
    }

    async fn find_by_status(&self, status: &str, offset: u64, limit: u64) -> Result<PaginatedResult<Task>> {
        let conn = self.pool.get()?;
        let repo = TaskRepository::new(&conn);
        let items = repo.find_by_status(status)?;
        let total = items.len() as u64;
        let start = offset as usize;
        let end = std::cmp::min(start + limit as usize, items.len());
        let paginated = if start < items.len() {
            items[start..end].to_vec()
        } else {
            vec![]
        };
        Ok(PaginatedResult::new(paginated, total, offset, limit))
    }

    async fn count(&self) -> Result<u64> {
        let conn = self.pool.get()?;
        let repo = TaskRepository::new(&conn);
        repo.count()
    }
}

/// SQLite-based agent store.
pub struct SqliteAgentStore {
    pool: Arc<ConnectionPool>,
}

impl SqliteAgentStore {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AgentStore for SqliteAgentStore {
    async fn create(&self, agent: &Agent) -> Result<Uuid> {
        let conn = self.pool.get()?;
        let repo = AgentRepository::new(&conn);
        repo.create(agent)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Agent>> {
        let conn = self.pool.get()?;
        let repo = AgentRepository::new(&conn);
        repo.find_by_id(id)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Agent>> {
        let conn = self.pool.get()?;
        let repo = AgentRepository::new(&conn);
        repo.find_by_name(name)
    }

    async fn list(&self) -> Result<Vec<Agent>> {
        let conn = self.pool.get()?;
        let repo = AgentRepository::new(&conn);
        repo.list()
    }

    async fn update(&self, agent: &Agent) -> Result<()> {
        let conn = self.pool.get()?;
        let repo = AgentRepository::new(&conn);
        repo.update(agent)
    }

    async fn delete(&self, id: &Uuid) -> Result<bool> {
        let conn = self.pool.get()?;
        let repo = AgentRepository::new(&conn);
        repo.delete(id)
    }
}

/// SQLite-based audit log store.
pub struct SqliteAuditLogStore {
    pool: Arc<ConnectionPool>,
}

impl SqliteAuditLogStore {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditLogStore for SqliteAuditLogStore {
    async fn create(&self, log: &AgentLog) -> Result<Uuid> {
        let conn = self.pool.get()?;
        let repo = AgentLogRepository::new(&conn);
        repo.create(log)
    }

    async fn find_by_agent(&self, agent_id: &Uuid, offset: u64, limit: u64) -> Result<PaginatedResult<AgentLog>> {
        let conn = self.pool.get()?;
        let repo = AgentLogRepository::new(&conn);
        let items = repo.find_by_agent(agent_id, limit)?;
        let total = repo.count_by_agent(agent_id)?;
        Ok(PaginatedResult::new(items, total, offset, limit))
    }

    async fn find_by_level(&self, level: &str, offset: u64, limit: u64) -> Result<PaginatedResult<AgentLog>> {
        let conn = self.pool.get()?;
        let repo = AgentLogRepository::new(&conn);
        let items = repo.find_by_level(level, limit)?;
        let total = items.len() as u64; // Approximate
        Ok(PaginatedResult::new(items, total, offset, limit))
    }

    async fn list_recent(&self, limit: u64) -> Result<Vec<AgentLog>> {
        let conn = self.pool.get()?;
        let repo = AgentLogRepository::new(&conn);
        repo.list_recent(limit)
    }

    async fn count_by_agent(&self, agent_id: &Uuid) -> Result<u64> {
        let conn = self.pool.get()?;
        let repo = AgentLogRepository::new(&conn);
        repo.count_by_agent(agent_id)
    }
}

/// SQLite-based vector store with configsurable dimensions.
///
/// Supports vector similarity search using brute-force cosine distance
/// computation in SQLite. For large-scale deployments, consider using
/// PostgreSQL with pgvector.
pub struct SqliteVectorStore {
    pool: Arc<ConnectionPool>,
    dimension: usize,
}

impl SqliteVectorStore {
    /// Create a new vector store with default dimension (384).
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self::with_dimension(pool, DEFAULT_VECTOR_DIMENSION)
    }

    /// Create a new vector store with custom dimension.
    ///
    /// # Arguments
    /// * `pool` - Connection pool for database access
    /// * `dimension` - Expected dimension of embedding vectors
    ///
    /// # Common dimensions
    /// - 384: nomic-embed-text, all-MiniLM-L6-v2
    /// - 768: all-mpnet-base-v2, e5-base
    /// - 1024: e5-large
    /// - 1536: text-embedding-ada-002 (OpenAI)
    /// - 3072: text-embedding-3-large (OpenAI)
    pub fn with_dimension(pool: Arc<ConnectionPool>, dimension: usize) -> Self {
        Self { pool, dimension }
    }

    /// Get the configsured vector dimension.
    pub fn dimension(&self) -> usize {
        self.dimension
    }
}

#[async_trait]
impl VectorStore for SqliteVectorStore {
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
        let conn = self.pool.get()?;
        let repo = EmbeddingRepository::new(&conn);
        repo.store(id, source_type, source_id, model_id, vector)
    }

    async fn find_similar(&self, query_vector: &[f32], limit: usize) -> Result<Vec<VectorSearchResult>> {
        // Validate query vector dimension
        if query_vector.len() != self.dimension {
            return Err(NoaError::Validation(ValidationError::new(
                "query_vector",
                format!("dimension mismatch: expected {}, got {}", self.dimension, query_vector.len()),
                "VECTOR_DIMENSION_MISMATCH",
            )));
        }
        let conn = self.pool.get()?;
        let repo = EmbeddingRepository::new(&conn);
        let results = repo.find_similar(query_vector, limit)?;
        Ok(results.into_iter().map(|r| VectorSearchResult {
            id: r.id,
            source_type: r.source_type,
            source_id: r.source_id,
            distance: r.distance,
        }).collect())
    }

    async fn find_similar_memories(&self, query_vector: &[f32], limit: usize) -> Result<Vec<(Uuid, f64)>> {
        // Validate query vector dimension
        if query_vector.len() != self.dimension {
            return Err(NoaError::Validation(ValidationError::new(
                "query_vector",
                format!("dimension mismatch: expected {}, got {}", self.dimension, query_vector.len()),
                "VECTOR_DIMENSION_MISMATCH",
            )));
        }
        let conn = self.pool.get()?;
        let repo = EmbeddingRepository::new(&conn);
        repo.find_similar_by_type(query_vector, "memory", limit)
    }

    async fn find_similar_knowledge(&self, query_vector: &[f32], limit: usize) -> Result<Vec<(Uuid, f64)>> {
        // Validate query vector dimension
        if query_vector.len() != self.dimension {
            return Err(NoaError::Validation(ValidationError::new(
                "query_vector",
                format!("dimension mismatch: expected {}, got {}", self.dimension, query_vector.len()),
                "VECTOR_DIMENSION_MISMATCH",
            )));
        }
        let conn = self.pool.get()?;
        let repo = EmbeddingRepository::new(&conn);
        repo.find_similar_by_type(query_vector, "knowledge_node", limit)
    }

    async fn delete_by_source(&self, source_type: &str, source_id: &Uuid) -> Result<bool> {
        let conn = self.pool.get()?;
        let repo = EmbeddingRepository::new(&conn);
        repo.delete_by_source(source_type, source_id)
    }
}

/// SQLite health checker.
pub struct SqliteHealth {
    pool: Arc<ConnectionPool>,
}

impl SqliteHealth {
    pub fn new(pool: Arc<ConnectionPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DatabaseHealth for SqliteHealth {
    fn kind(&self) -> DatabaseKind {
        DatabaseKind::Sqlite
    }

    async fn check_connection(&self) -> Result<HealthCheckResult> {
        match self.pool.get() {
            Ok(conn) => {
                match conn.execute_batch("SELECT 1") {
                    Ok(_) => Ok(HealthCheckResult::healthy()),
                    Err(e) => Ok(HealthCheckResult::unhealthy(format!("Query failed: {}", e))),
                }
            }
            Err(e) => Ok(HealthCheckResult::unhealthy(format!("Connection failed: {}", e))),
        }
    }

    async fn check_integrity(&self) -> Result<HealthCheckResult> {
        match self.pool.get() {
            Ok(conn) => {
                match check_integrity(&conn) {
                    Ok(true) => Ok(HealthCheckResult::healthy()),
                    Ok(false) => Ok(HealthCheckResult::unhealthy("Integrity check failed")),
                    Err(e) => Ok(HealthCheckResult::unhealthy(format!("Integrity check error: {}", e))),
                }
            }
            Err(e) => Ok(HealthCheckResult::unhealthy(format!("Connection failed: {}", e))),
        }
    }

    async fn check_schema(&self) -> Result<HealthCheckResult> {
        match self.pool.get() {
            Ok(conn) => {
                // Check for required tables
                let required_tables = ["memory", "model", "task", "agent", "embedding"];
                for table in &required_tables {
                    let exists: i64 = conn.query_row(
                        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                        [table],
                        |row| row.get(0),
                    ).unwrap_or(0);

                    if exists == 0 {
                        return Ok(HealthCheckResult::unhealthy(format!("Missing table: {}", table)));
                    }
                }
                Ok(HealthCheckResult::healthy())
            }
            Err(e) => Ok(HealthCheckResult::unhealthy(format!("Connection failed: {}", e))),
        }
    }

    async fn get_migration_version(&self) -> Result<Option<String>> {
        match self.pool.get() {
            Ok(conn) => {
                let version: rusqlite::Result<String> = conn.query_row(
                    "SELECT MAX(version) FROM _migrations",
                    [],
                    |row| row.get(0),
                );
                Ok(version.ok())
            }
            Err(_) => Ok(None),
        }
    }

    async fn get_stats(&self) -> Result<serde_json::Value> {
        match self.pool.get() {
            Ok(conn) => {
                let stats = get_stats(&conn)?;
                let pool_status = self.pool.status();
                Ok(serde_json::json!({
                    "total_size_bytes": stats.total_size_bytes,
                    "used_size_bytes": stats.used_size_bytes,
                    "total_pages": stats.total_pages,
                    "free_pages": stats.free_pages,
                    "pool_idle": pool_status.idle_connections,
                    "pool_active": pool_status.active_connections,
                    "pool_max": pool_status.max_connections,
                }))
            }
            Err(e) => Err(NoaError::Database(DatabaseError::ConnectionFailed(e.to_string()))),
        }
    }
}

/// The complete SQLite database backend.
///
/// Provides a unified interface to all SQLite-backed data stores.
/// Use `new()` for default settings or `with_vector_dimension()` for
/// custom embedding model configsurations.
pub struct SqliteBackend {
    pool: Arc<ConnectionPool>,
    memories: Arc<SqliteMemoryStore>,
    models: Arc<SqliteModelStore>,
    tasks: Arc<SqliteTaskStore>,
    agents: Arc<SqliteAgentStore>,
    audit_logs: Arc<SqliteAuditLogStore>,
    vectors: Arc<SqliteVectorStore>,
    health: Arc<SqliteHealth>,
}

impl SqliteBackend {
    /// Create a new SQLite backend with default settings.
    ///
    /// Uses the default vector dimension (384) for embedding storage.
    pub fn new(pool: ConnectionPool) -> Self {
        Self::with_vector_dimension(pool, DEFAULT_VECTOR_DIMENSION)
    }

    /// Create a new SQLite backend with custom vector dimension.
    ///
    /// # Arguments
    /// * `pool` - The SQLite connection pool
    /// * `vector_dimension` - The dimension of vectors for embedding storage
    ///
    /// # Common dimensions
    /// - 384: nomic-embed-text, all-MiniLM-L6-v2
    /// - 768: all-mpnet-base-v2, e5-base
    /// - 1024: e5-large
    /// - 1536: text-embedding-ada-002 (OpenAI)
    /// - 3072: text-embedding-3-large (OpenAI)
    pub fn with_vector_dimension(pool: ConnectionPool, vector_dimension: usize) -> Self {
        let pool = Arc::new(pool);
        Self {
            memories: Arc::new(SqliteMemoryStore::new(pool.clone())),
            models: Arc::new(SqliteModelStore::new(pool.clone())),
            tasks: Arc::new(SqliteTaskStore::new(pool.clone())),
            agents: Arc::new(SqliteAgentStore::new(pool.clone())),
            audit_logs: Arc::new(SqliteAuditLogStore::new(pool.clone())),
            vectors: Arc::new(SqliteVectorStore::with_dimension(pool.clone(), vector_dimension)),
            health: Arc::new(SqliteHealth::new(pool.clone())),
            pool,
        }
    }

    /// Get the raw connection pool for legacy code.
    pub fn pool(&self) -> &Arc<ConnectionPool> {
        &self.pool
    }
}

impl DatabaseBackend for SqliteBackend {
    fn kind(&self) -> DatabaseKind {
        DatabaseKind::Sqlite
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
    use tempfile::tempdir;

    fn create_test_pool() -> ConnectionPool {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        ConnectionPool::with_defaults(&db_path).unwrap()
    }

    #[tokio::test]
    async fn test_sqlite_health_check() {
        let pool = create_test_pool();
        let health = SqliteHealth::new(Arc::new(pool));

        let result = health.check_connection().await.unwrap();
        assert!(result.healthy);
    }

    #[test]
    fn test_default_vector_dimension() {
        assert_eq!(DEFAULT_VECTOR_DIMENSION, 384);
    }

    #[test]
    fn test_sqlite_vector_store_dimension() {
        let pool = create_test_pool();
        let arc_pool = Arc::new(pool);

        // Default dimension
        let store = SqliteVectorStore::new(arc_pool.clone());
        assert_eq!(store.dimension(), DEFAULT_VECTOR_DIMENSION);

        // Custom dimension
        let custom_store = SqliteVectorStore::with_dimension(arc_pool.clone(), 768);
        assert_eq!(custom_store.dimension(), 768);

        // OpenAI dimensions
        let openai_store = SqliteVectorStore::with_dimension(arc_pool.clone(), 1536);
        assert_eq!(openai_store.dimension(), 1536);
    }

    #[test]
    fn test_sqlite_backend_with_dimension() {
        let pool = create_test_pool();

        // Test with_vector_dimension factory
        let backend = SqliteBackend::with_vector_dimension(pool, 1024);
        assert_eq!(backend.kind(), DatabaseKind::Sqlite);
    }
}
