use crate::error::Result;
use crate::governance::drift_detection::{DriftSignal, DriftStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Individual correction action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionAction {
    pub description: String,
    pub severity: DriftStatus,
    pub rollback_required: bool,
    pub owner: String,
}

/// Correction plan derived from drift signals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionPlan {
    pub created_at: DateTime<Utc>,
    pub status: DriftStatus,
    pub actions: Vec<CorrectionAction>,
    pub signals: Vec<DriftSignal>,
    pub approved: bool,
}

/// Compliance correction engine.
pub struct CorrectionEngine {
    plans: Arc<RwLock<Vec<CorrectionPlan>>>,
}

impl CorrectionEngine {
    pub fn new() -> Self {
        Self {
            plans: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn plan(&self, status: DriftStatus, signals: Vec<DriftSignal>) -> CorrectionPlan {
        let actions = match status {
            DriftStatus::Healthy => Vec::new(),
            DriftStatus::Warning => vec![CorrectionAction {
                description: "Increase sampling and schedule human review of flagged agents"
                    .to_string(),
                severity: status,
                rollback_required: false,
                owner: "governance".to_string(),
            }],
            DriftStatus::Critical => vec![
                CorrectionAction {
                    description: "Freeze risky automation until governance review completes"
                        .to_string(),
                    severity: status,
                    rollback_required: true,
                    owner: "governance".to_string(),
                },
                CorrectionAction {
                    description: "Trigger rollback to last known good snapshot".to_string(),
                    severity: status,
                    rollback_required: true,
                    owner: "sre".to_string(),
                },
            ],
        };

        CorrectionPlan {
            created_at: Utc::now(),
            status,
            actions,
            signals,
            approved: matches!(status, DriftStatus::Healthy),
        }
    }

    pub async fn apply(&self, plan: CorrectionPlan) -> Result<()> {
        {
            let mut plans = self.plans.write().await;
            plans.push(plan.clone());
        }

        info!(
            target: "governance::correction",
            status = ?plan.status,
            actions = plan.actions.len(),
            "Correction plan recorded"
        );
        Ok(())
    }

    pub async fn recent(&self, limit: usize) -> Vec<CorrectionPlan> {
        let plans = self.plans.read().await;
        plans.iter().rev().take(limit).cloned().collect()
    }
}

impl Default for CorrectionEngine {
    fn default() -> Self {
        Self::new()
    }
}
