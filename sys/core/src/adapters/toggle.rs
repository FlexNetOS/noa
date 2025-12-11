//! Adapter toggle framework.
//!
//! Provides a registry for enabling and disabling adapters and performing
//! lightweight health checks so that external integrations can be gated behind
//! explicit toggles.

use crate::error::{NoaError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Health status for an adapter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterHealth {
    /// Whether the adapter is currently healthy.
    pub healthy: bool,
    /// Optional message describing the health state.
    pub message: Option<String>,
}

impl AdapterHealth {
    /// Create a healthy status.
    pub fn healthy() -> Self {
        Self {
            healthy: true,
            message: None,
        }
    }

    /// Create an unhealthy status with a message.
    pub fn unhealthy(message: impl Into<String>) -> Self {
        Self {
            healthy: false,
            message: Some(message.into()),
        }
    }
}

/// Basic adapter trait used by the registry.
pub trait Adapter: Send + Sync {
    /// Stable adapter identifier.
    fn id(&self) -> &'static str;
    /// Human-readable description.
    fn description(&self) -> &'static str;
    /// Whether the adapter should start enabled.
    fn default_enabled(&self) -> bool {
        true
    }
    /// Perform a lightweight health check.
    fn check_health(&self) -> AdapterHealth;
}

struct AdapterEntry {
    enabled: bool,
    adapter: Box<dyn Adapter + Send + Sync>,
    last_health: AdapterHealth,
}

/// Adapter status returned to callers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterStatus {
    /// Adapter id.
    pub id: String,
    /// Adapter description.
    pub description: String,
    /// Whether adapter is enabled.
    pub enabled: bool,
    /// Most recent health check.
    pub health: AdapterHealth,
}

/// Registry that tracks adapter states and health checks.
#[derive(Default)]
pub struct AdapterRegistry {
    entries: HashMap<String, AdapterEntry>,
}

impl AdapterRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Register a new adapter.
    pub fn register<A>(&mut self, adapter: A)
    where
        A: Adapter + Send + Sync + 'static,
    {
        let id = adapter.id().to_string();
        let enabled = adapter.default_enabled();
        let health = adapter.check_health();

        self.entries.insert(
            id,
            AdapterEntry {
                enabled,
                adapter: Box::new(adapter),
                last_health: health,
            },
        );
    }

    /// Enable an adapter by id.
    pub fn enable(&mut self, id: &str) -> Result<()> {
        self.set_enabled(id, true)
    }

    /// Disable an adapter by id.
    pub fn disable(&mut self, id: &str) -> Result<()> {
        self.set_enabled(id, false)
    }

    fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<()> {
        let entry = self.entries.get_mut(id).ok_or_else(|| NoaError::NotFound {
            resource: "adapter".to_string(),
            id: id.to_string(),
        })?;
        entry.enabled = enabled;
        Ok(())
    }

    /// Get status for a single adapter (performs a health check when enabled).
    pub fn status(&mut self, id: &str) -> Result<AdapterStatus> {
        let entry = self.entries.get_mut(id).ok_or_else(|| NoaError::NotFound {
            resource: "adapter".to_string(),
            id: id.to_string(),
        })?;

        let health = if entry.enabled {
            entry.adapter.check_health()
        } else {
            AdapterHealth::unhealthy("adapter disabled")
        };

        entry.last_health = health.clone();

        Ok(AdapterStatus {
            id: id.to_string(),
            description: entry.adapter.description().to_string(),
            enabled: entry.enabled,
            health,
        })
    }

    /// List the current status for all registered adapters.
    pub fn list(&mut self) -> Vec<AdapterStatus> {
        let ids: Vec<String> = self.entries.keys().cloned().collect();
        ids.into_iter().filter_map(|id| self.status(&id).ok()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestAdapter;

    impl Adapter for TestAdapter {
        fn id(&self) -> &'static str {
            "test"
        }

        fn description(&self) -> &'static str {
            "Test adapter"
        }

        fn check_health(&self) -> AdapterHealth {
            AdapterHealth::healthy()
        }
    }

    #[test]
    fn registers_and_reports_status() {
        let mut registry = AdapterRegistry::new();
        registry.register(TestAdapter);

        let status = registry.status("test").unwrap();
        assert!(status.enabled);
        assert!(status.health.healthy);
    }

    #[test]
    fn disables_adapter_and_marks_unhealthy() {
        let mut registry = AdapterRegistry::new();
        registry.register(TestAdapter);
        registry.disable("test").unwrap();

        let status = registry.status("test").unwrap();
        assert!(!status.enabled);
        assert!(!status.health.healthy);
        assert_eq!(status.health.message.as_deref(), Some("adapter disabled"));
    }

    #[test]
    fn returns_not_found_for_unknown_adapter() {
        let mut registry = AdapterRegistry::new();
        let err = registry.status("missing").err().unwrap();
        match err {
            NoaError::NotFound { resource, id } => {
                assert_eq!(resource, "adapter");
                assert_eq!(id, "missing");
            }
            other => panic!("Unexpected error: {:?}", other),
        }
    }
}
