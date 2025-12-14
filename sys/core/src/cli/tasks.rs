use crate::error::Result;

#[derive(Debug, Clone)]
pub enum TasksCmd {
    List,
}

pub async fn execute(command: TasksCmd) -> Result<()> {
    match command {
        TasksCmd::List => println!("tasks: []"),
    }
    Ok(())
}
