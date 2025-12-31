use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub tags: Vec<String>,
    pub implementation: SkillImplementation,
    pub metadata: SkillMetadata,
    pub permissions: SkillPermissions,
    pub dependencies: Vec<SkillDependency>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillImplementation {
    Code {
        language: String,
        code: String,
        entry_point: String,
        environment: HashMap<String, String>,
    },
    API {
        endpoint: String,
        method: String,
        headers: HashMap<String, String>,
        timeout_seconds: u64,
    },
    Composite {
        sub_skills: Vec<String>,
        orchestration: OrchestrationType,
        error_handling: ErrorHandlingStrategy,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrchestrationType {
    Sequential,
    Parallel,
    Conditional,
    Loop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorHandlingStrategy {
    FailFast,
    Continue,
    Retry,
    Fallback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub version: String,
    pub author: Option<String>,
    pub documentation: Option<String>,
    pub examples: Vec<SkillExample>,
    pub input_schema: serde_json::Value,
    pub output_schema: serde_json::Value,
    pub performance_metrics: PerformanceMetrics,
    pub cost_estimate: Option<CostEstimate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExample {
    pub name: String,
    pub description: Option<String>,
    pub input: serde_json::Value,
    pub output: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub average_latency_ms: Option<f64>,
    pub throughput_rps: Option<f64>,
    pub success_rate: Option<f64>,
    pub memory_usage_mb: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    pub compute_cost_per_call: f64,
    pub api_cost_per_call: f64,
    pub estimated_calls_per_month: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPermissions {
    pub public: bool,
    pub allowed_users: Vec<String>,
    pub allowed_roles: Vec<String>,
    pub sandbox_required: bool,
    pub network_access: bool,
    pub file_system_access: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDependency {
    pub skill_id: String,
    pub version_constraint: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExecutionContext {
    pub skill_id: String,
    pub input: serde_json::Value,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub environment: HashMap<String, String>,
    pub timeout_seconds: u64,
    pub sandbox_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExecutionResult {
    pub success: bool,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
    pub resource_usage: ResourceUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_time_ms: u64,
    pub memory_peak_mb: f64,
    pub network_bytes: usize,
    pub disk_io_bytes: usize,
}

pub struct SkillRegistry {
    skills: HashMap<String, Skill>,
    name_index: HashMap<String, String>,
    category_index: HashMap<String, Vec<String>>,
    tag_index: HashMap<String, Vec<String>>,
    dependency_graph: HashMap<String, Vec<String>>,
}

impl SkillRegistry {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
            name_index: HashMap::new(),
            category_index: HashMap::new(),
            tag_index: HashMap::new(),
            dependency_graph: HashMap::new(),
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        // Load skills from storage
        self.load_builtin_skills()?;
        Ok(())
    }

    pub async fn cleanup(&mut self) -> Result<()> {
        // Save skills to storage
        Ok(())
    }

    pub fn register_skill(&mut self, skill: Skill) -> Result<()> {
        let id = skill.id.clone();
        let name = skill.name.clone();

        // Validate skill
        self.validate_skill(&skill)?;
        
        // Update indices
        self.update_indices(&skill, true);
        
        // Add to registry
        self.skills.insert(id.clone(), skill);
        self.name_index.insert(name, id.clone());
        
        // Update dependency graph
        self.update_dependency_graph(&id);
        
        Ok(())
    }

    pub fn get_skill(&self, id: &str) -> Option<Skill> {
        self.skills.get(id).cloned()
    }

    pub fn get_skill_by_name(&self, name: &str) -> Option<Skill> {
        self.name_index.get(name)
            .and_then(|id| self.get_skill(id))
    }

    pub fn get_skills_by_category(&self, category: &str) -> Vec<Skill> {
        self.category_index.get(category)
            .map(|ids| ids.iter().filter_map(|id| self.get_skill(id)).collect())
            .unwrap_or_default()
    }

    pub fn get_skills_by_tag(&self, tag: &str) -> Vec<Skill> {
        self.tag_index.get(tag)
            .map(|ids| ids.iter().filter_map(|id| self.get_skill(id)).collect())
            .unwrap_or_default()
    }

    pub fn search_skills(&self, query: &str, category: Option<&str>, tags: Option<Vec<&str>>) -> Vec<Skill> {
        let query_lower = query.to_lowercase();
        
        self.skills.values()
            .filter(|skill| {
                // Text search
                let text_match = skill.name.to_lowercase().contains(&query_lower) ||
                    skill.description.to_lowercase().contains(&query_lower) ||
                    skill.metadata.documentation.as_ref()
                        .map(|doc| doc.to_lowercase().contains(&query_lower))
                        .unwrap_or(false);
                
                // Category filter
                let category_match = category.map_or(true, |cat| skill.category == cat);
                
                // Tags filter
                let tags_match = tags.as_ref().map_or(true, |search_tags| {
                    search_tags.iter().all(|tag| skill.tags.contains(&tag.to_string()))
                });
                
                text_match && category_match && tags_match
            })
            .cloned()
            .collect()
    }

    pub fn execute_skill(&self, context: SkillExecutionContext) -> Result<SkillExecutionResult> {
        let skill = self.get_skill(&context.skill_id)
            .ok_or_else(|| anyhow::anyhow!("Skill not found"))?;
        
        // Check permissions
        self.check_permissions(&skill, &context)?;
        
        // Execute based on implementation type
        match &skill.implementation {
            SkillImplementation::Code { language, code, entry_point, environment } => {
                self.execute_code_skill(&context, language, code, entry_point, environment)
            },
            SkillImplementation::API { endpoint, method, headers, timeout_seconds } => {
                self.execute_api_skill(&context, endpoint, method, headers, *timeout_seconds)
            },
            SkillImplementation::Composite { sub_skills, orchestration, error_handling } => {
                self.execute_composite_skill(&context, sub_skills, orchestration, error_handling)
            },
        }
    }

    pub fn update_skill(&mut self, id: &str, updates: SkillUpdates) -> Result<()> {
        if let Some(skill) = self.skills.get_mut(id) {
            if let Some(name) = updates.name {
                // Update name index
                self.name_index.remove(&skill.name);
                skill.name = name.clone();
                self.name_index.insert(name, id.to_string());
            }
            if let Some(description) = updates.description {
                skill.description = description;
            }
            if let Some(category) = updates.category {
                // Update category index
                if let Some(old_category) = self.category_index.get_mut(&skill.category) {
                    old_category.retain(|skill_id| skill_id != id);
                }
                skill.category = category.clone();
                self.category_index
                    .entry(category)
                    .or_default()
                    .push(id.to_string());
            }
            if let Some(tags) = updates.tags {
                // Update tag indices
                for tag in &skill.tags {
                    if let Some(tag_index) = self.tag_index.get_mut(tag) {
                        tag_index.retain(|skill_id| skill_id != id);
                    }
                }
                
                skill.tags = tags;
                for tag in &skill.tags {
                    self.tag_index
                        .entry(tag.clone())
                        .or_default()
                        .push(id.to_string());
                }
            }
            
            skill.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Skill not found"))
        }
    }

    pub fn remove_skill(&mut self, id: &str) -> Result<()> {
        if let Some(skill) = self.skills.remove(id) {
            // Remove from indices
            self.update_indices(&skill, false);
            self.name_index.remove(&skill.name);
            
            // Remove from dependency graph
            self.dependency_graph.remove(id);
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Skill not found"))
        }
    }

    pub fn get_dependency_graph(&self) -> &HashMap<String, Vec<String>> {
        &self.dependency_graph
    }

    pub fn get_dependent_skills(&self, skill_id: &str) -> Vec<String> {
        self.dependency_graph
            .iter()
            .filter(|(_, deps)| deps.contains(&skill_id.to_string()))
            .map(|(id, _)| id.clone())
            .collect()
    }

    pub fn validate_dependencies(&self, skill_id: &str) -> Result<()> {
        if let Some(deps) = self.dependency_graph.get(skill_id) {
            for dep in deps {
                if !self.skills.contains_key(dep) {
                    return Err(anyhow::anyhow!("Dependency {} not found for skill {}", dep, skill_id));
                }
            }
        }
        Ok(())
    }

    fn load_builtin_skills(&mut self) -> Result<()> {
        // Load built-in skills for common operations
        let builtin_skills = vec![
            Skill {
                id: Uuid::new_v4().to_string(),
                name: "generate_ui_component".to_string(),
                description: "Generate a Dioxus UI component from description".to_string(),
                category: "ui_generation".to_string(),
                tags: vec!["ui".to_string(), "dioxus".to_string(), "component".to_string()],
                implementation: SkillImplementation::Code {
                    language: "rust".to_string(),
                    code: "// Generated component code".to_string(),
                    entry_point: "generate".to_string(),
                    environment: HashMap::new(),
                },
                metadata: SkillMetadata {
                    version: "1.0".to_string(),
                    author: Some("system".to_string()),
                    documentation: Some("Generates UI components".to_string()),
                    examples: vec![],
                    input_schema: serde_json::json!({}),
                    output_schema: serde_json::json!({}),
                    performance_metrics: PerformanceMetrics::default(),
                    cost_estimate: None,
                },
                permissions: SkillPermissions {
                    public: true,
                    allowed_users: vec![],
                    allowed_roles: vec![],
                    sandbox_required: true,
                    network_access: false,
                    file_system_access: false,
                },
                dependencies: vec![],
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
        ];
        
        for skill in builtin_skills {
            let _ = self.register_skill(skill);
        }
        
        Ok(())
    }

    fn validate_skill(&self, skill: &Skill) -> Result<()> {
        // Validate skill structure
        if skill.name.is_empty() {
            return Err(anyhow::anyhow!("Skill name cannot be empty"));
        }
        
        if skill.description.is_empty() {
            return Err(anyhow::anyhow!("Skill description cannot be empty"));
        }
        
        // Validate dependencies
        for dep in &skill.dependencies {
            if !self.skills.contains_key(&dep.skill_id) {
                return Err(anyhow::anyhow!("Dependency {} not found", dep.skill_id));
            }
        }
        
        Ok(())
    }

    fn update_indices(&mut self, skill: &Skill, add: bool) {
        let id = &skill.id;
        
        // Category index
        if add {
            self.category_index
                .entry(skill.category.clone())
                .or_default()
                .push(id.clone());
        } else {
            if let Some(index) = self.category_index.get_mut(&skill.category) {
                index.retain(|skill_id| skill_id != id);
            }
        }
        
        // Tag index
        for tag in &skill.tags {
            if add {
                self.tag_index
                    .entry(tag.clone())
                    .or_default()
                    .push(id.clone());
            } else {
                if let Some(index) = self.tag_index.get_mut(tag) {
                    index.retain(|skill_id| skill_id != id);
                }
            }
        }
    }

    fn update_dependency_graph(&mut self, skill_id: &str) {
        if let Some(skill) = self.skills.get(skill_id) {
            let deps: Vec<String> = skill.dependencies.iter()
                .map(|dep| dep.skill_id.clone())
                .collect();
            
            if !deps.is_empty() {
                self.dependency_graph.insert(skill_id.to_string(), deps);
            }
        }
    }

    fn check_permissions(&self, skill: &Skill, context: &SkillExecutionContext) -> Result<()> {
        if !skill.permissions.public {
            if let Some(user_id) = &context.user_id {
                if !skill.permissions.allowed_users.contains(user_id) {
                    return Err(anyhow::anyhow!("User not authorized to execute this skill"));
                }
            } else {
                return Err(anyhow::anyhow!("User ID required for non-public skills"));
            }
        }
        
        Ok(())
    }

    fn execute_code_skill(&self, context: &SkillExecutionContext, language: &str, code: &str, entry_point: &str, environment: &HashMap<String, String>) -> Result<SkillExecutionResult> {
        // Implementation would execute code in sandbox
        Ok(SkillExecutionResult {
            success: true,
            output: Some(serde_json::json!({"result": "executed"})),
            error: None,
            execution_time_ms: 100,
            resource_usage: ResourceUsage::default(),
        })
    }

    fn execute_api_skill(&self, context: &SkillExecutionContext, endpoint: &str, method: &str, headers: &HashMap<String, String>, timeout_seconds: u64) -> Result<SkillExecutionResult> {
        // Implementation would make API call
        Ok(SkillExecutionResult {
            success: true,
            output: Some(serde_json::json!({"response": "success"})),
            error: None,
            execution_time_ms: 200,
            resource_usage: ResourceUsage::default(),
        })
    }

    fn execute_composite_skill(&self, context: &SkillExecutionContext, sub_skills: &[String], orchestration: &OrchestrationType, error_handling: &ErrorHandlingStrategy) -> Result<SkillExecutionResult> {
        // Implementation would orchestrate sub-skills
        Ok(SkillExecutionResult {
            success: true,
            output: Some(serde_json::json!({"result": "composed"})),
            error: None,
            execution_time_ms: 300,
            resource_usage: ResourceUsage::default(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SkillUpdates {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            average_latency_ms: None,
            throughput_rps: None,
            success_rate: None,
            memory_usage_mb: None,
        }
    }
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            cpu_time_ms: 0,
            memory_peak_mb: 0.0,
            network_bytes: 0,
            disk_io_bytes: 0,
        }
    }
}