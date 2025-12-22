//! Multi-Agent Task Execution Engine
//!
//! Orchestrates sequential execution of agent tasks

use crate::agents::{BaseAgent, CommanderChiefAgent, FileIOAgent, TerminalAgent, RAGAgent};
use crate::agents::commander::{ExecutionPlan, AgentTask, TaskStatus};
use crate::error::{NoaError, Result};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

/// Multi-agent execution engine
pub struct MultiAgentExecutor {
    agents: HashMap<String, Box<dyn BaseAgent>>,
    execution_history: Vec<ExecutionRecord>,
}

/// Execution record for a completed task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRecord {
    pub task_id: Uuid,
    pub agent_name: String,
    pub task_description: String,
    pub started_at: chrono::DateTime<Utc>,
    pub completed_at: chrono::DateTime<Utc>,
    pub status: TaskStatus,
    pub result: Option<String>,
    pub error: Option<String>,
}

/// Execution result for a plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanExecutionResult {
    pub plan_id: Uuid,
    pub total_tasks: usize,
    pub successful_tasks: usize,
    pub failed_tasks: usize,
    pub execution_time_ms: u64,
    pub task_results: Vec<ExecutionRecord>,
}

impl MultiAgentExecutor {
    /// Create a new multi-agent executor
    pub fn new() -> Self {
        let mut agents: HashMap<String, Box<dyn BaseAgent>> = HashMap::new();
        
        // Register available agents
        agents.insert("commander-chief".to_string(), Box::new(CommanderChiefAgent::new()));
        agents.insert("file-io".to_string(), Box::new(FileIOAgent::new()));
        agents.insert("terminal".to_string(), Box::new(TerminalAgent::new()));
        agents.insert("rag".to_string(), Box::new(RAGAgent::new()));
        // Note: model-selector requires a database connection, registered separately

        Self {
            agents,
            execution_history: Vec::new(),
        }
    }

    /// Execute a complete plan with all tasks
    pub fn execute_plan(&mut self, mut plan: ExecutionPlan) -> Result<PlanExecutionResult> {
        let start_time = std::time::Instant::now();
        let mut task_results = Vec::new();
        let mut successful = 0;
        let mut failed = 0;

        for task in &mut plan.tasks {
            let result = self.execute_task(task)?;
            
            if result.status == TaskStatus::Completed {
                successful += 1;
            } else {
                failed += 1;
            }
            
            task_results.push(result);
        }

        let elapsed = start_time.elapsed();

        Ok(PlanExecutionResult {
            plan_id: plan.id,
            total_tasks: plan.tasks.len(),
            successful_tasks: successful,
            failed_tasks: failed,
            execution_time_ms: elapsed.as_millis() as u64,
            task_results,
        })
    }

    /// Execute a single task
    pub fn execute_task(&mut self, task: &mut AgentTask) -> Result<ExecutionRecord> {
        let agent = self.agents.get(&task.agent_name)
            .ok_or_else(|| NoaError::Validation(
                crate::error::ValidationError::new(
                    "agent_name",
                    format!("Agent '{}' not found", task.agent_name),
                    "AGENT_NOT_FOUND"
                )
            ))?;

        let started_at = Utc::now();
        task.status = TaskStatus::InProgress;
        task.started_at = Some(started_at);

        let result = agent.execute(&task.description);

        let completed_at = Utc::now();
        task.completed_at = Some(completed_at);

        let record = match result {
            Ok(output) => {
                task.status = TaskStatus::Completed;
                task.result = Some(output.clone());
                ExecutionRecord {
                    task_id: task.id,
                    agent_name: task.agent_name.clone(),
                    task_description: task.description.clone(),
                    started_at,
                    completed_at,
                    status: TaskStatus::Completed,
                    result: Some(output),
                    error: None,
                }
            }
            Err(e) => {
                task.status = TaskStatus::Failed;
                task.error = Some(e.to_string());
                ExecutionRecord {
                    task_id: task.id,
                    agent_name: task.agent_name.clone(),
                    task_description: task.description.clone(),
                    started_at,
                    completed_at,
                    status: TaskStatus::Failed,
                    result: None,
                    error: Some(e.to_string()),
                }
            }
        };

        // Store in history
        self.execution_history.push(record.clone());

        Ok(record)
    }

    /// Get execution history
    pub fn get_history(&self) -> &[ExecutionRecord] {
        &self.execution_history
    }

    /// Clear execution history
    pub fn clear_history(&mut self) {
        self.execution_history.clear();
    }
}

impl Default for MultiAgentExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::commander::CommanderRequest;

    #[test]
    fn test_execute_single_task() {
        let mut executor = MultiAgentExecutor::new();
        let mut task = AgentTask {
            id: Uuid::new_v4(),
            description: "test task".to_string(),
            agent_name: "rag".to_string(),
            priority: crate::agents::commander::TaskPriority::Medium,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            result: None,
            error: None,
        };

        let result = executor.execute_task(&mut task).unwrap();
        assert_eq!(result.status, TaskStatus::Completed);
        assert!(result.result.is_some());
    }

    #[test]
    fn test_execute_plan() {
        let mut executor = MultiAgentExecutor::new();
        let commander = CommanderChiefAgent::new();
        
        let request = CommanderRequest {
            goal: "Read and write files".to_string(),
            context: None,
            constraints: None,
        };
        
        let plan = commander.plan_execution(request).unwrap();
        let result = executor.execute_plan(plan).unwrap();
        
        assert!(result.successful_tasks > 0);
        assert_eq!(result.total_tasks, result.successful_tasks + result.failed_tasks);
    }
}
