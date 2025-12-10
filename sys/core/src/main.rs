//! NOA Core - CLI Entry Point
//!
//! Main entry point for the NOA autonomous operating system.
//! §3.1: Core CLI implementation

use clap::{Parser, Subcommand};
use tracing::info;

// Modules are declared in lib.rs
// Access via crate:: paths
use crate::error::Result;

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

    /// Module registry and CAS operations
    Modules {
        #[command(subcommand)]
        command: ModuleCommands,
    },

    /// 3-Plane commands
    Plane {
        #[command(subcommand)]
        command: PlaneCommands,
    },

    /// Promotion commands
    Promotion {
        #[command(subcommand)]
        command: PromotionCommands,
    },

    /// Healing commands
    Healing {
        #[command(subcommand)]
        command: HealingCommands,
    },

    /// Model management commands
    Models {
        #[command(subcommand)]
        command: cli::models::ModelCommands,
    },

    /// Ask a question to the model
    Ask(cli::ask::AskArgs),

    /// P2P network management commands
    P2P(cli::p2p::P2PArgs),

    /// Agents command group
    Agents {
        #[command(subcommand)]
        command: AgentsCommands,
    },

    /// Tasks command group
    Tasks {
        #[command(subcommand)]
        command: TasksCommands,
    },

    /// Goal commands
    Goal {
        #[command(subcommand)]
        command: GoalCommands,
    },

    /// Logs commands
    Logs {
        #[command(subcommand)]
        command: LogsCommands,
    },

    /// Capsule commands
    Capsule {
        #[command(subcommand)]
        command: CapsuleCommands,
    },

    /// CRM commands
    Crm {
        #[command(subcommand)]
        command: CrmCommands,
    },

    /// Self-improvement lifecycle
    Improve {
        #[command(subcommand)]
        command: ImproveCommands,
    },

    /// Spec-Kit commands
    Speckit {
        #[command(subcommand)]
        command: cli::speckit::SpeckitCommands,
    },

    /// Digest pipeline commands
    Digest(cli::digest::DigestArgs),
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
    /// Enable a provider
    Enable { name: String },
    /// Disable a provider
    Disable { name: String },
    /// Test provider connectivity
    Test { name: String },
}

#[derive(Subcommand)]
enum AgentsCommands {
    /// List agents
    List,
}

#[derive(Subcommand)]
enum TasksCommands {
    /// List tasks
    List,
}

#[derive(Subcommand)]
enum GoalCommands {
    /// Submit a goal
    Submit { title: String },
}

#[derive(Subcommand)]
enum LogsCommands {
    /// Tail logs
    Tail,
}

#[derive(Subcommand)]
enum CapsuleCommands {
    /// Spawn a capsule
    Spawn { name: String },
}

#[derive(Subcommand)]
enum CrmCommands {
    /// Toggle CRM strangler mode
    Toggle { mode: String },
    /// Roll back CRM
    Rollback,
}

#[derive(Subcommand)]
enum ImproveCommands {
    /// Analyze current performance signals
    Analyze,
    /// Generate improvement proposals
    Propose,
    /// Apply improvements with safety checks
    Apply,
    /// Roll back to a prior snapshot
    Rollback { snapshot_id: Option<String> },
}

#[derive(Subcommand)]
enum ModuleCommands {
    /// List registered modules
    List,
    /// Show module details
    Info { name: String },
    /// Verify a module's stored hash
    Verify { name: String },
    /// Show dependency graph
    Deps { name: String },
}

#[derive(Subcommand)]
enum PlaneCommands {
    /// Show plane status
    Status,
    /// Switch active plane (emergency)
    Switch { name: String },
    /// Rollback to a plane
    Rollback { name: String },
}

#[derive(Subcommand)]
enum PromotionCommands {
    /// Show promotion status
    Status,
    /// Approve a promotion
    Approve { id: String },
}

