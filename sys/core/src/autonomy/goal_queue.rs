//! User Goal Queue Manager
//!
//! T622: Implement user goal queue manager
//! FR-052: System MUST maintain a unified goal queue (user, self-generated, constitutional)
//! §3.4: Adaptive & Self-Improving

use crate::error::{NoaError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
use uuid::Uuid;

/// Goal source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalSource {
    User,
    SelfGenerated,
    Constitutional,
}

/// Goal status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalStatus {
    Pending,
    Active,
    Decomposing,
    Executing,
    Completed,
    Failed,
}

/// Goal in the queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: Uuid,
    pub source: GoalSource,
    pub title: String,
    pub description: Option<String>,
    pub priority: i32,
    pub status: GoalStatus,
    pub rationale: Option<String>,
    pub parent_goal: Option<Uuid>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Goal queue manager
pub struct GoalQueueManager {
    queue: Arc<RwLock<VecDeque<Goal>>>,
    max_queue_size: usize,
}

impl GoalQueueManager {
    /// Create a new goal queue manager
    pub fn new(max_queue_size: usize) -> Self {
        Self {
            queue: Arc::new(RwLock::new(VecDeque::new())),
            max_queue_size,
        }
    }

    /// Add a goal to the queue
    pub async fn add_goal(&self, goal: Goal) -> Result<()> {
        let mut queue = self.queue.write().await;

        if queue.len() >= self.max_queue_size {
            return Err(NoaError::Internal {
                message: format!("Goal queue full (max: {})", self.max_queue_size),
                source: None,
            });
        }

        // Insert in priority order (higher priority first)
        let priority = goal.priority;
        let mut inserted = false;
        for (idx, existing) in queue.iter().enumerate() {
            if priority > existing.priority {
                queue.insert(idx, goal.clone());
                inserted = true;
                break;
            }
        }

        if !inserted {
            queue.push_back(goal);
        }

        info!("Goal added to queue");
        Ok(())
    }

    /// Get next goal from queue
    pub async fn get_next_goal(&self) -> Option<Goal> {
        let mut queue = self.queue.write().await;
        queue.pop_front()
    }

    /// Peek at next goal without removing it
    pub async fn peek_next_goal(&self) -> Option<Goal> {
        let queue = self.queue.read().await;
        queue.front().cloned()
    }

    /// Update goal status
    pub async fn update_goal_status(
        &self,
        goal_id: &Uuid,
        status: GoalStatus,
    ) -> Result<()> {
        let mut queue = self.queue.write().await;

        // Find goal in queue
        for goal in queue.iter_mut() {
            if goal.id == *goal_id {
                goal.status = status;

                match status {
                    GoalStatus::Active | GoalStatus::Decomposing | GoalStatus::Executing => {
                        if goal.started_at.is_none() {
                            goal.started_at = Some(Utc::now());
                        }
                    }
                    GoalStatus::Completed | GoalStatus::Failed => {
                        goal.completed_at = Some(Utc::now());
                    }
                    _ => {}
                }

                debug!(goal_id = %goal_id, status = ?status, "Updated goal status");
                return Ok(());
            }
        }

        Err(NoaError::NotFound {
            resource: "Goal".to_string(),
            id: goal_id.to_string(),
        })
    }

    /// Get queue size
    pub async fn queue_size(&self) -> usize {
        let queue = self.queue.read().await;
        queue.len()
    }

    /// Get all goals
    pub async fn get_all_goals(&self) -> Vec<Goal> {
        let queue = self.queue.read().await;
        queue.iter().cloned().collect()
    }

    /// Remove goal from queue
    pub async fn remove_goal(&self, goal_id: &Uuid) -> Result<()> {
        let mut queue = self.queue.write().await;
        let original_len = queue.len();
        queue.retain(|g| g.id != *goal_id);

        if queue.len() < original_len {
            debug!(goal_id = %goal_id, "Removed goal from queue");
            Ok(())
        } else {
            Err(NoaError::NotFound {
                resource: "Goal".to_string(),
                id: goal_id.to_string(),
            })
        }
    }
}

impl Default for GoalQueueManager {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_goal_queue_manager() {
        let manager = GoalQueueManager::new(100);
        assert_eq!(manager.queue_size().await, 0);

        let goal = Goal {
            id: Uuid::new_v4(),
            source: GoalSource::User,
            title: "Test Goal".to_string(),
            description: None,
            priority: 10,
            status: GoalStatus::Pending,
            rationale: None,
            parent_goal: None,
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
        };

        manager.add_goal(goal).await.unwrap();
        assert_eq!(manager.queue_size().await, 1);

        let next = manager.get_next_goal().await;
        assert!(next.is_some());
        assert_eq!(manager.queue_size().await, 0);
    }
}

