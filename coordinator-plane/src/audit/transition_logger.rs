//! Transition Logger - Before/After State Logging
//!
//! Implements T609: §3.5 Transition logger (before/after state)
//! Logs complete state snapshots before and after plane transitions for audit trail

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, Row};
use std::collections::HashMap;
use uuid::Uuid;

/// Transition type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionType {
    Promotion,
    Rollback,
    Migration,
    Failover,
}

impl TransitionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransitionType::Promotion => "promotion",
            TransitionType::Rollback => "rollback",
            TransitionType::Migration => "migration",
            TransitionType::Failover => "failover",
        }
    }
}

/// Transition status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransitionStatus {
    Pending,
    Preparing,
    InProgress,
    Validating,
    Completed,
    Failed,
    RolledBack,
}

impl TransitionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransitionStatus::Pending => "pending",
            TransitionStatus::Preparing => "preparing",
            TransitionStatus::InProgress => "in_progress",
            TransitionStatus::Validating => "validating",
            TransitionStatus::Completed => "completed",
            TransitionStatus::Failed => "failed",
            TransitionStatus::RolledBack => "rolled_back",
        }
    }
}

/// Validation status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Passed,
    Failed,
    Skipped,
}

impl ValidationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ValidationStatus::Passed => "passed",
            ValidationStatus::Failed => "failed",
            ValidationStatus::Skipped => "skipped",
        }
    }
}

/// Pre-check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub name: String,
    pub status: String, // "passed", "failed", "warning"
    pub message: Option<String>,
    pub details: Option<serde_json::Value>,
}

/// Plane state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaneState {
    pub plane_id: String,
    pub plane_name: String,
    pub version: String,
    pub status: String,
    pub health_status: String,
    pub components: HashMap<String, ComponentState>,
    pub resources: Option<ResourceState>,
    pub config_hash: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Component state within a plane
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentState {
    pub name: String,
    pub version: String,
    pub status: String,
    pub health: String,
    pub metadata: Option<serde_json::Value>,
}

/// Resource state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceState {
    pub cpu_usage: Option<f64>,
    pub memory_usage: Option<f64>,
    pub storage_usage: Option<f64>,
    pub network_io: Option<serde_json::Value>,
}

/// Transition record with before/after state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionRecord {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub transition_type: TransitionType,
    pub source_plane: String,
    pub target_plane: String,
    pub source_version: String,
    pub target_version: String,
    pub status: TransitionStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub before_state: PlaneState,
    pub after_state: Option<PlaneState>,
    pub pre_checks: Vec<CheckResult>,
    pub post_checks: Vec<CheckResult>,
    pub validation_status: Option<ValidationStatus>,
    pub artifacts_transferred: Vec<String>,
    pub outcome: Option<String>,
    pub error_message: Option<String>,
    pub rollback_reason: Option<String>,
    pub initiated_by: String,
    pub approved_by: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Transition Logger
///
/// Logs plane transitions with complete before/after state snapshots
/// for audit trail and compliance with §3.5 (Transparent & Auditable)
pub struct TransitionLogger {
    pool: SqlitePool,
}

