use crate::cecca::{CeccaCell, CeccaContext, CeccaDecision};

/// CC_CHOP: Capsule Surgeon guards blast radius.
pub struct CapsuleSurgeonCell {
    max_files: usize,
}

impl CapsuleSurgeonCell {
    pub fn new(max_files: usize) -> Self {
        Self { max_files }
    }
}

impl Default for CapsuleSurgeonCell {
    fn default() -> Self {
        Self::new(10)
    }
}

impl CeccaCell for CapsuleSurgeonCell {
    fn evaluate(&self, ctx: &CeccaContext) -> CeccaDecision {
        let changed_files = ctx
            .metadata
            .get("changed_files")
            .and_then(|v| v.as_array())
            .map(|v| v.len())
            .unwrap_or(0);

        if changed_files > self.max_files {
            return CeccaDecision::rejected(format!(
                "Change touches {} files (limit {})",
                changed_files, self.max_files
            ));
        }

        CeccaDecision {
            approved: true,
            rationale: format!("Blast radius within limit: {} files", changed_files),
            actions: vec!["continue_review".to_string()],
            score: 1.0,
        }
    }
}
