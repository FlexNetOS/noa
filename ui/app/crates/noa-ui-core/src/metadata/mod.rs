//! Metadata module for NOA UI
//!
//! Provides rich metadata management for entities including schemas,
//! data tables, relationships, provenance, and quality metrics.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod data_tables;
pub mod metadata_manager;
pub mod schemas;

pub use data_tables::*;
pub use metadata_manager::*;
pub use schemas::*;

/// Rich metadata for an entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichMetadata {
    pub id: String,
    pub entity_type: EntityType,
    pub schemas: Vec<SchemaDefinition>,
    pub data_tables: Vec<DataTable>,
    pub relationships: Vec<Relationship>,
    pub provenance: ProvenanceInfo,
    pub lineage: LineageInfo,
    pub quality: QualityMetrics,
    pub compliance: ComplianceInfo,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Entity types in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntityType {
    AiModel,
    Prompt,
    Embedding,
    Skill,
    Agent,
    Command,
    Dataset,
    Experiment,
    Pipeline,
    configsuration,
    Log,
    Metric,
    Artifact,
}

/// Relationship between entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: RelationshipType,
    pub properties: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// Types of relationships
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationshipType {
    DependsOn,
    DerivesFrom,
    Contains,
    References,
    Triggers,
    Produces,
    Consumes,
    SimilarTo,
}

/// Provenance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceInfo {
    pub created_by: String,
    pub creation_method: String,
    pub source_system: String,
    pub original_format: String,
    pub transformation_steps: Vec<TransformationStep>,
}

/// A transformation step in provenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationStep {
    pub step_id: String,
    pub operation: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub timestamp: DateTime<Utc>,
    pub input_checksum: String,
    pub output_checksum: String,
}

/// Lineage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageInfo {
    pub upstream_entities: Vec<String>,
    pub downstream_entities: Vec<String>,
    pub lineage_graph: String,
    pub impact_analysis: ImpactAnalysis,
}

/// Impact analysis for lineage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysis {
    pub affected_entities: Vec<String>,
    pub impact_score: f64,
    pub critical_path: Vec<String>,
}

/// Quality metrics for an entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub completeness: f64,
    pub accuracy: f64,
    pub consistency: f64,
    pub timeliness: f64,
    pub validity: f64,
    pub uniqueness: f64,
    pub overall_score: f64,
    pub issues: Vec<QualityIssue>,
}

/// A quality issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    pub issue_type: String,
    pub severity: IssueSeverity,
    pub description: String,
    pub field: Option<String>,
    pub suggested_fix: Option<String>,
}

/// Issue severity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Compliance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceInfo {
    pub classification: DataClassification,
    pub retention_days: u32,
    pub regulations: Vec<String>,
    pub access_controls: Vec<AccessControl>,
    pub audit_trail: Vec<AuditEntry>,
}

/// Data classification levels
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
}

/// Access control entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControl {
    pub principal: String,
    pub principal_type: PrincipalType,
    pub permissions: Vec<String>,
}

/// Principal types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrincipalType {
    User,
    Group,
    Role,
    Service,
}

/// Audit trail entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub actor: String,
    pub action: String,
    pub details: HashMap<String, serde_json::Value>,
}

impl Default for RichMetadata {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            entity_type: EntityType::Artifact,
            schemas: Vec::new(),
            data_tables: Vec::new(),
            relationships: Vec::new(),
            provenance: ProvenanceInfo::default(),
            lineage: LineageInfo::default(),
            quality: QualityMetrics::default(),
            compliance: ComplianceInfo::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl Default for ProvenanceInfo {
    fn default() -> Self {
        Self {
            created_by: "system".to_string(),
            creation_method: "auto".to_string(),
            source_system: "noa".to_string(),
            original_format: "json".to_string(),
            transformation_steps: Vec::new(),
        }
    }
}

impl Default for LineageInfo {
    fn default() -> Self {
        Self {
            upstream_entities: Vec::new(),
            downstream_entities: Vec::new(),
            lineage_graph: String::new(),
            impact_analysis: ImpactAnalysis {
                affected_entities: Vec::new(),
                impact_score: 0.0,
                critical_path: Vec::new(),
            },
        }
    }
}

impl Default for QualityMetrics {
    fn default() -> Self {
        Self {
            completeness: 1.0,
            accuracy: 1.0,
            consistency: 1.0,
            timeliness: 1.0,
            validity: 1.0,
            uniqueness: 1.0,
            overall_score: 1.0,
            issues: Vec::new(),
        }
    }
}

impl Default for ComplianceInfo {
    fn default() -> Self {
        Self {
            classification: DataClassification::Internal,
            retention_days: 365,
            regulations: Vec::new(),
            access_controls: Vec::new(),
            audit_trail: Vec::new(),
        }
    }
}
