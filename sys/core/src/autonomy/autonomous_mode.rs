//! Autonomous Execution Mode
//!
//! Implements autonomous execution mode for NOA system.
//! §3.4: Adaptive & Self-Improving
//! FR-061-065: Full Autonomy Operation
//!
//! T626: Implement autonomous execution mode

use crate::error::{NoaError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Autonomous execution mode state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutonomousState {
    /// Disabled - manual operation only
    Disabled,
    /// Enabled - autonomous execution active
    Enabled,
    /// Paused - temporarily suspended
    Paused,
    /// Emergency stop - immediate halt
    EmergencyStop,
}

/// Autonomous execution mode configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousConfig {
    /// Maximum concurrent autonomous goals
    pub max_concurrent_goals: usize,
    /// Timeout for goal execution (seconds)
    pub goal_timeout_seconds: u64,
    /// Enable self-generated goals
    pub allow_self_generated: bool,
    /// Enable co-improvement goals
    pub allow_co_improvement: bool,
    /// Safety check interval (seconds)
    pub safety_check_interval: u64,
    /// Maximum resource usage percentage
    pub max_resource_usage: f64,
}

impl Default for AutonomousConfig {
    fn default() -> Self {
        Self {
            max_concurrent_goals: 5,
            goal_timeout_seconds: 3600, // 1 hour
            allow_self_generated: true,
            allow_co_improvement: true,
            safety_check_interval: 60, // 1 minute
            max_resource_usage: 0.8,   // 80%
        }
    }
}

/// Autonomous execution mode manager
pub struct AutonomousMode {
    state: Arc<RwLock<AutonomousState>>,
    config: Arc<RwLock<AutonomousConfig>>,
    active_goals: Arc<RwLock<Vec<Uuid>>>,
    last_safety_check: Arc<RwLock<Option<DateTime<Utc>>>>,
}

impl AutonomousMode {
    /// Create a new autonomous mode manager
    pub fn new(config: AutonomousConfig) -> Self {
        Self {
            state: Arc::new(RwLock::new(AutonomousState::Disabled)),
            config: Arc::new(RwLock::new(config)),
            active_goals: Arc::new(RwLock::new(Vec::new())),
            last_safety_check: Arc::new(RwLock::new(None)),
        }
    }

    /// Enable autonomous execution mode
    pub async fn enable(&self) -> Result<()> {
        let mut state = self.state.write().await;
        match *state {
            AutonomousState::EmergencyStop => {
                return Err(NoaError::Internal {
                    message: "Cannot enable autonomous mode from emergency stop state".to_string(),
                    source: None,
                });
            }
            _ => {
                *state = AutonomousState::Enabled;
            }
        }
        Ok(())
    }

    /// Disable autonomous execution mode
    pub async fn disable(&self) -> Result<()> {
        let mut state = self.state.write().await;
        *state = AutonomousState::Disabled;
        Ok(())
    }

    /// Pause autonomous execution
    pub async fn pause(&self) -> Result<()> {
        let mut state = self.state.write().await;
        if *state == AutonomousState::Enabled {
            *state = AutonomousState::Paused;
        }
        Ok(())
    }

    /// Resume autonomous execution
    pub async fn resume(&self) -> Result<()> {
        let mut state = self.state.write().await;
        if *state == AutonomousState::Paused {
            *state = AutonomousState::Enabled;
        }
        Ok(())
    }

    /// Emergency stop - immediately halt all autonomous operations
    pub async fn emergency_stop(&self) -> Result<()> {
        let mut state = self.state.write().await;
        *state = AutonomousState::EmergencyStop;

        // Clear active goals
        let mut goals = self.active_goals.write().await;
        goals.clear();

        Ok(())
    }

    /// Get current state
    pub async fn state(&self) -> AutonomousState {
        *self.state.read().await
    }

    /// Check if autonomous mode is active
    pub async fn is_active(&self) -> bool {
        matches!(*self.state.read().await, AutonomousState::Enabled)
    }

