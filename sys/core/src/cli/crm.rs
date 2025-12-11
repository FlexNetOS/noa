use crate::error::Result;

#[derive(Debug, Clone)]
pub enum CrmCmd {
    Toggle { mode: String },
    Rollback,
}

pub async fn execute(cmd: CrmCmd) -> Result<()> {
    match cmd {
        CrmCmd::Toggle { mode } => println!("CRM mode set to {}", mode),
        CrmCmd::Rollback => println!("CRM rollback triggered"),
    }
    Ok(())
}
