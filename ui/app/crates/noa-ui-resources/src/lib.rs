//! # NOA UI Resources
//!
//! Resource management for NOA UI providing:
//! - Agent registry
//! - Command registry
//! - Data store
//! - Embedding cache
//! - Log store
//! - Prompt cache
//! - Resource sharing
//! - Skill registry

pub mod agent_registry;
pub mod command_registry;
pub mod data_store;
pub mod embedding_cache;
pub mod log_store;
pub mod prompt_cache;
pub mod resource_sharing;
pub mod skill_registry;

pub use agent_registry::{Agent, AgentRegistry, AgentStatus};
pub use command_registry::{Command, CommandRegistry};
pub use data_store::{DataItem, DataStore};
pub use embedding_cache::{CachedEmbedding, Embedding, EmbeddingCache, EmbeddingMetadata};
pub use log_store::{LogEntry, LogStore};
pub use prompt_cache::{
    AccessLevel, CachedPrompt, Prompt, PromptCache, PromptExample, PromptMetadata, UsageStats,
    ValidationRule,
};
pub use resource_sharing::{
    Permission, ResourceContent, ResourceMetadata, ResourceSharingManager,
    ResourceType, SharedResource, SharingConfig, SyncState, SyncStatus,
    Visibility, AccessLogEntry,
};
pub use skill_registry::{
    ExecutionContext, ExecutionResult, ExecutionStats, ParameterType, ParameterValidation,
    Skill, SkillCategory, SkillExample, SkillImplementation, SkillMetadata,
    SkillParameter, SkillRegistry, SkillStatus,
};

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main resource manager coordinating all resource sub-managers
pub struct ResourceManager {
    pub prompt_cache: Arc<RwLock<PromptCache>>,
    pub embedding_cache: Arc<RwLock<EmbeddingCache>>,
    pub skill_registry: Arc<RwLock<SkillRegistry>>,
    pub agent_registry: Arc<RwLock<AgentRegistry>>,
    pub command_registry: Arc<RwLock<CommandRegistry>>,
    pub data_store: Arc<RwLock<DataStore>>,
    pub log_store: Arc<RwLock<LogStore>>,
    pub sharing_manager: Arc<RwLock<ResourceSharingManager>>,
}

impl ResourceManager {
    /// Create a new ResourceManager
    pub fn new() -> Self {
        Self {
            prompt_cache: Arc::new(RwLock::new(PromptCache::new())),
            embedding_cache: Arc::new(RwLock::new(EmbeddingCache::new())),
            skill_registry: Arc::new(RwLock::new(SkillRegistry::new())),
            agent_registry: Arc::new(RwLock::new(AgentRegistry::new())),
            command_registry: Arc::new(RwLock::new(CommandRegistry::new())),
            data_store: Arc::new(RwLock::new(DataStore::new())),
            log_store: Arc::new(RwLock::new(LogStore::new())),
            sharing_manager: Arc::new(RwLock::new(ResourceSharingManager::new())),
        }
    }

    /// Initialize all resource managers
    pub async fn initialize(&self) -> Result<()> {
        self.prompt_cache.write().await.initialize().await?;
        self.embedding_cache.write().await.initialize().await?;
        self.skill_registry.write().await.initialize().await?;
        self.agent_registry.write().await.initialize().await?;
        self.command_registry.write().await.initialize().await?;
        self.data_store.write().await.initialize().await?;
        self.log_store.write().await.initialize().await?;
        self.sharing_manager.write().await.initialize().await?;

        Ok(())
    }

    /// Cleanup all resource managers
    pub async fn cleanup(&self) -> Result<()> {
        self.prompt_cache.write().await.cleanup().await?;
        self.embedding_cache.write().await.cleanup().await?;
        self.skill_registry.write().await.cleanup().await?;
        self.agent_registry.write().await.cleanup().await?;
        self.command_registry.write().await.cleanup().await?;
        self.data_store.write().await.cleanup().await?;
        self.log_store.write().await.cleanup().await?;
        self.sharing_manager.write().await.cleanup().await?;

        Ok(())
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resource_manager_creation() {
        let manager = ResourceManager::new();
        assert!(manager.initialize().await.is_ok());
    }
}