    /// Register an active goal
    pub async fn register_goal(&self, goal_id: Uuid) -> Result<()> {
        let state = self.state.read().await;
        if !matches!(*state, AutonomousState::Enabled) {
            return Err(NoaError::Internal {
                message: "Autonomous mode is not enabled".to_string(),
                source: None,
            });
        }

        let config = self.config.read().await;
        let mut goals = self.active_goals.write().await;

        if goals.len() >= config.max_concurrent_goals {
            return Err(NoaError::Internal {
                message: format!(
                    "Maximum concurrent goals ({}) reached",
                    config.max_concurrent_goals
                ),
                source: None,
            });
        }

        goals.push(goal_id);
        Ok(())
    }

    /// Unregister a goal
    pub async fn unregister_goal(&self, goal_id: Uuid) {
        let mut goals = self.active_goals.write().await;
        goals.retain(|&id| id != goal_id);
    }

    /// Get active goals count
    pub async fn active_goals_count(&self) -> usize {
        self.active_goals.read().await.len()
    }

    /// Update configuration
    pub async fn update_config(&self, config: AutonomousConfig) {
        *self.config.write().await = config;
    }

    /// Get current configuration
    pub async fn config(&self) -> AutonomousConfig {
        self.config.read().await.clone()
    }

    /// Perform safety check
    pub async fn safety_check(&self) -> Result<bool> {
        let mut last_check = self.last_safety_check.write().await;
        *last_check = Some(Utc::now());

        let state = self.state.read().await;
        if matches!(*state, AutonomousState::EmergencyStop) {
            return Ok(false);
        }

        // Check resource usage
        let config = self.config.read().await;
        let goals = self.active_goals.read().await;

        // Simple check: if we're at max concurrent goals, consider it a warning
        if goals.len() >= config.max_concurrent_goals {
            return Ok(false);
        }

        Ok(true)
    }

    /// Get last safety check time
    pub async fn last_safety_check(&self) -> Option<DateTime<Utc>> {
        *self.last_safety_check.read().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_autonomous_mode_lifecycle() {
        let mode = AutonomousMode::new(AutonomousConfig::default());

        assert!(!mode.is_active().await);
        assert_eq!(mode.state().await, AutonomousState::Disabled);

        mode.enable().await.unwrap();
        assert!(mode.is_active().await);
        assert_eq!(mode.state().await, AutonomousState::Enabled);

        mode.pause().await.unwrap();
        assert_eq!(mode.state().await, AutonomousState::Paused);

        mode.resume().await.unwrap();
        assert_eq!(mode.state().await, AutonomousState::Enabled);

        mode.disable().await.unwrap();
        assert!(!mode.is_active().await);
    }

    #[tokio::test]
    async fn test_emergency_stop() {
        let mode = AutonomousMode::new(AutonomousConfig::default());

        mode.enable().await.unwrap();
        mode.emergency_stop().await.unwrap();

        assert_eq!(mode.state().await, AutonomousState::EmergencyStop);
        assert_eq!(mode.active_goals_count().await, 0);

        // Cannot enable from emergency stop
        assert!(mode.enable().await.is_err());
    }

    #[tokio::test]
    async fn test_goal_registration() {
        let mode = AutonomousMode::new(AutonomousConfig {
            max_concurrent_goals: 2,
            ..Default::default()
        });

        mode.enable().await.unwrap();

        let goal1 = Uuid::new_v4();
        let goal2 = Uuid::new_v4();
        let goal3 = Uuid::new_v4();

        mode.register_goal(goal1).await.unwrap();
        mode.register_goal(goal2).await.unwrap();

        // Should fail when at max
        assert!(mode.register_goal(goal3).await.is_err());

        mode.unregister_goal(goal1).await;
        assert_eq!(mode.active_goals_count().await, 1);

        // Now should succeed
        mode.register_goal(goal3).await.unwrap();
        assert_eq!(mode.active_goals_count().await, 2);
    }
}
