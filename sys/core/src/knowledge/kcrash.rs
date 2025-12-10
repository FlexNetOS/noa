use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// KCRASH_CAP: Crash forensics store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    pub id: String,
    pub component: String,
    pub reason: String,
    pub captured_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

#[derive(Default, Debug)]
pub struct CrashForensics {
    reports: Vec<CrashReport>,
}

impl CrashForensics {
    pub fn new() -> Self {
        Self { reports: Vec::new() }
    }

    pub fn record(&mut self, component: &str, reason: &str, metadata: serde_json::Value) -> CrashReport {
        let report = CrashReport {
            id: uuid::Uuid::new_v4().to_string(),
            component: component.to_string(),
            reason: reason.to_string(),
            captured_at: Utc::now(),
            metadata,
        };
        self.reports.push(report.clone());
        report
    }

    pub fn list(&self) -> &[CrashReport] {
        &self.reports
    }
}
