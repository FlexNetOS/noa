//! Infinite Loop Detection for Agents
//!
//! VER050: Verify agent infinite loop detection and termination [Edge Case]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Loop detection configsuration
#[derive(Debug, Clone)]
pub struct LoopDetectionconfigs {
    /// Maximum number of identical operations before flagging
    pub max_repetitions: usize,
    /// Time window to check for repetitions
    pub time_window: Duration,
    /// Maximum execution time before flagging
    pub max_execution_time: Duration,
}

impl Default for LoopDetectionconfigs {
    fn default() -> Self {
        Self {
            max_repetitions: 10,
            time_window: Duration::from_secs(60),
            max_execution_time: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Operation signature for loop detection
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationSignature {
    pub agent_id: String,
    pub operation_type: String,
    pub parameters_hash: String, // Hash of parameters to detect identical operations
}

/// Operation history entry
#[derive(Debug, Clone)]
struct OperationEntry {
    signature: OperationSignature,
    timestamp: Instant,
}

/// Loop detector for agents
pub struct LoopDetector {
    configs: LoopDetectionconfigs,
    history: Arc<Mutex<Vec<OperationEntry>>>,
    agent_start_times: Arc<Mutex<HashMap<String, Instant>>>,
}

impl LoopDetector {
    /// Create a new loop detector
    pub fn new(configs: LoopDetectionconfigs) -> Self {
        Self {
            configs,
            history: Arc::new(Mutex::new(Vec::new())),
            agent_start_times: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Record an operation and check for loops
    pub fn record_operation(
        &self,
        agent_id: &str,
        operation_type: &str,
        parameters_hash: &str,
    ) -> Result<(), LoopDetectedError> {
        let signature = OperationSignature {
            agent_id: agent_id.to_string(),
            operation_type: operation_type.to_string(),
            parameters_hash: parameters_hash.to_string(),
        };

        let mut history = self.history.lock().unwrap();
        let now = Instant::now();

        // Clean old entries outside time window
        history.retain(|entry| now.duration_since(entry.timestamp) <= self.configs.time_window);

        // Add new entry
        history.push(OperationEntry {
            signature: signature.clone(),
            timestamp: now,
        });

        // Count repetitions of this signature
        let repetitions = history
            .iter()
            .filter(|entry| entry.signature == signature)
            .count();

        if repetitions > self.configs.max_repetitions {
            return Err(LoopDetectedError {
                agent_id: agent_id.to_string(),
                operation_type: operation_type.to_string(),
                repetitions,
                message: format!(
                    "Agent {} detected infinite loop: {} repeated {} times",
                    agent_id, operation_type, repetitions
                ),
            });
        }

        Ok(())
    }

    /// Start tracking execution time for an agent
    pub fn start_execution(&self, agent_id: &str) {
        let mut start_times = self.agent_start_times.lock().unwrap();
        start_times.insert(agent_id.to_string(), Instant::now());
    }

    /// Check if agent execution time exceeded limit
    pub fn check_execution_time(&self, agent_id: &str) -> Result<(), LoopDetectedError> {
        let start_times = self.agent_start_times.lock().unwrap();
        if let Some(start_time) = start_times.get(agent_id) {
            let elapsed = start_time.elapsed();
            if elapsed > self.configs.max_execution_time {
                return Err(LoopDetectedError {
                    agent_id: agent_id.to_string(),
                    operation_type: "execution".to_string(),
                    repetitions: 0,
                    message: format!(
                        "Agent {} execution time exceeded limit: {:?} > {:?}",
                        agent_id, elapsed, self.configs.max_execution_time
                    ),
                });
            }
        }
        Ok(())
    }

    /// Stop tracking execution time for an agent
    pub fn stop_execution(&self, agent_id: &str) {
        let mut start_times = self.agent_start_times.lock().unwrap();
        start_times.remove(agent_id);
    }

    /// Clear history for an agent
    pub fn clear_agent_history(&self, agent_id: &str) {
        let mut history = self.history.lock().unwrap();
        history.retain(|entry| entry.signature.agent_id != agent_id);
    }
}

impl Default for LoopDetector {
    fn default() -> Self {
        Self::new(LoopDetectionconfigs::default())
    }
}

/// Error when loop is detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopDetectedError {
    pub agent_id: String,
    pub operation_type: String,
    pub repetitions: usize,
    pub message: String,
}

impl std::fmt::Display for LoopDetectedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LoopDetectedError {}

