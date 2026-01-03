//! NOA Core - CLI Entry Point
//!
//! Main entry point for the NOA autonomous operating system.
//! §3.1: Core CLI implementation

use clap::{Parser, Subcommand};
use tracing::info;

use noa_core::cli;
use noa_core::config;
use noa_core::error::Result;
use noa_core::logging;

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

    /// Database management commands
    Db {
        #[command(subcommand)]
        command: cli::db::DbCommands,
    },

    #[cfg(feature = "full")]
    /// Start NOA services
    Start(cli::start::StartArgs),

    #[cfg(feature = "full")]
    /// Show NOA status
    Status(cli::status::StatusArgs),

    #[cfg(feature = "full")]
    /// Stop NOA services
    Stop(cli::stop::StopArgs),

    #[cfg(feature = "full")]
    /// Agent management commands
    Agent {
        #[command(subcommand)]
        command: AgentCommands,
    },

    #[cfg(feature = "full")]
    /// Provider management commands
    Provider {
        #[command(subcommand)]
        command: ProviderCommands,
    },

    #[cfg(feature = "full")]
    /// Module registry and CAS operations
    Modules {
        #[command(subcommand)]
        command: ModuleCommands,
    },

    #[cfg(feature = "full")]
    /// Virtual package detection (conda)
    VirtualPackages {
        #[command(subcommand)]
        command: VirtualPackagesCommands,
    },

    #[cfg(feature = "full")]
    /// 3-Plane commands
    Plane {
        #[command(subcommand)]
        command: PlaneCommands,
    },

    #[cfg(feature = "full")]
    /// Promotion commands
    Promotion {
        #[command(subcommand)]
        command: PromotionCommands,
    },

    #[cfg(feature = "full")]
    /// Healing commands
    Healing {
        #[command(subcommand)]
        command: HealingCommands,
    },

    #[cfg(feature = "full")]
    /// Model management commands
    Models {
        #[command(subcommand)]
        command: cli::models::ModelCommands,
    },

    #[cfg(feature = "full")]
    /// Ask a question to the model
    Ask(cli::ask::AskArgs),

    #[cfg(feature = "full")]
    /// P2P network management commands
    P2P(cli::p2p::P2PArgs),

    #[cfg(feature = "full")]
    /// Agents command group
    Agents {
        #[command(subcommand)]
        command: AgentsCommands,
    },

    #[cfg(feature = "full")]
    /// Tasks command group
    Tasks {
        #[command(subcommand)]
        command: TasksCommands,
    },

    #[cfg(feature = "full")]
    /// Goal commands
    Goal {
        #[command(subcommand)]
        command: GoalCommands,
    },

    #[cfg(feature = "full")]
    /// Logs commands
    Logs {
        #[command(subcommand)]
        command: LogsCommands,
    },

    #[cfg(feature = "full")]
    /// Capsule commands
    Capsule {
        #[command(subcommand)]
        command: CapsuleCommands,
    },

    #[cfg(feature = "full")]
    /// CRM commands
    Crm {
        #[command(subcommand)]
        command: CrmCommands,
    },

    #[cfg(feature = "full")]
    /// Wiki documentation generation commands
    Wiki {
        #[command(subcommand)]
        command: WikiCommands,
    },
}

#[cfg(feature = "full")]
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

#[cfg(feature = "full")]
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

#[cfg(feature = "full")]
#[derive(Subcommand)]
enum AgentsCommands {
    /// List agents
    List,
    /// Show agent information
    Info {
        /// Agent name
        agent_name: String,
    },
    /// Run an agent with a task
    Run {
        /// Agent name (commander-chief, file-io, terminal, rag)
        agent_name: String,
        /// Task specification (can be JSON or simple string)
        task: String,
    },
    /// Show agent logs
    Logs {
        /// Agent name (optional)
        agent_name: Option<String>,
    },
}

