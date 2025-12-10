use chrono::{DateTime, Utc};

use crate::autonomy::ampk::{AMPKAction, AMPKMode, ResourceSnapshot};

/// Sense input captures current signals.
#[derive(Debug, Clone)]
pub struct SenseInput {
    pub observations: serde_json::Value,
    pub resources: Option<ResourceSnapshot>,
}

/// Decision produced by the loop.
#[derive(Debug, Clone)]
pub struct DecideOutcome {
    pub actions: Vec<String>,
    pub state_delta: serde_json::Value,
}

/// SENSE → DECIDE → UPDATE loop orchestrator.
pub struct AutonomyLoop {
    pub last_cycle: Option<DateTime<Utc>>,
    ampk: AMPKMode,
}

impl AutonomyLoop {
    pub fn new(ampk: AMPKMode) -> Self {
        Self {
            last_cycle: None,
            ampk,
        }
    }

    pub fn run_cycle(&mut self, input: SenseInput) -> DecideOutcome {
        let mut actions = Vec::new();

        // SENSE: pull observations (placeholder stores)
        let resources = input.resources;

        // DECIDE: evaluate scarcity and choose control action
        if let Some(snapshot) = resources {
            match self.ampk.evaluate(&snapshot) {
                AMPKAction::Quiesce => actions.push("enter_quiesce".to_string()),
                AMPKAction::Throttle => actions.push("throttle_tasks".to_string()),
                AMPKAction::Resume => actions.push("resume_normal".to_string()),
            }
        }

        // UPDATE: record cycle completion
        self.last_cycle = Some(Utc::now());

        DecideOutcome {
            actions,
            state_delta: input.observations,
        }
    }
}

impl Default for AutonomyLoop {
    fn default() -> Self {
        Self::new(AMPKMode::default())
    }
}
