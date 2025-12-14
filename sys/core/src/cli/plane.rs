use crate::error::Result;

#[derive(Debug, Clone)]
pub enum PlaneCmd {
    Status,
    Switch { name: String },
    Rollback { name: String },
}

pub async fn execute(cmd: PlaneCmd) -> Result<()> {
    match cmd {
        PlaneCmd::Status => {
            println!("Plane status: sandbox=healthy, deployed=standby, coordinator=active");
        }
        PlaneCmd::Switch { name } => {
            println!("Switching active plane to {}", name);
        }
        PlaneCmd::Rollback { name } => {
            println!("Rolling back plane {}", name);
        }
    }
    Ok(())
}
