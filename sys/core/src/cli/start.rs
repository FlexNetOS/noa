//! NOA Start Command
//!
//! Starts NOA services including API server and agents.

use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

use clap::Args;
use tracing::{error, info};

use crate::api::server::{ApiConfig, ApiServerBuilder};
use crate::config::NoaConfig;
use crate::db::ConnectionPool;
use crate::error::Result;

/// Arguments for the start command
#[derive(Args, Debug)]
pub struct StartArgs {
    /// Run in foreground (don't daemonize)
    #[arg(short, long)]
    pub foreground: bool,

    /// API server host
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// API server port
    #[arg(short, long, default_value = "8080")]
    pub port: u16,

    /// Skip starting the API server
    #[arg(long)]
    pub no_api: bool,

    /// Skip starting agents
    #[arg(long)]
    pub no_agents: bool,
}

/// Execute the start command
pub async fn execute(args: StartArgs) -> Result<()> {
    // #region agent log
    let log_entry = serde_json::json!({
        "location": "cli/start.rs:40",
        "message": "Start command entry",
        "data": {"host": args.host.clone(), "port": args.port, "foreground": args.foreground},
        "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
        "sessionId": "debug-session",
        "runId": "startup",
        "hypothesisId": "A"
    });
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
        let _ = writeln!(file, "{}", log_entry);
    }
    // #endregion
    info!(
        host = %args.host,
        port = args.port,
        foreground = args.foreground,
        "Starting NOA services"
    );

    // Load configuration
    // #region agent log
    let log_entry = serde_json::json!({
        "location": "cli/start.rs:49",
        "message": "Before config load",
        "data": {},
        "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
        "sessionId": "debug-session",
        "runId": "startup",
        "hypothesisId": "A"
    });
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
        let _ = writeln!(file, "{}", log_entry);
    }
    // #endregion
    let config = NoaConfig::load()?;
    // #region agent log
    let log_entry = serde_json::json!({
        "location": "cli/start.rs:52",
        "message": "After config load",
        "data": {"instance_name": config.instance_name.clone(), "noa_root": config.noa_root.to_string_lossy().to_string()},
        "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
        "sessionId": "debug-session",
        "runId": "startup",
        "hypothesisId": "A"
    });
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
        let _ = writeln!(file, "{}", log_entry);
    }
    // #endregion
    info!(instance = %config.instance_name, "Configuration loaded");

    // Initialize database pool
    // #region agent log
    let db_path_str = config.noa_root.join(&config.database.path).to_string_lossy().to_string();
    let log_entry = serde_json::json!({
        "location": "cli/start.rs:55",
        "message": "Before DB pool init",
        "data": {"db_path": db_path_str},
        "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
        "sessionId": "debug-session",
        "runId": "startup",
        "hypothesisId": "B"
    });
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
        let _ = writeln!(file, "{}", log_entry);
    }
    // #endregion
    let db_path = config.noa_root.join(&config.database.path);
    let db_pool = ConnectionPool::with_defaults(&db_path)?;
    // #region agent log
    let log_entry = serde_json::json!({
        "location": "cli/start.rs:58",
        "message": "After DB pool init",
        "data": {"success": true},
        "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
        "sessionId": "debug-session",
        "runId": "startup",
        "hypothesisId": "B"
    });
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
        let _ = writeln!(file, "{}", log_entry);
    }
    // #endregion
    info!("Database pool initialized");

    // Start API server if not disabled
    if !args.no_api {
        let api_config = ApiConfig {
            host: args.host.clone(),
            port: args.port,
            timeout_secs: 30,
            enable_cors: true,
            cors_origins: vec![],
            enable_tracing: true,
            shutdown_timeout_secs: 30,
        };

        println!("Starting NOA API server on {}:{}", args.host, args.port);

        // #region agent log
        let log_entry = serde_json::json!({
            "location": "cli/start.rs:71",
            "message": "Before server build",
            "data": {},
            "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            "sessionId": "debug-session",
            "runId": "startup",
            "hypothesisId": "C"
        });
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
            let _ = writeln!(file, "{}", log_entry);
        }
        // #endregion
        let server = ApiServerBuilder::new()
            .with_config(api_config)
            .with_database(db_pool)
            .with_noa_config(config)
            .build()?;
        // #region agent log
        let log_entry = serde_json::json!({
            "location": "cli/start.rs:78",
            "message": "After server build, before start",
            "data": {},
            "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            "sessionId": "debug-session",
            "runId": "startup",
            "hypothesisId": "C"
        });
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
            let _ = writeln!(file, "{}", log_entry);
        }
        // #endregion

        // Start server
        if let Err(e) = server.start().await {
            // #region agent log
            let log_entry = serde_json::json!({
                "location": "cli/start.rs:85",
                "message": "Server start failed",
                "data": {"error": e.to_string()},
                "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
                "sessionId": "debug-session",
                "runId": "startup",
                "hypothesisId": "C"
            });
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("n:\\noa\\.cursor\\debug.log") {
                let _ = writeln!(file, "{}", log_entry);
            }
            // #endregion
            error!(error = %e, "Server failed");
            return Err(e);
        }
    }

    // Start agents if not disabled
    if !args.no_agents {
        info!("Agent startup not yet implemented");
        // TODO: Start configured agents
    }

    info!("NOA services started successfully");
    Ok(())
}
