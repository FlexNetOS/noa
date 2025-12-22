use crate::error::Result;
use crate::services::AgentService;
use crate::db::ConnectionPool;
use crate::agents::{
    CommanderChiefAgent, FileIOAgent, TerminalAgent, RAGAgent,
    base::BaseAgent,
};

#[derive(Debug, Clone)]
pub enum AgentsCmd {
    List,
    Info { agent_name: String },
    Run { agent_name: String, task: String },
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
                // Fallback to hardcoded list with descriptions
                println!("Available Agents (built-in)");
                println!("{:-<80}", "");
                
                let commander = CommanderChiefAgent::new();
                println!("  • {} - {}", commander.name(), commander.description());
                
                let file_io = FileIOAgent::new();
                println!("  • {} - {}", file_io.name(), file_io.description());
                
                let terminal = TerminalAgent::new();
                println!("  • {} - {}", terminal.name(), terminal.description());
                
                let rag = RAGAgent::new();
                println!("  • {} - {}", rag.name(), rag.description());
            }
            Ok(())
        }
        AgentsCmd::Info { agent_name } => {
            println!("Agent Information: {}", agent_name);
            println!("{:-<80}", "");
            
            let agent: Box<dyn BaseAgent> = match agent_name.as_str() {
                "commander-chief" => Box::new(CommanderChiefAgent::new()),
                "file-io" => Box::new(FileIOAgent::new()),
                "terminal" => Box::new(TerminalAgent::new()),
                "rag" => Box::new(RAGAgent::new()),
                _ => {
                    println!("Unknown agent: {}", agent_name);
                    return Ok(());
                }
            };
            
            println!("Name: {}", agent.name());
            println!("Description: {}", agent.description());
            println!("Capabilities:");
            for cap in agent.capabilities() {
                println!("  - {}", cap);
            }
            
            Ok(())
        }
        AgentsCmd::Run { agent_name, task } => {
            println!("Executing agent: {}", agent_name);
            println!("Task: {}", task);
            println!("{:-<80}", "");
            
            let agent: Box<dyn BaseAgent> = match agent_name.as_str() {
                "commander-chief" => Box::new(CommanderChiefAgent::new()),
                "file-io" => Box::new(FileIOAgent::new()),
                "terminal" => Box::new(TerminalAgent::new()),
                "rag" => Box::new(RAGAgent::new()),
                _ => {
                    println!("Error: Unknown agent '{}'", agent_name);
                    println!("Available agents: commander-chief, file-io, terminal, rag");
                    return Ok(());
                }
            };
            
            match agent.execute(&task) {
                Ok(result) => {
                    println!("\nResult:");
                    println!("{}", result);
                    println!("\n✓ Agent execution completed successfully");
                }
                Err(e) => {
                    println!("\n✗ Agent execution failed:");
                    println!("Error: {}", e);
                }
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
