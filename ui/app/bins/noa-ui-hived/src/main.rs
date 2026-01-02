//! NOA Hive Daemon (noa-hived)
//!
//! Background daemon for P2P coordination, state synchronization,
//! and distributed agent communication.
//!
//! # Features
//!
//! - P2P swarm management (libp2p)
//! - CRDT state synchronization (loro)
//! - Local gRPC/HTTP API
//! - Agent sandbox coordination
//!
//! # Usage
//!
//! ```bash
//! noa-hived --help
//! noa-hived start --port 9999
//! noa-hived status
//! ```

mod cli;
mod config;
mod server;
mod state;

#[cfg(test)]
mod tests;

use anyhow::Result;
use clap::Parser;
use tracing::{info, error};
use tracing_subscriber::{fmt, EnvFilter};

use cli::{Cli, Command};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .with_target(true)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Command::Start { port, data_dir } => {
            info!("Starting noa-hived on port {}", port);
            
            // Initialize configuration
            let config = config::DaemonConfig::new(port, data_dir)?;
            
            // Initialize state manager
            let state = state::StateManager::new(&config)?;
            
            // Start the HTTP/gRPC server
            server::run(config, state).await?;
        }
        Command::Status => {
            // Check if daemon is running
            match reqwest::get("http://127.0.0.1:9999/health").await {
                Ok(resp) if resp.status().is_success() => {
                    println!("✓ noa-hived is running");
                    let body: serde_json::Value = resp.json().await?;
                    println!("  Version: {}", body.get("version").unwrap_or(&serde_json::json!("unknown")));
                }
                _ => {
                    println!("✗ noa-hived is not running");
                    std::process::exit(1);
                }
            }
        }
        Command::Stop => {
            // Send shutdown signal
            match reqwest::Client::new()
                .post("http://127.0.0.1:9999/shutdown")
                .send()
                .await
            {
                Ok(_) => println!("✓ Shutdown signal sent"),
                Err(e) => {
                    error!("Failed to stop daemon: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
