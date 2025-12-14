use crate::error::Result;

#[derive(Debug, Clone)]
pub enum HealingCmd {
    Status,
}

pub async fn execute(cmd: HealingCmd) -> Result<()> {
    match cmd {
        HealingCmd::Status => {
            println!("Healing status: no active incidents");
        }
    }
    Ok(())
}
