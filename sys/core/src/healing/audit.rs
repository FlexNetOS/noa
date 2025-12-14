//! Healing Audit Logger
//!
//! T619: Implement healing audit logger
//! FR-075: System MUST log all healing events for audit trail
//! §3.5: Transparent & Auditable

use crate::error::{NoaError, Result};
use crate::healing::HealingEvent;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Healing audit logger
pub struct HealingAuditLogger {
    events: Arc<RwLock<Vec<HealingEvent>>>,
    max_events: usize,
}

impl HealingAuditLogger {
    /// Create a new healing audit logger
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
            max_events,
        }
    }

    /// Log a healing event
    pub async fn log_event(&self, event: &HealingEvent) -> Result<()> {
        info!(
            event_id = %event.id,
            component_id = %event.component_id,
            status = ?event.status,
            "Logging healing event"
        );

        let mut events = self.events.write().await;
        events.push(event.clone());

        // Trim if over limit
        if events.len() > self.max_events {
            events.drain(0..events.len() - self.max_events);
        }

        // TODO: Also persist to database (HealingEvent table)
        debug!(event_id = %event.id, "Healing event logged");

        Ok(())
    }

    /// Log resolution of a healing event
    pub async fn log_resolution(&self, event_id: &uuid::Uuid) -> Result<()> {
        info!(event_id = %event_id, "Logging healing event resolution");

        let mut events = self.events.write().await;
        if let Some(event) = events.iter_mut().find(|e| e.id == *event_id) {
            event.status = crate::healing::HealingStatus::Resolved;
            event.resolved_at = Some(Utc::now());
        }

        // TODO: Also update database

        Ok(())
    }

    /// Get all healing events
    pub async fn get_events(&self) -> Vec<HealingEvent> {
        let events = self.events.read().await;
        events.clone()
    }

    /// Get events for a component
    pub async fn get_component_events(&self, component_id: &str) -> Vec<HealingEvent> {
        let events = self.events.read().await;
        events
            .iter()
            .filter(|e| e.component_id == component_id)
            .cloned()
            .collect()
    }
}

impl Default for HealingAuditLogger {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_healing_audit_logger() {
        let logger = HealingAuditLogger::new(100);
        let event = HealingEvent {
            id: uuid::Uuid::new_v4(),
            component_id: "test-component".to_string(),
            component_type: "service".to_string(),
            detected_at: Utc::now(),
            status: crate::healing::HealingStatus::Detected,
            health_before: crate::healing::ComponentHealth::Unhealthy,
            anomaly_type: Some("threshold_exceeded".to_string()),
            root_cause: None,
            fix_applied: None,
            fix_attempts: 0,
            validated: false,
            escalated: false,
            resolved_at: None,
            metadata: serde_json::json!({}),
        };

        logger.log_event(&event).await.unwrap();
        let events = logger.get_events().await;
        assert_eq!(events.len(), 1);
    }
}

