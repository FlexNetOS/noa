//! Transition Query API
//!
//! Implements T611: Transition query API
//! Provides query interface for retrieving and filtering plane transition records

use chrono::{DateTime, Utc};
use rusqlite::{params_from_iter, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

use super::transition_logger::{TransitionRecord, TransitionType, TransitionStatus, TransitionRowData};

/// Type alias for transition statistics result
type StatsResult = (i64, HashMap<String, i64>, HashMap<String, i64>, HashMap<String, i64>, HashMap<String, i64>, Option<f64>, i64, i64);

/// Query filter for transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionFilter {
    pub transition_type: Option<TransitionType>,
    pub source_plane: Option<String>,
    pub target_plane: Option<String>,
    pub status: Option<TransitionStatus>,
    pub initiated_by: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl Default for TransitionFilter {
    fn default() -> Self {
        Self {
            transition_type: None,
            source_plane: None,
            target_plane: None,
            status: None,
            initiated_by: None,
            date_from: None,
            date_to: None,
            limit: Some(100),
            offset: Some(0),
        }
    }
}

/// Query result with pagination metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub transitions: Vec<TransitionRecord>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

/// Transition statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionStats {
    pub total_transitions: i64,
    pub by_type: HashMap<String, i64>,
    pub by_status: HashMap<String, i64>,
    pub by_source_plane: HashMap<String, i64>,
    pub by_target_plane: HashMap<String, i64>,
    pub average_duration_seconds: Option<f64>,
    pub success_rate: f64,
    pub failure_rate: f64,
}

/// Transition Query API
///
/// Provides query interface for retrieving and analyzing plane transition records
pub struct TransitionQuery {
    conn: Arc<Mutex<Connection>>,
}

impl TransitionQuery {
    /// Create a new TransitionQuery with database connection
    pub async fn new(conn: Connection) -> Result<Self, rusqlite::Error> {
        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
    }

