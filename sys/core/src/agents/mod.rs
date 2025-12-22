//! Agent Module (Phase 9 - US7)
//! Agent orchestration, executive, and board agents with full implementations.

pub mod base;
pub mod file_io;
pub mod terminal;
pub mod rag;
pub mod microservice_mgmt;
pub mod commander;
pub mod executive;
pub mod board;
pub mod model_selector;

pub use base::BaseAgent;
pub use file_io::{FileIOAgent, FileOperation, FileOperationResult};
pub use terminal::{TerminalAgent, TerminalCommand, TerminalResult};
pub use rag::{RAGAgent, RAGQuery, RAGResult, RAGResultItem};
pub use commander::{CommanderChiefAgent, CommanderRequest, ExecutionPlan, AgentTask, TaskPriority, TaskStatus};
pub use model_selector::ModelSelectorAgent;
