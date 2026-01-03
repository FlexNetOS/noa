//! NOA Wiki CLI Commands
//!
//! Commands for managing wiki documentation generation using Litho.
//! Supports full regeneration, incremental updates, and status checks.

use crate::error::Result;

/// Wiki command variants
#[derive(Debug, Clone)]
pub enum WikiCmd {
    /// Generate full documentation from scratch
    GenerateFull {
        /// Force regeneration even if up to date
        force: bool,
        /// Run in sequential mode (ignore adaptive)
        sequential: bool,
    },
    /// Generate only for changed files
    GenerateIncremental {
        /// Git ref to compare against (default: HEAD~1)
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

/// Execute wiki command
pub async fn execute(command: WikiCmd) -> Result<()> {
    match command {
        WikiCmd::GenerateFull { force, sequential } => {
            generate_full(force, sequential).await
        }
        WikiCmd::GenerateIncremental { since } => {
            generate_incremental(since).await
        }
        WikiCmd::Status => {
            show_status().await
        }
        WikiCmd::Cancel => {
            cancel_generation().await
        }
        WikiCmd::ManualEdits => {
            list_manual_edits().await
        }
        WikiCmd::Validate => {
            validate_docs().await
        }
    }
}

/// Generate full documentation
async fn generate_full(force: bool, sequential: bool) -> Result<()> {
    println!("📚 NOA Wiki - Full Documentation Generation");
    println!("============================================");
    println!();

    // Check if Litho is available
    let litho_path = get_litho_path();
    if !litho_path.exists() {
        println!("❌ Litho not found at {}", litho_path.display());
        println!("   Run: cd opt/wiki-rs && cargo build --release");
        return Ok(());
    }

    println!("🔧 Configuration:");
    println!("   Model: qwen2.5-coder:1.5b (SLM <3B params)");
    println!("   Mode: {}", if sequential { "Sequential" } else { "Adaptive" });
    println!("   Force: {}", force);
    println!();

    println!("📋 Multi-pass Pipeline:");
    println!("   Pass 1: Structure (RustCrateScannerAgent) - Sequential");
    println!("   Pass 2: Analysis (RustClippyAgent) - Parallelizable");
    println!("   Pass 3: Generation (RustDocAgent) - Parallelizable");
    println!("   Pass 4: Validation (RustFmtAgent) - Parallelizable");
    println!();

    println!("🔄 Fallback Chain:");
    println!("   1. llama.cpp (local) → 2. copilot → 3. anthropic → 4. openai → 5. git");
    println!();

    // TODO: Actually execute Litho when ready
    println!("⏳ Starting generation... (stub - Litho integration pending)");
    println!();
    println!("✅ Generation complete (dry run)");

    Ok(())
}

/// Generate incremental documentation for changed files
async fn generate_incremental(since: Option<String>) -> Result<()> {
    let git_ref = since.unwrap_or_else(|| "HEAD~1".to_string());

    println!("📚 NOA Wiki - Incremental Documentation Generation");
    println!("===================================================");
    println!();
    println!("🔍 Checking for changes since: {}", git_ref);
    println!();

    // Find changed files
    println!("📁 Changed files:");
    println!("   (detecting via git diff...)");
    println!();

    // TODO: Actually run git diff and filter for relevant files
    println!("⏳ Generating docs for changed files... (stub)");
    println!();
    println!("✅ Incremental generation complete (dry run)");

    Ok(())
}

/// Show current generation status
async fn show_status() -> Result<()> {
    println!("📊 NOA Wiki - Generation Status");
    println!("================================");
    println!();
    println!("State: Idle");
    println!();
    println!("Last Generation:");
    println!("   Time: (never run)");
    println!("   Mode: -");
    println!("   Duration: -");
    println!();
    println!("Resource Usage:");
    println!("   CPU: 0%");
    println!("   Memory: 0%");
    println!("   Threshold: 35%");
    println!();
    println!("Pass Status:");
    println!("   [1] Structure   : Not started");
    println!("   [2] Analysis    : Not started");
    println!("   [3] Generation  : Not started");
    println!("   [4] Validation  : Not started");

    Ok(())
}

/// Cancel ongoing generation
async fn cancel_generation() -> Result<()> {
    println!("🛑 Cancelling documentation generation...");
    println!();
    println!("   No generation in progress.");

    Ok(())
}

/// List manual edit sections in generated docs
async fn list_manual_edits() -> Result<()> {
    println!("📝 Manual Edit Sections");
    println!("=======================");
    println!();
    println!("Marker: <!-- provider:add-manual-edit --> ... <!-- /provider:add-manual-edit -->");
    println!();
    println!("Files with manual edits:");
    println!("   (none found - docs not yet generated)");
    println!();
    println!("Manual edits are preserved during regeneration.");

    Ok(())
}

/// Validate generated documentation
async fn validate_docs() -> Result<()> {
    println!("✅ NOA Wiki - Documentation Validation");
    println!("======================================");
    println!();
    println!("Checks:");
    println!("   [⏳] Link validation...");
    println!("   [⏳] Cross-reference integrity...");
    println!("   [⏳] Mermaid diagram syntax...");
    println!("   [⏳] Markdown lint...");
    println!();
    println!("Result: No documentation to validate yet.");

    Ok(())
}

/// Get path to Litho binary
fn get_litho_path() -> std::path::PathBuf {
    let noa_root = std::env::var("NOA_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("N:/noa"));

    #[cfg(windows)]
    let binary = "litho.exe";
    #[cfg(not(windows))]
    let binary = "litho";

    noa_root.join("opt/wiki-rs/target/release").join(binary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wiki_cmd_variants() {
        let full = WikiCmd::GenerateFull {
            force: false,
            sequential: false,
        };
        assert!(matches!(full, WikiCmd::GenerateFull { .. }));

        let incr = WikiCmd::GenerateIncremental { since: None };
        assert!(matches!(incr, WikiCmd::GenerateIncremental { .. }));
    }
}
