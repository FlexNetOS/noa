use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Mounted child VHDX descriptor.
#[derive(Debug, Clone)]
pub struct MountedChild {
    pub id: String,
    pub parent_id: String,
    pub mount_point: String,
    pub mounted_at: DateTime<Utc>,
}

/// Nested VHDX manager tracks child relationships.
pub struct NestedVhdxManager {
    children: HashMap<String, MountedChild>,
}

impl NestedVhdxManager {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }

    pub fn mount_child(&mut self, parent_id: &str, mount_point: &str) -> MountedChild {
        let child = MountedChild {
            id: uuid::Uuid::new_v4().to_string(),
            parent_id: parent_id.to_string(),
            mount_point: mount_point.to_string(),
            mounted_at: Utc::now(),
        };
        self.children.insert(child.id.clone(), child.clone());
        child
    }

    pub fn list(&self) -> Vec<MountedChild> {
        self.children.values().cloned().collect()
    }
}

impl Default for NestedVhdxManager {
    fn default() -> Self {
        Self::new()
    }
}
