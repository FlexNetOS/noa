use crate::error::Result;

#[derive(Debug, Clone)]
pub enum PromotionCmd {
    Status,
    Approve { id: String },
}

pub async fn execute(cmd: PromotionCmd) -> Result<()> {
    match cmd {
        PromotionCmd::Status => {
            println!("Promotion status: none pending");
        }
        PromotionCmd::Approve { id } => {
            println!("Approved promotion {}", id);
        }
    }
    Ok(())
}
