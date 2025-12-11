use crate::error::Result;

#[derive(Debug, Clone)]
pub enum AgentsCmd {
    List,
}

pub async fn execute(command: AgentsCmd) -> Result<()> {
    match command {
        AgentsCmd::List => {
            println!("agents: [commander-chief, file-io, terminal, rag]");
        }
    }
    Ok(())
}
