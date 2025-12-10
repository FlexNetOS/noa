use crate::cecca::{CeccaCell, CeccaContext, CeccaDecision};

/// CC_CONST: Ensures constitutional principles are declared and respected.
pub struct ConstitutionCell {
    required_principles: Vec<String>,
}

impl ConstitutionCell {
    pub fn new(required_principles: Vec<String>) -> Self {
        Self { required_principles }
    }
}

impl Default for ConstitutionCell {
    fn default() -> Self {
        Self::new(vec![
            "3.1".to_string(),
            "3.5".to_string(),
            "3.12".to_string(),
        ])
    }
}

impl CeccaCell for ConstitutionCell {
    fn evaluate(&self, ctx: &CeccaContext) -> CeccaDecision {
        let provided: Vec<String> = ctx
            .metadata
            .get("principles")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|s| s.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let missing: Vec<String> = self
            .required_principles
            .iter()
            .filter(|p| !provided.contains(p))
            .cloned()
            .collect();

        if missing.is_empty() {
            let mut decision = CeccaDecision::approved("All required principles present");
            decision.actions.push("record_constitutional_ack".to_string());
            decision.score = 1.0;
            decision
        } else {
            CeccaDecision::rejected(format!(
                "Missing constitutional principles: {}",
                missing.join(", ")
            ))
        }
    }
}
