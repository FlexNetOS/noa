//! Common types for NOA

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for NOA entities
pub type EntityId = Uuid;

/// Timestamp type
pub type Timestamp = DateTime<Utc>;

/// Generate a new entity ID
pub fn new_id() -> EntityId {
    Uuid::new_v4()
}

/// Get current timestamp
pub fn now() -> Timestamp {
    Utc::now()
}

/// Agent types in NOA
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentType {
    /// Permanent agents that persist across sessions
    Permanent,
    /// Reusable MicroAgentStack (mas_*)
    ReusableStack,
    /// Disposable MicroAgentStack (gen_mas)
    DisposableStack,
}

/// Agent lifecycle states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentState {
    Bootstrap,
    Execute,
    Validate,
    Package,
    Archive,
}

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low = 0,
    #[default]
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    #[default]
    Pending,
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Knowledge node types (from data-model.md)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeNodeType {
    Function,
    Class,
    Module,
    File,
    Repository,
    Concept,
}

/// Knowledge edge relationships (from data-model.md)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeEdgeType {
    Calls,
    Imports,
    Extends,
    Implements,
    Contains,
    References,
}

/// Digest source types (from data-model.md)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigestSourceType {
    Repository,
    File,
    Api,
    Document,
}

/// Device platform types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Windows,
    Linux,
    MacOS,
    Wsl,
}

/// Base metadata for all entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub id: EntityId,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}

impl Metadata {
    /// Create new metadata with current timestamp
    pub fn new() -> Self {
        let now = now();
        Self {
            id: new_id(),
            created_at: now,
            updated_at: now,
            checksum: None,
        }
    }

    /// Create metadata with specific ID
    pub fn with_id(id: EntityId) -> Self {
        let now = now();
        Self {
            id,
            created_at: now,
            updated_at: now,
            checksum: None,
        }
    }

    /// Update the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = now();
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}
