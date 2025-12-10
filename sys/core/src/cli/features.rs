use clap::{Args, Subcommand};

use crate::error::Result;
use crate::features::flags::FeatureFlagStore;

/// Arguments for feature flag commands
#[derive(Args, Debug)]
pub struct FeatureArgs {
    #[command(subcommand)]
    pub command: FeatureCmd,
}

/// Feature flag subcommands
#[derive(Subcommand, Debug)]
pub enum FeatureCmd {
    /// List all feature flags
    List,
    /// Show the status of a specific flag
    Status { name: Option<String> },
    /// Enable a feature flag
    Enable { name: String },
    /// Disable a feature flag
    Disable { name: String },
}

/// Execute feature flag commands
pub async fn execute(args: FeatureArgs) -> Result<()> {
    let mut store = FeatureFlagStore::load(None)?;

    match args.command {
        FeatureCmd::List => {
            println!("{:<32} {:<8} {}", "name", "enabled", "description");
            for flag in store.list() {
                println!(
                    "{:<32} {:<8} {}",
                    flag.name,
                    if flag.enabled { "true" } else { "false" },
                    flag.description.clone().unwrap_or_default()
                );
            }
        }
        FeatureCmd::Status { name } => {
            if let Some(name) = name {
                let enabled = store.is_enabled(&name);
                println!("{} => {}", name, if enabled { "enabled" } else { "disabled" });
            } else {
                println!("{:<32} {:<8}", "name", "enabled");
                for flag in store.list() {
                    println!(
                        "{:<32} {:<8}",
                        flag.name,
                        if flag.enabled { "true" } else { "false" }
                    );
                }
            }
        }
        FeatureCmd::Enable { name } => {
            store.set(&name, true)?;
            println!("enabled {}", name);
        }
        FeatureCmd::Disable { name } => {
            store.set(&name, false)?;
            println!("disabled {}", name);
        }
    }

    Ok(())
}
