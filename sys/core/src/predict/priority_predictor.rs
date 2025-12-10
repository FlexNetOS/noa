/// Priority prediction output.
#[derive(Debug, Clone)]
pub struct PriorityPrediction {
    pub item: String,
    pub score: f64,
}

/// Predicts task priority using value and risk.
pub struct PriorityPredictor {
    weight_value: f64,
    weight_risk: f64,
}

impl PriorityPredictor {
    pub fn new(weight_value: f64, weight_risk: f64) -> Self {
        Self {
            weight_value,
            weight_risk,
        }
    }

    pub fn prioritize(&self, candidates: &[(String, f64, f64)]) -> Vec<PriorityPrediction> {
        let mut predictions: Vec<PriorityPrediction> = candidates
            .iter()
            .map(|(item, value_score, risk_score)| PriorityPrediction {
                item: item.clone(),
                score: (value_score * self.weight_value) - (risk_score * self.weight_risk),
            })
            .collect();

        predictions
            .sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        predictions
    }
}

impl Default for PriorityPredictor {
    fn default() -> Self {
        Self::new(0.7, 0.3)
    }
}
