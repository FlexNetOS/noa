//! Circuit Breaker for Agent Timeout Handling
//!
//! VER049: Verify agent timeout triggers circuit breaker [Edge Case]
//! FR-008: Failure recovery with timeout handling

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitState {
    /// Circuit is closed - normal operation
    Closed,
    /// Circuit is open - failing, reject requests
    Open,
    /// Circuit is half-open - testing if service recovered
    HalfOpen,
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening circuit
    pub failure_threshold: usize,
    /// Timeout before attempting half-open
    pub recovery_timeout: Duration,
    /// Number of successes needed in half-open to close
    pub success_threshold: usize,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
            success_threshold: 2,
        }
    }
}

/// Circuit breaker state for an agent
#[derive(Debug, Clone)]
struct AgentCircuitState {
    state: CircuitState,
    failure_count: usize,
    success_count: usize,
    last_state_change: Instant,
}

impl Default for AgentCircuitState {
    fn default() -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            success_count: 0,
            last_state_change: Instant::now(),
        }
    }
}

/// Circuit breaker for managing agent timeouts and failures
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    states: Arc<Mutex<HashMap<String, AgentCircuitState>>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            states: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Check if circuit is open for an agent
    pub fn is_open(&self, agent_id: &str) -> bool {
        let mut states = self.states.lock().unwrap();
        let state = states.entry(agent_id.to_string()).or_insert_with(Default::default);

        match state.state {
            CircuitState::Closed => false,
            CircuitState::Open => {
                // Check if recovery timeout has passed
                if state.last_state_change.elapsed() >= self.config.recovery_timeout {
                    // Transition to half-open
                    state.state = CircuitState::HalfOpen;
                    state.success_count = 0;
                    state.last_state_change = Instant::now();
                    false
                } else {
                    true
                }
            }
            CircuitState::HalfOpen => false, // Allow requests in half-open
        }
    }

    /// Record a success for an agent
    pub fn record_success(&self, agent_id: &str) {
        let mut states = self.states.lock().unwrap();
        let state = states.entry(agent_id.to_string()).or_insert_with(Default::default);

        match state.state {
            CircuitState::Closed => {
                // Reset failure count on success
                state.failure_count = 0;
            }
            CircuitState::HalfOpen => {
                state.success_count += 1;
                if state.success_count >= self.config.success_threshold {
                    // Close the circuit
                    state.state = CircuitState::Closed;
                    state.failure_count = 0;
                    state.success_count = 0;
                    state.last_state_change = Instant::now();
                }
            }
            CircuitState::Open => {
                // Should not happen, but handle gracefully
            }
        }
    }

    /// Record a failure for an agent
    pub fn record_failure(&self, agent_id: &str) {
        let mut states = self.states.lock().unwrap();
        let state = states.entry(agent_id.to_string()).or_insert_with(Default::default);

        match state.state {
            CircuitState::Closed | CircuitState::HalfOpen => {
                state.failure_count += 1;
                if state.failure_count >= self.config.failure_threshold {
                    // Open the circuit
                    state.state = CircuitState::Open;
                    state.last_state_change = Instant::now();
                }
            }
            CircuitState::Open => {
                // Already open, update timestamp
                state.last_state_change = Instant::now();
            }
        }
    }

    /// Get current state for an agent
    pub fn get_state(&self, agent_id: &str) -> CircuitState {
        let states = self.states.lock().unwrap();
        states
            .get(agent_id)
            .map(|s| s.state)
            .unwrap_or(CircuitState::Closed)
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new(CircuitBreakerConfig::default())
    }
}