#[cfg(feature = "full")]
#[derive(Subcommand)]
enum TasksCommands {
    /// List tasks
    List,
}

#[cfg(feature = "full")]
#[derive(Subcommand)]
enum GoalCommands {
    /// Submit a goal
    Submit { title: String },
}

#[cfg(feature = "full")]
#[derive(Subcommand)]
enum LogsCommands {
    /// Tail logs
    Tail,
}

#[cfg(feature = "full")]
#[derive(Subcommand)]
enum CapsuleCommands {
    /// Spawn a capsule
    Spawn { name: String },
}

#[cfg(feature = "full")]
#[derive(Subcommand)]
enum CrmCommands {
    /// Toggle CRM strangler mode
    Toggle { mode: String },
    /// Roll back CRM
    Rollback,
}

#[cfg(feature = "full")]
#[derive(Subcommand)]
enum WikiCommands {
    /// Generate full documentation from scratch
    GenerateFull {
        /// Force regeneration even if up to date
        #[arg(short, long)]
        force: bool,
        /// Run in sequential mode (ignore adaptive)
        #[arg(short, long)]
        sequential: bool,
    },
    /// Generate documentation for changed files only
    GenerateIncremental {
        /// Git ref to compare against (default: HEAD~1)
        #[arg(short, long)]
        since: Option<String>,
    },
    /// Show generation status
    Status,
    /// Cancel ongoing generation
    Cancel,
    /// List manual edit sections
    ManualEdits,
    /// Validate generated documentation
    Validate,
}

#[cfg(feature = "full")]
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

#[cfg(feature = "full")]
#[derive(Subcommand)]
enum PlaneCommands {
    /// Show plane status
    Status,
    /// Switch active plane (emergency)
    Switch { name: String },
    /// Rollback to a plane
    Rollback { name: String },
}

#[cfg(feature = "full")]
#[derive(Subcommand)]
enum PromotionCommands {
    /// Show promotion status
    Status,
    /// Approve a promotion
    Approve { id: String },
}

#[cfg(feature = "full")]
#[derive(Subcommand)]
enum HealingCommands {
    /// Show healing status
    Status,
    /// List healing events
    Events,
}

#[cfg(feature = "full")]
#[derive(Subcommand)]
enum VirtualPackagesCommands {
    /// Print detected conda virtual packages as JSON
    Detect,
    /// Register detected conda virtual packages into the module registry
    Register,
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
        Commands::Db { command } => cli::db::execute(command).await,

        #[cfg(feature = "full")]
        Commands::Start(args) => cli::start::execute(args).await,
        #[cfg(feature = "full")]
        Commands::Status(args) => cli::status::execute(args).await,
        #[cfg(feature = "full")]
        Commands::Stop(args) => cli::stop::execute(args).await,
        #[cfg(feature = "full")]
        Commands::Agent { command } => handle_agent_command(command).await,
        #[cfg(feature = "full")]
        Commands::Provider { command } => handle_provider_command(command).await,
        #[cfg(feature = "full")]
        Commands::Modules { command } => handle_module_command(command, cli.noa_root.clone()).await,
        #[cfg(feature = "full")]
        Commands::VirtualPackages { command } => {
            handle_virtual_packages_command(command, cli.noa_root.clone()).await
        }
        #[cfg(feature = "full")]
        Commands::Plane { command } => handle_plane_command(command).await,
        #[cfg(feature = "full")]
        Commands::Promotion { command } => handle_promotion_command(command).await,
        #[cfg(feature = "full")]
        Commands::Healing { command } => handle_healing_command(command).await,
        #[cfg(feature = "full")]
        Commands::Models { command } => {
            cli::models::execute(cli::models::ModelArgs { command }, cli.noa_root.clone()).await
        }
        #[cfg(feature = "full")]
        Commands::Ask(args) => cli::ask::execute(args, cli.noa_root.clone()).await,
        #[cfg(feature = "full")]
        Commands::P2P(args) => {
            let db_path = cli
                .noa_root
                .clone()
                .map(|r| std::path::PathBuf::from(r).join("data").join("noa.db"))
                .unwrap_or_else(|| std::path::PathBuf::from("data").join("noa.db"));

            cli::p2p::execute_p2p(args, db_path).await
        }
        #[cfg(feature = "full")]
        Commands::Agents { command } => handle_agents_command(command).await,
        #[cfg(feature = "full")]
        Commands::Tasks { command } => handle_tasks_command(command).await,
        #[cfg(feature = "full")]
        Commands::Goal { command } => handle_goal_command(command).await,
        #[cfg(feature = "full")]
        Commands::Logs { command } => handle_logs_command(command).await,
        #[cfg(feature = "full")]
        Commands::Capsule { command } => handle_capsule_command(command).await,
        #[cfg(feature = "full")]
        Commands::Crm { command } => handle_crm_command(command).await,
        #[cfg(feature = "full")]
        Commands::Wiki { command } => handle_wiki_command(command).await,
    }
}

