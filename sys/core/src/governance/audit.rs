//! Governance audit trail (FR-025)
//!
//! Persists governance decisions and outcomes for full traceability.

use crate::error::{NoaError, Result};
use crate::governance::engine::{GovernanceDecision, GovernanceOutcome};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Audit event for a governance decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub decision: GovernanceDecision,
    pub outcome: GovernanceOutcome,
    pub recorded_at: DateTime<Utc>,
}

/// Persistent audit trail for governance evaluations.
pub struct GovernanceAuditTrail {
    ledger_path: PathBuf,
    in_memory: Arc<RwLock<Vec<AuditEvent>>>,
    max_in_memory: usize,
}

impl GovernanceAuditTrail {
    /// Create a new audit trail writing to the specified ledger path.
    pub fn new<P: Into<PathBuf>>(ledger_path: P) -> Self {
        Self {
            ledger_path: ledger_path.into(),
            in_memory: Arc::new(RwLock::new(Vec::new())),
            max_in_memory: 1000,
        }
    }

    /// Record an audit event to disk and retain a sliding in-memory window.
    pub async fn record_event(&self, event: AuditEvent) -> Result<()> {
        {
            let mut buffer = self.in_memory.write().await;
            buffer.push(event.clone());
            if buffer.len() > self.max_in_memory {
                let overflow = buffer.len() - self.max_in_memory;
                buffer.drain(0..overflow);
            }
        }

        let path = self.ledger_path.clone();
        tokio::task::spawn_blocking(move || -> Result<()> {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)?;
            serde_json::to_writer(&mut file, &event)?;
            file.write_all(b"\n")?;
            Ok(())
        })
        .await
        .map_err(|e| NoaError::Internal {
            message: "Failed to join audit logging task".to_string(),
            source: Some(Box::new(e)),
        })??;

        Ok(())
    }

    /// Return recent audit events kept in memory.
    pub async fn recent(&self, limit: usize) -> Vec<AuditEvent> {
        let buffer = self.in_memory.read().await;
        let len = buffer.len();
        let start = len.saturating_sub(limit);
        buffer[start..].to_vec()
    }

    /// Load audit events directly from disk (best effort).
    pub fn load_from_disk(&self, limit: usize) -> Result<Vec<AuditEvent>> {
        let mut events = Vec::new();
        if !self.ledger_path.exists() {
            return Ok(events);
        }

        let file = std::fs::File::open(&self.ledger_path)?;
        let reader = std::io::BufReader::new(file);
        for line in reader.lines().take(limit) {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let event: AuditEvent = serde_json::from_str(&line)?;
            events.push(event);
        }
        Ok(events)
    }
}

impl Default for GovernanceAuditTrail {
    fn default() -> Self {
        Self::new("logs/governance-audit.log")
    }
}
