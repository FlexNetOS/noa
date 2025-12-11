//! Agent Module (Phase 9 - US7)
//! Stubs for agent orchestration, executive, and board agents.

pub mod base;
pub mod board;
pub mod commander;
pub mod executive;
pub mod file_io;
pub mod microservice_mgmt;
pub mod model_selector;
pub mod rag;
pub mod terminal;

pub use model_selector::ModelSelectorAgent;
