use chrono::{DateTime, Utc};
use std::sync::Mutex;

/// Audit entry for improvement lifecycle.
#[derive(Debug, Clone)]
pub struct ImprovementAuditEntry {
    pub event: String,
    pub details: String,
    pub recorded_at: DateTime<Utc>,
}

/// Captures improvement audit trail.
pub struct ImprovementAuditLog {
    entries: Mutex<Vec<ImprovementAuditEntry>>,
}

impl ImprovementAuditLog {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(Vec::new()),
        }
    }

    pub fn record(&self, event: impl Into<String>, details: impl Into<String>) {
        if let Ok(mut entries) = self.entries.lock() {
            entries.push(ImprovementAuditEntry {
                event: event.into(),
                details: details.into(),
                recorded_at: Utc::now(),
            });
        }
    }

    pub fn entries(&self) -> Vec<ImprovementAuditEntry> {
        self.entries.lock().map(|e| e.clone()).unwrap_or_default()
    }
}

impl Default for ImprovementAuditLog {
    fn default() -> Self {
        Self::new()
    }
}
