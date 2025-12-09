//! NOA Core - CLI Entry Point
//!
//! Main entry point for the NOA autonomous operating system.
//! §3.1: Core CLI implementation

use clap::{Parser, Subcommand};
use tracing::info;

mod api;
mod cli;
mod config;
mod db;
mod error;
mod logging;
mod observability;

use error::Result;

/// NOA - Autonomous Agentic Operating System
#[derive(Parser)]
#[command(name = "noa")]
#[command(author = "NOA Project")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Autonomous Agentic Operating System", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Configuration file path
    #[arg(short, long, global = true, default_value = "config/noa.yaml")]
    config: String,

    /// NOA root directory
    #[arg(long, global = true, env = "NOA_ROOT")]
    noa_root: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new NOA installation
    Init(cli::init::InitArgs),

    /// Start NOA services
    Start(cli::start::StartArgs),

    /// Show NOA status
    Status(cli::status::StatusArgs),

    /// Stop NOA services
    Stop(cli::stop::StopArgs),

    /// Database management commands
    Db {
        #[command(subcommand)]
        command: cli::db::DbCommands,
    },

    /// Agent management commands
    Agent {
        #[command(subcommand)]
        command: AgentCommands,
    },

    /// Provider management commands
    Provider {
        #[command(subcommand)]
        command: ProviderCommands,
    },
}

#[derive(Subcommand)]
enum AgentCommands {
    /// List all agents
    List,
    /// Show agent details
    Info { name: String },
    /// Start an agent
    Start { name: String },
    /// Stop an agent
    Stop { name: String },
}

#[derive(Subcommand)]
enum ProviderCommands {
    /// List configured providers
    List,
    /// Show provider status
    Status { name: String },
    /// Test provider connectivity
    Test { name: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    let log_config = config::LoggingConfig {
        level: log_level.parse().unwrap_or_default(),
        format: config::LogFormat::Pretty,
        output: std::path::PathBuf::from("logs/noa.log"),
        rotate: true,
        max_size_mb: 100,
        max_files: 10,
    };
    logging::init(&log_config)?;

    info!(
        version = env!("CARGO_PKG_VERSION"),
        "NOA starting"
    );

    // Execute command
    match cli.command {
        Commands::Init(args) => cli::init::execute(args).await,
        Commands::Start(args) => cli::start::execute(args).await,
        Commands::Status(args) => cli::status::execute(args).await,
        Commands::Stop(args) => cli::stop::execute(args).await,
        Commands::Db { command } => cli::db::execute(command).await,
        Commands::Agent { command } => handle_agent_command(command).await,
        Commands::Provider { command } => handle_provider_command(command).await,
    }
}

async fn handle_agent_command(command: AgentCommands) -> Result<()> {
    match command {
        AgentCommands::List => {
            println!("Listing agents...");
            // TODO: Implement agent listing
            Ok(())
        }
        AgentCommands::Info { name } => {
            println!("Agent info: {}", name);
            // TODO: Implement agent info
            Ok(())
        }
        AgentCommands::Start { name } => {
            println!("Starting agent: {}", name);
            // TODO: Implement agent start
            Ok(())
        }
        AgentCommands::Stop { name } => {
            println!("Stopping agent: {}", name);
            // TODO: Implement agent stop
            Ok(())
        }
    }
}

async fn handle_provider_command(command: ProviderCommands) -> Result<()> {
    match command {
        ProviderCommands::List => {
            println!("Listing providers...");
            // TODO: Implement provider listing
            Ok(())
        }
        ProviderCommands::Status { name } => {
            println!("Provider status: {}", name);
            // TODO: Implement provider status
            Ok(())
        }
        ProviderCommands::Test { name } => {
            println!("Testing provider: {}", name);
            // TODO: Implement provider test
            Ok(())
        }
    }
}

