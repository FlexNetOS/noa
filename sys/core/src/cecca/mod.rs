//! CECCA (Constitutional Executive Capsule Control Assembly)
//!
//! Specialized cells enforce governance, truth, licensing, and promotion policy
//! during self-improvement (US8).

pub mod cells;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Common CECCA input envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CeccaContext {
    pub subject: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub submitted_at: DateTime<Utc>,
}

impl CeccaContext {
    pub fn new(
        subject: impl Into<String>,
        content: impl Into<String>,
        metadata: serde_json::Value,
    ) -> Self {
        Self {
            subject: subject.into(),
            content: content.into(),
            metadata,
            submitted_at: Utc::now(),
        }
    }
}

/// Decision issued by a CECCA cell.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CeccaDecision {
    pub approved: bool,
    pub rationale: String,
    pub actions: Vec<String>,
    pub score: f64,
}

impl CeccaDecision {
    pub fn approved(rationale: impl Into<String>) -> Self {
        Self {
            approved: true,
            rationale: rationale.into(),
            actions: Vec::new(),
            score: 1.0,
        }
    }

    pub fn rejected(rationale: impl Into<String>) -> Self {
        Self {
            approved: false,
            rationale: rationale.into(),
            actions: Vec::new(),
            score: 0.0,
        }
    }
}

/// Cell interface implemented by each specialization.
pub trait CeccaCell {
    fn evaluate(&self, ctx: &CeccaContext) -> CeccaDecision;
}
