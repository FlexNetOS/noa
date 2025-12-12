use crate::error::Result;
use crate::governance::engine::{DecisionVerdict, GovernanceOutcome};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Reward event captured for auditability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardEvent {
    pub agent_id: String,
    pub delta: f32,
    pub reason: String,
    pub verdict: DecisionVerdict,
    pub recorded_at: DateTime<Utc>,
}

/// Agent compliance reward system.
pub struct RewardSystem {
    scores: Arc<RwLock<HashMap<String, f32>>>,
    events: Arc<RwLock<Vec<RewardEvent>>>,
    max_history: usize,
}

impl RewardSystem {
    pub fn new() -> Self {
        Self {
            scores: Arc::new(RwLock::new(HashMap::new())),
            events: Arc::new(RwLock::new(Vec::new())),
            max_history: 500,
        }
    }

    /// Apply rewards or penalties based on a governance outcome.
    pub async fn apply_outcome(&self, outcome: &GovernanceOutcome) -> Result<f32> {
        let base = match outcome.final_verdict {
            DecisionVerdict::Allow => 0.5,
            DecisionVerdict::Escalate => -0.1,
            DecisionVerdict::RequireRollback => -0.5,
            DecisionVerdict::Deny => -1.0,
        };

        let delta = base + outcome.reward_delta;
        self.reward_agent(
            &outcome.agent_id,
            delta,
            format!("governance verdict {:?}", outcome.final_verdict).as_str(),
            outcome.final_verdict,
        )
        .await
    }

    /// Reward or penalize an agent manually.
    pub async fn reward_agent(
        &self,
        agent_id: &str,
        delta: f32,
        reason: &str,
        verdict: DecisionVerdict,
    ) -> Result<f32> {
        {
            let mut scores = self.scores.write().await;
            let entry = scores.entry(agent_id.to_string()).or_insert(0.0);
            *entry += delta;
            *entry = entry.clamp(-10.0, 10.0);
        }

        let event = RewardEvent {
            agent_id: agent_id.to_string(),
            delta,
            reason: reason.to_string(),
            verdict,
            recorded_at: Utc::now(),
        };
        {
            let mut history = self.events.write().await;
            history.push(event.clone());
            if history.len() > self.max_history {
                let overflow = history.len() - self.max_history;
                history.drain(0..overflow);
            }
        }

        info!(
            target: "governance::rewards",
            agent = agent_id,
            delta = delta,
            verdict = ?verdict,
            reason = reason,
            "Updated compliance reward score"
        );

        Ok(self.score(agent_id).await)
    }

    /// Get the current score for an agent.
    pub async fn score(&self, agent_id: &str) -> f32 {
        self.scores.read().await.get(agent_id).copied().unwrap_or(0.0)
    }

    /// Return recent reward events, optionally filtered by agent.
    pub async fn history(&self, agent_id: Option<&str>, limit: usize) -> Vec<RewardEvent> {
        let history = self.events.read().await;
        let filtered: Vec<RewardEvent> = history
            .iter()
            .rev()
            .filter(|e| agent_id.map(|id| id == e.agent_id).unwrap_or(true))
            .take(limit)
            .cloned()
            .collect();
        filtered
    }
}

impl Default for RewardSystem {
    fn default() -> Self {
        Self::new()
    }
}
