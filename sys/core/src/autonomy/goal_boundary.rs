//! Constitutional Goal Boundary Checker
//!
//! Implements constitutional goal boundary checking for self-generated goals.
//! §3.10: Biblical Governance
//! FR-066-070: Autonomous Goal Generation
//!
//! T632: Implement constitutional goal boundary checker

use crate::error::{Result, NoaError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Goal boundary check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalBoundaryResult {
    pub allowed: bool,
    pub reason: String,
    pub violations: Vec<String>,
    pub warnings: Vec<String>,
}

/// Goal boundary checker
pub struct GoalBoundaryChecker {
    enabled: Arc<RwLock<bool>>,
    rules: Arc<RwLock<Vec<String>>>,
}

impl GoalBoundaryChecker {
    /// Create a new goal boundary checker
    pub fn new() -> Self {
        Self {
            enabled: Arc::new(RwLock::new(true)),
            rules: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Check if a goal is within constitutional boundaries
    pub async fn check_goal(
        &self,
        title: &str,
        description: &str,
        rationale: &str,
    ) -> Result<GoalBoundaryResult> {
        let enabled = *self.enabled.read().await;
        if !enabled {
            return Ok(GoalBoundaryResult {
                allowed: true,
                reason: "Goal boundary checking is disabled".to_string(),
                violations: vec![],
                warnings: vec![],
            });
        }

        let mut violations = Vec::new();
        let mut warnings = Vec::new();
        let mut allowed = true;

        // Check for harmful keywords
        let harmful_keywords = vec!["harm", "violate", "destroy", "delete critical", "remove essential"];
        let text = format!("{} {} {}", title, description, rationale).to_lowercase();

        for keyword in harmful_keywords {
            if text.contains(keyword) {
                violations.push(format!("Goal contains harmful keyword: '{}'", keyword));
                allowed = false;
            }
        }

        // Check for risky operations
        let risky_keywords = vec!["modify core", "change system", "update critical"];
        for keyword in risky_keywords {
            if text.contains(keyword) {
                warnings.push(format!("Goal involves risky operation: '{}'", keyword));
            }
        }

        // Check rationale quality
        if rationale.len() < 20 {
            warnings.push("Goal rationale is too short (minimum 20 characters)".to_string());
        }

        let reason = if allowed && violations.is_empty() {
            if warnings.is_empty() {
                "Goal approved".to_string()
            } else {
                format!("Goal approved with {} warning(s)", warnings.len())
            }
        } else {
            format!("Goal rejected: {} violation(s)", violations.len())
        };

        Ok(GoalBoundaryResult {
            allowed,
            reason,
            violations,
            warnings,
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

    /// Add a custom rule
    pub async fn add_rule(&self, rule: String) {
        let mut rules = self.rules.write().await;
        rules.push(rule);
    }

    /// Get all rules
    pub async fn rules(&self) -> Vec<String> {
        self.rules.read().await.clone()
    }

    /// Clear rules
    pub async fn clear_rules(&self) {
        let mut rules = self.rules.write().await;
        rules.clear();
    }
}

impl Default for GoalBoundaryChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_harmful_keyword_detection() {
        let checker = GoalBoundaryChecker::new();

        let result = checker.check_goal(
            "Harm user data",
            "Delete all user data",
            "Remove sensitive information",
        ).await.unwrap();

        assert!(!result.allowed);
        assert!(!result.violations.is_empty());
    }

    #[tokio::test]
    async fn test_risky_operation_warning() {
        let checker = GoalBoundaryChecker::new();

        let result = checker.check_goal(
            "Modify core system",
            "Update critical components",
            "Improve system performance",
        ).await.unwrap();

        assert!(result.allowed); // Should allow but warn
        assert!(!result.warnings.is_empty());
    }

    #[tokio::test]
    async fn test_short_rationale_warning() {
        let checker = GoalBoundaryChecker::new();

        let result = checker.check_goal(
            "Valid goal",
            "Description",
            "Short", // Less than 20 chars
        ).await.unwrap();

        assert!(result.allowed);
        assert!(!result.warnings.is_empty());
    }

    #[tokio::test]
    async fn test_valid_goal() {
        let checker = GoalBoundaryChecker::new();

        let result = checker.check_goal(
            "Optimize database queries",
            "Improve query performance by adding indexes",
            "Query latency has increased by 50% over the past week, affecting user experience",
        ).await.unwrap();

        assert!(result.allowed);
        assert!(result.violations.is_empty());
        assert!(result.warnings.is_empty());
    }
}

