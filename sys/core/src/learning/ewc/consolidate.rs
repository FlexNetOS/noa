//! Importance-Weighted Parameter Consolidation
//!
//! T667: Implement importance-weighted parameter consolidation
//! US2: Apply EWC penalty to preserve important weights

use crate::error::Result;
use crate::learning::ewc::FisherComputer;
use serde::{Deserialize, Serialize};

/// EWC configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ewcconfigs {
    pub lambda: f64, // EWC penalty weight
    pub fisher_decay: f64, // Decay factor for Fisher Information
}

impl Default for Ewcconfigs {
    fn default() -> Self {
        Self {
            lambda: 0.4,
            fisher_decay: 0.95,
        }
    }
}

/// EWC trainer
pub struct EwcTrainer {
    fisher_computer: FisherComputer,
    configs: Ewcconfigs,
    old_parameters: std::collections::HashMap<String, f64>,
}

impl EwcTrainer {
    /// Create a new EWC trainer
    pub fn new(configs: Ewcconfigs) -> Self {
        Self {
            fisher_computer: FisherComputer::new(),
            configs,
            old_parameters: std::collections::HashMap::new(),
        }
    }

    /// Compute EWC penalty for loss
    pub fn compute_ewc_penalty(&self, current_parameters: &std::collections::HashMap<String, f64>) -> f64 {
        let mut penalty = 0.0;

        for (param_name, &current_value) in current_parameters {
            if let Some(&old_value) = self.old_parameters.get(param_name) {
                if let Some(fisher) = self.fisher_computer.get_fisher(param_name) {
                    let diff = current_value - old_value;
                    penalty += fisher * diff * diff;
                }
            }
        }

        self.configs.lambda * penalty
    }

    /// Update old parameters after training
    pub fn update_old_parameters(&mut self, parameters: std::collections::HashMap<String, f64>) {
        self.old_parameters = parameters;
    }

    /// Update Fisher Information
    pub fn update_fisher(&mut self, parameter_name: String, gradient: f64) -> Result<()> {
        self.fisher_computer.compute_fisher(parameter_name, gradient)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ewc_penalty() {
        let trainer = EwcTrainer::new(Ewcconfigs::default());
        let mut params = std::collections::HashMap::new();
        params.insert("param1".to_string(), 1.0);

        let penalty = trainer.compute_ewc_penalty(&params);
        assert!(penalty >= 0.0);
    }
}

