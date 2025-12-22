//! NOA Start Command
//!
//! Starts NOA services including API server and agents.

use clap::Args;
use tracing::{info, error};

use crate::api::server::{ApiConfig, ApiServerBuilder};
use crate::config::NoaConfig;
use crate::db::ConnectionPool;
use crate::error::{NoaError, Result};
use crate::init::paths::NoaPaths;

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
    info!(
        host = %args.host,
        port = args.port,
        foreground = args.foreground,
        "Starting NOA services"
    );

    // Load configuration
    let config = NoaConfig::load()?;
    info!(instance = %config.instance_name, "Configuration loaded");

    // PostgreSQL path (server deployments)
    if config.database.driver == "postgresql" {
        #[cfg(feature = "full")]
        {
            let url = config.database.url.as_deref().ok_or_else(|| NoaError::Internal {
                message: "database.url is required when database.driver=postgresql".to_string(),
                source: None,
            })?;

            let migrations_dir = NoaPaths::init_migrations_pg(&config.noa_root);
            let pool = crate::db::connect_postgres(url, config.database.max_connections).await?;
            crate::db::migrate_postgres(&pool, &migrations_dir).await?;
            crate::db::check_postgres(&pool).await?;

            if args.no_api && args.no_agents {
                info!("PostgreSQL migrations applied; nothing else to start");
                return Ok(());
            }

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

                println!("Starting NOA API server on {}:{} (PostgreSQL)", args.host, args.port);

                let server = ApiServerBuilder::new()
                    .with_config(api_config)
                    .with_postgres_pool(pool)
                    .with_noa_config(config)
                    .build()?;

                if let Err(e) = server.start().await {
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
            return Ok(());
        }

        #[cfg(not(feature = "full"))]
        {
            return Err(NoaError::Internal {
                message: "database.driver=postgresql requires building noa-core with feature \"full\"".to_string(),
                source: None,
            });
        }
    }

    // Initialize database pool
    let db_path = config.noa_root.join(&config.database.path);
    let db_pool = ConnectionPool::with_defaults(&db_path)?;
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

        let server = ApiServerBuilder::new()
            .with_config(api_config)
            .with_database(db_pool)
            .with_noa_config(config)
            .build()?;

        // Start server
        if let Err(e) = server.start().await {
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