impl TransitionLogger {
    /// Create a new TransitionLogger with database connection
    pub async fn new(pool: SqlitePool) -> Result<Self, sqlx::Error> {
        // Ensure the plane_transition table exists
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS plane_transition (
                id TEXT PRIMARY KEY,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                type TEXT NOT NULL CHECK (type IN ('promotion', 'rollback', 'migration', 'failover')),
                source_plane TEXT NOT NULL,
                target_plane TEXT NOT NULL,
                source_version TEXT NOT NULL,
                target_version TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN (
                    'pending', 'preparing', 'in_progress', 'validating',
                    'completed', 'failed', 'rolled_back'
                )),
                started_at TEXT,
                completed_at TEXT,
                duration_seconds INTEGER,
                pre_checks TEXT,
                post_checks TEXT,
                validation_status TEXT CHECK (validation_status IN ('passed', 'failed', 'skipped')),
                artifacts_transferred TEXT,
                outcome TEXT,
                error_message TEXT,
                rollback_reason TEXT,
                initiated_by TEXT NOT NULL,
                approved_by TEXT,
                metadata TEXT,
                checksum TEXT NOT NULL,
                before_state TEXT NOT NULL,
                after_state TEXT
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    /// Start logging a transition with before state
    ///
    /// Captures the complete state of the source plane before transition begins
    #[allow(clippy::too_many_arguments)] // Required for comprehensive audit logging
    pub async fn log_transition_start(
        &self,
        transition_type: TransitionType,
        source_plane: &str,
        target_plane: &str,
        source_version: &str,
        target_version: &str,
        before_state: PlaneState,
        pre_checks: Vec<CheckResult>,
        initiated_by: &str,
        metadata: Option<serde_json::Value>,
    ) -> Result<String, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        // Serialize state and checks
        let before_state_json = serde_json::to_string(&before_state)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let pre_checks_json = serde_json::to_string(&pre_checks)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let metadata_json = metadata
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        // Calculate checksum of before_state
        let checksum = self.calculate_checksum(&before_state_json);

        sqlx::query(
            r#"
            INSERT INTO plane_transition (
                id, created_at, type, source_plane, target_plane,
                source_version, target_version, status, before_state,
                pre_checks, initiated_by, metadata, checksum
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(now.to_rfc3339())
        .bind(transition_type.as_str())
        .bind(source_plane)
        .bind(target_plane)
        .bind(source_version)
        .bind(target_version)
        .bind(TransitionStatus::Pending.as_str())
        .bind(&before_state_json)
        .bind(&pre_checks_json)
        .bind(initiated_by)
        .bind(&metadata_json)
        .bind(&checksum)
        .execute(&self.pool)
        .await?;

        tracing::info!(
            transition_id = %id,
            transition_type = %transition_type.as_str(),
            source_plane = %source_plane,
            target_plane = %target_plane,
            "Transition logging started"
        );

        Ok(id)
    }

    /// Update transition status
    pub async fn update_status(
        &self,
        transition_id: &str,
        status: TransitionStatus,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE plane_transition
            SET status = ?, started_at = COALESCE(started_at, ?)
            WHERE id = ?
            "#,
        )
        .bind(status.as_str())
        .bind(now.to_rfc3339())
        .bind(transition_id)
        .execute(&self.pool)
        .await?;

        tracing::debug!(
            transition_id = %transition_id,
            status = %status.as_str(),
            "Transition status updated"
        );

        Ok(())
    }

    /// Log transition completion with after state
    ///
    /// Captures the complete state of the target plane after transition completes
    pub async fn log_transition_complete(
        &self,
        transition_id: &str,
        after_state: PlaneState,
        post_checks: Vec<CheckResult>,
        validation_status: Option<ValidationStatus>,
        outcome: Option<String>,
        error_message: Option<String>,
    ) -> Result<(), sqlx::Error> {
        let now = Utc::now();

        // Get started_at to calculate duration
        let started_at_str: Option<String> = sqlx::query(
            "SELECT started_at FROM plane_transition WHERE id = ?",
        )
        .bind(transition_id)
        .try_map(|row: sqlx::sqlite::SqliteRow| row.try_get("started_at"))
        .fetch_optional(&self.pool)
        .await?;

        let duration_seconds = started_at_str
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|start| (now - start.with_timezone(&Utc)).num_seconds());

        // Serialize state and checks
        let after_state_json = serde_json::to_string(&after_state)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let post_checks_json = serde_json::to_string(&post_checks)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        let status = if error_message.is_some() {
            TransitionStatus::Failed
        } else {
            TransitionStatus::Completed
        };

        sqlx::query(
            r#"
            UPDATE plane_transition
            SET status = ?, completed_at = ?, duration_seconds = ?,
                after_state = ?, post_checks = ?, validation_status = ?,
                outcome = ?, error_message = ?
            WHERE id = ?
            "#,
        )
        .bind(status.as_str())
        .bind(now.to_rfc3339())
        .bind(duration_seconds)
        .bind(&after_state_json)
        .bind(&post_checks_json)
        .bind(validation_status.map(|v| v.as_str()))
        .bind(&outcome)
        .bind(&error_message)
        .bind(transition_id)
        .execute(&self.pool)
        .await?;

        tracing::info!(
            transition_id = %transition_id,
            status = %status.as_str(),
            duration_seconds = ?duration_seconds,
            "Transition logging completed"
        );

        Ok(())
    }