#[cfg(feature = "full")]
async fn handle_agent_command(command: AgentCommands) -> Result<()> {
    match command {
        AgentCommands::List => {
            println!("Listing agents...");
            Ok(())
        }
        AgentCommands::Info { name } => {
            println!("Agent info: {}", name);
            Ok(())
        }
        AgentCommands::Start { name } => {
            println!("Starting agent: {}", name);
            Ok(())
        }
        AgentCommands::Stop { name } => {
            println!("Stopping agent: {}", name);
            Ok(())
        }
    }
}

#[cfg(feature = "full")]
async fn handle_module_command(command: ModuleCommands, noa_root: Option<String>) -> Result<()> {
    use cli::modules::ModuleCmd;
    match command {
        ModuleCommands::List => cli::modules::execute(ModuleCmd::List, noa_root).await,
        ModuleCommands::Info { name } => cli::modules::execute(ModuleCmd::Info { name }, noa_root).await,
        ModuleCommands::Verify { name } => cli::modules::execute(ModuleCmd::Verify { name }, noa_root).await,
        ModuleCommands::Deps { name } => cli::modules::execute(ModuleCmd::Deps { name }, noa_root).await,
    }
}

#[cfg(feature = "full")]
async fn handle_provider_command(command: ProviderCommands) -> Result<()> {
    match command {
        ProviderCommands::List => cli::providers::list().await,
        ProviderCommands::Status { name } => cli::providers::status(name).await,
        ProviderCommands::Enable { name } => cli::providers::enable(name).await,
        ProviderCommands::Disable { name } => cli::providers::disable(name).await,
        ProviderCommands::Test { name } => cli::providers::test(name).await,
    }
}

#[cfg(feature = "full")]
async fn handle_plane_command(command: PlaneCommands) -> Result<()> {
    use cli::plane::PlaneCmd;
    match command {
        PlaneCommands::Status => cli::plane::execute(PlaneCmd::Status).await,
        PlaneCommands::Switch { name } => cli::plane::execute(PlaneCmd::Switch { name }).await,
        PlaneCommands::Rollback { name } => cli::plane::execute(PlaneCmd::Rollback { name }).await,
    }
}

#[cfg(feature = "full")]
async fn handle_promotion_command(command: PromotionCommands) -> Result<()> {
    use cli::promotion::PromotionCmd;
    match command {
        PromotionCommands::Status => cli::promotion::execute(PromotionCmd::Status).await,
        PromotionCommands::Approve { id } => cli::promotion::execute(PromotionCmd::Approve { id }).await,
    }
}

#[cfg(feature = "full")]
async fn handle_healing_command(command: HealingCommands) -> Result<()> {
    use cli::healing::HealingCmd;
    match command {
        HealingCommands::Status => cli::healing::execute(HealingCmd::Status, None).await,
        HealingCommands::Events => cli::healing::execute(HealingCmd::Events, None).await,
    }
}

