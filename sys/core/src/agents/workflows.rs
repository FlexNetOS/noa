//! Real-World Agent Workflows
//!
//! Pre-built workflows for common automation tasks

use crate::agents::commander::{CommanderRequest, ExecutionPlan};
use crate::agents::executor::{MultiAgentExecutor, PlanExecutionResult};
use crate::agents::CommanderChiefAgent;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Workflow types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowType {
    CodeReview,
    Deployment,
    Testing,
    Documentation,
    SecurityAudit,
    Custom(String),
}

/// Workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    pub workflow_type: WorkflowType,
    pub parameters: serde_json::Value,
}

/// Workflow result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    pub workflow_type: WorkflowType,
    pub success: bool,
    pub execution_result: PlanExecutionResult,
    pub summary: String,
}

/// Workflow orchestrator
pub struct WorkflowOrchestrator {
    commander: CommanderChiefAgent,
    executor: MultiAgentExecutor,
}

impl WorkflowOrchestrator {
    pub fn new() -> Self {
        Self {
            commander: CommanderChiefAgent::new(),
            executor: MultiAgentExecutor::new(),
        }
    }

    /// Execute a workflow
    pub fn execute_workflow(&mut self, config: WorkflowConfig) -> Result<WorkflowResult> {
        let plan = self.create_plan(&config)?;
        let execution_result = self.executor.execute_plan(plan)?;
        
        let success = execution_result.failed_tasks == 0;
        let summary = self.generate_summary(&config.workflow_type, &execution_result);

        Ok(WorkflowResult {
            workflow_type: config.workflow_type,
            success,
            execution_result,
            summary,
        })
    }

    /// Create execution plan for a workflow
    fn create_plan(&self, config: &WorkflowConfig) -> Result<ExecutionPlan> {
        let goal = match &config.workflow_type {
            WorkflowType::CodeReview => {
                format!("Review code changes in {}", 
                    config.parameters.get("target").and_then(|v| v.as_str()).unwrap_or("repository"))
            }
            WorkflowType::Deployment => {
                format!("Deploy application to {}", 
                    config.parameters.get("environment").and_then(|v| v.as_str()).unwrap_or("production"))
            }
            WorkflowType::Testing => {
                format!("Run test suite for {}", 
                    config.parameters.get("component").and_then(|v| v.as_str()).unwrap_or("application"))
            }
            WorkflowType::Documentation => {
                format!("Generate documentation for {}", 
                    config.parameters.get("target").and_then(|v| v.as_str()).unwrap_or("project"))
            }
            WorkflowType::SecurityAudit => {
                format!("Perform security audit on {}", 
                    config.parameters.get("target").and_then(|v| v.as_str()).unwrap_or("codebase"))
            }
            WorkflowType::Custom(name) => {
                format!("Execute custom workflow: {}", name)
            }
        };

        let request = CommanderRequest {
            goal,
            context: Some(config.parameters.as_object()
                .map(|obj| obj.iter()
                    .map(|(k, v)| (k.clone(), v.to_string()))
                    .collect())
                .unwrap_or_default()),
            constraints: None,
        };

        self.commander.plan_execution(request)
    }

    /// Generate workflow summary
    fn generate_summary(&self, workflow_type: &WorkflowType, result: &PlanExecutionResult) -> String {
        let status = if result.failed_tasks == 0 { "✓ SUCCESS" } else { "✗ FAILED" };
        
        format!(
            "{} - {:?}\nTotal Tasks: {}\nSuccessful: {}\nFailed: {}\nExecution Time: {}ms",
            status,
            workflow_type,
            result.total_tasks,
            result.successful_tasks,
            result.failed_tasks,
            result.execution_time_ms
        )
    }
}

impl Default for WorkflowOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Pre-built workflow builders
pub mod workflows {
    use super::*;

    /// Code review workflow
    pub fn code_review(pr_number: String, target_branch: String) -> WorkflowConfig {
        WorkflowConfig {
            workflow_type: WorkflowType::CodeReview,
            parameters: serde_json::json!({
                "pr_number": pr_number,
                "target_branch": target_branch,
                "target": format!("PR #{}", pr_number)
            }),
        }
    }

    /// Deployment workflow
    pub fn deployment(environment: String, version: String) -> WorkflowConfig {
        WorkflowConfig {
            workflow_type: WorkflowType::Deployment,
            parameters: serde_json::json!({
                "environment": environment,
                "version": version
            }),
        }
    }

    /// Testing workflow
    pub fn testing(component: String, test_type: String) -> WorkflowConfig {
        WorkflowConfig {
            workflow_type: WorkflowType::Testing,
            parameters: serde_json::json!({
                "component": component,
                "test_type": test_type
            }),
        }
    }

    /// Documentation generation workflow
    pub fn documentation(target: String, output_format: String) -> WorkflowConfig {
        WorkflowConfig {
            workflow_type: WorkflowType::Documentation,
            parameters: serde_json::json!({
                "target": target,
                "format": output_format
            }),
        }
    }

    /// Security audit workflow
    pub fn security_audit(target: String, audit_type: String) -> WorkflowConfig {
        WorkflowConfig {
            workflow_type: WorkflowType::SecurityAudit,
            parameters: serde_json::json!({
                "target": target,
                "audit_type": audit_type
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_review_workflow() {
        let mut orchestrator = WorkflowOrchestrator::new();
        let config = workflows::code_review("123".to_string(), "main".to_string());
        
        let result = orchestrator.execute_workflow(config).unwrap();
        assert!(result.execution_result.total_tasks > 0);
    }

    #[test]
    fn test_deployment_workflow() {
        let mut orchestrator = WorkflowOrchestrator::new();
        let config = workflows::deployment("staging".to_string(), "v1.0.0".to_string());
        
        let result = orchestrator.execute_workflow(config).unwrap();
        assert!(result.execution_result.total_tasks > 0);
    }

    #[test]
    fn test_testing_workflow() {
        let mut orchestrator = WorkflowOrchestrator::new();
        let config = workflows::testing("core".to_string(), "integration".to_string());
        
        let result = orchestrator.execute_workflow(config).unwrap();
        assert!(result.execution_result.total_tasks > 0);
    }
}
