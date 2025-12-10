//! Constitutional Decision Boundary
//!
//! Implements constitutional decision boundary checking for autonomous operations.
//! §3.10: Biblical Governance
//! FR-061-065: Full Autonomy Operation
//!
//! T630: Implement constitutional decision boundary

use crate::error::{NoaError, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Decision type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionType {
    /// Goal creation
    GoalCreation,
    /// Goal execution
    GoalExecution,
    /// Code modification
    CodeModification,
    /// Resource allocation
    ResourceAllocation,
    /// Agent action
    AgentAction,
}

/// Decision context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionContext {
    pub decision_type: DecisionType,
    pub component: String,
    pub action: String,
    pub parameters: serde_json::Value,
    pub metadata: serde_json::Value,
}

/// Boundary check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundaryCheckResult {
    pub allowed: bool,
    pub reason: String,
    pub constraints: Vec<String>,
}

/// Constitutional boundary checker
pub struct ConstitutionalBoundary {
    enabled: Arc<RwLock<bool>>,
    constraints: Arc<RwLock<Vec<String>>>,
}

impl ConstitutionalBoundary {
    /// Create a new constitutional boundary checker
    pub fn new() -> Self {
        Self {
            enabled: Arc::new(RwLock::new(true)),
            constraints: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Check if a decision is within constitutional boundaries
    pub async fn check_decision(&self, context: &DecisionContext) -> Result<BoundaryCheckResult> {
        let enabled = *self.enabled.read().await;
        if !enabled {
            return Ok(BoundaryCheckResult {
                allowed: true,
                reason: "Constitutional boundary checking is disabled".to_string(),
                constraints: vec![],
            });
        }

        let mut constraints = Vec::new();
        let mut allowed = true;
        let mut reason = String::new();

        // Check based on decision type
        match &context.decision_type {
            DecisionType::GoalCreation => {
                // Check if goal violates constitutional principles
                if let Some(action) = context.parameters.get("action") {
                    if let Some(action_str) = action.as_str() {
                        if action_str.contains("harm") || action_str.contains("violate") {
                            allowed = false;
                            reason = "Goal violates constitutional principle: no harm".to_string();
                            constraints.push("No actions that cause harm".to_string());
                        }
                    }
                }
            }
            DecisionType::CodeModification => {
                // Check if modification is safe
                if let Some(scope) = context.parameters.get("scope") {
                    if let Some(scope_str) = scope.as_str() {
                        if scope_str == "critical_system" {
                            constraints
                                .push("Critical system modifications require approval".to_string());
                        }
                    }
                }
            }
            DecisionType::ResourceAllocation => {
                // Check resource limits
                if let Some(percentage) = context.parameters.get("percentage") {
                    if let Some(pct) = percentage.as_f64() {
                        if pct > 0.9 {
                            allowed = false;
                            reason = "Resource allocation exceeds safe threshold (90%)".to_string();
                            constraints.push("Maximum 90% resource usage".to_string());
                        }
                    }
                }
            }
            _ => {
                // Default: allow with constraints
                reason = "Decision within boundaries".to_string();
            }
        }

        if allowed && constraints.is_empty() {
            reason = "Decision approved".to_string();
        }

        Ok(BoundaryCheckResult {
            allowed,
            reason,
            constraints,
        })
    }

    /// Enable boundary checking
    pub async fn enable(&self) {
        *self.enabled.write().await = true;
    }

    /// Disable boundary checking
    pub async fn disable(&self) {
        *self.enabled.write().await = false;
    }

    /// Check if boundary checking is enabled
    pub async fn is_enabled(&self) -> bool {
        *self.enabled.read().await
    }

    /// Add a constraint
    pub async fn add_constraint(&self, constraint: String) {
        let mut constraints = self.constraints.write().await;
        constraints.push(constraint);
    }

    /// Get all constraints
    pub async fn constraints(&self) -> Vec<String> {
        self.constraints.read().await.clone()
    }

    /// Clear constraints
    pub async fn clear_constraints(&self) {
        let mut constraints = self.constraints.write().await;
        constraints.clear();
    }
}

impl Default for ConstitutionalBoundary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_goal_creation_check() {
        let boundary = ConstitutionalBoundary::new();

        let context = DecisionContext {
            decision_type: DecisionType::GoalCreation,
            component: "GoalGenerator".to_string(),
            action: "create_goal".to_string(),
            parameters: serde_json::json!({
                "action": "harm user"
            }),
            metadata: serde_json::json!({}),
        };

        let result = boundary.check_decision(&context).await.unwrap();
        assert!(!result.allowed);
        assert!(result.reason.contains("violates"));
    }

    #[tokio::test]
    async fn test_resource_allocation_check() {
        let boundary = ConstitutionalBoundary::new();

        let context = DecisionContext {
            decision_type: DecisionType::ResourceAllocation,
            component: "ResourceManager".to_string(),
            action: "allocate".to_string(),
            parameters: serde_json::json!({
                "percentage": 0.95
            }),
            metadata: serde_json::json!({}),
        };

        let result = boundary.check_decision(&context).await.unwrap();
        assert!(!result.allowed);
        assert!(result.reason.contains("exceeds"));
    }

    #[tokio::test]
    async fn test_enable_disable() {
        let boundary = ConstitutionalBoundary::new();

        assert!(boundary.is_enabled().await);

        boundary.disable().await;
        assert!(!boundary.is_enabled().await);

        let context = DecisionContext {
            decision_type: DecisionType::GoalCreation,
            component: "Test".to_string(),
            action: "test".to_string(),
            parameters: serde_json::json!({}),
            metadata: serde_json::json!({}),
        };

        let result = boundary.check_decision(&context).await.unwrap();
        assert!(result.allowed); // Should allow when disabled
    }
}
