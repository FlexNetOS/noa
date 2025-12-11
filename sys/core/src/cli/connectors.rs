use clap::{Args, Subcommand};
use serde_json::json;

use crate::connectors::base::load_context;
use crate::connectors::status::print_table;
use crate::connectors::{collect_states, default_connector_ids};
use crate::error::Result;

/// Arguments for connector commands
#[derive(Args, Debug)]
pub struct ConnectorArgs {
    #[command(subcommand)]
    pub command: ConnectorCmd,
}

/// Connector subcommands
#[derive(Subcommand, Debug)]
pub enum ConnectorCmd {
    /// Show connector status
    Status {
        /// Output format (text|json)
        #[arg(long, default_value = "text")]
        format: String,
    },
    /// List known connector identifiers
    List,
}

/// Execute connector commands
pub async fn execute(args: ConnectorArgs) -> Result<()> {
    match args.command {
        ConnectorCmd::Status { format } => {
            let ctx = load_context()?;
            let states = collect_states(&ctx).await?;
            if format == "json" {
                let payload = json!({
                    "connectors": states.iter().map(|s| {
                        json!({
                            "id": s.name,
                            "health": format!("{:?}", s.health),
                            "last_checked": s.last_checked.to_rfc3339(),
                            "message": s.message,
                        })
                    }).collect::<Vec<_>>()
                });
                println!("{}", serde_json::to_string_pretty(&payload).unwrap());
            } else {
                print_table(&states);
            }
        }
        ConnectorCmd::List => {
            for id in default_connector_ids() {
                println!("{}", id);
            }
        }
    }

    Ok(())
}