#[cfg(feature = "full")]
async fn handle_agents_command(command: AgentsCommands) -> Result<()> {
    use cli::agents::AgentsCmd;
    match command {
        AgentsCommands::List => cli::agents::execute(AgentsCmd::List, None).await,
        AgentsCommands::Info { agent_name } => cli::agents::execute(AgentsCmd::Info { agent_name }, None).await,
        AgentsCommands::Run { agent_name, task } => cli::agents::execute(AgentsCmd::Run { agent_name, task }, None).await,
        AgentsCommands::Logs { agent_name } => cli::agents::execute(AgentsCmd::Logs { agent_name }, None).await,
    }
}

#[cfg(feature = "full")]
async fn handle_tasks_command(command: TasksCommands) -> Result<()> {
    use cli::tasks::TasksCmd;
    match command {
        TasksCommands::List => cli::tasks::execute(TasksCmd::List).await,
    }
}

#[cfg(feature = "full")]
async fn handle_goal_command(command: GoalCommands) -> Result<()> {
    use cli::goal::GoalCmd;
    match command {
        GoalCommands::Submit { title } => cli::goal::execute(GoalCmd::Submit { title }).await,
    }
}

#[cfg(feature = "full")]
async fn handle_logs_command(command: LogsCommands) -> Result<()> {
    use cli::logs::LogsCmd;
    match command {
        LogsCommands::Tail => cli::logs::execute(LogsCmd::Tail).await,
    }
}

#[cfg(feature = "full")]
async fn handle_capsule_command(command: CapsuleCommands) -> Result<()> {
    use cli::capsule::CapsuleCmd;
    match command {
        CapsuleCommands::Spawn { name } => cli::capsule::execute(CapsuleCmd::Spawn { name }).await,
    }
}

#[cfg(feature = "full")]
async fn handle_crm_command(command: CrmCommands) -> Result<()> {
    use cli::crm::CrmCmd;
    match command {
        CrmCommands::Toggle { mode } => cli::crm::execute(CrmCmd::Toggle { mode }).await,
        CrmCommands::Rollback => cli::crm::execute(CrmCmd::Rollback).await,
    }
}

#[cfg(feature = "full")]
async fn handle_wiki_command(command: WikiCommands) -> Result<()> {
    use cli::wiki::WikiCmd;
    match command {
        WikiCommands::GenerateFull { force, sequential } => {
            cli::wiki::execute(WikiCmd::GenerateFull { force, sequential }).await
        }
        WikiCommands::GenerateIncremental { since } => {
            cli::wiki::execute(WikiCmd::GenerateIncremental { since }).await
        }
        WikiCommands::Status => cli::wiki::execute(WikiCmd::Status).await,
        WikiCommands::Cancel => cli::wiki::execute(WikiCmd::Cancel).await,
        WikiCommands::ManualEdits => cli::wiki::execute(WikiCmd::ManualEdits).await,
        WikiCommands::Validate => cli::wiki::execute(WikiCmd::Validate).await,
    }
}

#[cfg(feature = "full")]
async fn handle_virtual_packages_command(command: VirtualPackagesCommands, noa_root: Option<String>) -> Result<()> {
    use std::path::PathBuf;

    match command {
        VirtualPackagesCommands::Detect => {
            let json = noa_core::virtual_packages::detect_report_json()?;
            println!("{}", json);
            Ok(())
        }
        VirtualPackagesCommands::Register => {
            let root = PathBuf::from(noa_root.unwrap_or_else(|| ".".into()));
            let registry_path = root.join("data/modules/registry/registry.db");

            let registry = noa_core::modules::registry::ModuleRegistry::new(&registry_path)?;
            let meta = noa_core::virtual_packages::detect_as_module_metadata()?;
            registry.register(&meta)?;

            println!("Registered module: {} ({})", meta.name, meta.hash);
            Ok(())
        }
    }
}
