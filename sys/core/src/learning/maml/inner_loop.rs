//! Inner-Loop Task Adaptation
//!
//! T670: Implement inner-loop task adaptation
//! US2: Fast adaptation to new tasks

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Inner-loop adaptation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerLoopConfig {
    pub learning_rate: f64,
    pub adaptation_steps: usize,
    pub gradient_clip: Option<f64>,
}

impl Default for InnerLoopConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.01,
            adaptation_steps: 5,
            gradient_clip: Some(1.0),
        }
    }
}

/// Inner-loop adapter
pub struct InnerLoopAdapter {
    config: InnerLoopConfig,
}

impl InnerLoopAdapter {
    /// Create a new inner-loop adapter
    pub fn new(config: InnerLoopConfig) -> Self {
        Self { config }
    }

    /// Adapt model to task
    pub async fn adapt(
        &self,
        initial_params: &std::collections::HashMap<String, f64>,
        task_data: &serde_json::Value,
    ) -> Result<std::collections::HashMap<String, f64>> {
        // TODO: Implement actual gradient-based adaptation
        // For now, return initial parameters
        Ok(initial_params.clone())
    }

    /// Perform adaptation step
    pub fn adaptation_step(
        &self,
        params: &mut std::collections::HashMap<String, f64>,
        gradient: &std::collections::HashMap<String, f64>,
    ) {
        for (param_name, param_value) in params.iter_mut() {
            if let Some(&grad) = gradient.get(param_name) {
                let clipped_grad =
                    self.config.gradient_clip.map(|clip| grad.min(clip).max(-clip)).unwrap_or(grad);
                *param_value -= self.config.learning_rate * clipped_grad;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptation_step() {
        let adapter = InnerLoopAdapter::new(InnerLoopConfig::default());
        let mut params = std::collections::HashMap::new();
        params.insert("param1".to_string(), 1.0);

        let mut gradient = std::collections::HashMap::new();
        gradient.insert("param1".to_string(), 0.1);

        adapter.adaptation_step(&mut params, &gradient);
        assert!(params.get("param1").unwrap() < &1.0);
    }
}
