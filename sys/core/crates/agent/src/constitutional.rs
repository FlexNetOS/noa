//! Constitutional Principles Enforcement
//!
//! VER045: Verify constitutional principles enforced on all agents [FR-011]
//! FR-011: System MUST enforce constitutional principles on all agents

use serde::{Deserialize, Serialize};

/// Constitutional principles that must be enforced
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstitutionalPrinciple {
    /// §3.1: Self-Contained - All paths must resolve under noa_root
    SelfContained,
    /// §3.2: Local-First - Operations must work offline
    LocalFirst,
    /// §3.3: Security - No unauthorized access
    Security,
    /// §3.4: Adaptive - System must be self-improving
    Adaptive,
    /// §3.5: Auditable - All actions must be logged
    Auditable,
}

/// Result of constitutional check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalCheckResult {
    pub passed: bool,
    pub violated_principles: Vec<ConstitutionalPrinciple>,
    pub violations: Vec<String>,
}

/// Constitutional enforcer for agents
pub struct ConstitutionalEnforcer;

impl ConstitutionalEnforcer {
    /// Check if an operation violates constitutional principles
    pub fn check_operation(
        &self,
        operation: &AgentOperation,
    ) -> ConstitutionalCheckResult {
        let mut violated_principles = Vec::new();
        let mut violations = Vec::new();

        // Check self-contained principle (§3.1)
        if let Some(path) = &operation.file_path {
            if !path.starts_with(&operation.noa_root) {
                violated_principles.push(ConstitutionalPrinciple::SelfContained);
                violations.push(format!(
                    "Path {} is outside noa_root {}",
                    path, operation.noa_root
                ));
            }
        }

        // Check local-first principle (§3.2)
        if operation.requires_network && !operation.allows_offline {
            violated_principles.push(ConstitutionalPrinciple::LocalFirst);
            violations.push("Operation requires network but offline mode not supported".to_string());
        }

        // Check security principle (§3.3)
        if operation.requires_privileges {
            violated_principles.push(ConstitutionalPrinciple::Security);
            violations.push("Operation requires elevated privileges".to_string());
        }

        // Check auditable principle (§3.5)
        if !operation.is_logged {
            violated_principles.push(ConstitutionalPrinciple::Auditable);
            violations.push("Operation is not logged".to_string());
        }

        ConstitutionalCheckResult {
            passed: violated_principles.is_empty(),
            violated_principles,
            violations,
        }
    }

    /// Enforce constitutional principles (returns error if violated)
    pub fn enforce(
        &self,
        operation: &AgentOperation,
    ) -> Result<(), ConstitutionalViolationError> {
        let check = self.check_operation(operation);
        if !check.passed {
            return Err(ConstitutionalViolationError {
                violated_principles: check.violated_principles,
                violations: check.violations,
            });
        }
        Ok(())
    }
}

/// Agent operation to check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOperation {
    pub agent_id: String,
    pub operation_type: String,
    pub noa_root: String,
    pub file_path: Option<String>,
    pub requires_network: bool,
    pub allows_offline: bool,
    pub requires_privileges: bool,
    pub is_logged: bool,
}

/// Error when constitutional principles are violated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalViolationError {
    pub violated_principles: Vec<ConstitutionalPrinciple>,
    pub violations: Vec<String>,
}

impl std::fmt::Display for ConstitutionalViolationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Constitutional violations: {}",
            self.violations.join("; ")
        )
    }
}

impl std::error::Error for ConstitutionalViolationError {}

impl Default for ConstitutionalEnforcer {
    fn default() -> Self {
        Self
    }
}

