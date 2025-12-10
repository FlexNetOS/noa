//! Spec-Kit CLI Wrapper
//!
//! T375: Lightweight wrapper to invoke Spec-Kit with provider selection hints.

use clap::{Args, Subcommand};
use std::process::Command;
use tracing::info;

use crate::error::Result;

#[derive(Args, Debug)]
pub struct SpeckitArgs {
    #[command(subcommand)]
    pub command: SpeckitCommands,
}

#[derive(Subcommand, Debug)]
pub enum SpeckitCommands {
    /// Detect available Spec-Kit providers based on environment
    Detect,

    /// Run a Spec-Kit action (plan|scan) against a target path
    Run {
        /// Target path or URL to analyze
        target: String,
        /// Action to execute (plan, scan, summarize)
        #[arg(short, long, default_value = "plan")]
        action: String,
        /// Preferred provider (optional)
        #[arg(short, long)]
        provider: Option<String>,
    },
}

pub async fn execute(args: SpeckitArgs) -> Result<()> {
    match args.command {
        SpeckitCommands::Detect => {
            run_detect();
            Ok(())
        }
        SpeckitCommands::Run {
            target,
            action,
            provider,
        } => run_command(&target, &action, provider.as_deref()),
    }
}

fn run_detect() {
    let providers = [
        ("anthropic", "ANTHROPIC_API_KEY"),
        ("openai", "OPENAI_API_KEY"),
        ("azure-openai", "AZURE_OPENAI_ENDPOINT"),
        ("ollama", "OLLAMA_HOST"),
    ];

    println!("Spec-Kit provider detection:");
    for (name, env) in providers {
        let available = std::env::var(env).is_ok();
        let status = if available { "available" } else { "missing" };
        println!("- {name}: {status} ({env})");
    }
}

fn run_command(target: &str, action: &str, provider: Option<&str>) -> Result<()> {
    let binary = std::env::var("SPECKIT_BIN").unwrap_or_else(|_| "speckit".to_string());
    let mut cmd = Command::new(&binary);
    cmd.arg(action).arg(target);

    if let Some(provider) = provider {
        cmd.arg("--provider").arg(provider);
    }

    info!(binary = %binary, action = %action, target = %target, "Executing Spec-Kit command");

    match cmd.status() {
        Ok(status) if status.success() => {
            println!("Spec-Kit command succeeded: {} {} {}", binary, action, target);
        }
        Ok(status) => {
            eprintln!(
                "Spec-Kit command exited with status {}. Validate Spec-Kit installation.",
                status
            );
        }
        Err(err) => {
            eprintln!(
                "Unable to execute Spec-Kit ({}). Hint: set SPECKIT_BIN or install speckit binary.",
                err
            );
        }
    }

    Ok(())
}
