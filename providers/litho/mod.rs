//! Litho Provider Adapter
//!
//! Unified provider interface for wiki-rs/Litho documentation generator.
//! Implements fallback chain: llama.cpp → copilot → anthropic → openai → git
//!
//! §3.3: Multiple Small Language Models (SLMs) via llama.cpp (<3B params each)

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

pub mod configs;
pub mod executor;
pub mod fallback;
pub mod manual_edit;
pub mod resource_monitor;

pub use configs::Lithoconfigs;
pub use executor::{LithoExecutor, ResourceSnapshot};
pub use fallback::FallbackChain;
pub use manual_edit::ManualEditPreserver;
pub use resource_monitor::{ResourceMonitor, ResourceMonitorconfigs, ResourceSpikeHandler};

/// Provider priority for fallback chain
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProviderPriority {
    LlamaCpp = 1,
    Copilot = 2,
    Anthropic = 3,
    OpenAI = 4,
    Git = 5,
}

impl ProviderPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LlamaCpp => "llama.cpp",
            Self::Copilot => "copilot",
            Self::Anthropic => "anthropic",
            Self::OpenAI => "openai",
            Self::Git => "git",
        }
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Self::LlamaCpp | Self::Git)
    }
}

/// Litho pass definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LithoPass {
    pub id: u8,
    pub name: String,
    pub subagent: String,
    pub tools: Vec<String>,
    pub parallelizable: bool,
    pub depends_on: Vec<u8>,
}

impl Default for LithoPass {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            subagent: String::new(),
            tools: Vec::new(),
            parallelizable: false,
            depends_on: Vec::new(),
        }
    }
}

/// Standard 4-pass configsuration for documentation generation
pub fn default_passes() -> Vec<LithoPass> {
    vec![
        LithoPass {
            id: 1,
            name: "structure".to_string(),
            subagent: "RustCrateScannerAgent".to_string(),
            tools: vec!["tree-sitter".to_string()],
            parallelizable: false,
            depends_on: vec![],
        },
        LithoPass {
            id: 2,
            name: "analysis".to_string(),
            subagent: "RustClippyAgent".to_string(),
            tools: vec!["clippy".to_string(), "rustfmt".to_string()],
            parallelizable: true,
            depends_on: vec![1],
        },
        LithoPass {
            id: 3,
            name: "generation".to_string(),
            subagent: "RustDocAgent".to_string(),
            tools: vec!["mermaid-fixer".to_string(), "markdown-lint".to_string()],
            parallelizable: true,
            depends_on: vec![1],
        },
        LithoPass {
            id: 4,
            name: "validation".to_string(),
            subagent: "RustFmtAgent".to_string(),
            tools: vec!["link-checker".to_string(), "cross-ref-validator".to_string()],
            parallelizable: true,
            depends_on: vec![1],
        },
    ]
}

/// Litho provider adapter for NOA
pub struct LithoProvider {
    pub configs: Lithoconfigs,
    pub fallback_chain: FallbackChain,
    pub passes: Vec<LithoPass>,
    pub noa_root: PathBuf,
}

impl LithoProvider {
    /// Create a new Litho provider with default configsuration
    pub fn new(noa_root: PathBuf) -> Self {
        Self {
            configs: Lithoconfigs::default(),
            fallback_chain: FallbackChain::default(),
            passes: default_passes(),
            noa_root,
        }
    }

    /// Load configsuration from litho.toml
    pub fn from_configs(noa_root: PathBuf, configs_path: PathBuf) -> Result<Self, LithoError> {
        let configs = Lithoconfigs::load(&configs_path)?;
        let fallback_chain = FallbackChain::from_configs(&configs)?;
        
        Ok(Self {
            configs,
            fallback_chain,
            passes: default_passes(),
            noa_root,
        })
    }

    /// Get the Litho binary path
    pub fn binary_path(&self) -> PathBuf {
        self.noa_root.join("opt/wiki-rs/target/release/litho")
    }

    /// Get output directory
    pub fn output_dir(&self) -> PathBuf {
        self.noa_root.join(&self.configs.output_dir)
    }
}

/// Litho-specific errors
#[derive(Debug, thiserror::Error)]
pub enum LithoError {
    #[error("configsuration error: {0}")]
    configs(String),

    #[error("Provider unavailable: {0}")]
    ProviderUnavailable(String),

    #[error("All providers in fallback chain failed")]
    AllProvidersFailed,

    #[error("Pass execution failed: {pass} - {reason}")]
    PassFailed { pass: String, reason: String },

    #[error("Manual edit marker validation failed: {0}")]
    ManualEditLost(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_priority_order() {
        assert!(ProviderPriority::LlamaCpp < ProviderPriority::Copilot);
        assert!(ProviderPriority::Copilot < ProviderPriority::Anthropic);
        assert!(ProviderPriority::Git > ProviderPriority::OpenAI);
    }

    #[test]
    fn test_default_passes() {
        let passes = default_passes();
        assert_eq!(passes.len(), 4);
        assert!(!passes[0].parallelizable); // Pass 1 must be sequential
        assert!(passes[1].parallelizable);  // Passes 2-4 can parallelize
    }

    #[test]
    fn test_local_providers() {
        assert!(ProviderPriority::LlamaCpp.is_local());
        assert!(ProviderPriority::Git.is_local());
        assert!(!ProviderPriority::OpenAI.is_local());
    }
}
