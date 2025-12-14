//! Model Selector Agent
//!
//! T114-T115: Implement ModelSelectorAgent base logic and selection criteria
//! §3.3: Agentic Orchestration
//! US2: Model selection for optimal task routing

use crate::error::{Result, NoaError};
use crate::db::repositories::{ModelRepository, Model as DbModel, ModelType, ModelStatus};
use crate::db::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Task type for model selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    /// Code generation
    CodeGeneration,
    /// Reasoning/problem solving
    Reasoning,
    /// Summarization
    Summarization,
    /// Question answering
    QuestionAnswering,
    /// Translation
    Translation,
    /// General purpose
    General,
}

/// Model selection criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionCriteria {
    pub task_type: TaskType,
    pub required_context_length: Option<usize>,
    pub max_latency_ms: Option<u64>,
    pub available_resources: ResourceConstraints,
    pub cost_preference: CostPreference,
}

/// Resource constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraints {
    pub max_cpu_usage: f64,
    pub max_memory_gb: f64,
    pub gpu_available: bool,
    pub gpu_memory_gb: Option<f64>,
}

impl Default for ResourceConstraints {
    fn default() -> Self {
        Self {
            max_cpu_usage: 0.8,
            max_memory_gb: 8.0,
            gpu_available: false,
            gpu_memory_gb: None,
        }
    }
}

/// Cost preference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CostPreference {
    /// Prefer fastest model
    Speed,
    /// Prefer most capable model
    Capability,
    /// Prefer most efficient model
    Efficiency,
    /// Balance all factors
    Balanced,
}

/// Model selector agent
pub struct ModelSelectorAgent {
    repository: ModelRepository,
}

impl ModelSelectorAgent {
    /// Create a new model selector agent
    pub fn new(conn: Connection) -> Self {
        Self {
            repository: ModelRepository::new(conn),
        }
    }

    /// Select optimal model based on criteria
    pub fn select_model(&self, criteria: &SelectionCriteria) -> Result<Option<DbModel>> {
        // Get all available models
        let all_models = self.repository.find_by_status(ModelStatus::Available)?;

        if all_models.is_empty() {
            return Ok(None);
        }

        // Score each model
        let mut scored_models: Vec<(DbModel, f64)> = all_models
            .into_iter()
            .map(|model| {
                let score = self.score_model(&model, criteria);
                (model, score)
            })
            .collect();

        // Sort by score (highest first)
        scored_models.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Return best model if score is above threshold
        if let Some((model, score)) = scored_models.first() {
            if *score > 0.0 {
                return Ok(Some(model.clone()));
            }
        }

        Ok(None)
    }

    /// Score a model based on selection criteria
    fn score_model(&self, model: &DbModel, criteria: &SelectionCriteria) -> f64 {
        let mut score = 0.0;

        // Task type matching (T115)
        let task_match = match criteria.task_type {
            TaskType::CodeGeneration => {
                // Prefer models with code capabilities
                model.provider.contains("code") || model.name.to_lowercase().contains("code")
            }
            TaskType::Reasoning => {
                // Prefer larger models for reasoning
                model.parameters.as_ref().map(|p| p.contains("7B") || p.contains("13B")).unwrap_or(false)
            }
            TaskType::Summarization => {
                // General models work well
                true
            }
            TaskType::QuestionAnswering => {
                // Prefer models with good context
                model.context_length.unwrap_or(0) >= 2048
            }
            TaskType::Translation => {
                // Multilingual models
                model.name.to_lowercase().contains("multilingual")
            }
            TaskType::General => {
                true
            }
        };

        if task_match {
            score += 30.0;
        }

        // Context length matching
        if let Some(required) = criteria.required_context_length {
            if let Some(available) = model.context_length {
                if available >= required as i32 {
                    score += 20.0;
                } else {
                    score -= 10.0; // Penalty for insufficient context
                }
            }
        }

        // Resource constraints
        if criteria.available_resources.gpu_available {
            if let JsonValue::Number(n) = &model.config.get("n_gpu_layers").unwrap_or(&JsonValue::Null) {
                if n.as_i64().unwrap_or(0) > 0 {
                    score += 15.0;
                }
            }
        } else {
            // CPU-only models preferred when no GPU
            if let JsonValue::Number(n) = &model.config.get("n_gpu_layers").unwrap_or(&JsonValue::Null) {
                if n.as_i64().unwrap_or(0) == 0 {
                    score += 10.0;
                }
            }
        }

        // Cost preference (T115)
        match criteria.cost_preference {
            CostPreference::Speed => {
                // Prefer smaller, faster models
                if let Some(params) = &model.parameters {
                    if params.contains("1.5B") || params.contains("3B") {
                        score += 15.0;
                    }
                }
            }
            CostPreference::Capability => {
                // Prefer larger models
                if let Some(params) = &model.parameters {
                    if params.contains("7B") || params.contains("13B") || params.contains("70B") {
                        score += 15.0;
                    }
                }
            }
            CostPreference::Efficiency => {
                // Prefer quantized models
                if let Some(JsonValue::String(quant)) = model.config.get("quantization") {
                    if quant.contains("q4") || quant.contains("q5") {
                        score += 15.0;
                    }
                }
            }
            CostPreference::Balanced => {
                // Balanced scoring (already applied)
            }
        }

        // Historical performance (if metrics available)
        if let Some(metrics) = &model.metrics {
            if let Some(JsonValue::Number(n)) = metrics.get("tokens_per_second") {
                if let Some(tokens_per_sec) = n.as_f64() {
                    score += (tokens_per_sec / 100.0).min(10.0); // Cap at 10 points
                }
            }
        }

        score
    }

    /// Get selection criteria for a task type
    pub fn criteria_for_task(task_type: TaskType) -> SelectionCriteria {
        let (context_length, max_latency) = match task_type {
            TaskType::CodeGeneration => (Some(4096), Some(5000)),
            TaskType::Reasoning => (Some(8192), Some(10000)),
            TaskType::Summarization => (Some(4096), Some(3000)),
            TaskType::QuestionAnswering => (Some(2048), Some(2000)),
            TaskType::Translation => (Some(2048), Some(2000)),
            TaskType::General => (Some(2048), Some(2000)),
        };

        SelectionCriteria {
            task_type,
            required_context_length: context_length,
            max_latency_ms: max_latency,
            available_resources: ResourceConstraints::default(),
            cost_preference: CostPreference::Balanced,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_criteria_for_task() {
        let criteria = ModelSelectorAgent::criteria_for_task(TaskType::CodeGeneration);
        assert_eq!(criteria.task_type, TaskType::CodeGeneration);
        assert_eq!(criteria.required_context_length, Some(4096));
    }
}

