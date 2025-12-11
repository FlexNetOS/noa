/// Single verification evidence item.
#[derive(Debug, Clone)]
pub struct VerificationEvidence {
    pub source: String,
    pub verdict: bool,
    pub notes: String,
}

/// Triple-verify rule: three independent confirmations.
pub struct TripleVerifyRule;

impl TripleVerifyRule {
    pub fn new() -> Self {
        Self
    }

    pub fn verify(&self, evidences: &[VerificationEvidence]) -> (bool, usize) {
        let approvals = evidences.iter().filter(|e| e.verdict).count();
        (approvals >= 3, approvals)
    }
}

impl Default for TripleVerifyRule {
    fn default() -> Self {
        Self::new()
    }
}
