//! Constitutional governance engine (FR-025)
//!
//! Evaluates agent decisions against registered governance rules, records
//! every verdict with rationale, and emits audit-ready outcomes.

use crate::governance::audit::{AuditEvent, GovernanceAuditTrail};
use crate::error::{NoaError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

/// Decision submitted by an agent or subsystem for governance review.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceDecision {
    pub id: Uuid,
    pub agent_id: String,
    pub action: String,
    pub rationale: String,
    pub context: serde_json::Value,
    pub principles: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl GovernanceDecision {
    pub fn new(
        agent_id: impl Into<String>,
        action: impl Into<String>,
        rationale: impl Into<String>,
        context: serde_json::Value,
        principles: Vec<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            agent_id: agent_id.into(),
            action: action.into(),
            rationale: rationale.into(),
            context,
            principles,
            created_at: Utc::now(),
        }
    }
}

/// Final verdict for a decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionVerdict {
    Allow,
    Deny,
    Escalate,
    RequireRollback,
}

/// Outcome returned by a specific governance rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleVerdict {
    pub rule: String,
    pub verdict: DecisionVerdict,
    pub rationale: String,
    pub principles: Vec<String>,
    pub reward_delta: f32,
    pub requires_snapshot: bool,
    pub evidence: serde_json::Value,
}

impl RuleVerdict {
    pub fn allow(rule: impl Into<String>, rationale: impl Into<String>) -> Self {
        Self {
            rule: rule.into(),
            verdict: DecisionVerdict::Allow,
            rationale: rationale.into(),
            principles: Vec::new(),
            reward_delta: 0.0,
            requires_snapshot: false,
            evidence: serde_json::Value::Null,
        }
    }
}

/// Aggregated governance outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceOutcome {
    pub decision_id: Uuid,
    pub agent_id: String,
    pub final_verdict: DecisionVerdict,
    pub rule_verdicts: Vec<RuleVerdict>,
    pub reward_delta: f32,
    pub recorded_at: DateTime<Utc>,
    pub escalation_target: Option<String>,
    pub snapshot_hint: Option<String>,
}

#[async_trait::async_trait]
pub trait GovernanceRule: Send + Sync {
    fn name(&self) -> &'static str;
    async fn evaluate(&self, decision: &GovernanceDecision) -> Result<RuleVerdict>;
}

/// Core governance engine that orchestrates rule evaluation and auditing.
pub struct GovernanceEngine {
    audit: GovernanceAuditTrail,
    rules: Arc<RwLock<Vec<Arc<dyn GovernanceRule>>>>,
}

impl GovernanceEngine {
    /// Create a new governance engine with the provided audit trail.
    pub fn new(audit: GovernanceAuditTrail) -> Self {
        Self {
            audit,
            rules: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register a governance rule that will be evaluated for every decision.
    pub async fn register_rule(&self, rule: Arc<dyn GovernanceRule>) {
        let mut guard = self.rules.write().await;
        guard.push(rule);
    }

    /// Evaluate and record a decision.
    pub async fn enforce(&self, decision: GovernanceDecision) -> Result<GovernanceOutcome> {
        let outcome = self.evaluate(&decision).await?;

        let event = AuditEvent {
            decision: decision.clone(),
            outcome: outcome.clone(),
            recorded_at: Utc::now(),
        };

        self.audit.record_event(event).await?;
        Ok(outcome)
    }

    async fn evaluate(&self, decision: &GovernanceDecision) -> Result<GovernanceOutcome> {
        if decision.rationale.trim().is_empty() {
            return Err(NoaError::Validation(crate::error::ValidationError::new(
                "rationale",
                "Rationale is required for governance review",
                "GOV_MISSING_RATIONALE",
            )));
        }

        let rules = self.rules.read().await.clone();
        let mut rule_verdicts = Vec::new();
        let mut final_verdict = DecisionVerdict::Allow;
        let mut reward_delta = 0.0;
        let mut snapshot_hint = None;
        let mut escalation_target = None;

        for rule in rules {
            let verdict = rule.evaluate(decision).await?;
            reward_delta += verdict.reward_delta;

            match verdict.verdict {
                DecisionVerdict::Deny => {
                    final_verdict = DecisionVerdict::Deny;
                    escalation_target.get_or_insert("safety_officer".to_string());
                }
                DecisionVerdict::RequireRollback => {
                    final_verdict = DecisionVerdict::RequireRollback;
                    snapshot_hint.get_or_insert("latest".to_string());
                }
                DecisionVerdict::Escalate => {
                    if matches!(final_verdict, DecisionVerdict::Allow) {
                        final_verdict = DecisionVerdict::Escalate;
                    }
                    escalation_target.get_or_insert("human_review".to_string());
                }
                DecisionVerdict::Allow => {}
            }

            rule_verdicts.push(verdict);
        }

        if rule_verdicts.is_empty() {
            warn!(
                agent = decision.agent_id,
                action = decision.action,
                "No governance rules registered; defaulting to escalate"
            );
            final_verdict = DecisionVerdict::Escalate;
            escalation_target.get_or_insert("governance_missing_rules".to_string());
        }

        info!(
            agent = decision.agent_id,
            action = decision.action,
            verdict = ?final_verdict,
            rules = rule_verdicts.len(),
            "Governance evaluation complete"
        );

        Ok(GovernanceOutcome {
            decision_id: decision.id,
            agent_id: decision.agent_id.clone(),
            final_verdict,
            rule_verdicts,
            reward_delta,
            recorded_at: Utc::now(),
            escalation_target,
            snapshot_hint,
        })
    }
}
