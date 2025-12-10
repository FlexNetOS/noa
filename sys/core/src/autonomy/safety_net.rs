//! 3-Plane Rollback Safety Net
//!
//! Implements 3-plane rollback safety net for autonomous operations.
//! FR-061-065: Full Autonomy Operation
//! FR-028: Self-Modification Rollback
//!
//! T628: Implement 3-plane rollback safety net

use crate::error::{NoaError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Plane type in 3-plane architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaneType {
    /// Sandbox - testing/staging
    Sandbox,
    /// Deployed - production
    Deployed,
    /// Coordinator - long-term memory (constant)
    Coordinator,
}

/// Rollback trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RollbackTrigger {
    /// Test failure
    TestFailure,
    /// SLO violation
    SloViolation,
    /// Safety event
    SafetyEvent,
    /// Manual trigger
    Manual,
    /// Emergency
    Emergency,
}

/// Rollback scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RollbackScope {
    /// Single modification
    Single { modification_id: Uuid },
    /// Batch of modifications
    Batch { modification_ids: Vec<Uuid> },
    /// Checkpoint-based
    Checkpoint { checkpoint_id: Uuid },
    /// Time-based
    Time { before: DateTime<Utc> },
}

/// Rollback operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackOperation {
    pub id: Uuid,
    pub trigger: RollbackTrigger,
    pub scope: RollbackScope,
    pub source_plane: PlaneType,
    pub target_plane: PlaneType,
    pub created_at: DateTime<Utc>,
    pub status: RollbackStatus,
    pub error: Option<String>,
}

/// Rollback status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RollbackStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Safety net manager
pub struct SafetyNet {
    rollback_history: Arc<RwLock<Vec<RollbackOperation>>>,
    active_rollbacks: Arc<RwLock<Vec<Uuid>>>,
}

impl SafetyNet {
    /// Create a new safety net
    pub fn new() -> Self {
        Self {
            rollback_history: Arc::new(RwLock::new(Vec::new())),
            active_rollbacks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Initiate a rollback
    pub async fn initiate_rollback(
        &self,
        trigger: RollbackTrigger,
        scope: RollbackScope,
        source_plane: PlaneType,
        target_plane: PlaneType,
    ) -> Result<Uuid> {
        // Validate plane transition
        if source_plane == PlaneType::Coordinator {
            return Err(NoaError::Internal {
                message: "Cannot rollback from coordinator plane".to_string(),
                source: None,
            });
        }

        if target_plane == PlaneType::Coordinator {
            return Err(NoaError::Internal {
                message: "Cannot rollback to coordinator plane".to_string(),
                source: None,
            });
        }

        let operation = RollbackOperation {
            id: Uuid::new_v4(),
            trigger,
            scope,
            source_plane,
            target_plane,
            created_at: Utc::now(),
            status: RollbackStatus::Pending,
            error: None,
        };

        let mut history = self.rollback_history.write().await;
        history.push(operation.clone());

        let mut active = self.active_rollbacks.write().await;
        active.push(operation.id);

        Ok(operation.id)
    }

    /// Execute rollback
    pub async fn execute_rollback(&self, operation_id: Uuid) -> Result<()> {
        let mut history = self.rollback_history.write().await;
        let operation = history.iter_mut().find(|op| op.id == operation_id).ok_or_else(|| {
            NoaError::NotFound {
                resource: "RollbackOperation".to_string(),
                id: operation_id.to_string(),
            }
        })?;

        if operation.status != RollbackStatus::Pending {
            return Err(NoaError::Internal {
                message: format!("Rollback {} is not in pending state", operation_id),
                source: None,
            });
        }

        operation.status = RollbackStatus::InProgress;

        // TODO: Implement actual rollback logic
        // This would involve:
        // 1. Creating snapshot of current state
        // 2. Restoring from previous checkpoint
        // 3. Validating restored state
        // 4. Updating plane status

        operation.status = RollbackStatus::Completed;

        let mut active = self.active_rollbacks.write().await;
        active.retain(|&id| id != operation_id);

        Ok(())
    }

    /// Mark rollback as failed
    pub async fn mark_rollback_failed(&self, operation_id: Uuid, error: String) -> Result<()> {
        let mut history = self.rollback_history.write().await;
        let operation = history.iter_mut().find(|op| op.id == operation_id).ok_or_else(|| {
            NoaError::NotFound {
                resource: "RollbackOperation".to_string(),
                id: operation_id.to_string(),
            }
        })?;

        operation.status = RollbackStatus::Failed;
        operation.error = Some(error);

        let mut active = self.active_rollbacks.write().await;
        active.retain(|&id| id != operation_id);

        Ok(())
    }

    /// Get rollback history
    pub async fn history(&self) -> Vec<RollbackOperation> {
        self.rollback_history.read().await.clone()
    }

    /// Get active rollbacks
    pub async fn active_rollbacks(&self) -> Vec<Uuid> {
        self.active_rollbacks.read().await.clone()
    }

    /// Check if rollback is in progress
    pub async fn is_rollback_active(&self, operation_id: Uuid) -> bool {
        self.active_rollbacks.read().await.contains(&operation_id)
    }

    /// Get rollback operation by ID
    pub async fn get_operation(&self, operation_id: Uuid) -> Option<RollbackOperation> {
        self.rollback_history
            .read()
            .await
            .iter()
            .find(|op| op.id == operation_id)
            .cloned()
    }
}

impl Default for SafetyNet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initiate_rollback() {
        let safety_net = SafetyNet::new();

        let operation_id = safety_net
            .initiate_rollback(
                RollbackTrigger::TestFailure,
                RollbackScope::Single {
                    modification_id: Uuid::new_v4(),
                },
                PlaneType::Deployed,
                PlaneType::Sandbox,
            )
            .await
            .unwrap();

        assert!(safety_net.is_rollback_active(operation_id).await);
        assert_eq!(safety_net.active_rollbacks().await.len(), 1);
    }

    #[tokio::test]
    async fn test_coordinator_plane_restriction() {
        let safety_net = SafetyNet::new();

        // Cannot rollback from coordinator
        assert!(safety_net
            .initiate_rollback(
                RollbackTrigger::Emergency,
                RollbackScope::Checkpoint {
                    checkpoint_id: Uuid::new_v4(),
                },
                PlaneType::Coordinator,
                PlaneType::Sandbox,
            )
            .await
            .is_err());

        // Cannot rollback to coordinator
        assert!(safety_net
            .initiate_rollback(
                RollbackTrigger::Emergency,
                RollbackScope::Checkpoint {
                    checkpoint_id: Uuid::new_v4(),
                },
                PlaneType::Deployed,
                PlaneType::Coordinator,
            )
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_execute_rollback() {
        let safety_net = SafetyNet::new();

        let operation_id = safety_net
            .initiate_rollback(
                RollbackTrigger::SloViolation,
                RollbackScope::Batch {
                    modification_ids: vec![Uuid::new_v4()],
                },
                PlaneType::Deployed,
                PlaneType::Sandbox,
            )
            .await
            .unwrap();

        safety_net.execute_rollback(operation_id).await.unwrap();

        let operation = safety_net.get_operation(operation_id).await.unwrap();
        assert_eq!(operation.status, RollbackStatus::Completed);
        assert!(!safety_net.is_rollback_active(operation_id).await);
    }
}
