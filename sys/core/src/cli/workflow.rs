use crate::error::Result;
use crate::agents::workflows::{WorkflowOrchestrator, workflows, WorkflowType};

#[derive(Debug, Clone)]
pub enum WorkflowCmd {
    List,
    Run {
        workflow_type: String,
        parameters: serde_json::Value,
    },
    CodeReview {
        pr_number: String,
        target_branch: String,
    },
    Deploy {
        environment: String,
        version: String,
    },
    Test {
        component: String,
        test_type: String,
    },
    SecurityAudit {
        target: String,
    },
}

pub async fn execute(command: WorkflowCmd) -> Result<()> {
    let orchestrator = WorkflowOrchestrator::new();

    match command {
        WorkflowCmd::List => {
            println!("Available Workflows");
            println!("{:-<80}", "");
            println!("  • code-review    - Automated code review workflow");
            println!("  • deployment     - Application deployment workflow");
            println!("  • testing        - Test execution workflow");
            println!("  • security-audit - Security audit workflow");
            println!("  • documentation  - Documentation generation workflow");
            println!();
            println!("Usage: noa workflow run <type> [parameters]");
            Ok(())
        }
        WorkflowCmd::Run {
            workflow_type,
            parameters,
        } => {
            println!("Executing workflow: {}", workflow_type);
            println!("Parameters: {}", parameters);
            println!("{:-<80}", "");

            let workflow_enum = match workflow_type.as_str() {
                "code-review" => WorkflowType::CodeReview,
                "deployment" => WorkflowType::Deployment,
                "testing" => WorkflowType::Testing,
                "security-audit" => WorkflowType::SecurityAudit,
                "documentation" => WorkflowType::Documentation,
                name => WorkflowType::Custom(name.to_string()),
            };

            let config = crate::agents::workflows::WorkflowConfig {
                workflow_type: workflow_enum,
                parameters,
            };

            let result = orchestrator.execute_workflow(config).await?;

            println!("\n{}", result.summary);
            println!("\nDetailed Results:");
            for (i, task) in result.execution_result.task_results.iter().enumerate() {
                let status = match task.status {
                    crate::agents::TaskStatus::Completed => "✓",
                    crate::agents::TaskStatus::Failed => "✗",
                    _ => "•",
                };
                println!(
                    "  {} Task {}: [{}] {}",
                    status,
                    i + 1,
                    task.agent_name,
                    task.task_description
                );
                if let Some(error) = &task.error {
                    println!("      Error: {}", error);
                }
            }

            Ok(())
        }
        WorkflowCmd::CodeReview {
            pr_number,
            target_branch,
        } => {
            println!("Starting Code Review Workflow");
            println!("PR: #{}", pr_number);
            println!("Target Branch: {}", target_branch);
            println!("{:-<80}", "");

            let config = workflows::code_review(pr_number, target_branch);
            let result = orchestrator.execute_workflow(config).await?;

            println!("\n{}", result.summary);
            Ok(())
        }
        WorkflowCmd::Deploy {
            environment,
            version,
        } => {
            println!("Starting Deployment Workflow");
            println!("Environment: {}", environment);
            println!("Version: {}", version);
            println!("{:-<80}", "");

            let config = workflows::deployment(environment, version);
            let result = orchestrator.execute_workflow(config).await?;

            println!("\n{}", result.summary);
            Ok(())
        }
        WorkflowCmd::Test {
            component,
            test_type,
        } => {
            println!("Starting Testing Workflow");
            println!("Component: {}", component);
            println!("Test Type: {}", test_type);
            println!("{:-<80}", "");

            let config = workflows::testing(component, test_type);
            let result = orchestrator.execute_workflow(config).await?;

            println!("\n{}", result.summary);
            Ok(())
        }
        WorkflowCmd::SecurityAudit { target } => {
            println!("Starting Security Audit Workflow");
            println!("Target: {}", target);
            println!("{:-<80}", "");

            let config = workflows::security_audit(target, "full".to_string());
            let result = orchestrator.execute_workflow(config).await?;

            println!("\n{}", result.summary);
            Ok(())
        }
    }
}
