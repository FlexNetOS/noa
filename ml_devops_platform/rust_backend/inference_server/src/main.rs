//! ML DevOps Platform - Local Inference Server
//!
//! A privacy-focused local ML inference server built with Rust, Candle, and axum.
//! Provides OpenAI-compatible API for seamless integration with existing applications.

use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use inference_server::{models::ModelManager, server::InferenceServer};

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(name = "inference_server")]
#[command(about = "Local ML inference server with OpenAI-compatible API", long_about = None)]
struct Args {
    /// Host address to bind to
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to listen on
    #[arg(long, default_value_t = 8080)]
    port: u16,

    /// Model to preload (optional)
    #[arg(long)]
    model: Option<String>,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let args = Args::parse();

    // Initialize tracing
    let log_level = args
        .log_level
        .parse::<tracing::Level>()
        .unwrap_or(tracing::Level::INFO);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| log_level.to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Print banner
    print_banner();

    // Initialize model manager
    tracing::info!("🔧 Initializing model manager...");
    let model_manager = Arc::new(ModelManager::new());

    // Preload model if specified
    if let Some(model_name) = args.model {
        tracing::info!("📦 Preloading model: {}", model_name);
        if let Err(e) = model_manager.load_model(&model_name).await {
            tracing::warn!("⚠️  Failed to preload model: {}", e);
            tracing::info!("Model will be loaded on first request");
        }
    }

    // Create and run server
    let server = InferenceServer::new(args.host, args.port);
    server.run(model_manager).await?;

    Ok(())
}

fn print_banner() {
    println!(
        r#"
╔══════════════════════════════════════════════════════════╗
║                                                          ║
║   ML DevOps Platform - Local Inference Server           ║
║   Privacy-Focused Local ML with Rust & Candle           ║
║                                                          ║
║   🔒 Private: All data stays on your machine             ║
║   ⚡ Fast: Native Rust performance                        ║
║   🔌 Compatible: OpenAI-compatible API                   ║
║   🦀 Modern: Built with Rust, Candle, and axum           ║
║                                                          ║
╚══════════════════════════════════════════════════════════╝
    "#
    );
}