    /// Get transition record by ID
    pub async fn get_transition(
        &self,
        transition_id: &str,
    ) -> Result<Option<TransitionRecord>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, created_at, type, source_plane, target_plane,
                   source_version, target_version, status, started_at,
                   completed_at, duration_seconds, pre_checks, post_checks,
                   validation_status, artifacts_transferred, outcome,
                   error_message, rollback_reason, initiated_by, approved_by,
                   metadata, before_state, after_state
            FROM plane_transition
            WHERE id = ?
            "#,
        )
        .bind(transition_id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(|r| self.row_to_transition_record(&r))
            .transpose()
    }

    /// Calculate SHA-256 checksum for state integrity
    fn calculate_checksum(&self, data: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Convert database row to TransitionRecord
    fn row_to_transition_record(
        &self,
        row: &sqlx::sqlite::SqliteRow,
    ) -> Result<TransitionRecord, sqlx::Error> {
        let before_state_json: String = row.try_get("before_state")?;
        let before_state: PlaneState = serde_json::from_str(&before_state_json)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        let after_state: Option<PlaneState> = row
            .try_get::<Option<String>, _>("after_state")?
            .map(|json| {
                serde_json::from_str(&json).map_err(|e| sqlx::Error::Decode(Box::new(e)))
            })
            .transpose()?;

        let pre_checks_json: String = row.try_get("pre_checks")?;
        let pre_checks: Vec<CheckResult> = serde_json::from_str(&pre_checks_json)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        let post_checks_json: String = row.try_get("post_checks")?;
        let post_checks: Vec<CheckResult> = serde_json::from_str(&post_checks_json)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        let type_str: String = row.try_get("type")?;
        let transition_type = match type_str.as_str() {
            "promotion" => TransitionType::Promotion,
            "rollback" => TransitionType::Rollback,
            "migration" => TransitionType::Migration,
            "failover" => TransitionType::Failover,
            _ => return Err(sqlx::Error::Decode("Invalid transition type".into())),
        };

        let status_str: String = row.try_get("status")?;
        let status = match status_str.as_str() {
            "pending" => TransitionStatus::Pending,
            "preparing" => TransitionStatus::Preparing,
            "in_progress" => TransitionStatus::InProgress,
            "validating" => TransitionStatus::Validating,
            "completed" => TransitionStatus::Completed,
            "failed" => TransitionStatus::Failed,
            "rolled_back" => TransitionStatus::RolledBack,
            _ => return Err(sqlx::Error::Decode("Invalid transition status".into())),
        };

        let validation_status: Option<String> = row.try_get("validation_status")?;
        let validation_status = validation_status.map(|s| match s.as_str() {
            "passed" => ValidationStatus::Passed,
            "failed" => ValidationStatus::Failed,
            "skipped" => ValidationStatus::Skipped,
            _ => ValidationStatus::Skipped,
        });

        let created_at_str: String = row.try_get("created_at")?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
            .with_timezone(&Utc);

        let started_at: Option<String> = row.try_get("started_at")?;
        let started_at = started_at
            .map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .transpose()?;

        let completed_at: Option<String> = row.try_get("completed_at")?;
        let completed_at = completed_at
            .map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .transpose()?;

        let artifacts_transferred: Option<String> = row.try_get("artifacts_transferred")?;
        let artifacts_transferred: Vec<String> = artifacts_transferred
            .map(|json| {
                serde_json::from_str(&json)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))
            })
            .transpose()?
            .unwrap_or_default();

        let metadata: Option<String> = row.try_get("metadata")?;
        let metadata = metadata
            .map(|json| {
                serde_json::from_str(&json).map_err(|e| sqlx::Error::Decode(Box::new(e)))
            })
            .transpose()?;

        Ok(TransitionRecord {
            id: row.try_get("id")?,
            created_at,
            transition_type,
            source_plane: row.try_get("source_plane")?,
            target_plane: row.try_get("target_plane")?,
            source_version: row.try_get("source_version")?,
            target_version: row.try_get("target_version")?,
            status,
            started_at,
            completed_at,
            duration_seconds: row.try_get("duration_seconds")?,
            before_state,
            after_state,
            pre_checks,
            post_checks,
            validation_status,
            artifacts_transferred,
            outcome: row.try_get("outcome")?,
            error_message: row.try_get("error_message")?,
            rollback_reason: row.try_get("rollback_reason")?,
            initiated_by: row.try_get("initiated_by")?,
            approved_by: row.try_get("approved_by")?,
            metadata,
        })
    }
}

