//! Database Repositories
//!
//! Repository implementations for database entities.
//! §3.7: Total Memory Sovereignty - persistent storage

pub mod memory_repository;
pub mod embedding_repository;
pub mod device_repository;
pub mod sync_repository;
pub mod model_repository;
pub mod digest_repository;
pub mod knowledge_node_repository;
pub mod knowledge_edge_repository;
pub mod agent_repository;
pub mod agentlog_repository;
pub mod task_repository;
pub mod task_event_repository;
pub mod stack_repository;

pub use memory_repository::{MemoryRepository, Memory};
pub use embedding_repository::{EmbeddingRepository, Embedding};
pub use device_repository::{DeviceRepository, Device, DeviceType, Platform, DeviceStatus};
pub use sync_repository::SyncRepository;
pub use model_repository::{ModelRepository, Model, ModelStatus, ModelType};
pub use digest_repository::{DigestRepository, DigestSourceType, DigestStatus, DigestSource};
pub use knowledge_node_repository::{KnowledgeNodeRepository, KnowledgeNode, KnowledgeNodeType};
pub use knowledge_edge_repository::KnowledgeEdgeRepository;
pub use agent_repository::{AgentRepository, Agent};
pub use agentlog_repository::{AgentLogRepository, AgentLog};
pub use task_repository::{TaskRepository, Task};
pub use task_event_repository::{TaskEventRepository, TaskEvent};
pub use stack_repository::{StackRepository, StackRecord};
