use crate::cecca::{CeccaCell, CeccaContext, CeccaDecision};

/// CC_CONST: Ensures constitutional principles are declared and respected.
pub struct ConstitutionCell {
    required_principles: Vec<String>,
}

impl ConstitutionCell {
    pub fn new(required_principles: Vec<String>) -> Self {
        Self {
            required_principles,
        }
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
            .map(|arr| arr.iter().filter_map(|s| s.as_str().map(|s| s.to_string())).collect())
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn ctx_with_principles(principles: Vec<&str>) -> CeccaContext {
        CeccaContext::new("subject", "content", json!({ "principles": principles }))
    }

    #[test]
    fn approves_when_all_principles_present() {
        let cell = ConstitutionCell::default();
        let ctx = ctx_with_principles(vec!["3.1", "3.5", "3.12"]);
        let decision = cell.evaluate(&ctx);
        assert!(decision.approved);
        assert!(decision.actions.contains(&"record_constitutional_ack".to_string()));
        assert_eq!(decision.score, 1.0);
    }

    #[test]
    fn rejects_when_principles_missing() {
        let cell = ConstitutionCell::default();
        let ctx = ctx_with_principles(vec!["3.1"]);
        let decision = cell.evaluate(&ctx);
        assert!(!decision.approved);
        assert!(decision.rationale.contains("Missing constitutional principles"));
    }
}
