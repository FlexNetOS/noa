//! Goal Rationale Logger
//!
//! Implements goal rationale logging for transparency and auditability.
//! §3.5: Transparent & Auditable
//! FR-066-070: Autonomous Goal Generation
//!
//! T633: Implement goal rationale logger

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Rationale entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RationaleEntry {
    pub id: Uuid,
    pub goal_id: Uuid,
    pub rationale: String,
    pub evidence: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub created_by: String, // Agent or user name
}

/// Goal rationale logger
pub struct GoalRationaleLogger {
    entries: Arc<RwLock<Vec<RationaleEntry>>>,
}

impl GoalRationaleLogger {
    /// Create a new goal rationale logger
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Log goal rationale
    pub async fn log_rationale(
        &self,
        goal_id: Uuid,
        rationale: String,
        evidence: serde_json::Value,
        created_by: String,
    ) -> Result<Uuid> {
        if rationale.is_empty() {
            return Err(crate::error::NoaError::Validation(
                crate::error::ValidationError::new(
                    "rationale",
                    "Rationale cannot be empty",
                    "EMPTY_RATIONALE",
                ),
            ));
        }

        let entry = RationaleEntry {
            id: Uuid::new_v4(),
            goal_id,
            rationale,
            evidence,
            created_at: Utc::now(),
            created_by,
        };

        let mut entries = self.entries.write().await;
        entries.push(entry.clone());

        Ok(entry.id)
    }

    /// Get rationale for a goal
    pub async fn get_rationale(&self, goal_id: Uuid) -> Vec<RationaleEntry> {
        self.entries.read().await
            .iter()
            .filter(|e| e.goal_id == goal_id)
            .cloned()
            .collect()
    }

    /// Get all entries
    pub async fn all_entries(&self) -> Vec<RationaleEntry> {
        self.entries.read().await.clone()
    }

    /// Get entries by creator
    pub async fn entries_by_creator(&self, created_by: &str) -> Vec<RationaleEntry> {
        self.entries.read().await
            .iter()
            .filter(|e| e.created_by == created_by)
            .cloned()
            .collect()
    }

    /// Get entries since timestamp
    pub async fn entries_since(&self, since: DateTime<Utc>) -> Vec<RationaleEntry> {
        self.entries.read().await
            .iter()
            .filter(|e| e.created_at >= since)
            .cloned()
            .collect()
    }

    /// Get entry count
    pub async fn count(&self) -> usize {
        self.entries.read().await.len()
    }
}

impl Default for GoalRationaleLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_log_rationale() {
        let logger = GoalRationaleLogger::new();
        let goal_id = Uuid::new_v4();

        let entry_id = logger.log_rationale(
            goal_id,
            "Performance degradation detected".to_string(),
            serde_json::json!({"latency_increase": "50%"}),
            "GoalGenerator".to_string(),
        ).await.unwrap();

        let entries = logger.get_rationale(goal_id).await;
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id, entry_id);
    }

    #[tokio::test]
    async fn test_empty_rationale_validation() {
        let logger = GoalRationaleLogger::new();
        let goal_id = Uuid::new_v4();

        assert!(logger.log_rationale(
            goal_id,
            "".to_string(),
            serde_json::json!({}),
            "Test".to_string(),
        ).await.is_err());
    }

    #[tokio::test]
    async fn test_entries_by_creator() {
        let logger = GoalRationaleLogger::new();
        let goal_id = Uuid::new_v4();

        logger.log_rationale(
            goal_id,
            "Rationale 1".to_string(),
            serde_json::json!({}),
            "Agent1".to_string(),
        ).await.unwrap();

        logger.log_rationale(
            goal_id,
            "Rationale 2".to_string(),
            serde_json::json!({}),
            "Agent2".to_string(),
        ).await.unwrap();

        let agent1_entries = logger.entries_by_creator("Agent1").await;
        assert_eq!(agent1_entries.len(), 1);
    }
}

