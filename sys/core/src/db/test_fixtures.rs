//! Test Fixtures for Database Testing
//!
//! Provides common test utilities, sample data factories, and test database setup.
//! Used by both unit tests and integration tests for database components.

use crate::db::{init_database, Connection};
use crate::error::Result;
use std::path::PathBuf;
use tempfile::TempDir;
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashSet;

/// Test database wrapper that manages temporary database lifecycle
pub struct TestDb {
    pub conn: Connection,
    pub path: PathBuf,
    _temp_dir: TempDir, // Kept alive for test duration
}

impl TestDb {
    /// Create a new temporary test database with all schemas initialized
    pub fn new() -> Result<Self> {
        let temp_dir = TempDir::new()
            .map_err(|e| crate::error::NoaError::Io(e.into()))?;
        let db_path = temp_dir.path().join("test.db");
        let conn = init_database(&db_path)?;
        
        // Initialize all required schemas
        Self::init_schemas(&conn)?;
        
        Ok(Self {
            conn,
            path: db_path,
            _temp_dir: temp_dir,
        })
    }

    /// Create a new in-memory test database
    pub fn in_memory() -> Result<Self> {
        let temp_dir = TempDir::new()
            .map_err(|e| crate::error::NoaError::Io(e.into()))?;
        let conn = rusqlite::Connection::open_in_memory()
            .map_err(|e| crate::error::DatabaseError::ConnectionFailed(e.to_string()))?;
        
        // Apply basic pragmas
        conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            "#,
        ).map_err(|e| {
            crate::error::DatabaseError::QueryFailed {
                query: "PRAGMA configsuration".to_string(),
                error: e.to_string(),
            }
        })?;
        
        Self::init_schemas(&conn)?;
        
