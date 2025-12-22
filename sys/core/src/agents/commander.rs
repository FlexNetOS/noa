use crate::agents::base::BaseAgent;
use crate::error::Result;


/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Assigned,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Agent task assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    pub id: Uuid,
    pub description: String,
    pub agent_name: String,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub result: Option<String>,
    pub error: Option<String>,
}

/// Execution plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub id: Uuid,
    pub goal: String,
    pub tasks: Vec<AgentTask>,
    pub created_at: DateTime<Utc>,
    pub estimated_duration_secs: Option<u64>,
}

/// Commander task request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommanderRequest {
    pub goal: String,
    pub context: Option<HashMap<String, String>>,
    pub constraints: Option<Vec<String>>,
}

/// Commander Chief Agent - Executive Orchestrator
pub struct CommanderChiefAgent {
    available_agents: Vec<String>,
    task_history: Vec<AgentTask>,
}

impl CommanderChiefAgent {
    pub fn new() -> Self {
        Self {
            available_agents: vec![
                "file-io".into(),
                "terminal".into(),
                "rag".into(),
                "model-selector".into(),
            ],
            task_history: Vec::new(),
        }
    }

    /// Create commander with specific agents
    pub fn with_agents(agents: Vec<String>) -> Self {
        Self {
            available_agents: agents,
            task_history: Vec::new(),
        }
    }

    /// Decompose a high-level goal into agent tasks
    pub fn plan_execution(&self, request: CommanderRequest) -> Result<ExecutionPlan> {
        let tasks = self.decompose_goal(&request.goal, request.constraints.as_deref())?;
        let task_count = tasks.len();
        
        Ok(ExecutionPlan {
            id: Uuid::new_v4(),
            goal: request.goal,
            tasks,
            created_at: Utc::now(),
            estimated_duration_secs: Some(task_count as u64 * 30), // Rough estimate
        })
    }

    /// Decompose goal into agent tasks (rule-based strategy)
    fn decompose_goal(&self, goal: &str, _constraints: Option<&[String]>) -> Result<Vec<AgentTask>> {
        let mut tasks = Vec::new();
        let goal_lower = goal.to_lowercase();

        // File operations
        if goal_lower.contains("read") || goal_lower.contains("write") || goal_lower.contains("file") {
            tasks.push(AgentTask {
                id: Uuid::new_v4(),
                description: format!("File operation: {}", goal),
                agent_name: "file-io".into(),
                priority: TaskPriority::Medium,
                status: TaskStatus::Pending,
                created_at: Utc::now(),
                started_at: None,
                completed_at: None,
                result: None,
                error: None,
            });
        }

        // Shell/terminal operations
        if goal_lower.contains("run") || goal_lower.contains("execute") || goal_lower.contains("command") {
            tasks.push(AgentTask {
                id: Uuid::new_v4(),
                description: format!("Execute command: {}", goal),
                agent_name: "terminal".into(),
                priority: TaskPriority::Medium,
                status: TaskStatus::Pending,
                created_at: Utc::now(),
                started_at: None,
                completed_at: None,
                result: None,
                error: None,
            });
        }

        // Knowledge retrieval
        if goal_lower.contains("search") || goal_lower.contains("find") || goal_lower.contains("retrieve") {
            tasks.push(AgentTask {
                id: Uuid::new_v4(),
                description: format!("Search knowledge: {}", goal),
                agent_name: "rag".into(),
                priority: TaskPriority::High,
                status: TaskStatus::Pending,
                created_at: Utc::now(),
                started_at: None,
                completed_at: None,
                result: None,
                error: None,
            });
        }

        // Complex multi-step goals
        if goal_lower.contains("analyze") || goal_lower.contains("process") {
            // Step 1: Gather information (RAG)
            tasks.push(AgentTask {
                id: Uuid::new_v4(),
                description: "Gather relevant context".into(),
                agent_name: "rag".into(),
                priority: TaskPriority::High,
                status: TaskStatus::Pending,
                created_at: Utc::now(),
                started_at: None,
                completed_at: None,
                result: None,
                error: None,
            });

            // Step 2: Process/analyze
            tasks.push(AgentTask {
                id: Uuid::new_v4(),
                description: format!("Analyze: {}", goal),
                agent_name: "model-selector".into(),
                priority: TaskPriority::High,
                status: TaskStatus::Pending,
                created_at: Utc::now(),
                started_at: None,
                completed_at: None,
                result: None,
                error: None,
            });
        }

        // If no specific tasks identified, create a generic one
        if tasks.is_empty() {
            tasks.push(AgentTask {
                id: Uuid::new_v4(),
                description: goal.to_string(),
                agent_name: "model-selector".into(),
                priority: TaskPriority::Medium,
                status: TaskStatus::Pending,
                created_at: Utc::now(),
                started_at: None,
                completed_at: None,
                result: None,
                error: None,
            });
        }

        Ok(tasks)
    }

    /// Assign task to an agent
    pub fn assign_task(&mut self, _task_id: Uuid, agent: &str) -> Result<()> {
        if !self.available_agents.contains(&agent.to_string()) {
            return Err(NoaError::Validation(crate::error::ValidationError::new(
                "agent",
                "Agent not available",
                "AGENT_NOT_AVAILABLE",
            )));
        }

        // In a real implementation, this would update task state in the database
        Ok(())
    }

    /// Get task history
    pub fn get_task_history(&self) -> &[AgentTask] {
        &self.task_history
    }

    /// Get available agents
    pub fn get_available_agents(&self) -> &[String] {
        &self.available_agents
    }

    /// Coordinate execution of a plan (simplified)
    pub fn coordinate_execution(&self, plan: &ExecutionPlan) -> Result<String> {
        let mut summary = format!("Execution Plan for: {}\n", plan.goal);
        summary.push_str(&format!("Total tasks: {}\n", plan.tasks.len()));
        summary.push_str("---\n");

        for (i, task) in plan.tasks.iter().enumerate() {
            summary.push_str(&format!(
                "{}. [{}] {} - {:?}\n",
                i + 1,
                task.agent_name,
                task.description,
                task.priority
            ));
        }

        Ok(summary)
    }
}

impl Default for CommanderChiefAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseAgent for CommanderChiefAgent {
    fn name(&self) -> &str {
        "commander-chief"
    }

    fn description(&self) -> &str {
        "Coordinates micro agent stacks"
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["coordinate".into(), "delegate".into()]
    }

    fn execute(&self, task: &str) -> Result<String> {
        Ok(format!("CommanderChief delegated '{}'", task))
    }
}
