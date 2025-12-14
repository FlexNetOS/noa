//! Decision Rationale Recorder
//!
//! Implements T610: Decision rationale recorder
//! Records the reasoning behind promotion/rollback decisions for audit and transparency

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use uuid::Uuid;

/// Type alias for decision record database row data
type DecisionRowData = (
    String, String, String, String, String, String, String,
    Option<String>, Option<String>, Option<String>, String,
    Option<String>, Option<String>
);

/// Decision type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionType {
    Approve,
    Reject,
    Defer,
    RequireManualReview,
}

impl DecisionType {
    fn as_str(&self) -> &'static str {
        match self {
            DecisionType::Approve => "approve",
            DecisionType::Reject => "reject",
            DecisionType::Defer => "defer",
            DecisionType::RequireManualReview => "require_manual_review",
        }
    }
}

/// Risk tier enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskTier {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskTier {
    fn as_str(&self) -> &'static str {
        match self {
            RiskTier::Low => "low",
            RiskTier::Medium => "medium",
            RiskTier::High => "high",
            RiskTier::Critical => "critical",
        }
    }
}

/// Policy gate result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateResult {
    pub gate_name: String,
    pub passed: bool,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub required: bool,
}

/// Analytics evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsResult {
    pub model: String,
    pub score: f64,
    pub confidence: f64,
    pub reasoning: String,
    pub recommendations: Vec<String>,
}

/// Decision rationale record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub id: String,
    pub transition_id: String,
    pub created_at: DateTime<Utc>,
    pub decision_type: DecisionType,
    pub risk_tier: RiskTier,
    pub rationale: String,
    pub policy_gates: Vec<GateResult>,
    pub analytics_results: Vec<AnalyticsResult>,
    pub test_results: Option<serde_json::Value>,
    pub risk_assessment: Option<serde_json::Value>,
    pub decision_factors: Vec<String>,
    pub approved_by: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Decision Rationale Recorder
///
/// Records detailed decision rationale for all promotion/rollback decisions
/// Ensures transparency and auditability of coordinator decisions
pub struct DecisionRecorder {
    conn: Arc<Mutex<Connection>>,
}

impl DecisionRecorder {
    /// Create a new DecisionRecorder with database connection
    pub async fn new(conn: Connection) -> anyhow::Result<Self> {
        let conn = Arc::new(Mutex::new(conn));
        
        // Ensure the decision_record table exists
        {
            let conn_clone = Arc::clone(&conn);
            task::spawn_blocking(move || {
                let conn = conn_clone.blocking_lock();
                conn.execute(
                    r#"
                    CREATE TABLE IF NOT EXISTS decision_record (
                        id TEXT PRIMARY KEY,
                        transition_id TEXT NOT NULL,
                        created_at TEXT NOT NULL DEFAULT (datetime('now')),
                        decision_type TEXT NOT NULL CHECK (decision_type IN (
                            'approve', 'reject', 'defer', 'require_manual_review'
                        )),
                        risk_tier TEXT NOT NULL CHECK (risk_tier IN (
                            'low', 'medium', 'high', 'critical'
                        )),
                        rationale TEXT NOT NULL,
                        policy_gates TEXT NOT NULL,
                        analytics_results TEXT,
                        test_results TEXT,
                        risk_assessment TEXT,
                        decision_factors TEXT NOT NULL,
                        approved_by TEXT,
                        metadata TEXT,
                        checksum TEXT NOT NULL
                    )
                    "#,
                    [],
                )?;

                // Create index for fast lookup by transition_id
                conn.execute(
                    "CREATE INDEX IF NOT EXISTS idx_decision_transition ON decision_record(transition_id)",
                    [],
                )
            }).await.map_err(anyhow::Error::from)??;
        }

        Ok(Self { conn })
    }