#[derive(Subcommand)]
enum HealingCommands {
    /// Show healing status
    Status,
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
        Commands::Modules { command } => handle_module_command(command, cli.noa_root.clone()).await,
        Commands::Plane { command } => handle_plane_command(command).await,
        Commands::Promotion { command } => handle_promotion_command(command).await,
        Commands::Healing { command } => handle_healing_command(command).await,
        Commands::Models { command } => {
            cli::models::execute(cli::models::ModelArgs { command }, cli.noa_root.clone()).await
        }
        Commands::Ask(args) => cli::ask::execute(args, cli.noa_root.clone()).await,
        Commands::P2P(args) => {
            let db_path = std::path::PathBuf::from(
                cli.noa_root
                    .clone()
                    .unwrap_or_else(|| ".".to_string())
            ).join("data/noa.db");
            cli::p2p::execute_p2p(args, db_path).await
        }
        Commands::Agents { command } => handle_agents_command(command).await,
        Commands::Tasks { command } => handle_tasks_command(command).await,
        Commands::Goal { command } => handle_goal_command(command).await,
        Commands::Logs { command } => handle_logs_command(command).await,
        Commands::Capsule { command } => handle_capsule_command(command).await,
        Commands::Crm { command } => handle_crm_command(command).await,
        Commands::Improve { command } => handle_improve_command(command).await,
        Commands::Speckit { command } => cli::speckit::execute(cli::speckit::SpeckitArgs { command }).await,
        Commands::Digest(args) => cli::digest::execute(args).await,
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

async fn handle_module_command(command: ModuleCommands, noa_root: Option<String>) -> Result<()> {
    use cli::modules::ModuleCmd;
    match command {
        ModuleCommands::List => cli::modules::execute(ModuleCmd::List, noa_root).await,
        ModuleCommands::Info { name } => cli::modules::execute(ModuleCmd::Info { name }, noa_root).await,
        ModuleCommands::Verify { name } => cli::modules::execute(ModuleCmd::Verify { name }, noa_root).await,
        ModuleCommands::Deps { name } => cli::modules::execute(ModuleCmd::Deps { name }, noa_root).await,
    }
}

async fn handle_provider_command(command: ProviderCommands) -> Result<()> {
    match command {
        ProviderCommands::List => cli::providers::list().await,
        ProviderCommands::Status { name } => cli::providers::status(name).await,
        ProviderCommands::Enable { name } => cli::providers::enable(name).await,
        ProviderCommands::Disable { name } => cli::providers::disable(name).await,
        ProviderCommands::Test { name } => cli::providers::test(name).await,
    }
}

async fn handle_plane_command(command: PlaneCommands) -> Result<()> {
    use cli::plane::PlaneCmd;
    match command {
        PlaneCommands::Status => cli::plane::execute(PlaneCmd::Status).await,
        PlaneCommands::Switch { name } => cli::plane::execute(PlaneCmd::Switch { name }).await,
        PlaneCommands::Rollback { name } => cli::plane::execute(PlaneCmd::Rollback { name }).await,
    }
}

async fn handle_promotion_command(command: PromotionCommands) -> Result<()> {
    use cli::promotion::PromotionCmd;
    match command {
        PromotionCommands::Status => cli::promotion::execute(PromotionCmd::Status).await,
        PromotionCommands::Approve { id } => cli::promotion::execute(PromotionCmd::Approve { id }).await,
    }
}

async fn handle_healing_command(command: HealingCommands) -> Result<()> {
    use cli::healing::HealingCmd;
    match command {
        HealingCommands::Status => cli::healing::execute(HealingCmd::Status).await,
    }
}

async fn handle_agents_command(command: AgentsCommands) -> Result<()> {
    use cli::agents::AgentsCmd;
    match command {
        AgentsCommands::List => cli::agents::execute(AgentsCmd::List).await,
    }
}

async fn handle_tasks_command(command: TasksCommands) -> Result<()> {
    use cli::tasks::TasksCmd;
    match command {
        TasksCommands::List => cli::tasks::execute(TasksCmd::List).await,
    }
}

async fn handle_goal_command(command: GoalCommands) -> Result<()> {
    use cli::goal::GoalCmd;
    match command {
        GoalCommands::Submit { title } => cli::goal::execute(GoalCmd::Submit { title }).await,
    }
}

async fn handle_logs_command(command: LogsCommands) -> Result<()> {
    use cli::logs::LogsCmd;
    match command {
        LogsCommands::Tail => cli::logs::execute(LogsCmd::Tail).await,
    }
}

async fn handle_capsule_command(command: CapsuleCommands) -> Result<()> {
    use cli::capsule::CapsuleCmd;
    match command {
        CapsuleCommands::Spawn { name } => cli::capsule::execute(CapsuleCmd::Spawn { name }).await,
    }
}

async fn handle_improve_command(command: ImproveCommands) -> Result<()> {
    use cli::improve::ImproveCmd;
    match command {
        ImproveCommands::Analyze => cli::improve::execute(ImproveCmd::Analyze).await,
        ImproveCommands::Propose => cli::improve::execute(ImproveCmd::Propose).await,
        ImproveCommands::Apply => cli::improve::execute(ImproveCmd::Apply).await,
        ImproveCommands::Rollback { snapshot_id } => {
            cli::improve::execute(ImproveCmd::Rollback { snapshot_id }).await
        }
    }
}

async fn handle_crm_command(command: CrmCommands) -> Result<()> {
    use cli::crm::CrmCmd;
    match command {
        CrmCommands::Toggle { mode } => cli::crm::execute(CrmCmd::Toggle { mode }).await,
        CrmCommands::Rollback => cli::crm::execute(CrmCmd::Rollback).await,
    }
}
