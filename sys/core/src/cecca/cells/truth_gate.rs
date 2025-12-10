use crate::cecca::{CeccaCell, CeccaContext, CeccaDecision};

/// CC_TRUTH: Enforces evidence order and minimum corroboration.
pub struct TruthGateCell {
    minimum_sources: usize,
    preferred_order: Vec<String>,
}

impl TruthGateCell {
    pub fn new(minimum_sources: usize, preferred_order: Vec<String>) -> Self {
        Self {
            minimum_sources,
            preferred_order,
        }
    }
}

impl Default for TruthGateCell {
    fn default() -> Self {
        Self::new(
            2,
            vec![
                "primary".to_string(),
                "secondary".to_string(),
                "tertiary".to_string(),
            ],
        )
    }
}

impl CeccaCell for TruthGateCell {
    fn evaluate(&self, ctx: &CeccaContext) -> CeccaDecision {
        let sources: Vec<String> = ctx
            .metadata
            .get("sources")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|s| s.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        if sources.len() < self.minimum_sources {
            return CeccaDecision::rejected(format!(
                "Insufficient truth sources: {} < {}",
                sources.len(),
                self.minimum_sources
            ));
        }

        let mut score = 1.0;
        for (idx, preferred) in self.preferred_order.iter().enumerate() {
            if let Some(actual_idx) = sources.iter().position(|s| s == preferred) {
                // Earlier matches raise the score slightly.
                score += (self.preferred_order.len().saturating_sub(idx) as f64)
                    / ((actual_idx + 1) as f64 * 10.0);
            }
        }

        CeccaDecision {
            approved: true,
            rationale: format!("{} corroborating sources", sources.len()),
            actions: vec!["continue_pipeline".to_string()],
            score,
        }
    }
}
