use crate::error::Result;

#[derive(Debug, Clone)]
pub enum GoalCmd {
    Submit { title: String },
}

pub async fn execute(command: GoalCmd) -> Result<()> {
    match command {
        GoalCmd::Submit { title } => {
            println!("Goal submitted: {}", title);
        }
    }
    Ok(())
}
