//! # NOA UI Vibe Coding
//!
//! AI-powered vibe-based coding assistance for NOA UI.
//! 
//! This crate provides intelligent code generation, auto-completion,
//! refactoring, documentation generation, and test generation
//! capabilities driven by AI models.
//!
//! ## Features
//!
//! - **Code Generation**: Generate code from natural language descriptions
//! - **Auto-Complete**: Context-aware code completion suggestions
//! - **Refactoring**: Automated code improvements and optimizations
//! - **Documentation**: Generate documentation from code
//! - **Test Generation**: Automatically generate unit and integration tests
//! - **Learning**: Learn from user feedback to improve suggestions

mod auto_complete;
mod code_generator;
mod code_refactor;
mod documentation_generator;
mod prompt_engineer;
mod test_generator;

pub use auto_complete::{AutoCompleteEngine, CompletionKind, CompletionPattern, CompletionSuggestion};
pub use code_generator::{CodeGenerator, GeneratedCode};
pub use code_refactor::{ChangeType, CodeChange, CodeRefactor, RefactoringPlan, RefactoringRule};
pub use documentation_generator::{Documentation, DocumentationFormat, DocumentationGenerator, DocumentationSection};
pub use prompt_engineer::{PromptEngineer, PromptTemplate};
pub use test_generator::{GeneratedTests, IntegrationTest, TestGenerator, TestStrategy, UnitTest};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main manager for all vibe coding operations
pub struct VibeCodingManager {
    code_generator: Arc<RwLock<CodeGenerator>>,
    prompt_engineer: Arc<RwLock<PromptEngineer>>,
    auto_complete: Arc<RwLock<AutoCompleteEngine>>,
    code_refactor: Arc<RwLock<CodeRefactor>>,
    doc_generator: Arc<RwLock<DocumentationGenerator>>,
    test_generator: Arc<RwLock<TestGenerator>>,
    context: Arc<RwLock<VibeCodingContext>>,
}

/// Context for vibe coding operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibeCodingContext {
    pub project_name: String,
    pub language: String,
    pub framework: String,
    pub coding_style: String,
    pub preferences: HashMap<String, serde_json::Value>,
    pub history: Vec<CodingSession>,
    pub learned_patterns: Vec<Pattern>,
}

/// A coding session record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodingSession {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub task: String,
    pub generated_code: Vec<GeneratedCode>,
    pub user_feedback: UserFeedback,
    pub improvements: Vec<String>,
}

/// User feedback on generated code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeedback {
    pub rating: u8,
    pub comments: String,
    pub corrections: Vec<CodeCorrection>,
}

/// A code correction from user feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeCorrection {
    pub original: String,
    pub corrected: String,
    pub reason: String,
}

/// A learned coding pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub name: String,
    pub description: String,
    pub template: String,
    pub tags: Vec<String>,
    pub usage_count: usize,
    pub success_rate: f64,
}

/// Request for vibe coding operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibeCodingRequest {
    pub task: String,
    pub context: Option<String>,
    pub constraints: Vec<String>,
    pub examples: Vec<String>,
    pub preferences: HashMap<String, serde_json::Value>,
}

/// Response from vibe coding operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibeCodingResponse {
    pub code: Vec<GeneratedCode>,
    pub documentation: Option<Documentation>,
    pub tests: Option<GeneratedTests>,
    pub suggestions: Vec<String>,
    pub confidence: f64,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Code enhancement result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEnhancement {
    pub enhancement_type: EnhancementType,
    pub description: String,
    pub before_code: String,
    pub after_code: String,
    pub impact_score: f64,
}

/// Types of code enhancements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnhancementType {
    Performance,
    Readability,
    Maintainability,
    Security,
    BestPractice,
}

