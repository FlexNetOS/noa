use serde::{Deserialize, Serialize};

/// Truth source entry with optional weight.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruthSource {
    pub name: String,
    pub weight: f64,
    pub evidence: serde_json::Value,
}

/// Order of preferred truth sources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruthSourceOrder {
    pub priority: Vec<String>,
}

/// Truth source policy enforcing order and quorum.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruthSourcePolicy {
    pub order: TruthSourceOrder,
    pub minimum_sources: usize,
}

impl TruthSourcePolicy {
    pub fn new(priority: Vec<String>, minimum_sources: usize) -> Self {
        Self {
            order: TruthSourceOrder { priority },
            minimum_sources,
        }
    }

    pub fn evaluate(&self, sources: &[TruthSource]) -> (bool, f64) {
        if sources.len() < self.minimum_sources {
            return (false, 0.0);
        }

        let mut score = 0.0;
        for source in sources {
            let base = source.weight;
            let position_bonus = self
                .order
                .priority
                .iter()
                .position(|p| p == &source.name)
                .map(|idx| 1.0 / ((idx + 1) as f64))
                .unwrap_or(0.1);
            score += base * position_bonus;
        }

        (true, score)
    }
}

impl Default for TruthSourcePolicy {
    fn default() -> Self {
        Self::new(vec!["canonical".to_string(), "observed".to_string(), "cached".to_string()], 2)
    }
}
