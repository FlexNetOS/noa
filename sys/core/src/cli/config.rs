//! Configuration CLI Commands
//!
//! T415: Validate configuration files against loader + validator rules.

use clap::{Args, Subcommand};
use std::path::PathBuf;
use tracing::info;

use crate::config::{ConfigLoader, ConfigValidator};
use crate::error::Result;

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Validate configuration for the current NOA root (or provided path)
    Validate {
        /// NOA root path (defaults to current directory or $NOA_ROOT)
        #[arg(short, long)]
        path: Option<String>,
    },
}

pub async fn execute(args: ConfigArgs) -> Result<()> {
    match args.command {
        ConfigCommands::Validate { path } => validate(path),
    }
}

fn validate(path: Option<String>) -> Result<()> {
    let root = path
        .or_else(|| std::env::var("NOA_ROOT").ok())
        .unwrap_or_else(|| ".".to_string());

    let noa_root = PathBuf::from(root);
    let loader = ConfigLoader::new(&noa_root);
    let config = loader.load()?;

    info!(root = %noa_root.display(), "Validating configuration");
    ConfigValidator::validate(&config)?;

    println!("Configuration valid for root: {}", noa_root.display());
    Ok(())
}
