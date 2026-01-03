//! Schema definitions for metadata

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDefinition {
    pub id: String,
    pub name: String,
    pub version: String,
    pub namespace: Option<String>,
    pub fields: Vec<FieldDefinition>,
    pub constraints: Vec<SchemaConstraint>,
    pub indexes: Vec<IndexDefinition>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Field definition in a schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    pub field_type: FieldType,
    pub nullable: bool,
    pub default_value: Option<serde_json::Value>,
    pub description: Option<String>,
    pub constraints: Vec<FieldConstraint>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Field types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
    Date,
    Time,
    Binary,
    Json,
    Array { element_type: Box<FieldType> },
    Map { key_type: Box<FieldType>, value_type: Box<FieldType> },
    Struct { fields: Vec<FieldDefinition> },
    Enum { values: Vec<String> },
    Reference { target_schema: String },
}

/// Schema-level constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaConstraint {
    pub name: String,
    pub constraint_type: ConstraintType,
    pub fields: Vec<String>,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Types of constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintType {
    PrimaryKey,
    UniqueKey,
    ForeignKey,
    Check,
    NotNull,
    Default,
    Custom,
}

/// Field-level constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldConstraint {
    pub constraint_type: FieldConstraintType,
    pub parameters: HashMap<String, serde_json::Value>,
    pub error_message: Option<String>,
}

/// Types of field constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldConstraintType {
    Required,
    MinLength,
    MaxLength,
    Pattern,
    MinValue,
    MaxValue,
    Enum,
    Format,
    Custom,
}

/// Index definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDefinition {
    pub name: String,
    pub fields: Vec<String>,
    pub unique: bool,
    pub index_type: IndexType,
    pub partial_filter: Option<String>,
}

/// Types of indexes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IndexType {
    BTree,
    Hash,
    FullText,
    Spatial,
    Vector,
}

impl SchemaDefinition {
    /// Create a new schema definition
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            version: "1.0.0".to_string(),
            namespace: None,
            fields: Vec::new(),
            constraints: Vec::new(),
            indexes: Vec::new(),
            metadata: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Add a field to the schema
    pub fn add_field(&mut self, field: FieldDefinition) -> &mut Self {
        self.fields.push(field);
        self.updated_at = Utc::now();
        self
    }

    /// Add a constraint to the schema
    pub fn add_constraint(&mut self, constraint: SchemaConstraint) -> &mut Self {
        self.constraints.push(constraint);
        self.updated_at = Utc::now();
        self
    }

    /// Add an index to the schema
    pub fn add_index(&mut self, index: IndexDefinition) -> &mut Self {
        self.indexes.push(index);
        self.updated_at = Utc::now();
        self
    }
}

impl FieldDefinition {
    /// Create a new field definition
    pub fn new(name: impl Into<String>, field_type: FieldType) -> Self {
        Self {
            name: name.into(),
            field_type,
            nullable: true,
            default_value: None,
            description: None,
            constraints: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Set field as required (not nullable)
    pub fn required(mut self) -> Self {
        self.nullable = false;
        self
    }

    /// Set default value
    pub fn with_default(mut self, value: serde_json::Value) -> Self {
        self.default_value = Some(value);
        self
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}
