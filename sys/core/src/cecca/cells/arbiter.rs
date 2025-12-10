use crate::cecca::{CeccaCell, CeccaContext, CeccaDecision};

/// CC_ARBITER: Promotion arbiter balancing risk and readiness.
pub struct PromotionArbiterCell {
    max_risk: f64,
}

impl PromotionArbiterCell {
    pub fn new(max_risk: f64) -> Self {
        Self { max_risk }
    }
}

impl Default for PromotionArbiterCell {
    fn default() -> Self {
        Self::new(0.35)
    }
}

impl CeccaCell for PromotionArbiterCell {
    fn evaluate(&self, ctx: &CeccaContext) -> CeccaDecision {
        let risk = ctx
            .metadata
            .get("risk_score")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let tests_passed = ctx
            .metadata
            .get("tests_passed")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        if risk > self.max_risk {
            return CeccaDecision::rejected(format!(
                "Risk {:.2} above threshold {:.2}",
                risk, self.max_risk
            ));
        }

        if !tests_passed {
            return CeccaDecision::rejected("Required verification tests not complete");
        }

        CeccaDecision {
            approved: true,
            rationale: format!("Risk {:.2} within bounds with tests passed", risk),
            actions: vec!["promote_candidate".to_string()],
            score: 1.0 - risk,
        }
    }
}
