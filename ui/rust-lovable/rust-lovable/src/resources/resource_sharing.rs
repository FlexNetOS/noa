use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedResource {
    pub id: String,
    pub resource_type: ResourceType,
    pub data: serde_json::Value,
    pub metadata: ResourceMetadata,
    pub sharing_config: SharingConfig,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceType {
    Prompt,
    Embedding,
    Skill,
    Agent,
    Command,
    Log,
    Data,
    Model,
    Configuration,
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::Prompt => write!(f, "prompt"),
            ResourceType::Embedding => write!(f, "embedding"),
            ResourceType::Skill => write!(f, "skill"),
            ResourceType::Agent => write!(f, "agent"),
            ResourceType::Command => write!(f, "command"),
            ResourceType::Log => write!(f, "log"),
            ResourceType::Data => write!(f, "data"),
            ResourceType::Model => write!(f, "model"),
            ResourceType::Configuration => write!(f, "configuration"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetadata {
    pub owner_id: String,
    pub version: String,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub size_bytes: usize,
    pub checksum: String,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharingConfig {
    pub visibility: Visibility,
    pub allowed_users: Vec<String>,
    pub allowed_roles: Vec<String>,
    pub permissions: SharingPermissions,
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    pub synchronization: SynchronizationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Private,
    Internal,
    Public,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharingPermissions {
    pub read: bool,
    pub write: bool,
    pub delete: bool,
    pub share: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynchronizationConfig {
    pub enabled: bool,
    pub sync_interval_seconds: u64,
    pub conflict_resolution: ConflictResolutionStrategy,
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConflictResolutionStrategy {
    LastWriteWins,
    TimestampBased,
    OwnerPriority,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceChange {
    pub resource_id: String,
    pub change_type: ChangeType,
    pub old_data: Option<serde_json::Value>,
    pub new_data: Option<serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeType {
    Created,
    Updated,
    Deleted,
    Shared,
    Unshared,
}

#[derive(Debug, Clone)]
pub struct SharingEvent {
    pub event_id: String,
    pub event_type: SharingEventType,
    pub resource_id: String,
    pub user_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SharingEventType {
    ResourceShared,
    ResourceUnshared,
    PermissionChanged,
    SyncCompleted,
    ConflictDetected,
    AccessDenied,
}

pub struct ResourceSharingManager {
    resources: Arc<RwLock<HashMap<String, SharedResource>>>,
    change_log: Arc<RwLock<Vec<ResourceChange>>>,
    event_handlers: Arc<RwLock<Vec<tokio::sync::mpsc::Sender<SharingEvent>>>>,
    providers: Arc<RwLock<HashMap<String, Box<dyn SharingProvider>>>>,
    sync_queue: Arc<RwLock<Vec<SyncTask>>>,
}

#[async_trait::async_trait]
pub trait SharingProvider: Send + Sync {
    async fn store(&self, resource: SharedResource) -> Result<()>;
    async fn retrieve(&self, resource_id: &str) -> Result<Option<SharedResource>>;
    async fn update(&self, resource_id: &str, updates: HashMap<String, serde_json::Value>) -> Result<()>;
    async fn delete(&self, resource_id: &str) -> Result<()>;
    async fn list(&self, filters: HashMap<String, String>) -> Result<Vec<SharedResource>>;
    async fn sync(&self, since: chrono::DateTime<chrono::Utc>) -> Result<Vec<ResourceChange>>;
}

#[derive(Debug, Clone)]
pub struct SyncTask {
    pub resource_id: String,
    pub action: SyncAction,
    pub priority: u32,
    pub retry_count: u32,
    pub max_retries: u32,
}

#[derive(Debug, Clone)]
pub enum SyncAction {
    Store,
    Update,
    Delete,
}

impl ResourceSharingManager {
    pub fn new() -> Self {
        Self {
            resources: Arc::new(RwLock::new(HashMap::new())),
            change_log: Arc::new(RwLock::new(Vec::new())),
            event_handlers: Arc::new(RwLock::new(Vec::new())),
            providers: Arc::new(RwLock::new(HashMap::new())),
            sync_queue: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        // Register default providers
        self.register_provider("local".to_string(), Box::new(LocalFileProvider::new()));
        
        // Start sync worker
        self.start_sync_worker();
        
        Ok(())
    }

    pub async fn cleanup(&mut self) -> Result<()> {
        // Cleanup resources
        self.sync_all().await?;
        Ok(())
    }

    pub fn register_provider(&mut self, name: String, provider: Box<dyn SharingProvider>) {
        self.providers.blocking_write().insert(name, provider);
    }

    pub async fn share_resource(&self, resource: SharedResource) -> Result<()> {
        let resource_id = resource.id.clone();
        
        // Store locally
        self.resources.write().await.insert(resource_id.clone(), resource.clone());
        
        // Add to change log
        let change = ResourceChange {
            resource_id: resource_id.clone(),
            change_type: ChangeType::Created,
            old_data: None,
            new_data: Some(resource.data.clone()),
            timestamp: chrono::Utc::now(),
            user_id: resource.metadata.owner_id.clone(),
        };
        self.change_log.write().await.push(change);
        
        // Queue for sync
        self.queue_sync_task(SyncTask {
            resource_id: resource_id.clone(),
            action: SyncAction::Store,
            priority: 1,
            retry_count: 0,
            max_retries: 3,
        });
        
        // Emit event
        self.emit_event(SharingEvent {
            event_id: Uuid::new_v4().to_string(),
            event_type: SharingEventType::ResourceShared,
            resource_id: resource_id.clone(),
            user_id: resource.metadata.owner_id,
            timestamp: chrono::Utc::now(),
            data: HashMap::new(),
        }).await;
        
        Ok(())
    }

    pub async fn update_resource(&self, resource_id: &str, updates: HashMap<String, serde_json::Value>) -> Result<()> {
        let mut resources = self.resources.write().await;
        
        if let Some(mut resource) = resources.get_mut(resource_id) {
            let old_data = resource.data.clone();
            
            // Apply updates
            for (key, value) in updates {
                resource.data[key.clone()] = value;
            }
            
            resource.updated_at = chrono::Utc::now();
            
            // Add to change log
            let change = ResourceChange {
                resource_id: resource_id.to_string(),
                change_type: ChangeType::Updated,
                old_data: Some(old_data),
                new_data: Some(resource.data.clone()),
                timestamp: chrono::Utc::now(),
                user_id: resource.metadata.owner_id.clone(),
            };
            self.change_log.write().await.push(change);
            
            // Queue for sync
            self.queue_sync_task(SyncTask {
                resource_id: resource_id.to_string(),
                action: SyncAction::Update,
                priority: 2,
                retry_count: 0,
                max_retries: 3,
            });
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Resource not found"))
        }
    }

    pub async fn get_resource(&self, resource_id: &str, user_id: Option<&str>) -> Result<Option<SharedResource>> {
        let resources = self.resources.read().await;
        
        if let Some(resource) = resources.get(resource_id) {
            // Check permissions
            if self.check_permissions(resource, user_id, "read").await {
                Ok(Some(resource.clone()))
            } else {
                Err(anyhow::anyhow!("Access denied"))
            }
        } else {
            Ok(None)
        }
    }

    pub async fn list_resources(&self, filters: HashMap<String, String>, user_id: Option<&str>) -> Result<Vec<SharedResource>> {
        let resources = self.resources.read().await;
        let mut filtered_resources = Vec::new();
        
        for resource in resources.values() {
            // Apply filters
            let mut matches = true;
            for (key, value) in &filters {
                match key.as_str() {
                    "type" => matches = resource.resource_type.to_string() == *value,
                    "category" => matches = resource.metadata.tags.contains(value),
                    "owner" => matches = resource.metadata.owner_id == *value,
                    _ => continue,
                }
                if !matches {
                    break;
                }
            }
            
            // Check permissions
            if matches && self.check_permissions(resource, user_id, "read").await {
                filtered_resources.push(resource.clone());
            }
        }
        
        Ok(filtered_resources)
    }

    pub async fn change_permissions(&self, resource_id: &str, permissions: SharingPermissions, user_id: &str) -> Result<()> {
        let mut resources = self.resources.write().await;
        
        if let Some(resource) = resources.get_mut(resource_id) {
            // Verify ownership
            if resource.metadata.owner_id != user_id {
                return Err(anyhow::anyhow!("Only the owner can change permissions"));
            }
            
            resource.sharing_config.permissions = permissions;
            resource.updated_at = chrono::Utc::now();
            
            // Emit event
            self.emit_event(SharingEvent {
                event_id: Uuid::new_v4().to_string(),
                event_type: SharingEventType::PermissionChanged,
                resource_id: resource_id.to_string(),
                user_id: user_id.to_string(),
                timestamp: chrono::Utc::now(),
                data: HashMap::new(),
            }).await;
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Resource not found"))
        }
    }

    pub async fn add_event_handler(&self, handler: tokio::sync::mpsc::Sender<SharingEvent>) {
        self.event_handlers.write().await.push(handler);
    }

    pub async fn get_change_history(&self, resource_id: &str, since: Option<chrono::DateTime<chrono::Utc>>) -> Vec<ResourceChange> {
        let change_log = self.change_log.read().await;
        
        change_log.iter()
            .filter(|change| {
                change.resource_id == resource_id &&
                since.map_or(true, |since_time| change.timestamp > since_time)
            })
            .cloned()
            .collect()
    }

    pub async fn sync_all(&self) -> Result<()> {
        let resources = self.resources.read().await;
        
        for resource in resources.values() {
            self.queue_sync_task(SyncTask {
                resource_id: resource.id.clone(),
                action: SyncAction::Update,
                priority: 3,
                retry_count: 0,
                max_retries: 3,
            });
        }
        
        Ok(())
    }

    async fn check_permissions(&self, resource: &SharedResource, user_id: Option<&str>, action: &str) -> bool {
        let permissions = &resource.sharing_config.permissions;
        
        match action {
            "read" => permissions.read,
            "write" => permissions.write,
            "delete" => permissions.delete,
            "share" => permissions.share,
            _ => false,
        }
    }

    fn queue_sync_task(&self, task: SyncTask) {
        self.sync_queue.blocking_write().push(task);
    }

    fn start_sync_worker(&self) {
        let sync_queue = self.sync_queue.clone();
        let providers = self.providers.clone();
        let resources = self.resources.clone();
        
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                
                let mut queue = sync_queue.write().await;
                if !queue.is_empty() {
                    let task = queue.remove(0);
                    drop(queue);
                    
                    // Process sync task
                    if let Err(e) = process_sync_task(task, &providers, &resources).await {
                        eprintln!("Sync task failed: {}", e);
                    }
                }
            }
        });
    }

    async fn emit_event(&self, event: SharingEvent) {
        let handlers = self.event_handlers.read().await;
        
        for handler in handlers.iter() {
            let _ = handler.send(event.clone()).await;
        }
    }
}

async fn process_sync_task(
    task: SyncTask,
    providers: &Arc<RwLock<HashMap<String, Box<dyn SharingProvider>>>>,
    resources: &Arc<RwLock<HashMap<String, SharedResource>>>,
) -> Result<()> {
    let providers = providers.read().await;
    let resources = resources.read().await;
    
    if let Some(resource) = resources.get(&task.resource_id) {
        for (_, provider) in providers.iter() {
            match task.action {
                SyncAction::Store => {
                    provider.store(resource.clone()).await?;
                },
                SyncAction::Update => {
                    let updates = HashMap::new(); // Extract updates from resource
                    provider.update(&resource.id, updates).await?;
                },
                SyncAction::Delete => {
                    provider.delete(&resource.id).await?;
                },
            }
        }
    }
    
    Ok(())
}

pub struct LocalFileProvider {
    base_path: std::path::PathBuf,
}

impl LocalFileProvider {
    pub fn new() -> Self {
        Self {
            base_path: std::path::PathBuf::from("./shared_resources"),
        }
    }
}

#[async_trait::async_trait]
impl SharingProvider for LocalFileProvider {
    async fn store(&self, resource: SharedResource) -> Result<()> {
        let path = self.base_path.join(format!("{}.json", resource.id));
        let json = serde_json::to_string_pretty(&resource)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }

    async fn retrieve(&self, resource_id: &str) -> Result<Option<SharedResource>> {
        let path = self.base_path.join(format!("{}.json", resource_id));
        
        match tokio::fs::read_to_string(path).await {
            Ok(contents) => {
                let resource: SharedResource = serde_json::from_str(&contents)?;
                Ok(Some(resource))
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn update(&self, resource_id: &str, updates: HashMap<String, serde_json::Value>) -> Result<()> {
        if let Some(mut resource) = self.retrieve(resource_id).await? {
            // Apply updates
            for (key, value) in updates {
                resource.data[key.clone()] = value;
            }
            
            resource.updated_at = chrono::Utc::now();
            self.store(resource).await?;
        }
        Ok(())
    }

    async fn delete(&self, resource_id: &str) -> Result<()> {
        let path = self.base_path.join(format!("{}.json", resource_id));
        tokio::fs::remove_file(path).await?;
        Ok(())
    }

    async fn list(&self, filters: HashMap<String, String>) -> Result<Vec<SharedResource>> {
        let mut resources = Vec::new();
        
        let mut entries = tokio::fs::read_dir(&self.base_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            if let Some(extension) = entry.path().extension() {
                if extension == "json" {
                    if let Some(filename) = entry.file_name().to_str() {
                        let resource_id = filename.trim_end_matches(".json");
                        if let Some(resource) = self.retrieve(resource_id).await? {
                            // Apply filters
                            let mut matches = true;
                            for (key, value) in &filters {
                                match key.as_str() {
                                    "type" => matches = resource.resource_type.to_string() == *value,
                                    "owner" => matches = resource.metadata.owner_id == *value,
                                    _ => continue,
                                }
                                if !matches {
                                    break;
                                }
                            }
                            
                            if matches {
                                resources.push(resource);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(resources)
    }

    async fn sync(&self, since: chrono::DateTime<chrono::Utc>) -> Result<Vec<ResourceChange>> {
        // Simple implementation - in production, this would track changes
        Ok(Vec::new())
    }
}