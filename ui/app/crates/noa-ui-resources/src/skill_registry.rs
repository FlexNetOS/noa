//! Skill registry for managing skills and capabilities

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A skill definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: SkillCategory,
    pub implementation: SkillImplementation,
    pub parameters: Vec<SkillParameter>,
    pub dependencies: Vec<String>,
    pub metadata: SkillMetadata,
    pub status: SkillStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Skill categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SkillCategory {
    CodeGeneration,
    CodeAnalysis,
    TextProcessing,
    DataProcessing,
    FileOperations,
    WebOperations,
    SystemOperations,
    AIOperations,
    Custom(String),
}

/// Skill implementation types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SkillImplementation {
    Native {
        function_name: String,
    },
    Script {
        language: String,
        code: String,
    },
    External {
        endpoint: String,
        method: String,
    },
    Composite {
        skill_ids: Vec<String>,
        execution_order: Vec<String>,
    },
    Plugin {
        plugin_id: String,
        function_name: String,
    },
}

/// A skill parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillParameter {
    pub name: String,
    pub param_type: ParameterType,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub description: Option<String>,
    pub validation: Option<ParameterValidation>,
}

/// Parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    File,
    Any,
}

/// Parameter validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterValidation {
    pub pattern: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub allowed_values: Option<Vec<serde_json::Value>>,
}

/// Skill metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub author: Option<String>,
    pub version: String,
    pub tags: Vec<String>,
    pub examples: Vec<SkillExample>,
    pub documentation_url: Option<String>,
    pub execution_stats: ExecutionStats,
}

/// A skill example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExample {
    pub name: String,
    pub description: Option<String>,
    pub input: serde_json::Value,
    pub expected_output: Option<serde_json::Value>,
}

/// Execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStats {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub average_duration_ms: f64,
    pub last_execution: Option<DateTime<Utc>>,
}

/// Skill status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SkillStatus {
    Active,
    Inactive,
    Deprecated,
    Experimental,
    Error,
}

/// Execution context for skills
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub session_id: String,
    pub user_id: String,
    pub timeout_ms: u64,
    pub environment: HashMap<String, String>,
    pub working_directory: Option<String>,
}

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: serde_json::Value,
    pub error: Option<String>,
    pub duration_ms: u64,
    pub logs: Vec<String>,
}

/// Registry for managing skills
pub struct SkillRegistry {
    skills: HashMap<String, Skill>,
    category_index: HashMap<SkillCategory, Vec<String>>,
    tag_index: HashMap<String, Vec<String>>,
}

