//! Litho Configuration
//!
//! Parses and manages litho.toml configuration

use std::path::Path;
use serde::{Deserialize, Serialize};

use super::LithoError;

/// Main Litho configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LithoConfig {
    pub litho: LithoSettings,
    pub model: ModelSettings,
    pub runtime: RuntimeSettings,
    pub incremental: IncrementalSettings,
    pub manual_edits: ManualEditSettings,
    pub subagents: SubagentSettings,
    pub output: OutputSettings,
}

impl Default for LithoConfig {
    fn default() -> Self {
        Self {
            litho: LithoSettings::default(),
            model: ModelSettings::default(),
            runtime: RuntimeSettings::default(),
            incremental: IncrementalSettings::default(),
            manual_edits: ManualEditSettings::default(),
            subagents: SubagentSettings::default(),
            output: OutputSettings::default(),
        }
    }
}

impl LithoConfig {
    /// Load configuration from a TOML file
    pub fn load(path: &Path) -> Result<Self, LithoError> {
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content).map_err(|e| LithoError::TomlParse(e.to_string()))
    }

    /// Get the output directory
    pub fn output_dir(&self) -> &str {
        &self.litho.output_dir
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LithoSettings {
    pub target_language: String,
    pub output_dir: String,
    pub disable_preset_tools: bool,
    #[serde(default)]
    pub scanning: ScanningSettings,
}

impl Default for LithoSettings {
    fn default() -> Self {
        Self {
            target_language: "en".to_string(),
            output_dir: "docs/wiki".to_string(),
            disable_preset_tools: false,
            scanning: ScanningSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScanningSettings {
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSettings {
    pub name: String,
    pub passes: u8,
    pub context_length: usize,
    #[serde(default)]
    pub tools: ModelTools,
    #[serde(default)]
    pub provider: ProviderSettings,
}

impl Default for ModelSettings {
    fn default() -> Self {
        Self {
            name: "qwen2.5-coder:1.5b".to_string(),
            passes: 4,
            context_length: 4096,
            tools: ModelTools::default(),
            provider: ProviderSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelTools {
    pub tree_sitter: bool,
    pub mermaid_fixer: bool,
    pub link_checker: bool,
    pub cross_ref_validator: bool,
}

impl Default for ModelTools {
    fn default() -> Self {
        Self {
            tree_sitter: true,
            mermaid_fixer: true,
            link_checker: true,
            cross_ref_validator: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSettings {
    pub primary: String,
    pub llm_api_base_url: String,
    #[serde(default)]
    pub fallback: Vec<String>,
}

impl Default for ProviderSettings {
    fn default() -> Self {
        Self {
            primary: "llama.cpp".to_string(),
            llm_api_base_url: "http://localhost:8080/v1".to_string(),
            fallback: vec![
                "copilot".to_string(),
                "anthropic".to_string(),
                "openai".to_string(),
                "git".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSettings {
    pub mode: String,
    pub execution: String,
    pub parallel_threshold: f32,
    pub check_interval_ms: u64,
    pub max_parallel_passes: u8,
    pub batch_size: usize,
    #[serde(default)]
    pub transitions: TransitionSettings,
    #[serde(default)]
    pub limits: LimitSettings,
}

impl Default for RuntimeSettings {
    fn default() -> Self {
        Self {
            mode: "background".to_string(),
            execution: "adaptive".to_string(),
            parallel_threshold: 0.35,
            check_interval_ms: 500,
            max_parallel_passes: 3,
            batch_size: 5,
            transitions: TransitionSettings::default(),
            limits: LimitSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionSettings {
    pub mode: String,
    pub drain_timeout_sec: u64,
    pub cooldown_before_parallel: u64,
}

impl Default for TransitionSettings {
    fn default() -> Self {
        Self {
            mode: "graceful_drain".to_string(),
            drain_timeout_sec: 5,
            cooldown_before_parallel: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitSettings {
    pub max_memory_mb: u64,
    pub max_cpu_percent: u8,
    pub pause_on_foreground: bool,
}

impl Default for LimitSettings {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            max_cpu_percent: 25,
            pause_on_foreground: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalSettings {
    pub enabled: bool,
    pub use_git_diff: bool,
    pub track_file: String,
}

impl Default for IncrementalSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            use_git_diff: true,
            track_file: ".litho-state.json".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualEditSettings {
    pub marker_start: String,
    pub marker_end: String,
    pub behavior: String,
    pub validation: String,
}

impl Default for ManualEditSettings {
    fn default() -> Self {
        Self {
            marker_start: "<!-- provider:add-manual-edit -->".to_string(),
            marker_end: "<!-- /provider:add-manual-edit -->".to_string(),
            behavior: "preserve".to_string(),
            validation: "fail_on_loss".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentSettings {
    pub pass1_structure: String,
    pub pass2_analysis: String,
    pub pass3_generation: String,
    pub pass4_validation: String,
    #[serde(default)]
    pub error_handlers: ErrorHandlerSettings,
}

impl Default for SubagentSettings {
    fn default() -> Self {
        Self {
            pass1_structure: "RustCrateScannerAgent".to_string(),
            pass2_analysis: "RustClippyAgent".to_string(),
            pass3_generation: "RustDocAgent".to_string(),
            pass4_validation: "RustFmtAgent".to_string(),
            error_handlers: ErrorHandlerSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandlerSettings {
    pub oom: String,
    pub rate_limited: String,
    pub file_lock: String,
    pub io_error: String,
}

impl Default for ErrorHandlerSettings {
    fn default() -> Self {
        Self {
            oom: "OOM Subagent".to_string(),
            rate_limited: "RateLimited Subagent".to_string(),
            file_lock: "FileLockContention Subagent".to_string(),
            io_error: "RetryableIOError Subagent".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSettings {
    pub inline_with_manual: bool,
    pub add_timestamps: bool,
    #[serde(default)]
    pub timestamp_format: String,
    #[serde(default)]
    pub paths: OutputPaths,
}

impl Default for OutputSettings {
    fn default() -> Self {
        Self {
            inline_with_manual: true,
            add_timestamps: true,
            timestamp_format: "%Y-%m-%d %H:%M:%S UTC".to_string(),
            paths: OutputPaths::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputPaths {
    pub crates: String,
    pub internal: String,
    pub architecture: String,
    pub diagrams: String,
}

impl Default for OutputPaths {
    fn default() -> Self {
        Self {
            crates: "docs/wiki/crates/".to_string(),
            internal: "docs/wiki/internal-crates/".to_string(),
            architecture: "docs/wiki/architecture/".to_string(),
            diagrams: "docs/wiki/diagrams/".to_string(),
        }
    }
}
