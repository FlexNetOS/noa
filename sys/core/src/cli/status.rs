//! NOA Status Command
//!
//! Shows the status of NOA services and system health.

use std::time::Duration;

use clap::Args;
use tracing::info;

use crate::api::server::ApiConfig;
use crate::config::NoaConfig;
use crate::db;
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
        print_json_status(&config, args.detailed).await?;
    } else {
        print_text_status(&config, args.detailed).await?;
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct ApiProbe {
    status: String,
    base_url: String,
    http_status: Option<u16>,
    health: Option<serde_json::Value>,
    error: Option<String>,
}

async fn probe_api_server(config: &NoaConfig) -> ApiProbe {
    let api_cfg = ApiConfig::from_noa_config(config);
    let base_url = format!("http://{}:{}", api_cfg.host, api_cfg.port);
    let url = format!("{}/health", base_url);

    // Keep CLI probes snappy even if the server's request timeout is high.
    let timeout_secs = api_cfg.timeout_secs.clamp(1, 5);
    let timeout = Duration::from_secs(timeout_secs);

    let client = match reqwest::Client::builder().timeout(timeout).build() {
        Ok(c) => c,
        Err(e) => {
            return ApiProbe {
                status: "error".to_string(),
                base_url,
                http_status: None,
                health: None,
                error: Some(format!("failed to build http client: {e}")),
            };
        }
    };

    let resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            return ApiProbe {
                status: "unreachable".to_string(),
                base_url,
                http_status: None,
                health: None,
                error: Some(e.to_string()),
            };
        }
    };

    let http_status = Some(resp.status().as_u16());
    let is_success = resp.status().is_success();

    let health: Option<serde_json::Value> = match resp.json::<serde_json::Value>().await {
        Ok(v) => Some(v),
        Err(_) => None,
    };

    ApiProbe {
        status: if is_success { "ok" } else { "error" }.to_string(),
        base_url,
        http_status,
        health,
        error: None,
    }
}

async fn print_text_status(config: &NoaConfig, detailed: bool) -> Result<()> {
    println!("NOA Status");
    println!("==========");
    println!();
    println!("Instance: {}", config.instance_name);
    println!("Environment: {:?}", config.environment);
    println!("NOA Root: {}", config.noa_root.display());
    println!();

    // Check database
    println!("Components:");
    if config.database.driver == "postgresql" {
        #[cfg(not(feature = "full"))]
        {
            println!("  [!] Database: PostgreSQL requires --features full");
        }

        #[cfg(feature = "full")]
        {
            let url = match config.database.url.as_deref() {
                Some(u) => u,
                None => {
                    println!("  [✗] Database: Missing database.url");
                    return Ok(());
                }
            };

            match crate::db::connect_postgres(url, config.database.max_connections).await {
                Ok(pool) => match crate::db::check_postgres(&pool).await {
                    Ok(()) => println!("  [✓] Database (PostgreSQL): OK"),
                    Err(e) => println!("  [✗] Database (PostgreSQL): Error - {}", e),
                },
                Err(e) => println!("  [✗] Database (PostgreSQL): Connection failed - {}", e),
            }
        }
    } else {
        let db_path = config.noa_root.join(&config.database.path);
        if db_path.exists() {
            match db::init_database(&db_path) {
                Ok(conn) => match db::check_integrity(&conn) {
                    Ok(true) => {
                        println!("  [✓] Database (SQLite): OK");
                        if detailed {
                            if let Ok(stats) = db::get_stats(&conn) {
                                println!("      Size: {} bytes", stats.total_size_bytes);
                                println!("      Pages: {}", stats.total_pages);
                            }
                        }
                    }
                    Ok(false) => println!("  [✗] Database: INTEGRITY CHECK FAILED"),
                    Err(e) => println!("  [!] Database: Error - {}", e),
                },
                Err(e) => println!("  [✗] Database: Connection failed - {}", e),
            }
        } else {
            println!("  [!] Database: Not found at {}", db_path.display());
        }
    }

    // Check API server
    let api = probe_api_server(config).await;
    match api.status.as_str() {
        "ok" => {
            println!("  [✓] API Server: OK ({})", api.base_url);
            if detailed {
                if let Some(code) = api.http_status {
                    println!("      HTTP: {}", code);
                }
                if let Some(health) = api.health {
                    if let Some(status) = health.get("status") {
                        println!("      Health: {}", status);
                    }
                }
            }
        }
        _ => {
            if let Some(err) = api.error {
                println!("  [!] API Server: {} ({})", err, api.base_url);
            } else if let Some(code) = api.http_status {
                println!("  [!] API Server: HTTP {} ({})", code, api.base_url);
            } else {
                println!("  [!] API Server: Unknown ({})", api.base_url);
            }
        }
    }

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

async fn print_json_status(config: &NoaConfig, detailed: bool) -> Result<()> {
    let api = probe_api_server(config).await;

    let (db_status, db_ref) = if config.database.driver == "postgresql" {
        #[cfg(not(feature = "full"))]
        {
            ("requires_full", config.database.url.clone().unwrap_or_default())
        }

        #[cfg(feature = "full")]
        {
            let url = config.database.url.clone().unwrap_or_default();
            let status = if !url.is_empty() {
                match crate::db::connect_postgres(&url, config.database.max_connections).await {
                    Ok(pool) => match crate::db::check_postgres(&pool).await {
                        Ok(()) => "ok",
                        Err(_) => "error",
                    },
                    Err(_) => "error",
                }
            } else {
                "missing_url"
            };
            (status, url)
        }
    } else {
        let db_path = config.noa_root.join(&config.database.path);
        let status = if db_path.exists() {
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
        (status, db_path.display().to_string())
    };

    let db_details = if detailed && db_status == "ok" && config.database.driver != "postgresql" {
        let db_path = config.noa_root.join(&config.database.path);
        match db::init_database(&db_path)
            .and_then(|conn| db::get_stats(&conn).map(|s| (conn, s)))
        {
            Ok((_conn, stats)) => Some(serde_json::json!({
                "total_size_bytes": stats.total_size_bytes,
                "used_size_bytes": stats.used_size_bytes,
                "total_pages": stats.total_pages,
                "page_size": stats.page_size,
                "free_pages": stats.free_pages,
            })),
            Err(_) => None,
        }
    } else {
        None
    };

    let mut status = serde_json::json!({
        "instance": config.instance_name,
        "environment": format!("{:?}", config.environment),
        "noa_root": config.noa_root.display().to_string(),
        "components": {
            "database": {
                "status": db_status,
                "ref": db_ref,
                "details": db_details,
            },
            "api_server": {
                "status": api.status,
                "base_url": api.base_url,
                "http_status": api.http_status,
                "error": api.error,
                "health": if detailed { api.health } else { None },
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

    if detailed {
        if let Some(obj) = status.as_object_mut() {
            obj.insert(
                "feature_flags".to_string(),
                serde_json::Value::Object(
                    config
                        .feature_flags
                        .iter()
                        .map(|(k, v)| (k.clone(), serde_json::Value::Bool(*v)))
                        .collect(),
                ),
            );
        }
    }

    println!("{}", serde_json::to_string_pretty(&status).unwrap());
    Ok(())
}

