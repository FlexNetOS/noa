//! Dynamic Architecture Adapter Modules
//!
//! T668: Implement dynamic architecture adapter modules
//! US2: Add task-specific adapters without modifying base model

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Task adapter configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adapterconfigs {
    pub adapter_size: usize,
    pub adapter_layers: Vec<usize>,
    pub activation: String,
}

impl Default for Adapterconfigs {
    fn default() -> Self {
        Self {
            adapter_size: 64,
            adapter_layers: vec![],
            activation: "relu".to_string(),
        }
    }
}

/// Task adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAdapter {
    pub id: Uuid,
    pub task_name: String,
    pub configs: Adapterconfigs,
    pub parameters: HashMap<String, Vec<f64>>,
}

/// Adapter manager
pub struct AdapterManager {
    adapters: HashMap<String, TaskAdapter>,
}

impl AdapterManager {
    /// Create a new adapter manager
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }

    /// Create adapter for a task
    pub fn create_adapter(&mut self, task_name: String, configs: Adapterconfigs) -> Uuid {
        let adapter = TaskAdapter {
            id: Uuid::new_v4(),
            task_name: task_name.clone(),
            configs,
            parameters: HashMap::new(),
        };

        let id = adapter.id;
        self.adapters.insert(task_name, adapter);
        id
    }

    /// Get adapter for a task
    pub fn get_adapter(&self, task_name: &str) -> Option<&TaskAdapter> {
        self.adapters.get(task_name)
    }

    /// List all adapters
    pub fn list_adapters(&self) -> Vec<&TaskAdapter> {
        self.adapters.values().collect()
    }
}

impl Default for AdapterManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_creation() {
        let mut manager = AdapterManager::new();
        let id = manager.create_adapter("test_task".to_string(), Adapterconfigs::default());
        assert!(manager.get_adapter("test_task").is_some());
    }
}

