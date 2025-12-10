/// Risk assessment result.
#[derive(Debug, Clone)]
pub struct RiskAssessment {
    pub score: f64,
    pub factors: Vec<String>,
}

/// Computes risk score from likelihood/impact signals.
pub struct RiskAssessor {
    pub impact_weight: f64,
    pub likelihood_weight: f64,
    pub detectability_weight: f64,
}

impl RiskAssessor {
    pub fn new(impact_weight: f64, likelihood_weight: f64, detectability_weight: f64) -> Self {
        Self {
            impact_weight,
            likelihood_weight,
            detectability_weight,
        }
    }

    pub fn assess(&self, impact: f64, likelihood: f64, detectability: f64) -> RiskAssessment {
        let score = (impact * self.impact_weight)
            + (likelihood * self.likelihood_weight)
            + ((1.0 - detectability) * self.detectability_weight);

        let mut factors = Vec::new();
        if impact > 0.6 {
            factors.push("high_impact".to_string());
        }
        if likelihood > 0.5 {
            factors.push("likely".to_string());
        }
        if detectability < 0.4 {
            factors.push("hard_to_detect".to_string());
        }

        RiskAssessment { score, factors }
    }
}

impl Default for RiskAssessor {
    fn default() -> Self {
        Self::new(0.5, 0.35, 0.15)
    }
}
