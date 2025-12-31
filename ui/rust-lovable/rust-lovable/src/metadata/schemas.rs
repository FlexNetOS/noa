use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDefinition {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub fields: Vec<SchemaField>,
    pub constraints: Vec<Constraint>,
    pub indexes: Vec<IndexDefinition>,
    pub relationships: Vec<SchemaRelationship>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub field_type: FieldType,
    pub nullable: bool,
    pub default_value: Option<serde_json::Value>,
    pub description: Option<String>,
    pub validation_rules: Vec<ValidationRule>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
    Json,
    Array(Box<FieldType>),
    Map(Box<FieldType>),
    Reference(String),
    Enum(Vec<String>),
    Binary,
    UUID,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: ValidationRuleType,
    pub parameters: HashMap<String, serde_json::Value>,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationRuleType {
    MinLength,
    MaxLength,
    MinValue,
    MaxValue,
    Pattern,
    Required,
    Unique,
    ForeignKey,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub name: String,
    pub constraint_type: ConstraintType,
    pub fields: Vec<String>,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey,
    Unique,
    Check,
    NotNull,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDefinition {
    pub name: String,
    pub index_type: IndexType,
    pub fields: Vec<String>,
    pub unique: bool,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IndexType {
    BTree,
    Hash,
    Gist,
    Gin,
    FullText,
    Spatial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaRelationship {
    pub name: String,
    pub target_schema: String,
    pub relationship_type: RelationshipType,
    pub local_field: String,
    pub foreign_field: String,
    pub on_delete: ReferentialAction,
    pub on_update: ReferentialAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReferentialAction {
    Cascade,
    Restrict,
    SetNull,
    NoAction,
}

impl SchemaDefinition {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: None,
            fields: Vec::new(),
            constraints: Vec::new(),
            indexes: Vec::new(),
            relationships: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn add_field(&mut self, field: SchemaField) {
        self.fields.push(field);
    }
    
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }
    
    pub fn add_index(&mut self, index: IndexDefinition) {
        self.indexes.push(index);
    }
    
    pub fn add_relationship(&mut self, relationship: SchemaRelationship) {
        self.relationships.push(relationship);
    }
    
    pub fn validate(&self) -> Result<()> {
        // Check for duplicate field names
        let mut field_names = std::collections::HashSet::new();
        for field in &self.fields {
            if !field_names.insert(&field.name) {
                return Err(anyhow::anyhow!("Duplicate field name: {}", field.name));
            }
        }
        
        // Check for duplicate constraint names
        let mut constraint_names = std::collections::HashSet::new();
        for constraint in &self.constraints {
            if !constraint_names.insert(&constraint.name) {
                return Err(anyhow::anyhow!("Duplicate constraint name: {}", constraint.name));
            }
        }
        
        // Validate field references
        for field in &self.fields {
            if let FieldType::Reference(ref_schema) = &field.field_type {
                if ref_schema != &self.name {
                    // In a real implementation, we would check if the referenced schema exists
                }
            }
        }
        
        // Validate relationships
        for relationship in &self.relationships {
            // Check if local field exists
            if !self.fields.iter().any(|f| f.name == relationship.local_field) {
                return Err(anyhow::anyhow!(
                    "Relationship references non-existent local field: {}", 
                    relationship.local_field
                ));
            }
        }
        
        Ok(())
    }
    
    pub fn get_field(&self, name: &str) -> Option<&SchemaField> {
        self.fields.iter().find(|f| f.name == name)
    }
    
    pub fn is_field_required(&self, name: &str) -> bool {
        self.fields.iter()
            .find(|f| f.name == name)
            .map(|f| !f.nullable && f.default_value.is_none())
            .unwrap_or(false)
    }
    
    pub fn get_primary_key_fields(&self) -> Vec<String> {
        self.constraints.iter()
            .filter(|c| matches!(c.constraint_type, ConstraintType::PrimaryKey))
            .flat_map(|c| c.fields.clone())
            .collect()
    }
}

impl SchemaField {
    pub fn new(name: String, field_type: FieldType) -> Self {
        Self {
            name,
            field_type,
            nullable: true,
            default_value: None,
            description: None,
            validation_rules: Vec::new(),
            tags: Vec::new(),
        }
    }
    
    pub fn required(mut self) -> Self {
        self.nullable = false;
        self
    }
    
    pub fn with_default(mut self, value: serde_json::Value) -> Self {
        self.default_value = Some(value);
        self
    }
    
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    pub fn add_validation_rule(mut self, rule: ValidationRule) -> Self {
        self.validation_rules.push(rule);
        self
    }
    
    pub fn validate_value(&self, value: &serde_json::Value) -> Result<()> {
        // Check nullability
        if value.is_null() && !self.nullable {
            return Err(anyhow::anyhow!("Field {} cannot be null", self.name));
        }
        
        // Apply validation rules
        for rule in &self.validation_rules {
            rule.validate_value(value, &self.field_type)?;
        }
        
        Ok(())
    }
}

impl ValidationRule {
    pub fn validate_value(&self, value: &serde_json::Value, field_type: &FieldType) -> Result<()> {
        match self.rule_type {
            ValidationRuleType::MinLength => {
                if let Some(min_length) = self.parameters.get("value").and_then(|v| v.as_u64()) {
                    match value {
                        serde_json::Value::String(s) if s.len() < min_length as usize => {
                            return Err(anyhow::anyhow!("{}", self.error_message));
                        }
                        serde_json::Value::Array(arr) if arr.len() < min_length as usize => {
                            return Err(anyhow::anyhow!("{}", self.error_message));
                        }
                        _ => {}
                    }
                }
            }
            ValidationRuleType::MaxLength => {
                if let Some(max_length) = self.parameters.get("value").and_then(|v| v.as_u64()) {
                    match value {
                        serde_json::Value::String(s) if s.len() > max_length as usize => {
                            return Err(anyhow::anyhow!("{}", self.error_message));
                        }
                        serde_json::Value::Array(arr) if arr.len() > max_length as usize => {
                            return Err(anyhow::anyhow!("{}", self.error_message));
                        }
                        _ => {}
                    }
                }
            }
            ValidationRuleType::MinValue => {
                if let Some(min_value) = self.parameters.get("value").and_then(|v| v.as_f64()) {
                    match value {
                        serde_json::Value::Number(n) if n.as_f64().unwrap_or(0.0) < min_value => {
                            return Err(anyhow::anyhow!("{}", self.error_message));
                        }
                        _ => {}
                    }
                }
            }
            ValidationRuleType::MaxValue => {
                if let Some(max_value) = self.parameters.get("value").and_then(|v| v.as_f64()) {
                    match value {
                        serde_json::Value::Number(n) if n.as_f64().unwrap_or(0.0) > max_value => {
                            return Err(anyhow::anyhow!("{}", self.error_message));
                        }
                        _ => {}
                    }
                }
            }
            ValidationRuleType::Pattern => {
                if let Some(pattern) = self.parameters.get("pattern").and_then(|v| v.as_str()) {
                    if let serde_json::Value::String(s) = value {
                        let regex = regex::Regex::new(pattern)
                            .map_err(|e| anyhow::anyhow!("Invalid regex pattern: {}", e))?;
                        if !regex.is_match(s) {
                            return Err(anyhow::anyhow!("{}", self.error_message));
                        }
                    }
                }
            }
            ValidationRuleType::Required => {
                if value.is_null() || (value.is_string() && value.as_str().unwrap().is_empty()) {
                    return Err(anyhow::anyhow!("{}", self.error_message));
                }
            }
            ValidationRuleType::Custom(ref custom_fn) => {
                // In a real implementation, we would call the custom validation function
                // For now, we'll just skip custom validation
            }
            _ => {
                // Other validation types would be implemented similarly
            }
        }
        
        Ok(())
    }
}

pub fn create_prompt_schema() -> SchemaDefinition {
    let mut schema = SchemaDefinition::new("prompt".to_string(), "1.0".to_string());
    
    schema.add_field(SchemaField::new("id".to_string(), FieldType::UUID).required());
    schema.add_field(SchemaField::new("content".to_string(), FieldType::String).required());
    schema.add_field(SchemaField::new("category".to_string(), FieldType::String).required());
    schema.add_field(SchemaField::new("tags".to_string(), FieldType::Array(Box::new(FieldType::String))));
    schema.add_field(SchemaField::new("version".to_string(), FieldType::String).required());
    schema.add_field(SchemaField::new("created_at".to_string(), FieldType::DateTime).required());
    schema.add_field(SchemaField::new("updated_at".to_string(), FieldType::DateTime).required());
    schema.add_field(SchemaField::new("author".to_string(), FieldType::String));
    schema.add_field(SchemaField::new("description".to_string(), FieldType::String));
    schema.add_field(SchemaField::new("parameters".to_string(), FieldType::Map(Box::new(FieldType::String))));
    schema.add_field(SchemaField::new("usage_count".to_string(), FieldType::Integer)
        .with_default(serde_json::Value::Number(0.into())));
    
    // Add constraints
    schema.add_constraint(Constraint {
        name: "prompt_pkey".to_string(),
        constraint_type: ConstraintType::PrimaryKey,
        fields: vec!["id".to_string()],
        parameters: HashMap::new(),
    });
    
    // Add indexes
    schema.add_index(IndexDefinition {
        name: "idx_prompt_category".to_string(),
        index_type: IndexType::BTree,
        fields: vec!["category".to_string()],
        unique: false,
        parameters: HashMap::new(),
    });
    
    schema.add_index(IndexDefinition {
        name: "idx_prompt_created_at".to_string(),
        index_type: IndexType::BTree,
        fields: vec!["created_at".to_string()],
        unique: false,
        parameters: HashMap::new(),
    });
    
    schema
}

pub fn create_embedding_schema() -> SchemaDefinition {
    let mut schema = SchemaDefinition::new("embedding".to_string(), "1.0".to_string());
    
    schema.add_field(SchemaField::new("id".to_string(), FieldType::UUID).required());
    schema.add_field(SchemaField::new("vector".to_string(), FieldType::Array(Box::new(FieldType::Float))).required());
    schema.add_field(SchemaField::new("source_type".to_string(), FieldType::String).required());
    schema.add_field(SchemaField::new("source_id".to_string(), FieldType::String).required());
    schema.add_field(SchemaField::new("model".to_string(), FieldType::String).required());
    schema.add_field(SchemaField::new("dimensions".to_string(), FieldType::Integer).required());
    schema.add_field(SchemaField::new("token_count".to_string(), FieldType::Integer));
    schema.add_field(SchemaField::new("tags".to_string(), FieldType::Array(Box::new(FieldType::String))));
    schema.add_field(SchemaField::new("created_at".to_string(), FieldType::DateTime).required());
    
    // Add constraints
    schema.add_constraint(Constraint {
        name: "embedding_pkey".to_string(),
        constraint_type: ConstraintType::PrimaryKey,
        fields: vec!["id".to_string()],
        parameters: HashMap::new(),
    });
    
    // Add indexes for similarity search
    schema.add_index(IndexDefinition {
        name: "idx_embedding_source".to_string(),
        index_type: IndexType::BTree,
        fields: vec!["source_type".to_string(), "source_id".to_string()],
        unique: false,
        parameters: HashMap::new(),
    });
    
    schema.add_index(IndexDefinition {
        name: "idx_embedding_model".to_string(),
        index_type: IndexType::BTree,
        fields: vec!["model".to_string()],
        unique: false,
        parameters: HashMap::new(),
    });
    
    schema
}

pub fn create_skill_schema() -> SchemaDefinition {
    let mut schema = SchemaDefinition::new("skill".to_string(), "1.0".to_string());
    
    schema.add_field(SchemaField::new("id".to_string(), FieldType::UUID).required());
    schema.add_field(SchemaField::new("name".to_string(), FieldType::String).required());
    schema.add_field(SchemaField::new("description".to_string(), FieldType::String).required());
    schema.add_field(SchemaField::new("category".to_string(), FieldType::String).required());
    schema.add_field(SchemaField::new("tags".to_string(), FieldType::Array(Box::new(FieldType::String))));
    schema.add_field(SchemaField::new("implementation_type".to_string(), FieldType::Enum(vec![
        "code".to_string(), "api".to_string(), "composite".to_string()
    ])).required());
    schema.add_field(SchemaField::new("implementation".to_string(), FieldType::Json).required());
    schema.add_field(SchemaField::new("permissions".to_string(), FieldType::Json).required());
    schema.add_field(SchemaField::new("dependencies".to_string(), FieldType::Array(Box::new(FieldType::String))));
    schema.add_field(SchemaField::new("version".to_string(), FieldType::String).required());
    schema.add_field(SchemaField::new("created_at".to_string(), FieldType::DateTime).required());
    schema.add_field(SchemaField::new("updated_at".to_string(), FieldType::DateTime).required());
    
    // Add constraints
    schema.add_constraint(Constraint {
        name: "skill_pkey".to_string(),
        constraint_type: ConstraintType::PrimaryKey,
        fields: vec!["id".to_string()],
        parameters: HashMap::new(),
    });
    
    schema.add_constraint(Constraint {
        name: "skill_name_unique".to_string(),
        constraint_type: ConstraintType::Unique,
        fields: vec!["name".to_string()],
        parameters: HashMap::new(),
    });
    
    schema
}