        Ok(Self {
            conn,
            path: PathBuf::from(":memory:"),
            _temp_dir: temp_dir,
        })
    }

    /// Initialize all database schemas for testing
    /// 
    /// NOTE: Table names and column names MUST match the actual repository SQL queries.
    /// Repositories use singular table names (memory, embedding, model, device, etc.)
    /// and specific column names (type vs memory_type, etc.)
    fn init_schemas(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            r#"
            -- =====================================================================
            -- Core tables - names match repository SQL queries
            -- =====================================================================
            
            -- memory_repository: Table 'memory', uses 'type' column
            CREATE TABLE IF NOT EXISTS memory (
                id TEXT PRIMARY KEY,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                type TEXT NOT NULL,
                content TEXT NOT NULL,
                metadata TEXT,
                source_agent TEXT,
                parent_id TEXT,
                tags TEXT NOT NULL DEFAULT '[]',
                embedding_id TEXT,
                checksum TEXT NOT NULL,
                FOREIGN KEY (parent_id) REFERENCES memory(id),
                FOREIGN KEY (source_agent) REFERENCES agents(id)
            );

            -- embedding_repository: Table 'embedding'
            CREATE TABLE IF NOT EXISTS embedding (
                id TEXT PRIMARY KEY,
                created_at TEXT NOT NULL,
                vector BLOB NOT NULL,
                model TEXT NOT NULL,
                source_type TEXT NOT NULL,
                source_id TEXT NOT NULL
            );

            -- agent_repository: Table 'agents'
            CREATE TABLE IF NOT EXISTS agents (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'active'
            );

            -- agentlog_repository: Table 'agent_logs'
            CREATE TABLE IF NOT EXISTS agent_logs (
                id TEXT PRIMARY KEY,
                agent_id TEXT NOT NULL,
                level TEXT NOT NULL,
                message TEXT NOT NULL,
                fields TEXT,
                FOREIGN KEY (agent_id) REFERENCES agents(id)
            );

            -- task_repository: Table 'tasks'
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                agent_id TEXT,
                title TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                payload TEXT,
                FOREIGN KEY (agent_id) REFERENCES agents(id)
            );

            -- task_event_repository: Table 'task_events'
            CREATE TABLE IF NOT EXISTS task_events (
                task_id TEXT NOT NULL,
                kind TEXT NOT NULL,
                message TEXT NOT NULL,
                FOREIGN KEY (task_id) REFERENCES tasks(id)
            );

            -- model_repository: Table 'model', uses 'type' column
            CREATE TABLE IF NOT EXISTS model (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                type TEXT NOT NULL,
                provider TEXT,
                path TEXT,
                uri TEXT,
                size_bytes INTEGER,
                parameters INTEGER,
                context_length INTEGER,
                license TEXT,
                configs TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                metrics TEXT
            );

            -- device_repository: Table 'device', uses 'type' column
            CREATE TABLE IF NOT EXISTS device (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                type TEXT NOT NULL,
                platform TEXT NOT NULL,
                peer_id TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'offline',
                last_seen TEXT,
                capabilities TEXT,
                resources TEXT,
                is_local INTEGER NOT NULL DEFAULT 0
            );

            -- knowledge_node_repository: Table 'knowledge_node'
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

            -- knowledge_edge_repository: Table 'knowledge_edge'
            CREATE TABLE IF NOT EXISTS knowledge_edge (
                id TEXT PRIMARY KEY,
                source_node TEXT NOT NULL,
                target_node TEXT NOT NULL,
                relationship TEXT NOT NULL,
                weight REAL DEFAULT 1.0,
                properties TEXT,
                FOREIGN KEY (source_node) REFERENCES knowledge_node(id),
                FOREIGN KEY (target_node) REFERENCES knowledge_node(id)
            );

            -- sync_repository: Table 'sync_state'
            CREATE TABLE IF NOT EXISTS sync_state (
                id TEXT PRIMARY KEY,
                device_id TEXT NOT NULL,
                entity_type TEXT NOT NULL,
                last_sync TEXT,
                local_version INTEGER NOT NULL DEFAULT 0,
                remote_version INTEGER NOT NULL DEFAULT 0,
                pending_ops TEXT,
                conflicts TEXT,
                FOREIGN KEY (device_id) REFERENCES device(id)
            );

            -- digest_repository: Table 'digest_source'
            CREATE TABLE IF NOT EXISTS digest_source (
                id TEXT PRIMARY KEY,
                type TEXT NOT NULL,
                uri TEXT NOT NULL,
                name TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                last_digest TEXT,
                version TEXT,
                profile TEXT,
                sbom TEXT,
                security_report TEXT,
                stats TEXT
            );

            -- stack_repository: Table 'stacks' (uses INTEGER autoincrement id)
            CREATE TABLE IF NOT EXISTS stacks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                stack_type TEXT NOT NULL,
                version TEXT,
                configs TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            -- =====================================================================
            -- Vector search configs table
            -- =====================================================================
            CREATE TABLE IF NOT EXISTS vss_configs (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            INSERT OR IGNORE INTO vss_configs (key, value) VALUES ('model', 'all-MiniLM-L6-v2');
            INSERT OR IGNORE INTO vss_configs (key, value) VALUES ('dimensions', '384');
            INSERT OR IGNORE INTO vss_configs (key, value) VALUES ('distance_metric', 'cosine');

            -- =====================================================================
            -- Create indexes for query performance
            -- =====================================================================
            CREATE INDEX IF NOT EXISTS idx_memory_type ON memory(type);
            CREATE INDEX IF NOT EXISTS idx_memory_source ON memory(source_agent);
            CREATE INDEX IF NOT EXISTS idx_embedding_source ON embedding(source_id);
            CREATE INDEX IF NOT EXISTS idx_tasks_agent ON tasks(agent_id);
            CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
            CREATE INDEX IF NOT EXISTS idx_agent_logs_agent ON agent_logs(agent_id);
            CREATE INDEX IF NOT EXISTS idx_device_status ON device(status);
            CREATE INDEX IF NOT EXISTS idx_device_platform ON device(platform);
            CREATE INDEX IF NOT EXISTS idx_knowledge_node_type ON knowledge_node(node_type);
            CREATE INDEX IF NOT EXISTS idx_knowledge_edge_source ON knowledge_edge(source_node);
            CREATE INDEX IF NOT EXISTS idx_knowledge_edge_target ON knowledge_edge(target_node);
            CREATE INDEX IF NOT EXISTS idx_sync_state_device ON sync_state(device_id);
            CREATE INDEX IF NOT EXISTS idx_digest_source_type ON digest_source(type);
            "#,
        ).map_err(|e| {
            crate::error::DatabaseError::QueryFailed {
                query: "Schema initialization".to_string(),
                error: e.to_string(),
            }
        })?;

        Ok(())
    }

    /// Get a reference to the connection
    pub fn conn(&self) -> &Connection {
        &self.conn
    }
}