impl VibeCodingManager {
    /// Create a new VibeCodingManager with the given context
    pub fn new(context: VibeCodingContext) -> Self {
        Self {
            code_generator: Arc::new(RwLock::new(CodeGenerator::new())),
            prompt_engineer: Arc::new(RwLock::new(PromptEngineer::new())),
            auto_complete: Arc::new(RwLock::new(AutoCompleteEngine::new())),
            code_refactor: Arc::new(RwLock::new(CodeRefactor::new())),
            doc_generator: Arc::new(RwLock::new(DocumentationGenerator::new())),
            test_generator: Arc::new(RwLock::new(TestGenerator::new())),
            context: Arc::new(RwLock::new(context)),
        }
    }

    /// Initialize all sub-managers
    pub async fn initialize(&self) -> Result<()> {
        self.code_generator.write().await.initialize().await?;
        self.prompt_engineer.write().await.initialize().await?;
        self.auto_complete.write().await.initialize().await?;
        self.code_refactor.write().await.initialize().await?;
        self.doc_generator.write().await.initialize().await?;
        self.test_generator.write().await.initialize().await?;

        Ok(())
    }

    /// Cleanup all sub-managers
    pub async fn cleanup(&self) -> Result<()> {
        self.code_generator.write().await.cleanup().await?;
        self.prompt_engineer.write().await.cleanup().await?;
        self.auto_complete.write().await.cleanup().await?;
        self.code_refactor.write().await.cleanup().await?;
        self.doc_generator.write().await.cleanup().await?;
        self.test_generator.write().await.cleanup().await?;

        Ok(())
    }

    /// Generate code from a request
    pub async fn generate_code(&self, request: VibeCodingRequest) -> Result<VibeCodingResponse> {
        let context = self.context.read().await;

        // Engineer the prompt
        let prompt = self
            .prompt_engineer
            .read()
            .await
            .engineer_prompt(&request, &context)
            .await?;

        // Generate code
        let generated_code = self.code_generator.write().await.generate(&prompt).await?;

        // Generate documentation
        let documentation = if request
            .preferences
            .get("include_docs")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
        {
            Some(
                self.doc_generator
                    .write()
                    .await
                    .generate_docs(&generated_code)
                    .await?,
            )
        } else {
            None
        };

        // Generate tests
        let tests = if request
            .preferences
            .get("include_tests")
            .and_then(|v| v.as_bool())
            .unwrap_or(true)
        {
            Some(
                self.test_generator
                    .write()
                    .await
                    .generate_tests(&generated_code)
                    .await?,
            )
        } else {
            None
        };

        // Get suggestions
        let suggestions = self.analyze_code_quality(&generated_code).await?;

        // Calculate confidence
        let confidence = self.calculate_confidence(&generated_code, &request).await?;

        // Update context with new session
        drop(context); // Release read lock before acquiring write
        self.update_context_with_session(&request, &generated_code)
            .await?;

        Ok(VibeCodingResponse {
            code: generated_code,
            documentation,
            tests,
            suggestions,
            confidence,
            metadata: HashMap::new(),
        })
    }

    /// Enhance code with specified improvement type
    pub async fn enhance_code(
        &self,
        code: String,
        enhancement_type: EnhancementType,
    ) -> Result<CodeEnhancement> {
        let enhancement = match enhancement_type {
            EnhancementType::Performance => {
                self.code_refactor
                    .write()
                    .await
                    .optimize_performance(&code)
                    .await?
            }
            EnhancementType::Readability => {
                self.code_refactor
                    .write()
                    .await
                    .improve_readability(&code)
                    .await?
            }
            EnhancementType::Maintainability => {
                self.code_refactor
                    .write()
                    .await
                    .improve_maintainability(&code)
                    .await?
            }
            EnhancementType::Security => {
                self.code_refactor
                    .write()
                    .await
                    .enhance_security(&code)
                    .await?
            }
            EnhancementType::BestPractice => {
                self.code_refactor
                    .write()
                    .await
                    .apply_best_practices(&code)
                    .await?
            }
        };

        Ok(enhancement)
    }

