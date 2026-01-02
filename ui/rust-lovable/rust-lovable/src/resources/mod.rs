use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod agent_registry;
pub mod command_registry;
pub mod data_store;
pub mod embedding_cache;
pub mod log_store;
pub mod prompt_cache;
pub mod resource_sharing;
pub mod skill_registry;

use agent_registry::AgentRegistry;
use command_registry::CommandRegistry;
use data_store::DataStore;
use embedding_cache::EmbeddingCache;
use log_store::LogStore;
use prompt_cache::PromptCache;
use resource_sharing::ResourceSharingManager;
use skill_registry::SkillRegistry;

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

    pub async fn initialize(&self) -> Result<()> {
        // Initialize all resource managers
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

    pub async fn cleanup(&self) -> Result<()> {
        // Cleanup all resource managers
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