impl SkillRegistry {
    /// Create a new SkillRegistry
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
            category_index: HashMap::new(),
            tag_index: HashMap::new(),
        }
    }

    /// Initialize the registry
    pub async fn initialize(&mut self) -> Result<()> {
        self.load_builtin_skills().await?;
        Ok(())
    }

    /// Cleanup resources
    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Register a skill
    pub fn register_skill(&mut self, skill: Skill) -> Result<()> {
        let id = skill.id.clone();

        // Update indices
        self.update_indices(&skill, true);

        // Add skill
        self.skills.insert(id, skill);

        Ok(())
    }

    /// Get a skill by ID
    pub fn get_skill(&self, id: &str) -> Option<Skill> {
        self.skills.get(id).cloned()
    }

    /// Get skills by category
    pub fn get_by_category(&self, category: &SkillCategory) -> Vec<Skill> {
        self.category_index
            .get(category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.skills.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get skills by tag
    pub fn get_by_tag(&self, tag: &str) -> Vec<Skill> {
        self.tag_index
            .get(tag)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.skills.get(id).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Search skills by query
    pub fn search(&self, query: &str) -> Vec<Skill> {
        let query_lower = query.to_lowercase();
        self.skills
            .values()
            .filter(|skill| {
                skill.name.to_lowercase().contains(&query_lower)
                    || skill.description.to_lowercase().contains(&query_lower)
                    || skill
                        .metadata
                        .tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect()
    }

    /// List all skills
    pub fn list_skills(&self) -> Vec<Skill> {
        self.skills.values().cloned().collect()
    }

    /// Execute a skill
    pub async fn execute_skill(
        &mut self,
        id: &str,
        params: HashMap<String, serde_json::Value>,
        context: ExecutionContext,
    ) -> Result<ExecutionResult> {
        let skill = self.skills.get(id).ok_or_else(|| anyhow::anyhow!("Skill not found"))?;

        if skill.status != SkillStatus::Active {
            return Err(anyhow::anyhow!("Skill is not active"));
        }

        // Validate parameters
        self.validate_params(&skill.parameters, &params)?;

        let start = std::time::Instant::now();
        let result = self.execute_implementation(&skill.implementation, params, &context).await?;
        let duration_ms = start.elapsed().as_millis() as u64;

        // Update stats
        if let Some(skill) = self.skills.get_mut(id) {
            skill.metadata.execution_stats.total_executions += 1;
            if result.success {
                skill.metadata.execution_stats.successful_executions += 1;
            } else {
                skill.metadata.execution_stats.failed_executions += 1;
            }
            skill.metadata.execution_stats.last_execution = Some(Utc::now());
            
            // Update average duration
            let total = skill.metadata.execution_stats.total_executions as f64;
            let old_avg = skill.metadata.execution_stats.average_duration_ms;
            skill.metadata.execution_stats.average_duration_ms =
                old_avg + (duration_ms as f64 - old_avg) / total;
        }

        Ok(result)
    }

    /// Remove a skill
    pub fn remove_skill(&mut self, id: &str) -> Option<Skill> {
        if let Some(skill) = self.skills.remove(id) {
            self.update_indices(&skill, false);
            Some(skill)
        } else {
            None
        }
    }

    fn update_indices(&mut self, skill: &Skill, add: bool) {
        let id = &skill.id;

        if add {
            self.category_index
                .entry(skill.category.clone())
                .or_default()
                .push(id.clone());

            for tag in &skill.metadata.tags {
                self.tag_index
                    .entry(tag.clone())
                    .or_default()
                    .push(id.clone());
            }
        } else {
            if let Some(ids) = self.category_index.get_mut(&skill.category) {
                ids.retain(|i| i != id);
            }
            for tag in &skill.metadata.tags {
                if let Some(ids) = self.tag_index.get_mut(tag) {
                    ids.retain(|i| i != id);
                }
            }
        }
    }

    fn validate_params(
        &self,
        parameters: &[SkillParameter],
        params: &HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        for param in parameters {
            if param.required && !params.contains_key(&param.name) {
                return Err(anyhow::anyhow!("Required parameter '{}' is missing", param.name));
            }
        }
        Ok(())
    }

    async fn execute_implementation(
        &self,
        implementation: &SkillImplementation,
        _params: HashMap<String, serde_json::Value>,
        _context: &ExecutionContext,
    ) -> Result<ExecutionResult> {
        match implementation {
            SkillImplementation::Native { function_name } => {
                // Placeholder for native function execution
                Ok(ExecutionResult {
                    success: true,
                    output: serde_json::json!({"message": format!("Executed native function: {}", function_name)}),
                    error: None,
                    duration_ms: 0,
                    logs: vec![],
                })
            }
            SkillImplementation::Script { language, code: _ } => {
                Ok(ExecutionResult {
                    success: true,
                    output: serde_json::json!({"message": format!("Executed {} script", language)}),
                    error: None,
                    duration_ms: 0,
                    logs: vec![],
                })
            }
            SkillImplementation::External { endpoint, method } => {
                Ok(ExecutionResult {
                    success: true,
                    output: serde_json::json!({"message": format!("Called {} {}", method, endpoint)}),
                    error: None,
                    duration_ms: 0,
                    logs: vec![],
                })
            }
            SkillImplementation::Composite { skill_ids, .. } => {
                Ok(ExecutionResult {
                    success: true,
                    output: serde_json::json!({"message": format!("Executed composite skill with {} sub-skills", skill_ids.len())}),
                    error: None,
                    duration_ms: 0,
                    logs: vec![],
                })
            }
            SkillImplementation::Plugin { plugin_id, function_name } => {
                Ok(ExecutionResult {
                    success: true,
                    output: serde_json::json!({"message": format!("Executed plugin {} function: {}", plugin_id, function_name)}),
                    error: None,
                    duration_ms: 0,
                    logs: vec![],
                })
            }
        }
    }

    async fn load_builtin_skills(&mut self) -> Result<()> {
        // Placeholder for loading builtin skills
        Ok(())
    }
}

impl Default for SkillRegistry {
    fn default() -> Self {
        Self::new()
    }
}
