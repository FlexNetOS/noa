use crate::cecca::{CeccaCell, CeccaContext, CeccaDecision};

/// CC_STEM_DIFF: Computes stem delta for change risk sizing.
pub struct StemDifferentiatorCell;

impl StemDifferentiatorCell {
    pub fn new() -> Self {
        Self
    }

    fn diff_ratio(&self, baseline: &str, candidate: &str) -> f64 {
        if baseline.is_empty() && candidate.is_empty() {
            return 0.0;
        }

        let max_len = baseline.len().max(candidate.len()) as f64;
        (baseline.len() as f64 - candidate.len() as f64).abs() / max_len
    }
}

impl Default for StemDifferentiatorCell {
    fn default() -> Self {
        Self::new()
    }
}

impl CeccaCell for StemDifferentiatorCell {
    fn evaluate(&self, ctx: &CeccaContext) -> CeccaDecision {
        let baseline = ctx
            .metadata
            .get("baseline")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let candidate = ctx
            .metadata
            .get("candidate")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let ratio = self.diff_ratio(baseline, candidate);
        let rationale = format!("Diff ratio {:.2}", ratio);

        if ratio <= 0.35 {
            CeccaDecision {
                approved: true,
                rationale,
                actions: vec!["apply_low_risk_patch".to_string()],
                score: 1.0 - ratio,
            }
        } else {
            CeccaDecision::rejected(format!("{} exceeds safe delta threshold", rationale))
        }
    }
}
