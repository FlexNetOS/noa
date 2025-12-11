use crate::cecca::{CeccaCell, CeccaContext, CeccaDecision};

/// CC_QUORUM: Virtual Board quorum check.
pub struct QuorumCell {
    quorum_size: usize,
    approvals: Vec<String>,
}

impl QuorumCell {
    pub fn new(quorum_size: usize) -> Self {
        Self {
            quorum_size,
            approvals: Vec::new(),
        }
    }

    pub fn record_vote(&mut self, member: &str, approved: bool) {
        if approved && !self.approvals.contains(&member.to_string()) {
            self.approvals.push(member.to_string());
        }
    }
}

impl Default for QuorumCell {
    fn default() -> Self {
        Self::new(3)
    }
}

impl CeccaCell for QuorumCell {
    fn evaluate(&self, ctx: &CeccaContext) -> CeccaDecision {
        let mut total = self.approvals.clone();
        if let Some(extra) = ctx.metadata.get("votes").and_then(|v| v.as_array()) {
            for vote in extra {
                if let Some(member) = vote.get("member").and_then(|m| m.as_str()) {
                    if vote.get("approved").and_then(|a| a.as_bool()).unwrap_or(false) {
                        if !total.contains(&member.to_string()) {
                            total.push(member.to_string());
                        }
                    }
                }
            }
        }

        if total.len() >= self.quorum_size {
            let mut decision = CeccaDecision::approved(format!(
                "Quorum reached: {}/{} approvals",
                total.len(),
                self.quorum_size
            ));
            decision.actions.push("proceed_promotion".to_string());
            decision.score = (total.len() as f64) / (self.quorum_size as f64);
            decision
        } else {
            CeccaDecision::rejected(format!(
                "Insufficient approvals: {}/{}",
                total.len(),
                self.quorum_size
            ))
        }
    }
}