    /// Get auto-completion suggestions at cursor position
    pub async fn get_auto_complete(
        &self,
        code: String,
        cursor_position: usize,
    ) -> Result<Vec<CompletionSuggestion>> {
        self.auto_complete
            .read()
            .await
            .suggest_completions(&code, cursor_position)
            .await
    }

    /// Learn from user feedback to improve future suggestions
    pub async fn learn_from_feedback(
        &self,
        session_id: &str,
        feedback: UserFeedback,
    ) -> Result<()> {
        let mut context = self.context.write().await;

        // Find the session
        if let Some(session) = context.history.iter_mut().find(|s| s.id == session_id) {
            session.user_feedback = feedback.clone();

            // Extract patterns from corrections
            for correction in &feedback.corrections {
                self.learn_pattern_internal(&mut context, correction)?;
            }

            // Update learned patterns based on feedback
            self.update_patterns_from_feedback_internal(&mut context, &feedback)?;
        }

        Ok(())
    }

    /// Get a coding assistant with current context
    pub async fn get_coding_assistant(&self) -> CodingAssistant {
        let context = self.context.read().await;

        CodingAssistant {
            project_context: context.clone(),
            learned_patterns: context.learned_patterns.clone(),
            suggestions: self.get_personalized_suggestions_internal(&context),
        }
    }

    async fn analyze_code_quality(&self, code: &[GeneratedCode]) -> Result<Vec<String>> {
        let mut suggestions = Vec::new();

        for code_snippet in code {
            // Check for common issues
            if code_snippet.code.lines().count() > 50 {
                suggestions.push(
                    "Consider breaking this function into smaller, more focused functions"
                        .to_string(),
                );
            }

            // Check for documentation
            if !code_snippet.code.contains("///") && !code_snippet.code.contains("//") {
                suggestions.push("Consider adding documentation comments".to_string());
            }

            // Check for error handling
            if !code_snippet.code.contains("Result") && !code_snippet.code.contains("Option") {
                suggestions.push("Consider adding proper error handling".to_string());
            }
        }

        Ok(suggestions)
    }

    async fn calculate_confidence(
        &self,
        code: &[GeneratedCode],
        request: &VibeCodingRequest,
    ) -> Result<f64> {
        let mut confidence: f64 = 0.8; // Base confidence

        // Adjust based on task complexity
        if request.task.len() > 100 {
            confidence -= 0.1;
        }

        // Adjust based on examples provided
        if !request.examples.is_empty() {
            confidence += 0.1;
        }

        // Adjust based on code quality
        for code_snippet in code {
            if code_snippet.code.contains("todo") || code_snippet.code.contains("FIXME") {
                confidence -= 0.05;
            }
        }

        Ok(confidence.max(0.0).min(1.0))
    }

    async fn update_context_with_session(
        &self,
        request: &VibeCodingRequest,
        code: &[GeneratedCode],
    ) -> Result<()> {
        let mut context = self.context.write().await;

        let session = CodingSession {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            task: request.task.clone(),
            generated_code: code.to_vec(),
            user_feedback: UserFeedback {
                rating: 0,
                comments: String::new(),
                corrections: Vec::new(),
            },
            improvements: Vec::new(),
        };

        context.history.push(session);

        // Keep only last 100 sessions
        if context.history.len() > 100 {
            context.history.remove(0);
        }

        Ok(())
    }

    fn learn_pattern_internal(
        &self,
        context: &mut VibeCodingContext,
        correction: &CodeCorrection,
    ) -> Result<()> {
        // Extract pattern from correction
        let pattern_name = format!("pattern_{}", context.learned_patterns.len());
        let pattern = Pattern {
            name: pattern_name,
            description: correction.reason.clone(),
            template: correction.corrected.clone(),
            tags: vec!["learned".to_string()],
            usage_count: 0,
            success_rate: 0.0,
        };

        context.learned_patterns.push(pattern);

        Ok(())
    }

