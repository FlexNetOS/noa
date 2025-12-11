//! NOA Init Command
//!
//! T092-T094: Enhanced init command with --root, --force, progress display, and verification
//! Initializes a new NOA installation or reinitializes an existing one.

use std::path::PathBuf;
use std::fs;

use clap::Args;
use tracing::{info, warn};

use crate::error::Result;
use crate::db;
use crate::init::{ConfigGenerator, DatabaseInitializer, DirectoryStructure};
use crate::services::InitService;

/// Arguments for the init command
#[derive(Args, Debug)]
pub struct InitArgs {
    /// NOA root directory (defaults to current directory or NOA_ROOT env var)
    #[arg(long, env = "NOA_ROOT")]
    pub root: Option<PathBuf>,

    /// Target directory for NOA installation (deprecated, use --root)
    #[arg(short, long, default_value = ".")]
    pub target: PathBuf,

    /// Force reinitialization even if already initialized
    #[arg(short, long)]
    pub force: bool,

    /// Skip database initialization
    #[arg(long)]
    pub skip_db: bool,

    /// Skip directory structure creation
    #[arg(long)]
    pub skip_dirs: bool,
}

/// Execute the init command
pub async fn execute(args: InitArgs) -> Result<()> {
    // Determine root directory (--root takes precedence over --target)
    let target = if let Some(root) = args.root {
        root
    } else {
        args.target.canonicalize().unwrap_or(args.target.clone())
    };

    info!(target = %target.display(), "Initializing NOA");

    // Display progress
    display_progress("Starting initialization...");

    // Check if already initialized
    let marker_path = target.join(".noa-env");
    if marker_path.exists() && !args.force {
        warn!("NOA already initialized at {}. Use --force to reinitialize.", target.display());
        return Ok(());
    }

    // Use InitService for full initialization
    display_progress("Creating directory structure...");
    if !args.skip_dirs {
        DirectoryStructure::create_all(&target, args.force)?;
        display_progress("✓ Directory structure created");
    }

    display_progress("Generating default configurations...");
    ConfigGenerator::generate_all(&target)?;
    display_progress("✓ Default configurations generated");

    display_progress("Initializing database...");
    if !args.skip_db {
        DatabaseInitializer::initialize(&target, args.force)?;
        display_progress("✓ Database initialized");
    }

    // Create marker file
    create_marker_file(&target)?;

    // Verify initialization
    display_progress("Verifying initialization...");
    let verification = InitService::verify(&target)?;
    display_verification(&verification);

    println!("\n✓ NOA initialized successfully at {}", target.display());
    println!("\nNext steps:");
    println!("  1. Review configuration in config/");
    println!("  2. Configure AI providers in config/ai-providers.json");
    println!("  3. Run 'noa start' to start NOA services");

    Ok(())
}

/// Display initialization progress
fn display_progress(message: &str) {
    println!("  {}", message);
}

/// Display verification results
fn display_verification(result: &crate::services::VerificationResult) {
    if result.errors.is_empty() {
        display_progress("✓ All checks passed");
    } else {
        for error in &result.errors {
            warn!("  ✗ {}", error);
        }
    }
}


/// Create the .noa-env marker file
fn create_marker_file(target: &PathBuf) -> Result<()> {
    let marker_path = target.join(".noa-env");
    let content = format!(
        r#"# NOA Environment Marker
# Created: {}
# Version: {}

NOA_ROOT={}
NOA_ENV=development
"#,
        chrono::Utc::now().to_rfc3339(),
        env!("CARGO_PKG_VERSION"),
        target.display()
    );

    fs::write(&marker_path, content)?;
    Ok(())
}


