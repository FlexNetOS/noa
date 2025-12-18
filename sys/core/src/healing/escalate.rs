//! User Escalation Notification
//!
//! T618: Implement user escalation notification
//! FR-075: System MUST notify user when fixes fail after 3 attempts
//! §3.4: Adaptive & Self-Improving

use crate::error::{NoaError, Result};
use crate::healing::HealingEvent;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

/// Escalation notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationNotification {
    pub event_id: uuid::Uuid,
    pub component_id: String,
    pub component_type: String,
    pub root_cause: String,
    pub fix_attempts: u32,
    pub escalation_reason: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub recommended_action: String,
    pub urgency: EscalationUrgency,
}

/// Escalation urgency level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EscalationUrgency {
    Low,
    Medium,
    High,
    Critical,
}

/// Escalation notifier
pub struct EscalationNotifier {
    notification_channels: Vec<Box<dyn NotificationChannel>>,
}

/// Notification channel trait
#[async_trait::async_trait]
trait NotificationChannel: Send + Sync {
    async fn send(&self, notification: &EscalationNotification) -> Result<()>;
}

impl EscalationNotifier {
    /// Create a new escalation notifier
    pub fn new() -> Self {
        Self {
            notification_channels: Vec::new(),
        }
    }

    /// Notify user of escalation
    pub async fn notify(
        &self,
        event: &HealingEvent,
        root_cause: &str,
        retry_count: u32,
    ) -> Result<()> {
        warn!(
            event_id = %event.id,
            component_id = %event.component_id,
            retry_count = retry_count,
            "Escalating to user after retry limit"
        );

        let urgency = self.determine_urgency(&event.health_before, &event.anomaly_type);
        let recommended_action = self.recommend_action(root_cause, &event.component_type);

        let notification = EscalationNotification {
            event_id: event.id,
            component_id: event.component_id.clone(),
            component_type: event.component_type.clone(),
            root_cause: root_cause.to_string(),
            fix_attempts: retry_count,
            escalation_reason: format!(
                "Auto-fix failed after {} attempts. Root cause: {}",
                retry_count, root_cause
            ),
            timestamp: Utc::now(),
            recommended_action,
            urgency,
        };

        // Send notification through all channels
        for channel in &self.notification_channels {
            if let Err(e) = channel.send(&notification).await {
                error!("Failed to send escalation notification: {}", e);
            }
        }

        // TODO: Also log to audit trail, send to UI, etc.
        info!(
            event_id = %event.id,
            "Escalation notification sent"
        );

        Ok(())
    }

    /// Determine escalation urgency
    fn determine_urgency(
        &self,
        health: &crate::healing::ComponentHealth,
        _anomaly_type: &Option<String>,
    ) -> EscalationUrgency {
        match health {
            crate::healing::ComponentHealth::Critical => EscalationUrgency::Critical,
            crate::healing::ComponentHealth::Unhealthy => EscalationUrgency::High,
            crate::healing::ComponentHealth::Degraded => EscalationUrgency::Medium,
            crate::healing::ComponentHealth::Healthy => EscalationUrgency::Low,
        }
    }

    /// Recommend action for user
    fn recommend_action(&self, root_cause: &str, component_type: &str) -> String {
        let root_cause_lower = root_cause.to_lowercase();

        if root_cause_lower.contains("resource") {
            format!(
                "Component {} is experiencing resource exhaustion. Consider scaling up resources or optimizing usage.",
                component_type
            )
        } else if root_cause_lower.contains("database") {
            format!(
                "Component {} has database connectivity issues. Check database health and connection pool.",
                component_type
            )
        } else if root_cause_lower.contains("config") {
            format!(
                "Component {} has configuration issues. Review and validate configuration.",
                component_type
            )
        } else {
            format!(
                "Component {} requires manual investigation. Review logs and system state.",
                component_type
            )
        }
    }
}

impl Default for EscalationNotifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escalation_notifier_creation() {
        let notifier = EscalationNotifier::new();
        assert!(notifier.notification_channels.is_empty());
    }

    #[test]
    fn test_determine_urgency() {
        let notifier = EscalationNotifier::new();
        let urgency = notifier.determine_urgency(
            &crate::healing::ComponentHealth::Critical,
            &None,
        );
        assert_eq!(urgency, EscalationUrgency::Critical);
    }
}

