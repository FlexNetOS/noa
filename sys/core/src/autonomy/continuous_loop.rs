//! Always-On Continuous Loop
//!
//! T621: Implement always-on continuous loop
//! FR-051: System MUST run an always-on continuous loop for autonomous operation
//! §3.4: Adaptive & Self-Improving

use crate::error::{NoaError, Result};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Continuous loop state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopState {
    Stopped,
    Starting,
    Running,
    Stopping,
}

/// Always-on continuous loop
pub struct ContinuousLoop {
    state: Arc<RwLock<LoopState>>,
    cycle_interval_secs: u64,
}

impl ContinuousLoop {
    /// Create a new continuous loop
    pub fn new(cycle_interval_secs: u64) -> Self {
        Self {
            state: Arc::new(RwLock::new(LoopState::Stopped)),
            cycle_interval_secs,
        }
    }

    /// Start the continuous loop
    pub async fn start(&self) -> Result<()> {
        let mut state = self.state.write().await;
        if *state != LoopState::Stopped {
            return Err(NoaError::Internal {
                message: "Continuous loop already running".to_string(),
                source: None,
            });
        }
        *state = LoopState::Starting;
        drop(state);

        info!("Starting always-on continuous loop");

        let state = Arc::clone(&self.state);
        let cycle_interval = self.cycle_interval_secs;

        tokio::spawn(async move {
            {
                let mut s = state.write().await;
                *s = LoopState::Running;
            }

            loop {
                // Check if still running
                {
                    let s = state.read().await;
                    if *s != LoopState::Running {
                        break;
                    }
                }

                // Execute one cycle
                match Self::execute_cycle().await {
                    Ok(_) => {
                        debug!("Continuous loop cycle completed");
                    }
                    Err(e) => {
                        warn!("Continuous loop cycle failed: {}", e);
                    }
                }

                // Sleep until next cycle
                tokio::time::sleep(tokio::time::Duration::from_secs(cycle_interval)).await;
            }

            {
                let mut s = state.write().await;
                *s = LoopState::Stopped;
            }
        });

        Ok(())
    }

    /// Stop the continuous loop
    pub async fn stop(&self) -> Result<()> {
        let mut state = self.state.write().await;
        *state = LoopState::Stopping;
        info!("Stopping continuous loop");
        Ok(())
    }

    /// Get current loop state
    pub async fn get_state(&self) -> LoopState {
        *self.state.read().await
    }

    /// Execute one cycle of the continuous loop
    async fn execute_cycle() -> Result<()> {
        // TODO: Implement cycle logic
        // 1. Process goal queue
        // 2. Monitor resource usage
        // 3. Optimize resource allocation
        // 4. Self-monitor performance
        // 5. Generate new goals if needed

        debug!("Executing continuous loop cycle");
        Ok(())
    }
}

impl Default for ContinuousLoop {
    fn default() -> Self {
        Self::new(10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_continuous_loop_creation() {
        let loop_ = ContinuousLoop::new(5);
        assert_eq!(loop_.get_state().await, LoopState::Stopped);
    }

    #[test]
    fn test_loop_state() {
        assert_ne!(LoopState::Running, LoopState::Stopped);
    }
}
