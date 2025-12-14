//! NOA Status Command
//!
//! Shows the status of NOA services and system health.

use std::path::PathBuf;

use clap::Args;
use tracing::info;

use crate::config::NoaConfig;
use crate::db::{self, ConnectionPool};
use crate::error::Result;

/// Arguments for the status command
#[derive(Args, Debug)]
pub struct StatusArgs {
    /// Show detailed status
    #[arg(short, long)]
    pub detailed: bool,

    /// Output format (text, json)
    #[arg(short, long, default_value = "text")]
    pub format: String,
}

/// Execute the status command
pub async fn execute(args: StatusArgs) -> Result<()> {
    info!(detailed = args.detailed, "Checking NOA status");

    // Load configuration
    let config = match NoaConfig::load() {
        Ok(c) => c,
        Err(e) => {
            println!("NOA Status: NOT INITIALIZED");
            println!("  Error: {}", e);
            println!("\n  Run 'noa init' to initialize NOA.");
            return Ok(());
        }
    };

    if args.format == "json" {
        print_json_status(&config, args.detailed)?;
    } else {
        print_text_status(&config, args.detailed)?;
    }

    Ok(())
}

fn print_text_status(config: &NoaConfig, detailed: bool) -> Result<()> {
    println!("NOA Status");
    println!("==========");
    println!();
    println!("Instance: {}", config.instance_name);
    println!("Environment: {:?}", config.environment);
    println!("NOA Root: {}", config.noa_root.display());
    println!();

    // Check database
    println!("Components:");
    let db_path = config.noa_root.join(&config.database.path);
    if db_path.exists() {
        match db::init_database(&db_path) {
            Ok(conn) => {
                match db::check_integrity(&conn) {
                    Ok(true) => {
                        println!("  [✓] Database: OK");
                        if detailed {
                            if let Ok(stats) = db::get_stats(&conn) {
                                println!("      Size: {} bytes", stats.total_size_bytes);
                                println!("      Pages: {}", stats.total_pages);
                            }
                        }
                    }
                    Ok(false) => println!("  [✗] Database: INTEGRITY CHECK FAILED"),
                    Err(e) => println!("  [!] Database: Error - {}", e),
                }
            }
            Err(e) => println!("  [✗] Database: Connection failed - {}", e),
        }
    } else {
        println!("  [!] Database: Not found at {}", db_path.display());
    }

    // Check API server
    println!("  [?] API Server: Status check not implemented");

    // Check agents
    println!("  [?] Agents: Status check not implemented");

    // Check providers
    println!();
    println!("AI Providers:");
    for (name, settings) in &config.providers.providers {
        let status = if settings.enabled { "enabled" } else { "disabled" };
        println!("  {} ({}): {}", name, settings.provider_type, status);
    }

    if detailed {
        println!();
        println!("Feature Flags:");
        for (name, enabled) in &config.feature_flags {
            let status = if *enabled { "✓" } else { "✗" };
            println!("  [{}] {}", status, name);
        }
    }

    Ok(())
}

fn print_json_status(config: &NoaConfig, detailed: bool) -> Result<()> {
    let db_path = config.noa_root.join(&config.database.path);

    let db_status = if db_path.exists() {
        match db::init_database(&db_path) {
            Ok(conn) => match db::check_integrity(&conn) {
                Ok(true) => "ok",
                _ => "error",
            },
            Err(_) => "error",
        }
    } else {
        "not_found"
    };

    let status = serde_json::json!({
        "instance": config.instance_name,
        "environment": format!("{:?}", config.environment),
        "noa_root": config.noa_root.display().to_string(),
        "components": {
            "database": {
                "status": db_status,
                "path": db_path.display().to_string(),
            },
            "api_server": {
                "status": "unknown",
            },
            "agents": {
                "status": "unknown",
            }
        },
        "providers": config.providers.providers.iter().map(|(name, settings)| {
            (name.clone(), serde_json::json!({
                "type": settings.provider_type,
                "enabled": settings.enabled,
            }))
        }).collect::<serde_json::Map<String, serde_json::Value>>(),
    });

    println!("{}", serde_json::to_string_pretty(&status).unwrap());
    Ok(())
}

