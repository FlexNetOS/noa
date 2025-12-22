//! Fisher Information Computation
//!
//! T666: Implement Fisher Information computation
//! US2: Compute Fisher Information for weight importance

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Fisher Information for a parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FisherInfo {
    pub parameter_name: String,
    pub fisher_value: f64,
    pub importance: f64,
}

/// Fisher Information computer
pub struct FisherComputer {
    fisher_info: HashMap<String, f64>,
}

impl FisherComputer {
    /// Create a new Fisher computer
    pub fn new() -> Self {
        Self {
            fisher_info: HashMap::new(),
        }
    }

    /// Compute Fisher Information for parameters
    pub fn compute_fisher(&mut self, parameter_name: String, gradient: f64) -> Result<f64> {
        // Fisher Information = gradient^2
        let fisher = gradient * gradient;
        self.fisher_info.insert(parameter_name.clone(), fisher);
        Ok(fisher)
    }

    /// Get Fisher Information for a parameter
    pub fn get_fisher(&self, parameter_name: &str) -> Option<f64> {
        self.fisher_info.get(parameter_name).copied()
    }

    /// Get importance score (normalized Fisher)
    pub fn get_importance(&self, parameter_name: &str) -> f64 {
        if let Some(fisher) = self.get_fisher(parameter_name) {
            // Normalize by max Fisher value
            let max_fisher = self.fisher_info.values().fold(0.0_f64, |a, &b| a.max(b));
            if max_fisher > 0.0 {
                fisher / max_fisher
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Get all Fisher Information
    pub fn get_all_fisher(&self) -> Vec<FisherInfo> {
        let max_fisher = self.fisher_info.values().fold(0.0_f64, |a, &b| a.max(b));

        self.fisher_info.iter()
            .map(|(name, &fisher)| FisherInfo {
                parameter_name: name.clone(),
                fisher_value: fisher,
                importance: if max_fisher > 0.0 { fisher / max_fisher } else { 0.0 },
            })
            .collect()
    }
}

impl Default for FisherComputer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fisher_computation() {
        let mut computer = FisherComputer::new();
        computer.compute_fisher("param1".to_string(), 0.5).unwrap();
        assert!(computer.get_fisher("param1").unwrap() > 0.0);
    }
}

