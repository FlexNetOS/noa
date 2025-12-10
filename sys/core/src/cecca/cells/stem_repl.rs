use crate::cecca::{CeccaCell, CeccaContext, CeccaDecision};

/// CC_STEM_REPL: Replicates capsule stem for redundancy.
pub struct StemReplicatorCell {
    replicas: usize,
}

impl StemReplicatorCell {
    pub fn new(replicas: usize) -> Self {
        Self { replicas }
    }
}

impl Default for StemReplicatorCell {
    fn default() -> Self {
        Self::new(2)
    }
}

impl CeccaCell for StemReplicatorCell {
    fn evaluate(&self, ctx: &CeccaContext) -> CeccaDecision {
        let requested = ctx
            .metadata
            .get("replicas")
            .and_then(|v| v.as_u64())
            .unwrap_or(self.replicas as u64) as usize;

        CeccaDecision {
            approved: true,
            rationale: format!("Replicating stem {} time(s)", requested),
            actions: (0..requested)
                .map(|i| format!("replicate_stem_copy_{}", i + 1))
                .collect(),
            score: 1.0,
        }
    }
}
