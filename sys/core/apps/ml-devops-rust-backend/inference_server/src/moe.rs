//! Mixture of Experts (MOE) Router
//!
//! Intelligent model routing system that classifies queries and routes them to
//! specialized expert models for optimal performance and quality.
//!
//! Architecture:
//! - Query Classification: Detects intent (code, math, reasoning, general)
//! - Expert Selection: Routes to specialized models based on classification
//! - Parallel Consultation: Consults multiple experts and aggregates results
//! - Confidence Scoring: Fallback chains when confidence is low

use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::models::ModelManager;

/// Expert specialization domains
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Specialization {
    /// Code generation and programming tasks
    CodeGeneration,
    /// Mathematical reasoning and calculations
    Mathematics,
    /// General reasoning and analysis
    Reasoning,
    /// General purpose queries and conversation
    GeneralPurpose,
}

impl Specialization {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CodeGeneration => "code",
            Self::Mathematics => "math",
            Self::Reasoning => "reasoning",
            Self::GeneralPurpose => "general",
        }
    }
}

/// Expert model configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expertconfigs {
    pub model_id: String,
    pub model_file: String,
    pub specialization: Specialization,
    pub confidence_threshold: f32,  // Minimum confidence to use this expert
    pub priority: u8,  // Higher priority experts are preferred on ties
}

impl Expertconfigs {
    /// Get available expert configsurations
    pub fn get_experts() -> Vec<Expertconfigs> {
        vec![
            // Qwen3-1.7B: Best for reasoning and general tasks
            Expertconfigs {
                model_id: "llmware/qwen3-1.7b-gguf".to_string(),
                model_file: "qwen3-1.7b-instruct-q4_k_m.gguf".to_string(),
                specialization: Specialization::Reasoning,
                confidence_threshold: 0.6,
                priority: 10,
            },
            // TinyLlama: Fast general purpose fallback
            Expertconfigs {
                model_id: "TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF".to_string(),
                model_file: "tinyllama-1.1b-chat-v1.0.Q6_K.gguf".to_string(),
                specialization: Specialization::GeneralPurpose,
                confidence_threshold: 0.4,
                priority: 5,
            },
            // DeepSeek-Coder: Code generation specialist (placeholder - using Qwen3 for MVP)
            Expertconfigs {
                model_id: "llmware/qwen3-1.7b-gguf".to_string(),  // TODO: Replace with DeepSeek-Coder
                model_file: "qwen3-1.7b-instruct-q4_k_m.gguf".to_string(),
                specialization: Specialization::CodeGeneration,
                confidence_threshold: 0.7,
                priority: 8,
            },
            // Math specialist (placeholder - using TinyLlama for MVP)
            Expertconfigs {
                model_id: "TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF".to_string(),  // TODO: Replace with math model
                model_file: "tinyllama-1.1b-chat-v1.0.Q6_K.gguf".to_string(),
                specialization: Specialization::Mathematics,
                confidence_threshold: 0.65,
                priority: 7,
            },
        ]
    }
}

/// Query classification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryClassification {
    pub specialization: Specialization,
    pub confidence: f32,
    pub keywords: Vec<String>,
    pub reasoning: String,
}

/// Query classifier for intent detection
pub struct QueryClassifier;

