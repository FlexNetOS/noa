use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::error::{NoaError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct ApprovalRequest {
    pub id: String,
    pub summary: String,
    pub rationale: String,
    pub status: ApprovalStatus,
    pub approver: Option<String>,
    pub decided_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Human co-improvement approval workflow.
pub struct ImprovementApprovalWorkflow {
    requests: HashMap<String, ApprovalRequest>,
}

impl ImprovementApprovalWorkflow {
    pub fn new() -> Self {
        Self {
            requests: HashMap::new(),
        }
    }

    pub fn submit(&mut self, summary: impl Into<String>, rationale: impl Into<String>) -> ApprovalRequest {
        let req = ApprovalRequest {
            id: uuid::Uuid::new_v4().to_string(),
            summary: summary.into(),
            rationale: rationale.into(),
            status: ApprovalStatus::Pending,
            approver: None,
            decided_at: None,
            created_at: Utc::now(),
        };
        self.requests.insert(req.id.clone(), req.clone());
        req
    }

    pub fn approve(&mut self, id: &str, approver: impl Into<String>) -> Result<()> {
        let req = self.requests.get_mut(id).ok_or_else(|| NoaError::NotFound {
            resource: "approval_request".to_string(),
            id: id.to_string(),
        })?;
        req.status = ApprovalStatus::Approved;
        req.approver = Some(approver.into());
        req.decided_at = Some(Utc::now());
        Ok(())
    }

    pub fn reject(&mut self, id: &str, approver: impl Into<String>, rationale: impl Into<String>) -> Result<()> {
        let req = self.requests.get_mut(id).ok_or_else(|| NoaError::NotFound {
            resource: "approval_request".to_string(),
            id: id.to_string(),
        })?;
        req.status = ApprovalStatus::Rejected;
        req.approver = Some(approver.into());
        req.rationale = rationale.into();
        req.decided_at = Some(Utc::now());
        Ok(())
    }

    pub fn status(&self, id: &str) -> Option<ApprovalStatus> {
        self.requests.get(id).map(|r| r.status.clone())
    }
}

impl Default for ImprovementApprovalWorkflow {
    fn default() -> Self {
        Self::new()
    }
}