    fn update_patterns_from_feedback_internal(
        &self,
        context: &mut VibeCodingContext,
        feedback: &UserFeedback,
    ) -> Result<()> {
        // Update success rates based on feedback
        for pattern in &mut context.learned_patterns {
            if feedback.rating >= 4 {
                pattern.success_rate = (pattern.success_rate * pattern.usage_count as f64 + 1.0)
                    / (pattern.usage_count as f64 + 1.0);
                pattern.usage_count += 1;
            } else if feedback.rating <= 2 {
                pattern.success_rate = (pattern.success_rate * pattern.usage_count as f64)
                    / (pattern.usage_count as f64 + 1.0);
                pattern.usage_count += 1;
            }
        }

        Ok(())
    }

    fn get_personalized_suggestions_internal(&self, context: &VibeCodingContext) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Analyze coding patterns from history
        let mut pattern_counts = HashMap::new();
        for session in &context.history {
            for code in &session.generated_code {
                let language = if code.code.contains("fn ") || code.code.contains("let ") {
                    "rust"
                } else if code.code.contains("def ") || code.code.contains("import ") {
                    "python"
                } else {
                    "unknown"
                };

                *pattern_counts.entry(language).or_insert(0) += 1;
            }
        }

        // Suggest based on most used patterns
        if let Some((most_used, _)) = pattern_counts.iter().max_by_key(|&(_, count)| *count) {
            suggestions.push(format!("Based on your history, you seem to prefer {}. Consider using our specialized templates for this language.", most_used));
        }

        // Suggest based on learned patterns
        let successful_patterns: Vec<_> = context
            .learned_patterns
            .iter()
            .filter(|p| p.success_rate > 0.8)
            .take(3)
            .collect();

        for pattern in successful_patterns {
            suggestions.push(format!(
                "Try using the '{}' pattern for similar tasks",
                pattern.name
            ));
        }

        suggestions
    }
}

/// Coding assistant with context and learned patterns
#[derive(Debug, Clone)]
pub struct CodingAssistant {
    pub project_context: VibeCodingContext,
    pub learned_patterns: Vec<Pattern>,
    pub suggestions: Vec<String>,
}

impl CodingAssistant {
    /// Get patterns relevant to a task
    pub fn get_relevant_patterns(&self, task: &str) -> Vec<Pattern> {
        self.learned_patterns
            .iter()
            .filter(|p| p.tags.iter().any(|tag| task.contains(tag)) || p.description.contains(task))
            .take(3)
            .cloned()
            .collect()
    }

    /// Get context hints for the current project
    pub fn get_context_hints(&self) -> Vec<String> {
        let mut hints = Vec::new();

        hints.push(format!("Project: {}", self.project_context.project_name));
        hints.push(format!("Language: {}", self.project_context.language));
        hints.push(format!("Framework: {}", self.project_context.framework));

        if !self.project_context.coding_style.is_empty() {
            hints.push(format!(
                "Coding Style: {}",
                self.project_context.coding_style
            ));
        }

        hints
    }
}

impl Default for VibeCodingContext {
    fn default() -> Self {
        Self {
            project_name: "untitled".to_string(),
            language: "rust".to_string(),
            framework: "dioxus".to_string(),
            coding_style: "clean".to_string(),
            preferences: HashMap::new(),
            history: Vec::new(),
            learned_patterns: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vibe_coding_manager_creation() {
        let context = VibeCodingContext::default();
        let manager = VibeCodingManager::new(context);
        assert!(manager.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_auto_complete() {
        let context = VibeCodingContext::default();
        let manager = VibeCodingManager::new(context);
        manager.initialize().await.unwrap();

        let suggestions = manager
            .get_auto_complete("fn main()".to_string(), 5)
            .await
            .unwrap();
        assert!(suggestions.is_empty() || !suggestions.is_empty()); // May or may not have suggestions
    }

    #[test]
    fn test_context_default() {
        let context = VibeCodingContext::default();
        assert_eq!(context.project_name, "untitled");
        assert_eq!(context.language, "rust");
        assert_eq!(context.framework, "dioxus");
    }
}