impl QueryClassifier {
    /// Classify a query into a specialization domain
    pub fn classify(query: &str) -> QueryClassification {
        let query_lower = query.to_lowercase();
        
        // Code detection patterns
        let code_keywords = vec![
            "code", "function", "class", "implement", "debug", "error",
            "python", "rust", "javascript", "typescript", "java",
            "algorithm", "refactor", "syntax", "compile", "api",
            "programming", "software", "library", "framework",
        ];
        
        // Math detection patterns
        let math_keywords = vec![
            "calculate", "compute", "solve", "equation", "formula",
            "mathematics", "algebra", "geometry", "statistics",
            "probability", "derivative", "integral", "matrix",
            "number", "sum", "average", "mean", "median",
        ];
        
        // Reasoning detection patterns
        let reasoning_keywords = vec![
            "explain", "analyze", "compare", "evaluate", "reason",
            "why", "how", "what if", "consider", "think",
            "pros and cons", "advantages", "disadvantages",
            "strategy", "approach", "solution", "problem",
        ];
        
        // Count keyword matches
        let code_score = Self::count_matches(&query_lower, &code_keywords);
        let math_score = Self::count_matches(&query_lower, &math_keywords);
        let reasoning_score = Self::count_matches(&query_lower, &reasoning_keywords);
        
        // Detect code blocks or specific syntax patterns
        let has_code_block = query.contains("```") || query.contains("def ") || 
                             query.contains("fn ") || query.contains("function ");
        let has_math_symbols = query.contains("=") && (query.contains("+") || 
                                                        query.contains("-") || 
                                                        query.contains("*") || 
                                                        query.contains("/"));
        
        // Boost scores based on patterns
        let code_score = code_score + if has_code_block { 5 } else { 0 };
        let math_score = math_score + if has_math_symbols { 3 } else { 0 };
        
        // Determine specialization and confidence
        let total_score = code_score + math_score + reasoning_score;
        let max_score = code_score.max(math_score).max(reasoning_score);
        
        let (specialization, confidence, keywords, reasoning) = if max_score == 0 {
            // No keywords matched - general purpose
            (
                Specialization::GeneralPurpose,
                0.8,
                vec![],
                "No domain-specific keywords detected. Using general purpose expert.".to_string(),
            )
        } else if code_score > math_score && code_score > reasoning_score {
            let conf = (code_score as f32 / (total_score as f32 + 1.0)).min(0.95);
            (
                Specialization::CodeGeneration,
                conf,
                code_keywords.iter().filter(|k| query_lower.contains(*k)).map(|s| s.to_string()).collect(),
                format!("Code-related keywords detected (score: {}). Routing to code expert.", code_score),
            )
        } else if math_score > reasoning_score {
            let conf = (math_score as f32 / (total_score as f32 + 1.0)).min(0.95);
            (
                Specialization::Mathematics,
                conf,
                math_keywords.iter().filter(|k| query_lower.contains(*k)).map(|s| s.to_string()).collect(),
                format!("Math-related keywords detected (score: {}). Routing to math expert.", math_score),
            )
        } else {
            let conf = (reasoning_score as f32 / (total_score as f32 + 1.0)).min(0.95);
            (
                Specialization::Reasoning,
                conf,
                reasoning_keywords.iter().filter(|k| query_lower.contains(*k)).map(|s| s.to_string()).collect(),
                format!("Reasoning keywords detected (score: {}). Routing to reasoning expert.", reasoning_score),
            )
        };
        
        QueryClassification {
            specialization,
            confidence,
            keywords,
            reasoning,
        }
    }
    
    fn count_matches(query: &str, keywords: &[&str]) -> i32 {
        keywords.iter()
            .filter(|k| query.contains(*k))
            .count() as i32
    }
}

/// Aggregation strategy for parallel consultation
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AggregationStrategy {
    /// Use the first response (fastest)
    First,
    /// Vote on best response by quality
    Vote,
    /// Concatenate all responses
    Concatenate,
    /// Use response from highest priority expert
    Priority,
}

/// MOE Router configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moeconfigs {
    pub enabled: bool,
    pub parallel_consultation: bool,
    pub max_parallel_experts: usize,
    pub aggregation_strategy: AggregationStrategy,
    pub fallback_to_general: bool,
}

impl Default for Moeconfigs {
    fn default() -> Self {
        Self {
            enabled: true,
            parallel_consultation: false,  // Disabled by default for performance
            max_parallel_experts: 3,
            aggregation_strategy: AggregationStrategy::Priority,
            fallback_to_general: true,
        }
    }
}

/// MOE Router state
pub struct MoeRouter {
    configs: Arc<RwLock<Moeconfigs>>,
    experts: Arc<RwLock<Vec<Expertconfigs>>>,
    // We'll use the existing ModelManager for inference
    // Each expert will be loaded on-demand
}

impl MoeRouter {
    pub fn new() -> Self {
        Self {
            configs: Arc::new(RwLock::new(Moeconfigs::default())),
            experts: Arc::new(RwLock::new(Expertconfigs::get_experts())),
        }
    }
    
