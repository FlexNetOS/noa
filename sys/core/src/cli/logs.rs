use crate::error::Result;

#[derive(Debug, Clone)]
pub enum LogsCmd {
    Tail,
}

pub async fn execute(cmd: LogsCmd) -> Result<()> {
    match cmd {
        LogsCmd::Tail => println!("Tailing logs... (stub)"),
    }
    Ok(())
}
