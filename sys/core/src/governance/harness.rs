use crate::error::Result;
use crate::governance::biblical::ethics::EthicsGuard;
use crate::governance::{GovernanceAuditTrail, GovernanceDecision, GovernanceEngine, RewardSystem};
use std::path::PathBuf;
use std::sync::Arc;

/// Lightweight harness to exercise governance evaluation end-to-end in tests.
pub struct GovernanceHarness {
    engine: GovernanceEngine,
    rewards: RewardSystem,
}

impl GovernanceHarness {
    /// Create a new harness with default ethics guard and a writable audit path.
    pub fn new(audit_path: PathBuf) -> Self {
        let audit = GovernanceAuditTrail::new(audit_path);
        let engine = GovernanceEngine::new(audit);
        Self {
            engine,
            rewards: RewardSystem::new(),
        }
    }

    /// Register default governance rules (currently only EthicsGuard).
    pub async fn init_defaults(&self) {
        let guard = Arc::new(EthicsGuard::default_guard());
        self.engine.register_rule(guard).await;
    }

    /// Evaluate a decision and apply rewards; returns (outcome, new score).
    pub async fn evaluate(&self, decision: GovernanceDecision) -> Result<(crate::governance::GovernanceOutcome, f32)> {
        let outcome = self.engine.enforce(decision).await?;
        let score = self.rewards.apply_outcome(&outcome).await?;
        Ok((outcome, score))
    }

    /// Get the current score for an agent.
    pub async fn score(&self, agent_id: &str) -> f32 {
        self.rewards.score(agent_id).await
    }
}
