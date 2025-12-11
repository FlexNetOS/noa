use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{NoaError, Result, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub required: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaDefinition {
    pub name: String,
    pub version: String,
    pub description: String,
    pub fields: Vec<SchemaField>,
}

/// KSCHEMA_CAP: Schema registry for capsules.
pub struct SchemaRegistry {
    schemas: HashMap<String, SchemaDefinition>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
        }
    }

    pub fn register(&mut self, schema: SchemaDefinition) -> Result<()> {
        let key = format!("{}:{}", schema.name, schema.version);
        if self.schemas.contains_key(&key) {
            return Err(NoaError::Validation(ValidationError::new(
                "schema",
                format!("Schema {} already registered", key),
                "DUPLICATE_SCHEMA",
            )));
        }
        self.schemas.insert(key, schema);
        Ok(())
    }

    pub fn validate(
        &self,
        schema_name: &str,
        version: &str,
        payload: &serde_json::Value,
    ) -> Result<()> {
        let key = format!("{}:{}", schema_name, version);
        let schema = self.schemas.get(&key).ok_or_else(|| NoaError::NotFound {
            resource: "schema".to_string(),
            id: key.clone(),
        })?;

        for field in &schema.fields {
            if field.required {
                let missing = payload.get(&field.name).is_none();
                if missing {
                    return Err(NoaError::Validation(ValidationError::new(
                        &field.name,
                        "Field required by schema",
                        "SCHEMA_FIELD_MISSING",
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn list(&self) -> Vec<SchemaDefinition> {
        self.schemas.values().cloned().collect()
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}