// ============================================================================
// Sample Data Factories
// ============================================================================

use crate::db::repositories::{
    Memory, MemoryType, Task, Agent, 
    Embedding, Model, ModelType, ModelStatus,
    Device, DeviceType, Platform, DeviceStatus,
    KnowledgeNode, KnowledgeNodeType,
    DigestSource, DigestSourceType, DigestStatus,
};
use crate::db::repositories::sync_repository::SyncState;
use crate::db::repositories::knowledge_edge_repository::{KnowledgeEdge, RelationshipType};

/// Create a sample Memory for testing
pub fn sample_memory() -> Memory {
    Memory {
        id: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        memory_type: MemoryType::Interaction,
        content: "Test memory content".to_string(),
        metadata: None,
        source_agent: None,
        parent_id: None,
        tags: HashSet::from(["test".to_string(), "sample".to_string()]),
        embedding_id: None,
        checksum: "abc123".to_string(),
    }
}

/// Create a sample Memory with custom content
pub fn sample_memory_with_content(content: &str, memory_type: MemoryType) -> Memory {
    let mut memory = sample_memory();
    memory.content = content.to_string();
    memory.memory_type = memory_type;
    memory
}

/// Create a sample Task for testing
pub fn sample_task() -> Task {
    Task {
        id: Uuid::new_v4(),
        agent_id: None,
        title: "Test task".to_string(),
        status: "pending".to_string(),
        payload: None,
    }
}

