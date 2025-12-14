//! Self-Generated Goal Engine
//!
//! Implements self-generated goal engine for autonomous goal creation.
//! §3.4: Adaptive & Self-Improving
//! FR-066-070: Autonomous Goal Generation
//!
//! T631: Implement self-generated goal engine

use crate::error::{Result, NoaError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Goal source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalSource {
    /// User-provided goal
    User,
    /// Self-generated goal
    SelfGenerated,
    /// Constitutional goal
    Constitutional,
}

/// Goal status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalStatus {
    Pending,
    Active,
    Decomposing,
    Executing,
    Completed,
    Failed,
}

/// Generated goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedGoal {
    pub id: Uuid,
    pub source: GoalSource,
    pub title: String,
    pub description: String,
    pub priority: i32,
    pub status: GoalStatus,
    pub rationale: String,
    pub parent_goal: Option<Uuid>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Goal generation trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerationTrigger {
    /// Performance degradation detected
    PerformanceDegradation,
    /// Error rate increase
    ErrorRateIncrease,
    /// Resource inefficiency
    ResourceInefficiency,
    /// Pattern analysis suggestion
    PatternAnalysis,
    /// User feedback
    UserFeedback,
    /// Dependency update needed
    DependencyUpdate,
}

/// Goal generator
pub struct GoalGenerator {
    generated_goals: Arc<RwLock<Vec<GeneratedGoal>>>,
    generation_history: Arc<RwLock<Vec<(GenerationTrigger, Uuid)>>>,
}

impl GoalGenerator {
    /// Create a new goal generator
    pub fn new() -> Self {
        Self {
            generated_goals: Arc::new(RwLock::new(Vec::new())),
            generation_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Generate a new goal
    pub async fn generate_goal(
        &self,
        trigger: GenerationTrigger,
        title: String,
        description: String,
        priority: i32,
        rationale: String,
        parent_goal: Option<Uuid>,
        metadata: serde_json::Value,
    ) -> Result<Uuid> {
        // Validate inputs
        if title.is_empty() {
            return Err(NoaError::Validation(crate::error::ValidationError::new(
                "title",
                "Title cannot be empty",
                "EMPTY_TITLE",
            )));
        }

        if rationale.is_empty() {
            return Err(NoaError::Validation(crate::error::ValidationError::new(
                "rationale",
                "Rationale cannot be empty for self-generated goals",
                "EMPTY_RATIONALE",
            )));
        }

        let goal = GeneratedGoal {
            id: Uuid::new_v4(),
            source: GoalSource::SelfGenerated,
            title,
            description,
            priority,
            status: GoalStatus::Pending,
            rationale,
            parent_goal,
            metadata,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
        };

        let mut goals = self.generated_goals.write().await;
        goals.push(goal.clone());

        let mut history = self.generation_history.write().await;
        history.push((trigger, goal.id));

        Ok(goal.id)
    }

    /// Get all generated goals
    pub async fn all_goals(&self) -> Vec<GeneratedGoal> {
        self.generated_goals.read().await.clone()
    }

    /// Get goals by status
    pub async fn goals_by_status(&self, status: GoalStatus) -> Vec<GeneratedGoal> {
        self.generated_goals.read().await
            .iter()
            .filter(|g| g.status == status)
            .cloned()
            .collect()
    }

    /// Get goal by ID
    pub async fn get_goal(&self, goal_id: Uuid) -> Option<GeneratedGoal> {
        self.generated_goals.read().await
            .iter()
            .find(|g| g.id == goal_id)
            .cloned()
    }

    /// Update goal status
    pub async fn update_status(&self, goal_id: Uuid, status: GoalStatus) -> Result<()> {
        let mut goals = self.generated_goals.write().await;
        let goal = goals.iter_mut().find(|g| g.id == goal_id)
            .ok_or_else(|| NoaError::NotFound {
                resource: "GeneratedGoal".to_string(),
                id: goal_id.to_string(),
            })?;

        goal.status = status;

        match status {
            GoalStatus::Active | GoalStatus::Decomposing | GoalStatus::Executing => {
                if goal.started_at.is_none() {
                    goal.started_at = Some(Utc::now());
                }
            }
            GoalStatus::Completed | GoalStatus::Failed => {
                goal.completed_at = Some(Utc::now());
            }
            _ => {}
        }

        Ok(())
    }

    /// Get generation history
    pub async fn generation_history(&self) -> Vec<(GenerationTrigger, Uuid)> {
        self.generation_history.read().await.clone()
    }

    /// Get goals by trigger
    pub async fn goals_by_trigger(&self, trigger: &GenerationTrigger) -> Vec<GeneratedGoal> {
        let history = self.generation_history.read().await;
        let goal_ids: Vec<Uuid> = history
            .iter()
            .filter(|(t, _)| {
                match (t, trigger) {
                    (GenerationTrigger::PerformanceDegradation, GenerationTrigger::PerformanceDegradation) => true,
                    (GenerationTrigger::ErrorRateIncrease, GenerationTrigger::ErrorRateIncrease) => true,
                    (GenerationTrigger::ResourceInefficiency, GenerationTrigger::ResourceInefficiency) => true,
                    (GenerationTrigger::PatternAnalysis, GenerationTrigger::PatternAnalysis) => true,
                    (GenerationTrigger::UserFeedback, GenerationTrigger::UserFeedback) => true,
                    (GenerationTrigger::DependencyUpdate, GenerationTrigger::DependencyUpdate) => true,
                    _ => false,
                }
            })
            .map(|(_, id)| *id)
            .collect();

        self.generated_goals.read().await
            .iter()
            .filter(|g| goal_ids.contains(&g.id))
            .cloned()
            .collect()
    }
}

impl Default for GoalGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_goal() {
        let generator = GoalGenerator::new();

        let goal_id = generator.generate_goal(
            GenerationTrigger::PerformanceDegradation,
            "Optimize database queries".to_string(),
            "Improve query performance".to_string(),
            10,
            "Query latency increased by 50%".to_string(),
            None,
            serde_json::json!({}),
        ).await.unwrap();

        let goal = generator.get_goal(goal_id).await.unwrap();
        assert_eq!(goal.title, "Optimize database queries");
        assert_eq!(goal.status, GoalStatus::Pending);
    }

    #[tokio::test]
    async fn test_update_status() {
        let generator = GoalGenerator::new();

        let goal_id = generator.generate_goal(
            GenerationTrigger::ErrorRateIncrease,
            "Fix error handling".to_string(),
            "Description".to_string(),
            5,
            "Error rate increased".to_string(),
            None,
            serde_json::json!({}),
        ).await.unwrap();

        generator.update_status(goal_id, GoalStatus::Active).await.unwrap();
        let goal = generator.get_goal(goal_id).await.unwrap();
        assert_eq!(goal.status, GoalStatus::Active);
        assert!(goal.started_at.is_some());
    }

    #[tokio::test]
    async fn test_validation() {
        let generator = GoalGenerator::new();

        // Empty title should fail
        assert!(generator.generate_goal(
            GenerationTrigger::PatternAnalysis,
            "".to_string(),
            "Description".to_string(),
            5,
            "Rationale".to_string(),
            None,
            serde_json::json!({}),
        ).await.is_err());
    }
}

