use crate::cecca::{CeccaCell, CeccaContext, CeccaDecision};

/// CC_LICENSE: License gate to avoid incompatible dependencies.
pub struct LicenseGateCell {
    allowed: Vec<String>,
}

impl LicenseGateCell {
    pub fn new(allowed: Vec<String>) -> Self {
        Self { allowed }
    }
}

impl Default for LicenseGateCell {
    fn default() -> Self {
        Self::new(vec![
            "MIT".to_string(),
            "Apache-2.0".to_string(),
            "BSD-3-Clause".to_string(),
        ])
    }
}

impl CeccaCell for LicenseGateCell {
    fn evaluate(&self, ctx: &CeccaContext) -> CeccaDecision {
        let license = ctx
            .metadata
            .get("license")
            .and_then(|v| v.as_str())
            .unwrap_or("UNKNOWN");

        if self.allowed.iter().any(|l| l == license) {
            CeccaDecision {
                approved: true,
                rationale: format!("License {} permitted", license),
                actions: vec!["record_license".to_string()],
                score: 1.0,
            }
        } else {
            CeccaDecision::rejected(format!("License {} not permitted", license))
        }
    }
}
