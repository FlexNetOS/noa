/// Evaluates value of an action using simple cost/benefit heuristics.
pub struct ValueEvaluator;

impl ValueEvaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, expected_benefit: f64, expected_cost: f64) -> f64 {
        if expected_cost <= 0.0 {
            return 1.0;
        }
        (expected_benefit - expected_cost) / expected_cost
    }
}

impl Default for ValueEvaluator {
    fn default() -> Self {
        Self::new()
    }
}
