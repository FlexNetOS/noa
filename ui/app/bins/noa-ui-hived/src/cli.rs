//! CLI argument parsing for noa-hived.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// NOA Hive Daemon - P2P coordination and state sync service.
#[derive(Parser, Debug)]
#[command(name = "noa-hived")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

/// Daemon commands.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start the daemon.
    Start {
        /// Port to listen on.
        #[arg(short, long, default_value = "9999", env = "NOA_HIVED_PORT")]
        port: u16,

        /// Data directory for state storage.
        #[arg(short, long, env = "NOA_HIVED_DATA_DIR")]
        data_dir: Option<PathBuf>,
    },

    /// Check daemon status.
    Status,

    /// Stop the daemon.
    Stop,
}
