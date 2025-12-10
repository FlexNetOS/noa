//! Database Repositories
//!
//! Repository implementations for database entities.
//! §3.7: Total Memory Sovereignty - persistent storage

pub mod agent_repository;
pub mod agentlog_repository;
pub mod device_repository;
pub mod digest_repository;
pub mod embedding_repository;
pub mod knowledge_edge_repository;
pub mod knowledge_node_repository;
pub mod memory_repository;
pub mod model_repository;
pub mod sync_repository;
pub mod task_event_repository;
pub mod task_repository;

pub use agent_repository::{Agent, AgentRepository};
pub use agentlog_repository::{AgentLog, AgentLogRepository};
pub use device_repository::{Device, DeviceRepository, DeviceStatus, DeviceType, Platform};
pub use digest_repository::{DigestRepository, DigestSource, DigestSourceType, DigestStatus};
pub use embedding_repository::{Embedding, EmbeddingRepository};
pub use knowledge_edge_repository::KnowledgeEdgeRepository;
pub use knowledge_node_repository::{KnowledgeNodeRepository, KnowledgeNodeType};
pub use memory_repository::{Memory, MemoryRepository};
pub use model_repository::{Model, ModelRepository, ModelStatus, ModelType};
pub use sync_repository::SyncRepository;
pub use task_event_repository::{TaskEvent, TaskEventRepository};
pub use task_repository::{Task, TaskRepository};