    /// Record a decision rationale
    /// Captures the complete reasoning behind a promotion/rollback decision,
    /// including policy gate results, analytics evaluations, and risk assessment
    #[allow(clippy::too_many_arguments)]
    pub async fn record_decision(
        &self,
        transition_id: &str,
        decision_type: DecisionType,
        risk_tier: RiskTier,
        rationale: &str,
        policy_gates: Vec<GateResult>,
        analytics_results: Vec<AnalyticsResult>,
        test_results: Option<serde_json::Value>,
        risk_assessment: Option<serde_json::Value>,
        decision_factors: Vec<String>,
        approved_by: Option<&str>,
        metadata: Option<serde_json::Value>,
    ) -> anyhow::Result<String> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        // Serialize complex fields
        let policy_gates_json = serde_json::to_string(&policy_gates)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let analytics_results_json = serde_json::to_string(&analytics_results)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let test_results_json = test_results
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let risk_assessment_json = risk_assessment
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let decision_factors_json = serde_json::to_string(&decision_factors)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let metadata_json = metadata
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        // Calculate checksum of rationale and key decision data
        let checksum_data = format!("{}{}{}", rationale, decision_type.as_str(), risk_tier.as_str());
        let checksum = self.calculate_checksum(&checksum_data);

        let conn = Arc::clone(&self.conn);
        let transition_id_clone = transition_id.to_string();
        let rationale_clone = rationale.to_string();
        let approved_by_clone = approved_by.map(|s| s.to_string());
        let id_clone = id.clone();
        