/// Create a sample Task with custom title and status
pub fn sample_task_with_details(title: &str, status: &str, agent_id: Option<Uuid>) -> Task {
    Task {
        id: Uuid::new_v4(),
        agent_id,
        title: title.to_string(),
        status: status.to_string(),
        payload: Some(r#"{"priority": "high"}"#.to_string()),
    }
}

/// Create a sample Agent for testing
pub fn sample_agent() -> Agent {
    Agent {
        id: Uuid::new_v4(),
        name: "Test Agent".to_string(),
        description: Some("A test agent for unit testing".to_string()),
        status: "active".to_string(),
    }
}

/// Create a sample Agent with custom name
pub fn sample_agent_with_name(name: &str) -> Agent {
    Agent {
        id: Uuid::new_v4(),
        name: name.to_string(),
        description: Some(format!("Agent: {}", name)),
        status: "active".to_string(),
    }
}

/// Create a sample Embedding for testing
pub fn sample_embedding() -> Embedding {
    Embedding {
        id: Uuid::new_v4(),
        created_at: Utc::now(),
        vector: sample_embedding_vector(384),
        model: "all-MiniLM-L6-v2".to_string(),
        source_type: "memory".to_string(),
        source_id: Uuid::new_v4(),
    }
}

/// Create a sample Embedding with specific source
pub fn sample_embedding_for_source(source_type: &str, source_id: Uuid) -> Embedding {
    Embedding {
        id: Uuid::new_v4(),
        created_at: Utc::now(),
        vector: sample_embedding_vector(384),
        model: "all-MiniLM-L6-v2".to_string(),
        source_type: source_type.to_string(),
        source_id,
    }
}

/// Create a sample Model for testing
pub fn sample_model() -> Model {
    Model {
        id: Uuid::new_v4(),
        name: "test-model-7b".to_string(),
        model_type: ModelType::LLM,
        provider: "ollama".to_string(),
        path: Some("/models/test-model.gguf".to_string()),
        uri: Some("hf://test/model".to_string()),
        size_bytes: Some(4_000_000_000),
        parameters: Some("7B".to_string()),
        context_length: Some(4096),
        license: Some("Apache-2.0".to_string()),
        configs: serde_json::json!({"temperature": 0.7}),
        status: ModelStatus::Available,
        metrics: None,
    }
}

/// Create a sample Device for testing
pub fn sample_device() -> Device {
    Device {
        id: Uuid::new_v4(),
        name: "Test Workstation".to_string(),
        device_type: DeviceType::Desktop,
        platform: Platform::Windows,
        peer_id: format!("12D3KooW{}", Uuid::new_v4().simple()),
        status: DeviceStatus::Online,
        last_seen: Some(Utc::now()),
        capabilities: None,
        resources: None,
        is_local: true,
    }
}

/// Create a sample Device with custom platform
pub fn sample_device_with_platform(platform: Platform, device_type: DeviceType) -> Device {
    Device {
        id: Uuid::new_v4(),
        name: format!("Test {:?}", device_type),
        device_type,
        platform,
        peer_id: format!("12D3KooW{}", Uuid::new_v4().simple()),
        status: DeviceStatus::Online,
        last_seen: Some(Utc::now()),
        capabilities: None,
        resources: None,
        is_local: false,
    }
}

/// Create a sample KnowledgeNode for testing
pub fn sample_knowledge_node() -> KnowledgeNode {
    KnowledgeNode {
        id: Uuid::new_v4(),
        node_type: KnowledgeNodeType::Function,
        name: "test_function".to_string(),
        qualified_name: Some("module::test_function".to_string()),
        description: Some("A test function for unit testing".to_string()),
        source_digest: None,
        location: None,
        properties: None,
        embedding_id: None,
        created_at: Utc::now(),
    }
}

/// Create a sample KnowledgeEdge for testing
pub fn sample_knowledge_edge(source: Uuid, target: Uuid) -> KnowledgeEdge {
    KnowledgeEdge {
        id: Uuid::new_v4(),
        source_node: source,
        target_node: target,
        relationship: RelationshipType::Calls,
        weight: 1.0,
        properties: None,
    }
}

/// Create a sample SyncState for testing
pub fn sample_sync_state(device_id: Uuid) -> SyncState {
    SyncState {
        id: Uuid::new_v4(),
        device_id,
        entity_type: "memory".to_string(),
        last_sync: Some(Utc::now()),
        local_version: 1,
        remote_version: Some(1),
        pending_ops: None,
        conflicts: None,
    }
}

/// Create a sample DigestSource for testing
pub fn sample_digest_source() -> DigestSource {
    DigestSource {
        id: Uuid::new_v4(),
        source_type: DigestSourceType::Repository,
        uri: "file:///path/to/repo".to_string(),
        name: "test-repo".to_string(),
        status: DigestStatus::Pending,
        last_digest: None,
        version: Some("1.0.0".to_string()),
        profile: None,
        sbom: None,
        security_report: None,
        stats: None,
    }
}

/// Create a sample embedding vector (mock)
pub fn sample_embedding_vector(dimensions: usize) -> Vec<f32> {
    // Generate deterministic mock embedding for testing
    (0..dimensions)
        .map(|i| ((i as f32 * 0.1).sin() + 1.0) / 2.0)
        .collect()
}

/// Serialize embedding vector to bytes (for SQLite BLOB storage)
pub fn embedding_to_bytes(embedding: &[f32]) -> Vec<u8> {
    embedding
        .iter()
        .flat_map(|f| f.to_le_bytes())
        .collect()
}

/// Deserialize embedding vector from bytes
pub fn bytes_to_embedding(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks(4)
        .map(|chunk| {
            let arr: [u8; 4] = chunk.try_into().unwrap_or([0; 4]);
            f32::from_le_bytes(arr)
        })
        .collect()
}

// ============================================================================
// Assertion Helpers
// ============================================================================

/// Assert that two UUIDs are equal with a descriptive message
#[macro_export]
macro_rules! assert_uuid_eq {
    ($left:expr, $right:expr) => {
        assert_eq!($left.to_string(), $right.to_string(), "UUIDs should match");
    };
    ($left:expr, $right:expr, $msg:expr) => {
        assert_eq!($left.to_string(), $right.to_string(), $msg);
    };
}

/// Assert that a Result is Ok and return the value
#[macro_export]
macro_rules! assert_ok {
    ($result:expr) => {
        match $result {
            Ok(val) => val,
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    };
}

/// Assert that a Result is Err
#[macro_export]
macro_rules! assert_err {
    ($result:expr) => {
        match $result {
            Ok(val) => panic!("Expected Err, got Ok: {:?}", val),
            Err(_) => (),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_creation() {
        let db = TestDb::new().expect("Failed to create test database");
        assert!(db.path.exists());
    }

    #[test]
    fn test_in_memory_db() {
        let db = TestDb::in_memory().expect("Failed to create in-memory database");
        assert_eq!(db.path.to_string_lossy(), ":memory:");
    }

    #[test]
    fn test_sample_memory() {
        let memory = sample_memory();
        assert!(!memory.id.is_nil());
        assert_eq!(memory.memory_type, MemoryType::Interaction);
        assert!(memory.tags.contains("test"));
    }

    #[test]
    fn test_sample_task() {
        let task = sample_task();
        assert!(!task.id.is_nil());
        assert_eq!(task.status, "pending");
    }

    #[test]
    fn test_sample_agent() {
        let agent = sample_agent();
        assert!(!agent.id.is_nil());
        assert_eq!(agent.status, "active");
    }

    #[test]
    fn test_embedding_serialization() {
        let embedding = sample_embedding_vector(384);
        let bytes = embedding_to_bytes(&embedding);
        let restored = bytes_to_embedding(&bytes);
        
        assert_eq!(embedding.len(), restored.len());
        for (a, b) in embedding.iter().zip(restored.iter()) {
            assert!((a - b).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn test_sample_embedding() {
        let embedding = sample_embedding();
        assert!(!embedding.id.is_nil());
        assert_eq!(embedding.model, "all-MiniLM-L6-v2");
        assert_eq!(embedding.vector.len(), 384);
    }

    #[test]
    fn test_sample_model() {
        let model = sample_model();
        assert!(!model.id.is_nil());
        assert_eq!(model.model_type, ModelType::LLM);
        assert_eq!(model.status, ModelStatus::Available);
    }

    #[test]
    fn test_sample_device() {
        let device = sample_device();
        assert!(!device.id.is_nil());
        assert_eq!(device.device_type, DeviceType::Desktop);
        assert_eq!(device.platform, Platform::Windows);
        assert!(device.is_local);
    }

    #[test]
    fn test_sample_knowledge_node() {
        let node = sample_knowledge_node();
        assert!(!node.id.is_nil());
        assert_eq!(node.node_type, KnowledgeNodeType::Function);
    }

    #[test]
    fn test_sample_knowledge_edge() {
        let source = Uuid::new_v4();
        let target = Uuid::new_v4();
        let edge = sample_knowledge_edge(source, target);
        assert!(!edge.id.is_nil());
        assert_eq!(edge.source_node, source);
        assert_eq!(edge.target_node, target);
        assert_eq!(edge.relationship, RelationshipType::Calls);
    }

    #[test]
    fn test_sample_sync_state() {
        let device_id = Uuid::new_v4();
        let sync_state = sample_sync_state(device_id);
        assert!(!sync_state.id.is_nil());
        assert_eq!(sync_state.device_id, device_id);
    }

    #[test]
    fn test_sample_digest_source() {
        let source = sample_digest_source();
        assert!(!source.id.is_nil());
        assert_eq!(source.source_type, DigestSourceType::Repository);
        assert_eq!(source.status, DigestStatus::Pending);
    }
}