    /// Query transitions with filters
    pub async fn query_transitions(
        &self,
        filter: TransitionFilter,
    ) -> anyhow::Result<QueryResult> {
        let conn = Arc::clone(&self.conn);
        let filter_clone = filter.clone();
        
        let result: Result<(Vec<TransitionRowData>, i64, i64), anyhow::Error> = task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            
            // Build the SQL query dynamically
            let mut sql = r#"
                SELECT id, created_at, type, source_plane, target_plane,
                       source_version, target_version, status, started_at,
                       completed_at, duration_seconds, pre_checks, post_checks,
                       validation_status, artifacts_transferred, outcome,
                       error_message, rollback_reason, initiated_by, approved_by,
                       metadata, before_state, after_state
                FROM plane_transition
                WHERE 1=1
            "#.to_string();
            
            let mut params_vec: Vec<String> = Vec::new();
            
            // Apply filters
            if let Some(ref transition_type) = filter_clone.transition_type {
                sql.push_str(" AND type = ?");
                params_vec.push(transition_type.as_str().to_string());
            }

            if let Some(ref source_plane) = filter_clone.source_plane {
                sql.push_str(" AND source_plane = ?");
                params_vec.push(source_plane.clone());
            }

            if let Some(ref target_plane) = filter_clone.target_plane {
                sql.push_str(" AND target_plane = ?");
                params_vec.push(target_plane.clone());
            }

            if let Some(ref status) = filter_clone.status {
                sql.push_str(" AND status = ?");
                params_vec.push(status.as_str().to_string());
            }

            if let Some(ref initiated_by) = filter_clone.initiated_by {
                sql.push_str(" AND initiated_by = ?");
                params_vec.push(initiated_by.clone());
            }

            if let Some(ref date_from) = filter_clone.date_from {
                sql.push_str(" AND created_at >= ?");
                params_vec.push(date_from.to_rfc3339());
            }

            if let Some(ref date_to) = filter_clone.date_to {
                sql.push_str(" AND created_at <= ?");
                params_vec.push(date_to.to_rfc3339());
            }

            // Order by created_at descending
            sql.push_str(" ORDER BY created_at DESC");

            // Apply limit and offset
            let limit = filter_clone.limit.unwrap_or(100);
            let offset = filter_clone.offset.unwrap_or(0);
            sql.push_str(" LIMIT ? OFFSET ?");
            params_vec.push(limit.to_string());
            params_vec.push(offset.to_string());

            let mut stmt = conn.prepare(&sql)?;
            let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
            
            let mut rows = stmt.query(params_from_iter(params_refs))?;
            
            let mut transitions = Vec::new();
            while let Some(row) = rows.next()? {
                let row_data = (
                    row.get::<_, String>(0)?,  // id
                    row.get::<_, String>(1)?,  // created_at
                    row.get::<_, String>(2)?,  // type
                    row.get::<_, String>(3)?,  // source_plane
                    row.get::<_, String>(4)?,  // target_plane
                    row.get::<_, String>(5)?,  // source_version
                    row.get::<_, String>(6)?,  // target_version
                    row.get::<_, String>(7)?,  // status
                    row.get::<_, Option<String>>(8)?,  // started_at
                    row.get::<_, Option<String>>(9)?,  // completed_at
                    row.get::<_, Option<i64>>(10)?,  // duration_seconds
                    row.get::<_, String>(11)?,  // pre_checks
                    row.get::<_, String>(12)?,  // post_checks
                    row.get::<_, Option<String>>(13)?,  // validation_status
                    row.get::<_, Option<String>>(14)?,  // artifacts_transferred
                    row.get::<_, Option<String>>(15)?,  // outcome
                    row.get::<_, Option<String>>(16)?,  // error_message
                    row.get::<_, Option<String>>(17)?,  // rollback_reason
                    row.get::<_, String>(18)?,  // initiated_by
                    row.get::<_, Option<String>>(19)?,  // approved_by
                    row.get::<_, Option<String>>(20)?,  // metadata
                    row.get::<_, String>(21)?,  // before_state
                    row.get::<_, Option<String>>(22)?,  // after_state
                );
                transitions.push(row_data);
            }
            
            Ok((transitions, limit, offset))
        }).await.map_err(anyhow::Error::from)?;

        let (row_data_vec, limit, offset) = result?;
        
        // Get total count
        let total = self.count_transitions(&filter).await?;

        // Convert rows to TransitionRecord
        let transitions: Result<Vec<TransitionRecord>, _> = row_data_vec
            .into_iter()
            .map(|row_data| self.row_data_to_transition_record(&row_data))
            .collect();

        Ok(QueryResult {
            transitions: transitions?,
            total,
            limit,
            offset,
        })
    }

    /// Get transition by ID
    pub async fn get_transition(
        &self,
        transition_id: &str,
    ) -> anyhow::Result<Option<TransitionRecord>> {
        let conn = Arc::clone(&self.conn);
        let transition_id = transition_id.to_string();
        
        let result = task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            let mut stmt = conn.prepare(
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
            )?;
            
            let mut rows = stmt.query_map([transition_id], |row| {
                Ok((
                    row.get::<_, String>(0)?,  // id
                    row.get::<_, String>(1)?,  // created_at
                    row.get::<_, String>(2)?,  // type
                    row.get::<_, String>(3)?,  // source_plane
                    row.get::<_, String>(4)?,  // target_plane
                    row.get::<_, String>(5)?,  // source_version
                    row.get::<_, String>(6)?,  // target_version
                    row.get::<_, String>(7)?,  // status
                    row.get::<_, Option<String>>(8)?,  // started_at
                    row.get::<_, Option<String>>(9)?,  // completed_at
                    row.get::<_, Option<i64>>(10)?,  // duration_seconds
                    row.get::<_, String>(11)?,  // pre_checks
                    row.get::<_, String>(12)?,  // post_checks
                    row.get::<_, Option<String>>(13)?,  // validation_status
                    row.get::<_, Option<String>>(14)?,  // artifacts_transferred
                    row.get::<_, Option<String>>(15)?,  // outcome
                    row.get::<_, Option<String>>(16)?,  // error_message
                    row.get::<_, Option<String>>(17)?,  // rollback_reason
                    row.get::<_, String>(18)?,  // initiated_by
                    row.get::<_, Option<String>>(19)?,  // approved_by
                    row.get::<_, Option<String>>(20)?,  // metadata
                    row.get::<_, String>(21)?,  // before_state
                    row.get::<_, Option<String>>(22)?,  // after_state
                ))
            })?;

            rows.next().transpose()
        }).await.map_err(anyhow::Error::from)?;

        match result {
            Ok(Some(row_data)) => {
                let record = self.row_data_to_transition_record(&row_data)?;
                Ok(Some(record))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(anyhow::Error::from(e)),
        }
    }

    /// Get statistics for transitions
    pub async fn get_statistics(
        &self,
        date_from: Option<DateTime<Utc>>,
        date_to: Option<DateTime<Utc>>,
    ) -> anyhow::Result<TransitionStats> {
        let conn = Arc::clone(&self.conn);
        let date_from_clone = date_from;
        let date_to_clone = date_to;
        
        let result: Result<StatsResult, anyhow::Error> = task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            
            // Build total count query
            let mut total_sql = "SELECT COUNT(*) as total FROM plane_transition WHERE 1=1".to_string();
            let mut total_params: Vec<String> = Vec::new();
            
            if let Some(ref date_from) = date_from_clone {
                total_sql.push_str(" AND created_at >= ?");
                total_params.push(date_from.to_rfc3339());
            }
            
            if let Some(ref date_to) = date_to_clone {
                total_sql.push_str(" AND created_at <= ?");
                total_params.push(date_to.to_rfc3339());
            }
            
            let total: i64 = {
                let mut stmt = conn.prepare(&total_sql)?;
                let params_refs: Vec<&dyn rusqlite::ToSql> = total_params.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
                stmt.query_row(params_from_iter(params_refs), |row| row.get(0))?
            };

            // Count by type
            let mut by_type = HashMap::new();
            {
                let mut stmt = conn.prepare(
                    r#"
                    SELECT type, COUNT(*) as count
                    FROM plane_transition
                    GROUP BY type
                    "#,
                )?;
                let mut rows = stmt.query([])?;
                while let Some(row) = rows.next()? {
                    let type_name: String = row.get(0)?;
                    let count: i64 = row.get(1)?;
                    by_type.insert(type_name, count);
                }
            }

            // Count by status
            let mut by_status = HashMap::new();
            {
                let mut stmt = conn.prepare(
                    r#"
                    SELECT status, COUNT(*) as count
                    FROM plane_transition
                    GROUP BY status
                    "#,
                )?;
                let mut rows = stmt.query([])?;
                while let Some(row) = rows.next()? {
                    let status_name: String = row.get(0)?;
                    let count: i64 = row.get(1)?;
                    by_status.insert(status_name, count);
                }
            }

            // Count by source plane
            let mut by_source_plane = HashMap::new();
            {
                let mut stmt = conn.prepare(
                    r#"
                    SELECT source_plane, COUNT(*) as count
                    FROM plane_transition
                    GROUP BY source_plane
                    "#,
                )?;
                let mut rows = stmt.query([])?;
                while let Some(row) = rows.next()? {
                    let plane: String = row.get(0)?;
                    let count: i64 = row.get(1)?;
                    by_source_plane.insert(plane, count);
                }
            }

            // Count by target plane
            let mut by_target_plane = HashMap::new();
            {
                let mut stmt = conn.prepare(
                    r#"
                    SELECT target_plane, COUNT(*) as count
                    FROM plane_transition
                    GROUP BY target_plane
                    "#,
                )?;
                let mut rows = stmt.query([])?;
                while let Some(row) = rows.next()? {
                    let plane: String = row.get(0)?;
                    let count: i64 = row.get(1)?;
                    by_target_plane.insert(plane, count);
                }
            }

            // Calculate average duration
            let avg_duration: Option<f64> = {
                let mut stmt = conn.prepare(
                    r#"
                    SELECT AVG(duration_seconds) as avg_duration
                    FROM plane_transition
                    WHERE duration_seconds IS NOT NULL
                    "#,
                )?;
                stmt.query_row([], |row| row.get(0)).ok()
            };

            // Calculate success and failure rates
            let completed: i64 = {
                let mut stmt = conn.prepare(
                    r#"
                    SELECT COUNT(*) as count
                    FROM plane_transition
                    WHERE status = 'completed'
                    "#,
                )?;
                stmt.query_row([], |row| row.get(0))?
            };

            let failed: i64 = {
                let mut stmt = conn.prepare(
                    r#"
                    SELECT COUNT(*) as count
                    FROM plane_transition
                    WHERE status = 'failed'
                    "#,
                )?;
                stmt.query_row([], |row| row.get(0))?
            };

            Ok((total, by_type, by_status, by_source_plane, by_target_plane, avg_duration, completed, failed))
        }).await.map_err(anyhow::Error::from)?;

        let (total, by_type, by_status, by_source_plane, by_target_plane, avg_duration, completed, failed) = result?;

        let success_rate = if total > 0 {
            (completed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        let failure_rate = if total > 0 {
            (failed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        Ok(TransitionStats {
            total_transitions: total,
            by_type,
            by_status,
            by_source_plane,
            by_target_plane,
            average_duration_seconds: avg_duration,
            success_rate,
            failure_rate,
        })
    }

    /// Count transitions matching filter
    async fn count_transitions(&self, filter: &TransitionFilter) -> anyhow::Result<i64> {
        let conn = Arc::clone(&self.conn);
        let filter_clone = filter.clone();
        
        let result: Result<i64, anyhow::Error> = task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            
            let mut sql = "SELECT COUNT(*) as total FROM plane_transition WHERE 1=1".to_string();
            let mut params_vec: Vec<String> = Vec::new();
            
            if let Some(ref transition_type) = filter_clone.transition_type {
                sql.push_str(" AND type = ?");
                params_vec.push(transition_type.as_str().to_string());
            }

            if let Some(ref source_plane) = filter_clone.source_plane {
                sql.push_str(" AND source_plane = ?");
                params_vec.push(source_plane.clone());
            }

            if let Some(ref target_plane) = filter_clone.target_plane {
                sql.push_str(" AND target_plane = ?");
                params_vec.push(target_plane.clone());
            }

            if let Some(ref status) = filter_clone.status {
                sql.push_str(" AND status = ?");
                params_vec.push(status.as_str().to_string());
            }

            if let Some(ref initiated_by) = filter_clone.initiated_by {
                sql.push_str(" AND initiated_by = ?");
                params_vec.push(initiated_by.clone());
            }

            if let Some(ref date_from) = filter_clone.date_from {
                sql.push_str(" AND created_at >= ?");
                params_vec.push(date_from.to_rfc3339());
            }

            if let Some(ref date_to) = filter_clone.date_to {
                sql.push_str(" AND created_at <= ?");
                params_vec.push(date_to.to_rfc3339());
            }

            let mut stmt = conn.prepare(&sql)?;
            let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
            let total: i64 = stmt.query_row(params_from_iter(params_refs), |row| row.get(0))?;
            
            Ok(total)
        }).await.map_err(anyhow::Error::from)?;
        
        result
    }

    /// Convert database row data to TransitionRecord
    fn row_data_to_transition_record(
        &self,
        row_data: &TransitionRowData,
    ) -> anyhow::Result<TransitionRecord> {
        let (
            id, created_at_str, type_str, source_plane, target_plane,
            source_version, target_version, status_str, started_at_str,
            completed_at_str, duration_seconds, pre_checks_json, post_checks_json,
            validation_status_str, artifacts_transferred_json, outcome,
            error_message, rollback_reason, initiated_by, approved_by,
            metadata_json, before_state_json, after_state_json
        ) = row_data;

        let before_state: super::transition_logger::PlaneState = serde_json::from_str(before_state_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;

        let after_state: Option<super::transition_logger::PlaneState> = after_state_json
            .as_ref()
            .map(|json| {
                serde_json::from_str(json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?;

        let pre_checks: Vec<super::transition_logger::CheckResult> = serde_json::from_str(pre_checks_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;

        let post_checks: Vec<super::transition_logger::CheckResult> = serde_json::from_str(post_checks_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;

        let transition_type = match type_str.as_str() {
            "promotion" => TransitionType::Promotion,
            "rollback" => TransitionType::Rollback,
            "migration" => TransitionType::Migration,
            "failover" => TransitionType::Failover,
            _ => return Err(anyhow::anyhow!("Invalid transition type: {}", type_str)),
        };

        let status = match status_str.as_str() {
            "pending" => TransitionStatus::Pending,
            "preparing" => TransitionStatus::Preparing,
            "in_progress" => TransitionStatus::InProgress,
            "validating" => TransitionStatus::Validating,
            "completed" => TransitionStatus::Completed,
            "failed" => TransitionStatus::Failed,
            "rolled_back" => TransitionStatus::RolledBack,
            _ => return Err(anyhow::anyhow!("Invalid transition status: {}", status_str)),
        };

        let validation_status = validation_status_str.as_ref().map(|s| match s.as_str() {
            "passed" => super::transition_logger::ValidationStatus::Passed,
            "failed" => super::transition_logger::ValidationStatus::Failed,
            "skipped" => super::transition_logger::ValidationStatus::Skipped,
            _ => super::transition_logger::ValidationStatus::Skipped,
        });

        let created_at = DateTime::parse_from_rfc3339(created_at_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
            .with_timezone(&Utc);

        let started_at = started_at_str
            .as_ref()
            .map(|s| {
                DateTime::parse_from_rfc3339(s)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .transpose()?;

        let completed_at = completed_at_str
            .as_ref()
            .map(|s| {
                DateTime::parse_from_rfc3339(s)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .transpose()?;

        let artifacts_transferred: Vec<String> = artifacts_transferred_json
            .as_ref()
            .map(|json| {
                serde_json::from_str(json)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?
            .unwrap_or_default();

        let metadata = metadata_json
            .as_ref()
            .map(|json| {
                serde_json::from_str(json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?;

        Ok(TransitionRecord {
            id: id.clone(),
            created_at,
            transition_type,
            source_plane: source_plane.clone(),
            target_plane: target_plane.clone(),
            source_version: source_version.clone(),
            target_version: target_version.clone(),
            status,
            started_at,
            completed_at,
            duration_seconds: *duration_seconds,
            before_state,
            after_state,
            pre_checks,
            post_checks,
            validation_status,
            artifacts_transferred,
            outcome: outcome.clone(),
            error_message: error_message.clone(),
            rollback_reason: rollback_reason.clone(),
            initiated_by: initiated_by.clone(),
            approved_by: approved_by.clone(),
            metadata,
        })
    }
}