        task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            conn.execute(
                r#"
                INSERT INTO decision_record (
                    id, transition_id, created_at, decision_type, risk_tier,
                    rationale, policy_gates, analytics_results, test_results,
                    risk_assessment, decision_factors, approved_by, metadata, checksum
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                params![
                    &id_clone,
                    &transition_id_clone,
                    now.to_rfc3339(),
                    decision_type.as_str(),
                    risk_tier.as_str(),
                    rationale_clone,
                    &policy_gates_json,
                    &analytics_results_json,
                    &test_results_json,
                    &risk_assessment_json,
                    &decision_factors_json,
                    &approved_by_clone,
                    &metadata_json,
                    &checksum
                ],
            )
        }).await.map_err(anyhow::Error::from)??;

        tracing::info!(
            decision_id = %id,
            transition_id = %transition_id,
            decision_type = %decision_type.as_str(),
            risk_tier = %risk_tier.as_str(),
            "Decision rationale recorded"
        );

        Ok(id)
    }

    /// Get decision record by transition ID
    pub async fn get_decision_by_transition(
        &self,
        transition_id: &str,
    ) -> anyhow::Result<Option<DecisionRecord>> {
        let conn = Arc::clone(&self.conn);
        let transition_id = transition_id.to_string();
        let result = task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            let mut stmt = conn.prepare(
                r#"
                SELECT id, transition_id, created_at, decision_type, risk_tier,
                       rationale, policy_gates, analytics_results, test_results,
                       risk_assessment, decision_factors, approved_by, metadata
                FROM decision_record
                WHERE transition_id = ?
                ORDER BY created_at DESC
                LIMIT 1
                "#,
            )?;
            
            let mut rows = stmt.query_map([transition_id], |row| {
                Ok((
                    row.get::<_, String>(0)?,  // id
                    row.get::<_, String>(1)?,  // transition_id
                    row.get::<_, String>(2)?,  // created_at
                    row.get::<_, String>(3)?,  // decision_type
                    row.get::<_, String>(4)?,  // risk_tier
                    row.get::<_, String>(5)?,  // rationale
                    row.get::<_, String>(6)?,  // policy_gates
                    row.get::<_, Option<String>>(7)?,  // analytics_results
                    row.get::<_, Option<String>>(8)?,  // test_results
                    row.get::<_, Option<String>>(9)?,  // risk_assessment
                    row.get::<_, String>(10)?,  // decision_factors
                    row.get::<_, Option<String>>(11)?,  // approved_by
                    row.get::<_, Option<String>>(12)?,  // metadata
                ))
            })?;

            rows.next().transpose()
        }).await.map_err(anyhow::Error::from)?;

        match result {
            Ok(Some(row_data)) => {
                let record = self.row_data_to_decision_record(&row_data)?;
                Ok(Some(record))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Get decision record by ID
    pub async fn get_decision(
        &self,
        decision_id: &str,
    ) -> anyhow::Result<Option<DecisionRecord>> {
        let conn = Arc::clone(&self.conn);
        let decision_id = decision_id.to_string();
        let result = task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            let mut stmt = conn.prepare(
                r#"
                SELECT id, transition_id, created_at, decision_type, risk_tier,
                       rationale, policy_gates, analytics_results, test_results,
                       risk_assessment, decision_factors, approved_by, metadata
                FROM decision_record
                WHERE id = ?
                "#,
            )?;
            
            let mut rows = stmt.query_map([decision_id], |row| {
                Ok((
                    row.get::<_, String>(0)?,  // id
                    row.get::<_, String>(1)?,  // transition_id
                    row.get::<_, String>(2)?,  // created_at
                    row.get::<_, String>(3)?,  // decision_type
                    row.get::<_, String>(4)?,  // risk_tier
                    row.get::<_, String>(5)?,  // rationale
                    row.get::<_, String>(6)?,  // policy_gates
                    row.get::<_, Option<String>>(7)?,  // analytics_results
                    row.get::<_, Option<String>>(8)?,  // test_results
                    row.get::<_, Option<String>>(9)?,  // risk_assessment
                    row.get::<_, String>(10)?,  // decision_factors
                    row.get::<_, Option<String>>(11)?,  // approved_by
                    row.get::<_, Option<String>>(12)?,  // metadata
                ))
            })?;

            rows.next().transpose()
        }).await.map_err(anyhow::Error::from)?;

        match result {
            Ok(Some(row_data)) => {
                let record = self.row_data_to_decision_record(&row_data)?;
                Ok(Some(record))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// List all decisions for a given risk tier
    pub async fn list_decisions_by_risk_tier(
        &self,
        risk_tier: RiskTier,
        limit: Option<i64>,
    ) -> anyhow::Result<Vec<DecisionRecord>> {
        let conn = Arc::clone(&self.conn);
        let risk_tier_str = risk_tier.as_str().to_string();
        let limit_val = limit;
        
        let result: Result<Vec<DecisionRowData>, anyhow::Error> = task::spawn_blocking(move || {
            let conn = conn.blocking_lock();
            let sql = if limit_val.is_some() {
                r#"
                SELECT id, transition_id, created_at, decision_type, risk_tier,
                       rationale, policy_gates, analytics_results, test_results,
                       risk_assessment, decision_factors, approved_by, metadata
                FROM decision_record
                WHERE risk_tier = ?
                ORDER BY created_at DESC
                LIMIT ?
                "#
            } else {
                r#"
                SELECT id, transition_id, created_at, decision_type, risk_tier,
                       rationale, policy_gates, analytics_results, test_results,
                       risk_assessment, decision_factors, approved_by, metadata
                FROM decision_record
                WHERE risk_tier = ?
                ORDER BY created_at DESC
                "#
            };
            
            let mut stmt = conn.prepare(sql)?;
            let rows: Vec<_> = if let Some(limit) = limit_val {
                let params = [risk_tier_str.clone(), limit.to_string()];
                stmt.query_map(params, |row| {
                    Ok((
                        row.get::<_, String>(0)?,  // id
                        row.get::<_, String>(1)?,  // transition_id
                        row.get::<_, String>(2)?,  // created_at
                        row.get::<_, String>(3)?,  // decision_type
                        row.get::<_, String>(4)?,  // risk_tier
                        row.get::<_, String>(5)?,  // rationale
                        row.get::<_, String>(6)?,  // policy_gates
                        row.get::<_, Option<String>>(7)?,  // analytics_results
                        row.get::<_, Option<String>>(8)?,  // test_results
                        row.get::<_, Option<String>>(9)?,  // risk_assessment
                        row.get::<_, String>(10)?,  // decision_factors
                        row.get::<_, Option<String>>(11)?,  // approved_by
                        row.get::<_, Option<String>>(12)?,  // metadata
                    ))
                })?.collect::<Result<Vec<_>, _>>()?
            } else {
                let params = [risk_tier_str];
                stmt.query_map(params, |row| {
                    Ok((
                        row.get::<_, String>(0)?,  // id
                        row.get::<_, String>(1)?,  // transition_id
                        row.get::<_, String>(2)?,  // created_at
                        row.get::<_, String>(3)?,  // decision_type
                        row.get::<_, String>(4)?,  // risk_tier
                        row.get::<_, String>(5)?,  // rationale
                        row.get::<_, String>(6)?,  // policy_gates
                        row.get::<_, Option<String>>(7)?,  // analytics_results
                        row.get::<_, Option<String>>(8)?,  // test_results
                        row.get::<_, Option<String>>(9)?,  // risk_assessment
                        row.get::<_, String>(10)?,  // decision_factors
                        row.get::<_, Option<String>>(11)?,  // approved_by
                        row.get::<_, Option<String>>(12)?,  // metadata
                    ))
                })?.collect::<Result<Vec<_>, _>>()?
            };

            Ok(rows)
        }).await.map_err(anyhow::Error::from)?;

        let rows = result?;
        rows.into_iter()
            .map(|row_data| self.row_data_to_decision_record(&row_data))
            .collect()
    }

    /// Calculate SHA-256 checksum for integrity
    fn calculate_checksum(&self, data: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Convert database row data to DecisionRecord
    fn row_data_to_decision_record(
        &self,
        row_data: &DecisionRowData,
    ) -> anyhow::Result<DecisionRecord> {
        let (
            id, transition_id, created_at_str, decision_type_str, risk_tier_str,
            rationale, policy_gates_json, analytics_results_json, test_results_json,
            risk_assessment_json, decision_factors_json, approved_by, metadata_json
        ) = row_data;

        let decision_type = match decision_type_str.as_str() {
            "approve" => DecisionType::Approve,
            "reject" => DecisionType::Reject,
            "defer" => DecisionType::Defer,
            "require_manual_review" => DecisionType::RequireManualReview,
            _ => return Err(anyhow::anyhow!("Invalid decision type: {}", decision_type_str)),
        };

        let risk_tier = match risk_tier_str.as_str() {
            "low" => RiskTier::Low,
            "medium" => RiskTier::Medium,
            "high" => RiskTier::High,
            "critical" => RiskTier::Critical,
            _ => return Err(anyhow::anyhow!("Invalid risk tier: {}", risk_tier_str)),
        };

        let policy_gates: Vec<GateResult> = serde_json::from_str(policy_gates_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;

        let analytics_results: Vec<AnalyticsResult> = analytics_results_json
            .as_ref()
            .map(|json| {
                serde_json::from_str(json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?
            .unwrap_or_default();

        let test_results = test_results_json
            .as_ref()
            .map(|json| {
                serde_json::from_str(json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?;

        let risk_assessment = risk_assessment_json
            .as_ref()
            .map(|json| {
                serde_json::from_str(json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?;

        let decision_factors: Vec<String> = serde_json::from_str(decision_factors_json)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;

        let metadata = metadata_json
            .as_ref()
            .map(|json| {
                serde_json::from_str(json).map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
            })
            .transpose()?;

        let created_at = DateTime::parse_from_rfc3339(created_at_str)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
            .with_timezone(&Utc);

        Ok(DecisionRecord {
            id: id.clone(),
            transition_id: transition_id.clone(),
            created_at,
            decision_type,
            risk_tier,
            rationale: rationale.clone(),
            policy_gates,
            analytics_results,
            test_results,
            risk_assessment,
            decision_factors,
            approved_by: approved_by.clone(),
            metadata,
        })
    }
}

