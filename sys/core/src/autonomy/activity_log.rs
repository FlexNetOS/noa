//! Real-Time Activity Log
//!
//! Implements real-time activity log for observation-only monitoring.
//! §3.5: Transparent & Auditable
//! FR-061-065: Full Autonomy Operation
//!
//! T629: Implement real-time activity log (observation only)

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Activity type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivityType {
    /// Goal created
    GoalCreated,
    /// Goal started
    GoalStarted,
    /// Goal completed
    GoalCompleted,
    /// Goal failed
    GoalFailed,
    /// Agent action
    AgentAction,
    /// Plane transition
    PlaneTransition,
    /// Rollback initiated
    RollbackInitiated,
    /// Safety check
    SafetyCheck,
    /// Resource usage
    ResourceUsage,
}

/// Activity log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEntry {
    pub id: Uuid,
    pub activity_type: ActivityType,
    pub timestamp: DateTime<Utc>,
    pub component: String,
    pub message: String,
    pub metadata: serde_json::Value,
}

/// Activity log manager
pub struct ActivityLog {
    entries: Arc<RwLock<Vec<ActivityEntry>>>,
    max_entries: usize,
}

impl ActivityLog {
    /// Create a new activity log
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
            max_entries,
        }
    }

    /// Log an activity
    pub async fn log(
        &self,
        activity_type: ActivityType,
        component: String,
        message: String,
        metadata: serde_json::Value,
    ) {
        let entry = ActivityEntry {
            id: Uuid::new_v4(),
            activity_type,
            timestamp: Utc::now(),
            component,
            message,
            metadata,
        };

        let mut entries = self.entries.write().await;
        entries.push(entry);

        // Maintain max entries limit
        if entries.len() > self.max_entries {
            entries.remove(0);
        }
    }

    /// Get recent entries
    pub async fn recent_entries(&self, limit: usize) -> Vec<ActivityEntry> {
        let entries = self.entries.read().await;
        let start = entries.len().saturating_sub(limit);
        entries[start..].to_vec()
    }

    /// Get all entries
    pub async fn all_entries(&self) -> Vec<ActivityEntry> {
        self.entries.read().await.clone()
    }

    /// Get entries by type
    pub async fn entries_by_type(&self, activity_type: &ActivityType) -> Vec<ActivityEntry> {
        self.entries.read().await
            .iter()
            .filter(|e| {
                match (&e.activity_type, activity_type) {
                    (ActivityType::GoalCreated, ActivityType::GoalCreated) => true,
                    (ActivityType::GoalStarted, ActivityType::GoalStarted) => true,
                    (ActivityType::GoalCompleted, ActivityType::GoalCompleted) => true,
                    (ActivityType::GoalFailed, ActivityType::GoalFailed) => true,
                    (ActivityType::AgentAction, ActivityType::AgentAction) => true,
                    (ActivityType::PlaneTransition, ActivityType::PlaneTransition) => true,
                    (ActivityType::RollbackInitiated, ActivityType::RollbackInitiated) => true,
                    (ActivityType::SafetyCheck, ActivityType::SafetyCheck) => true,
                    (ActivityType::ResourceUsage, ActivityType::ResourceUsage) => true,
                    _ => false,
                }
            })
            .cloned()
            .collect()
    }

    /// Get entries since timestamp
    pub async fn entries_since(&self, since: DateTime<Utc>) -> Vec<ActivityEntry> {
        self.entries.read().await
            .iter()
            .filter(|e| e.timestamp >= since)
            .cloned()
            .collect()
    }

    /// Clear all entries
    pub async fn clear(&self) {
        let mut entries = self.entries.write().await;
        entries.clear();
    }

    /// Get entry count
    pub async fn count(&self) -> usize {
        self.entries.read().await.len()
    }
}

impl Default for ActivityLog {
    fn default() -> Self {
        Self::new(10000) // Default: 10,000 entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_log_entry() {
        let log = ActivityLog::new(100);

        log.log(
            ActivityType::GoalCreated,
            "GoalGenerator".to_string(),
            "New goal created".to_string(),
            serde_json::json!({}),
        ).await;

        assert_eq!(log.count().await, 1);
    }

    #[tokio::test]
    async fn test_max_entries() {
        let log = ActivityLog::new(5);

        for i in 0..10 {
            log.log(
                ActivityType::AgentAction,
                "Test".to_string(),
                format!("Entry {}", i),
                serde_json::json!({}),
            ).await;
        }

        assert_eq!(log.count().await, 5);

        let entries = log.all_entries().await;
        assert_eq!(entries[0].message, "Entry 5"); // First entry should be the 6th logged
    }

    #[tokio::test]
    async fn test_recent_entries() {
        let log = ActivityLog::new(100);

        for i in 0..10 {
            log.log(
                ActivityType::GoalCompleted,
                "Test".to_string(),
                format!("Entry {}", i),
                serde_json::json!({}),
            ).await;
        }

        let recent = log.recent_entries(3).await;
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[2].message, "Entry 9");
    }
}

