//! Resource sharing for distributed resource management

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Resource type enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Model,
    Dataset,
    Prompt,
    Embedding,
    Configuration,
    Artifact,
    Custom(String),
}

/// A shared resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedResource {
    pub id: String,
    pub name: String,
    pub resource_type: ResourceType,
    pub owner_id: String,
    pub content: ResourceContent,
    pub sharing_config: SharingConfig,
    pub metadata: ResourceMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Resource content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceContent {
    pub data: serde_json::Value,
    pub content_type: String,
    pub size_bytes: usize,
    pub checksum: Option<String>,
}

/// Configuration for sharing a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharingConfig {
    pub visibility: Visibility,
    pub shared_with: Vec<String>,
    pub permissions: HashMap<String, Vec<Permission>>,
    pub expiration: Option<DateTime<Utc>>,
    pub max_access_count: Option<u64>,
    pub sync_enabled: bool,
}

/// Visibility levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Private,
    Shared,
    Public,
}

/// Permission types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Permission {
    Read,
    Write,
    Execute,
    Share,
    Delete,
    Admin,
}

/// Metadata for a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetadata {
    pub description: Option<String>,
    pub version: String,
    pub tags: Vec<String>,
    pub labels: HashMap<String, String>,
    pub source: Option<String>,
    pub license: Option<String>,
}

/// Access log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessLogEntry {
    pub resource_id: String,
    pub user_id: String,
    pub action: String,
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub details: Option<String>,
}

/// Sync status for a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub last_sync: Option<DateTime<Utc>>,
    pub sync_state: SyncState,
    pub pending_changes: usize,
    pub error: Option<String>,
}

/// Sync states
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SyncState {
    Synced,
    Pending,
    Syncing,
    Error,
    Disabled,
}

/// Manager for resource sharing
pub struct ResourceSharingManager {
    resources: Arc<RwLock<HashMap<String, SharedResource>>>,
    access_log: Arc<RwLock<Vec<AccessLogEntry>>>,
    sync_status: Arc<RwLock<HashMap<String, SyncStatus>>>,
    current_user_id: String,
}

impl ResourceSharingManager {
    /// Create a new ResourceSharingManager
    pub fn new() -> Self {
        Self {
            resources: Arc::new(RwLock::new(HashMap::new())),
            access_log: Arc::new(RwLock::new(Vec::new())),
            sync_status: Arc::new(RwLock::new(HashMap::new())),
            current_user_id: String::from("default-user"),
        }
    }

    /// Initialize the manager
    pub async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// Cleanup resources
    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Share a resource
    pub async fn share_resource(&self, resource: SharedResource) -> Result<()> {
        let mut resources = self.resources.write().await;
        resources.insert(resource.id.clone(), resource);
        Ok(())
    }

    /// Get a shared resource
    pub async fn get_resource(&self, id: &str) -> Option<SharedResource> {
        let resources = self.resources.read().await;
        let resource = resources.get(id).cloned();

        if let Some(ref res) = resource {
            self.log_access(id, "read", true).await;
            
            // Check permissions
            if !self.check_permission(res, Permission::Read) {
                return None;
            }
        }

        resource
    }

    /// List resources by type
    pub async fn list_by_type(&self, resource_type: &ResourceType) -> Vec<SharedResource> {
        let resources = self.resources.read().await;
        resources
            .values()
            .filter(|r| &r.resource_type == resource_type)
            .filter(|r| self.check_permission(r, Permission::Read))
            .cloned()
            .collect()
    }

    /// List resources shared with user
    pub async fn list_shared_with_me(&self) -> Vec<SharedResource> {
        let resources = self.resources.read().await;
        resources
            .values()
            .filter(|r| {
                r.sharing_config.shared_with.contains(&self.current_user_id)
                    || r.sharing_config.visibility == Visibility::Public
            })
            .cloned()
            .collect()
    }

    /// Update sharing configuration
    pub async fn update_sharing_config(
        &self,
        id: &str,
        config: SharingConfig,
    ) -> Result<()> {
        let mut resources = self.resources.write().await;
        if let Some(resource) = resources.get_mut(id) {
            if !self.check_permission(resource, Permission::Admin) {
                return Err(anyhow::anyhow!("Permission denied"));
            }
            resource.sharing_config = config;
            resource.updated_at = Utc::now();
            self.log_access(id, "update_sharing", true).await;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Resource not found"))
        }
    }

    /// Remove a shared resource
    pub async fn remove_resource(&self, id: &str) -> Option<SharedResource> {
        let mut resources = self.resources.write().await;
        if let Some(resource) = resources.get(id) {
            if !self.check_permission(resource, Permission::Delete) {
                return None;
            }
        }
        let removed = resources.remove(id);
        if removed.is_some() {
            drop(resources);
            self.log_access(id, "delete", true).await;
        }
        removed
    }

    /// Get sync status for a resource
    pub async fn get_sync_status(&self, id: &str) -> Option<SyncStatus> {
        let status = self.sync_status.read().await;
        status.get(id).cloned()
    }

    /// Trigger sync for a resource
    pub async fn sync_resource(&self, id: &str) -> Result<()> {
        let resources = self.resources.read().await;
        if let Some(resource) = resources.get(id) {
            if !resource.sharing_config.sync_enabled {
                return Err(anyhow::anyhow!("Sync disabled for this resource"));
            }
        } else {
            return Err(anyhow::anyhow!("Resource not found"));
        }
        drop(resources);

        // Update sync status
        let mut status = self.sync_status.write().await;
        status.insert(
            id.to_string(),
            SyncStatus {
                last_sync: Some(Utc::now()),
                sync_state: SyncState::Synced,
                pending_changes: 0,
                error: None,
            },
        );

        Ok(())
    }

    /// Get access log for a resource
    pub async fn get_access_log(&self, id: &str, limit: usize) -> Vec<AccessLogEntry> {
        let log = self.access_log.read().await;
        log.iter()
            .rev()
            .filter(|entry| entry.resource_id == id)
            .take(limit)
            .cloned()
            .collect()
    }

    fn check_permission(&self, resource: &SharedResource, permission: Permission) -> bool {
        // Owner has all permissions
        if resource.owner_id == self.current_user_id {
            return true;
        }

        // Check visibility
        if resource.sharing_config.visibility == Visibility::Public {
            return matches!(permission, Permission::Read);
        }

        // Check explicit permissions
        if let Some(permissions) = resource
            .sharing_config
            .permissions
            .get(&self.current_user_id)
        {
            return permissions.contains(&permission) || permissions.contains(&Permission::Admin);
        }

        false
    }

    async fn log_access(&self, resource_id: &str, action: &str, success: bool) {
        let mut log = self.access_log.write().await;
        log.push(AccessLogEntry {
            resource_id: resource_id.to_string(),
            user_id: self.current_user_id.clone(),
            action: action.to_string(),
            timestamp: Utc::now(),
            success,
            details: None,
        });

        // Keep log size manageable
        if log.len() > 10000 {
            log.drain(0..1000);
        }
    }
}

impl Default for ResourceSharingManager {
    fn default() -> Self {
        Self::new()
    }
}
