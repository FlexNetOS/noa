//! Co-Improvement Goal Intake
//!
//! Implements co-improvement goal intake for human-AI collaborative improvement.
//! FR-061-065: Full Autonomy Operation
//!
//! T627: Implement co-improvement goal intake

use crate::error::{Result, NoaError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Co-improvement goal source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoImprovementSource {
    /// User-suggested improvement
    User,
    /// AI-detected improvement opportunity
    AiDetected,
    /// Collaborative session outcome
    Collaborative,
}

/// Co-improvement goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoImprovementGoal {
    pub id: Uuid,
    pub source: CoImprovementSource,
    pub title: String,
    pub description: String,
    pub priority: i32,
    pub rationale: String,
    pub expected_benefit: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>, // User ID or agent name
    pub metadata: serde_json::Value,
}

/// Co-improvement intake manager
pub struct CoImprovementIntake {
    pending_goals: Arc<RwLock<Vec<CoImprovementGoal>>>,
    processed_goals: Arc<RwLock<Vec<Uuid>>>,
}

impl CoImprovementIntake {
    /// Create a new co-improvement intake manager
    pub fn new() -> Self {
        Self {
            pending_goals: Arc::new(RwLock::new(Vec::new())),
            processed_goals: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Submit a co-improvement goal
    pub async fn submit_goal(
        &self,
        source: CoImprovementSource,
        title: String,
        description: String,
        priority: i32,
        rationale: String,
        expected_benefit: String,
        created_by: Option<String>,
        metadata: serde_json::Value,
    ) -> Result<Uuid> {
        // Validate inputs
        if title.is_empty() {
            return Err(NoaError::Validation(crate::error::ValidationError::new(
                "title",
                "Title cannot be empty",
                "EMPTY_TITLE",
            )));
        }

        if description.is_empty() {
            return Err(NoaError::Validation(crate::error::ValidationError::new(
                "description",
                "Description cannot be empty",
                "EMPTY_DESCRIPTION",
            )));
        }

        if rationale.is_empty() {
            return Err(NoaError::Validation(crate::error::ValidationError::new(
                "rationale",
                "Rationale cannot be empty",
                "EMPTY_RATIONALE",
            )));
        }

        let goal = CoImprovementGoal {
            id: Uuid::new_v4(),
            source,
            title,
            description,
            priority,
            rationale,
            expected_benefit,
            created_at: Utc::now(),
            created_by,
            metadata,
        };

        let mut goals = self.pending_goals.write().await;
        goals.push(goal.clone());

        // Sort by priority (higher first)
        goals.sort_by(|a, b| b.priority.cmp(&a.priority));

        Ok(goal.id)
    }

    /// Get next pending goal
    pub async fn next_goal(&self) -> Option<CoImprovementGoal> {
        let mut goals = self.pending_goals.write().await;
        goals.pop()
    }

    /// Mark goal as processed
    pub async fn mark_processed(&self, goal_id: Uuid) {
        let mut processed = self.processed_goals.write().await;
        processed.push(goal_id);
    }

    /// Get all pending goals
    pub async fn pending_goals(&self) -> Vec<CoImprovementGoal> {
        self.pending_goals.read().await.clone()
    }

    /// Get pending goals count
    pub async fn pending_count(&self) -> usize {
        self.pending_goals.read().await.len()
    }

    /// Check if goal was processed
    pub async fn is_processed(&self, goal_id: Uuid) -> bool {
        self.processed_goals.read().await.contains(&goal_id)
    }

    /// Remove goal from pending (e.g., if rejected)
    pub async fn remove_goal(&self, goal_id: Uuid) -> bool {
        let mut goals = self.pending_goals.write().await;
        let initial_len = goals.len();
        goals.retain(|g| g.id != goal_id);
        goals.len() < initial_len
    }
}

impl Default for CoImprovementIntake {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_submit_goal() {
        let intake = CoImprovementIntake::new();

        let goal_id = intake.submit_goal(
            CoImprovementSource::User,
            "Improve error handling".to_string(),
            "Add better error messages".to_string(),
            10,
            "Users report unclear errors".to_string(),
            "Better UX".to_string(),
            Some("user123".to_string()),
            serde_json::json!({}),
        ).await.unwrap();

        assert_eq!(intake.pending_count().await, 1);
        assert!(!intake.is_processed(goal_id).await);
    }

    #[tokio::test]
    async fn test_validation() {
        let intake = CoImprovementIntake::new();

        // Empty title should fail
        assert!(intake.submit_goal(
            CoImprovementSource::User,
            "".to_string(),
            "Description".to_string(),
            10,
            "Rationale".to_string(),
            "Benefit".to_string(),
            None,
            serde_json::json!({}),
        ).await.is_err());
    }

    #[tokio::test]
    async fn test_priority_ordering() {
        let intake = CoImprovementIntake::new();

        intake.submit_goal(
            CoImprovementSource::User,
            "Low priority".to_string(),
            "Description".to_string(),
            5,
            "Rationale".to_string(),
            "Benefit".to_string(),
            None,
            serde_json::json!({}),
        ).await.unwrap();

        intake.submit_goal(
            CoImprovementSource::User,
            "High priority".to_string(),
            "Description".to_string(),
            10,
            "Rationale".to_string(),
            "Benefit".to_string(),
            None,
            serde_json::json!({}),
        ).await.unwrap();

        let next = intake.next_goal().await.unwrap();
        assert_eq!(next.title, "High priority");
    }
}

