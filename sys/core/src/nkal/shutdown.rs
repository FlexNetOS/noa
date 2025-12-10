use crate::error::{NoaError, Result, ValidationError};

/// Tracks whether services were drained before switching kernel modes.
#[derive(Debug, Clone, Default)]
pub struct ShutdownState {
    pub inflight_tasks: usize,
    pub drains_completed: bool,
    pub message: Option<String>,
}

pub struct ShutdownGuard;

impl ShutdownGuard {
    /// Enforce that a graceful shutdown has completed before proceeding.
    pub fn require_graceful(state: &ShutdownState) -> Result<()> {
        if !state.drains_completed || state.inflight_tasks > 0 {
            return Err(NoaError::Validation(ValidationError::new(
                "shutdown",
                "Kernel mode switch blocked: services not drained",
                "NKAL_SHUTDOWN_REQUIRED",
            )));
        }
        Ok(())
    }

    /// Helper to build a shutdown state snapshot from inflight task counts.
    pub fn from_inflight(inflight_tasks: usize, message: Option<String>) -> ShutdownState {
        ShutdownState {
            inflight_tasks,
            drains_completed: inflight_tasks == 0,
            message,
        }
    }
}
