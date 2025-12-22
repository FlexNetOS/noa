use crate::error::Result;
use crate::services::AgentService;
use crate::db::ConnectionPool;

#[derive(Debug, Clone)]
pub enum AgentsCmd {
    List,
    Logs { agent_name: Option<String> },
}

pub async fn execute(command: AgentsCmd, pool: Option<ConnectionPool>) -> Result<()> {
    match command {
        AgentsCmd::List => {
            if let Some(pool) = pool {
                let service = AgentService::new(pool);
                let agents = service.list()?;
                
                println!("Available Agents");
                println!("{:-<60}", "");
                if agents.is_empty() {
                    println!("No agents registered");
                } else {
                    for agent in agents {
                        println!("  • {}", agent);
                    }
                }
            } else {
                // Fallback to hardcoded list
                println!("Available Agents (built-in)");
                println!("{:-<60}", "");
                println!("  • commander-chief   - Executive orchestrator");
                println!("  • file-io          - File operations agent");
                println!("  • terminal         - Terminal command agent");
                println!("  • rag              - Retrieval-augmented generation");
                println!("  • model-selector   - Model selection agent");
            }
            Ok(())
        }
        AgentsCmd::Logs { agent_name } => {
            if let Some(_pool) = pool {
                if let Some(name) = agent_name {
                    println!("Logs for agent: {}", name);
                    println!("(Log viewing implementation pending)");
                } else {
                    println!("Please specify an agent name");
                }
            } else {
                println!("Database connection not available");
            }
            Ok(())
        }
    }
}