    /// Route a query to the best expert
    pub async fn route_query(
        &self,
        query: &str,
    ) -> anyhow::Result<(Specialization, String, f32)> {
        let classification = QueryClassifier::classify(query);
        
        tracing::info!(
            "Query classified as {:?} with confidence {:.2}",
            classification.specialization,
            classification.confidence
        );
        
        tracing::debug!(
            "Classification reasoning: {}",
            classification.reasoning
        );
        
        // Find matching expert
        let experts = self.experts.read().await;
        let expert = experts
            .iter()
            .find(|e| e.specialization == classification.specialization)
            .or_else(|| {
                // Fallback to general purpose
                experts.iter().find(|e| e.specialization == Specialization::GeneralPurpose)
            })
            .ok_or_else(|| anyhow::anyhow!("No suitable expert found"))?;
        
        Ok((
            expert.specialization,
            expert.model_id.clone(),
            classification.confidence,
        ))
    }
    
    /// Get expert configsuration by specialization
    pub async fn get_expert(&self, spec: Specialization) -> Option<Expertconfigs> {
        let experts = self.experts.read().await;
        experts.iter()
            .find(|e| e.specialization == spec)
            .cloned()
    }
    
    /// Execute parallel consultation with multiple experts
    pub async fn parallel_consultation(
        &self,
        query: &str,
        _model_manager: &ModelManager,
    ) -> anyhow::Result<Vec<(Specialization, String)>> {
        let classification = QueryClassifier::classify(query);
        let configs = self.configs.read().await;
        
        if !configs.parallel_consultation {
            return Ok(vec![]);
        }
        
        // Select experts to consult
        let experts = self.experts.read().await;
        let mut selected_experts: Vec<_> = experts
            .iter()
            .filter(|e| e.confidence_threshold <= classification.confidence)
            .take(configs.max_parallel_experts)
            .cloned()
            .collect();
        
        // Sort by priority (highest first)
        selected_experts.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        tracing::info!(
            "Consulting {} experts in parallel",
            selected_experts.len()
        );
        
        // For MVP, we'll return the expert info
        // Full implementation would execute inference on each model
        let results: Vec<_> = selected_experts
            .iter()
            .map(|expert| {
                (
                    expert.specialization,
                    format!(
                        "Expert {} (priority {}) would process this query",
                        expert.specialization.as_str(),
                        expert.priority
                    ),
                )
            })
            .collect();
        
        Ok(results)
    }
    
    /// Get MOE statistics
    pub async fn get_stats(&self) -> MoeStats {
        let experts = self.experts.read().await;
        let configs = self.configs.read().await;
        
        MoeStats {
            enabled: configs.enabled,
            total_experts: experts.len(),
            parallel_enabled: configs.parallel_consultation,
            max_parallel: configs.max_parallel_experts,
            aggregation: configs.aggregation_strategy,
            specializations: experts
                .iter()
                .map(|e| e.specialization.as_str().to_string())
                .collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoeStats {
    pub enabled: bool,
    pub total_experts: usize,
    pub parallel_enabled: bool,
    pub max_parallel: usize,
    pub aggregation: AggregationStrategy,
    pub specializations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_code_classification() {
        let query = "Write a Python function to sort a list";
        let result = QueryClassifier::classify(query);
        assert_eq!(result.specialization, Specialization::CodeGeneration);
        assert!(result.confidence > 0.5);
    }
    
    #[test]
    fn test_math_classification() {
        let query = "Calculate the derivative of x^2 + 3x + 5";
        let result = QueryClassifier::classify(query);
        assert_eq!(result.specialization, Specialization::Mathematics);
        assert!(result.confidence > 0.5);
    }
    
    #[test]
    fn test_reasoning_classification() {
        let query = "Explain the advantages and disadvantages of microservices";
        let result = QueryClassifier::classify(query);
        assert_eq!(result.specialization, Specialization::Reasoning);
        assert!(result.confidence > 0.5);
    }
    
    #[test]
    fn test_general_classification() {
        let query = "Hello, how are you today?";
        let result = QueryClassifier::classify(query);
        assert_eq!(result.specialization, Specialization::GeneralPurpose);
    }
}
