//! Fix Validation
//!
//! T616: Implement fix validation
//! FR-074: System MUST validate that fixes resolve the issue
//! §3.4: Adaptive & Self-Improving

use crate::error::{NoaError, Result};
use crate::healing::anomaly::Anomaly;
use crate::healing::fix::FixResult;
use crate::healing::monitor::{ComponentHealth, ComponentHealthSnapshot};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info, warn};

/// Fix validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub success: bool,
    pub component_id: String,
    pub health_before: ComponentHealth,
    pub health_after: ComponentHealth,
    pub metric_improved: bool,
    pub validation_duration_ms: u64,
    pub message: String,
}

/// Fix validator
pub struct FixValidator {
    validation_timeout_secs: u64,
    health_check_interval_secs: u64,
}

impl FixValidator {
    /// Create a new fix validator
    pub fn new(validation_timeout_secs: u64, health_check_interval_secs: u64) -> Self {
        Self {
            validation_timeout_secs,
            health_check_interval_secs,
        }
    }

    /// Validate that fix resolved the issue
    pub async fn validate(
        &self,
        anomaly: &Anomaly,
        fix_result: &FixResult,
    ) -> Result<ValidationResult> {
        info!(
            component_id = %anomaly.component_id,
            fix_type = %fix_result.fix_type,
            "Validating fix"
        );

        let start = std::time::Instant::now();
        let health_before = anomaly.health_status;

        // Wait for component to stabilize after fix
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Check health multiple times to ensure stability
        let mut health_checks = Vec::new();
        let timeout = Duration::from_secs(self.validation_timeout_secs);
        let check_interval = Duration::from_secs(self.health_check_interval_secs);
        let mut elapsed = Duration::from_secs(0);

        while elapsed < timeout {
            // TODO: Get actual component health
            // For now, simulate health check
            let health_after = self.check_component_health(&anomaly.component_id).await?;
            health_checks.push(health_after);

            if health_checks.len() >= 3 {
                // Check if health is consistently improved
                let all_healthy = health_checks
                    .iter()
                    .all(|h| matches!(h, ComponentHealth::Healthy | ComponentHealth::Degraded));

                if all_healthy {
                    let validation_duration_ms = start.elapsed().as_millis() as u64;

                    return Ok(ValidationResult {
                        success: true,
                        component_id: anomaly.component_id.clone(),
                        health_before,
                        health_after: ComponentHealth::Healthy,
                        metric_improved: true,
                        validation_duration_ms,
                        message: format!(
                            "Fix validated: component {} is healthy",
                            anomaly.component_id
                        ),
                    });
                }
            }

            tokio::time::sleep(check_interval).await;
            elapsed += check_interval;
        }

        // Timeout or validation failed
        let health_after = health_checks.last().copied().unwrap_or(health_before);
        let validation_duration_ms = start.elapsed().as_millis() as u64;

        let success = matches!(
            health_after,
            ComponentHealth::Healthy | ComponentHealth::Degraded
        ) && health_after != ComponentHealth::Critical;

        Ok(ValidationResult {
            success,
            component_id: anomaly.component_id.clone(),
            health_before,
            health_after,
            metric_improved: success,
            validation_duration_ms,
            message: if success {
                format!("Fix validated: component {} improved", anomaly.component_id)
            } else {
                format!(
                    "Fix validation failed: component {} still unhealthy",
                    anomaly.component_id
                )
            },
        })
    }

    /// Check component health
    async fn check_component_health(&self, component_id: &str) -> Result<ComponentHealth> {
        // TODO: Implement actual health check
        // - Query health monitor
        // - Check service endpoints
        // - Verify metrics

        debug!(component_id = %component_id, "Checking component health");

        // Simulate health check
        Ok(ComponentHealth::Healthy)
    }
}

impl Default for FixValidator {
    fn default() -> Self {
        Self::new(30, 5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fix_validator_creation() {
        let validator = FixValidator::new(30, 5);
        assert_eq!(validator.validation_timeout_secs, 30);
        assert_eq!(validator.health_check_interval_secs, 5);
    }
}
