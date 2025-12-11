use crate::error::Result;

#[derive(Debug, Clone)]
pub enum CapsuleCmd {
    Spawn { name: String },
}

pub async fn execute(cmd: CapsuleCmd) -> Result<()> {
    match cmd {
        CapsuleCmd::Spawn { name } => println!("Spawning capsule {}", name),
    }
    Ok(())
}
