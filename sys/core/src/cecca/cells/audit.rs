use chrono::{DateTime, Utc};
use std::sync::Mutex;

use crate::cecca::{CeccaCell, CeccaContext, CeccaDecision};

#[derive(Debug, Clone)]
pub struct AuditRecord {
    pub subject: String,
    pub approved: bool,
    pub rationale: String,
    pub recorded_at: DateTime<Utc>,
}

/// CC_AUDIT: Global auditor captures CECCA decisions for traceability.
pub struct AuditCell {
    records: Mutex<Vec<AuditRecord>>,
}

impl AuditCell {
    pub fn new() -> Self {
        Self {
            records: Mutex::new(Vec::new()),
        }
    }

    pub fn records(&self) -> Vec<AuditRecord> {
        self.records.lock().unwrap().clone()
    }
}

impl Default for AuditCell {
    fn default() -> Self {
        Self::new()
    }
}

impl CeccaCell for AuditCell {
    fn evaluate(&self, ctx: &CeccaContext) -> CeccaDecision {
        let decision = CeccaDecision::approved("Audit trail recorded");
        let record = AuditRecord {
            subject: ctx.subject.clone(),
            approved: decision.approved,
            rationale: decision.rationale.clone(),
            recorded_at: Utc::now(),
        };
        if let Ok(mut records) = self.records.lock() {
            records.push(record);
        }
        decision
    }
}
