//! Orchestration engine (Phase 9 - T282)
use crate::error::Result;
use crate::orchestration::decomposer;
use crate::orchestration::scheduler::Scheduler;

pub struct OrchestrationEngine {
    scheduler: Scheduler,
}

impl OrchestrationEngine {
    pub fn new() -> Self {
        Self {
            scheduler: Scheduler::new(),
        }
    }

    pub fn submit_goal(&mut self, goal: &str) -> Result<()> {
        for task in decomposer::decompose(goal)? {
            self.scheduler.enqueue(0, task);
        }
        Ok(())
    }

    pub fn next_task(&mut self) -> Option<String> {
        self.scheduler.next()
    }
}
