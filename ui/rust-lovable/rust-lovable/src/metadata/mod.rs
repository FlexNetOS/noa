use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

pub mod schemas;
pub mod data_tables;
pub mod metadata_manager;

use schemas::*;
use data_tables::*;
use metadata_manager::MetadataManager;

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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    Configuration,
    Log,
    Metric,
    Artifact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: RelationshipType,
    pub properties: HashMap<String, serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceInfo {
    pub created_by: String,
    pub creation_method: String,
    pub source_system: String,
    pub original_format: String,
    pub transformation_steps: Vec<TransformationStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationStep {
    pub step_id: String,
    pub operation: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub input_checksum: String,
    pub output_checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageInfo {
    pub upstream_entities: Vec<String>,
    pub downstream_entities: Vec<String>,
    pub lineage_graph: String, // Graph representation
    pub impact_analysis: ImpactAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysis {
    pub affected_entities: Vec<String>,
    pub impact_score: f64,
    pub critical_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub completeness: f64,
    pub accuracy: f64,
    pub consistency: f64,
    pub timeliness: f64,
    pub validity: f64,
    pub uniqueness: f64,
    pub overall_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceInfo {
    pub gdpr_compliant: bool,
    pub ccpa_compliant: bool,
    pub hipaa_compliant: bool,
    pub sox_compliant: bool,
    pub data_residency: String,
    pub retention_policy: RetentionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub retention_period_days: u32,
    pub deletion_method: String,
    pub archival_enabled: bool,
    pub legal_hold: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataQuery {
    pub entity_type: Option<EntityType>,
    pub schema_name: Option<String>,
    pub tags: Vec<String>,
    pub date_range: Option<DateRange>,
    pub quality_threshold: Option<f64>,
    pub relationships: Vec<RelationshipQuery>,
    pub provenance: Option<ProvenanceQuery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipQuery {
    pub target_id: String,
    pub relationship_type: Option<RelationshipType>,
    pub direction: RelationshipDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationshipDirection {
    Incoming,
    Outgoing,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceQuery {
    pub created_by: Option<String>,
    pub source_system: Option<String>,
    pub creation_method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub error_type: ValidationErrorType,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationErrorType {
    MissingRequired,
    InvalidFormat,
    OutOfRange,
    DuplicateValue,
    ReferenceNotFound,
    SchemaViolation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub field: String,
    pub warning_type: ValidationWarningType,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationWarningType {
    MissingRecommended,
    DeprecatedField,
    InconsistentData,
    PerformanceImpact,
}

impl RichMetadata {
    pub fn new(entity_type: EntityType, created_by: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            entity_type,
            schemas: Vec::new(),
            data_tables: Vec::new(),
            relationships: Vec::new(),
            provenance: ProvenanceInfo {
                created_by,
                creation_method: "manual".to_string(),
                source_system: "rust-lovable".to_string(),
                original_format: "json".to_string(),
                transformation_steps: Vec::new(),
            },
            lineage: LineageInfo {
                upstream_entities: Vec::new(),
                downstream_entities: Vec::new(),
                lineage_graph: String::new(),
                impact_analysis: ImpactAnalysis {
                    affected_entities: Vec::new(),
                    impact_score: 0.0,
                    critical_path: Vec::new(),
                },
            },
            quality: QualityMetrics {
                completeness: 1.0,
                accuracy: 1.0,
                consistency: 1.0,
                timeliness: 1.0,
                validity: 1.0,
                uniqueness: 1.0,
                overall_score: 1.0,
            },
            compliance: ComplianceInfo {
                gdpr_compliant: true,
                ccpa_compliant: true,
                hipaa_compliant: false,
                sox_compliant: false,
                data_residency: "US".to_string(),
                retention_policy: RetentionPolicy {
                    retention_period_days: 365,
                    deletion_method: "secure_delete".to_string(),
                    archival_enabled: true,
                    legal_hold: false,
                },
            },
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn add_schema(&mut self, schema: SchemaDefinition) {
        self.schemas.push(schema);
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn add_data_table(&mut self, table: DataTable) {
        self.data_tables.push(table);
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.push(relationship);
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn update_quality_metrics(&mut self, metrics: QualityMetrics) {
        self.quality = metrics;
        self.updated_at = chrono::Utc::now();
    }
    
    pub fn validate(&self) -> MetadataValidationResult {
        let mut result = MetadataValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        };
        
        // Validate required fields
        if self.schemas.is_empty() {
            result.errors.push(ValidationError {
                field: "schemas".to_string(),
                error_type: ValidationErrorType::MissingRequired,
                message: "At least one schema is required".to_string(),
                severity: Severity::Critical,
            });
            result.is_valid = false;
        }
        
        // Validate schema consistency
        for schema in &self.schemas {
            if let Err(e) = schema.validate() {
                result.errors.push(ValidationError {
                    field: format!("schema.{}", schema.name),
                    error_type: ValidationErrorType::SchemaViolation,
                    message: format!("Schema validation failed: {}", e),
                    severity: Severity::High,
                });
                result.is_valid = false;
            }
        }
        
        // Validate relationships
        for relationship in &self.relationships {
            if relationship.source_id.is_empty() || relationship.target_id.is_empty() {
                result.errors.push(ValidationError {
                    field: "relationships".to_string(),
                    error_type: ValidationErrorType::InvalidFormat,
                    message: "Relationship source and target IDs cannot be empty".to_string(),
                    severity: Severity::High,
                });
                result.is_valid = false;
            }
        }
        
        // Add suggestions
        if self.quality.completeness < 0.8 {
            result.suggestions.push("Consider adding more metadata fields to improve completeness".to_string());
        }
        
        if self.relationships.is_empty() {
            result.suggestions.push("Consider adding relationships to improve data connectivity".to_string());
        }
        
        result
    }
}
