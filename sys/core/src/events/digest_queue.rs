//! Digest Job Queue
//!
//! T183: Implement digest job queue
//! §3.4: Digest Everything Pipeline
//! US4: Digest Everything Pipeline

use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Digest job status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Digest job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestJob {
    pub id: Uuid,
    pub source_id: Uuid,
    pub source_uri: String,
    pub status: JobStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub progress: f64,
    pub current_stage: Option<String>,
}

impl DigestJob {
    pub fn new(source_id: Uuid, source_uri: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_id,
            source_uri: source_uri.into(),
            status: JobStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            error: None,
            progress: 0.0,
            current_stage: None,
        }
    }
}

/// Digest job queue for managing digest pipeline jobs
pub struct DigestJobQueue {
    jobs: std::sync::Arc<std::sync::Mutex<Vec<DigestJob>>>,
}

impl DigestJobQueue {
    /// Create a new digest job queue
    pub fn new() -> Self {
        Self {
            jobs: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    /// Enqueue a new digest job
    pub fn enqueue(&self, job: DigestJob) -> Result<Uuid> {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.push(job.clone());
        Ok(job.id)
    }

    /// Get a job by ID
    pub fn get_job(&self, job_id: &Uuid) -> Result<Option<DigestJob>> {
        let jobs = self.jobs.lock().unwrap();
        Ok(jobs.iter().find(|j| j.id == *job_id).cloned())
    }

    /// Get next pending job
    pub fn dequeue(&self) -> Result<Option<DigestJob>> {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(pos) = jobs.iter().position(|j| j.status == JobStatus::Pending) {
            let job = jobs.remove(pos);
            Ok(Some(job))
        } else {
            Ok(None)
        }
    }

    /// Update job status
    pub fn update_status(
        &self,
        job_id: &Uuid,
        status: JobStatus,
        progress: Option<f64>,
        current_stage: Option<String>,
    ) -> Result<()> {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.iter_mut().find(|j| j.id == *job_id) {
            job.status = status;
            if let Some(prog) = progress {
                job.progress = prog;
            }
            if let Some(stage) = current_stage {
                job.current_stage = Some(stage);
            }
            match status {
                JobStatus::Running => {
                    if job.started_at.is_none() {
                        job.started_at = Some(Utc::now());
                    }
                }
                JobStatus::Completed | JobStatus::Failed | JobStatus::Cancelled => {
                    job.completed_at = Some(Utc::now());
                }
                _ => {}
            }
            Ok(())
        } else {
            Err(crate::error::NoaError::NotFound {
                resource: "digest_job".to_string(),
                id: job_id.to_string(),
            })
        }
    }

    /// List all jobs
    pub fn list_jobs(&self, limit: Option<usize>) -> Result<Vec<DigestJob>> {
        let jobs = self.jobs.lock().unwrap();
        let mut result: Vec<DigestJob> = jobs.iter().cloned().collect();
        result.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        if let Some(limit) = limit {
            result.truncate(limit);
        }
        Ok(result)
    }
}

impl Default for DigestJobQueue {
    fn default() -> Self {
        Self::new()
    }
}

