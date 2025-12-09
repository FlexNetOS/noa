//! Permanent agents that persist across sessions

use noa_common::{AgentType, EntityId, Metadata};
use serde::{Deserialize, Serialize};

/// FileIO Agent - handles file system operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileIOAgent {
    pub metadata: Metadata,
    pub noa_root: String,
}

impl FileIOAgent {
    /// Create a new FileIO agent
    pub fn new(noa_root: &str) -> Self {
        Self {
            metadata: Metadata::new(),
            noa_root: noa_root.to_string(),
        }
    }

    /// Read a file within noa_root
    pub async fn read_file(&self, path: &str) -> noa_common::Result<Vec<u8>> {
        // Validate path is within noa_root
        if !path.starts_with(&self.noa_root) {
            return Err(noa_common::NoaError::Unauthorized(
                "Path outside noa_root".into(),
            ));
        }
        tokio::fs::read(path)
            .await
            .map_err(|e| noa_common::NoaError::Io { source: e })
    }

    /// Write a file within noa_root
    pub async fn write_file(&self, path: &str, contents: &[u8]) -> noa_common::Result<()> {
        if !path.starts_with(&self.noa_root) {
            return Err(noa_common::NoaError::Unauthorized(
                "Path outside noa_root".into(),
            ));
        }
        tokio::fs::write(path, contents)
            .await
            .map_err(|e| noa_common::NoaError::Io { source: e })
    }
}

/// Terminal Agent - executes shell commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalAgent {
    pub metadata: Metadata,
    pub timeout_secs: u64,
}

impl Default for TerminalAgent {
    fn default() -> Self {
        Self {
            metadata: Metadata::new(),
            timeout_secs: 30, // FR-008: 30s default timeout
        }
    }
}

/// RAG Agent - retrieves relevant context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGAgent {
    pub metadata: Metadata,
    pub relevance_threshold: f32,
}

impl Default for RAGAgent {
    fn default() -> Self {
        Self {
            metadata: Metadata::new(),
            relevance_threshold: 0.8, // FR-008: >80% relevance
        }
    }
}

/// Microservice Management Agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroserviceManagementAgent {
    pub metadata: Metadata,
    pub deploy_timeout_secs: u64,
    pub health_check_interval_secs: u64,
}

impl Default for MicroserviceManagementAgent {
    fn default() -> Self {
        Self {
            metadata: Metadata::new(),
            deploy_timeout_secs: 10,    // FR-008: deploys within 10s
            health_check_interval_secs: 1, // FR-008: health check <1s
        }
    }
}

