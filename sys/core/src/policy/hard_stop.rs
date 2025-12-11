/// Hard stop rule halts execution when risk crosses threshold.
#[derive(Debug, Clone)]
pub struct HardStopRule {
    pub max_risk: f64,
    pub require_human: bool,
}

impl HardStopRule {
    pub fn new(max_risk: f64, require_human: bool) -> Self {
        Self {
            max_risk,
            require_human,
        }
    }

    pub fn should_stop(&self, risk_score: f64, human_approved: bool, coverage: f64) -> bool {
        if risk_score > self.max_risk {
            return true;
        }
        if self.require_human && !human_approved {
            return true;
        }
        coverage < 0.5
    }
}

impl Default for HardStopRule {
    fn default() -> Self {
        Self::new(0.4, true)
    }
}
