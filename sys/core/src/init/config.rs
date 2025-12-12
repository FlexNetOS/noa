//! Configuration Generation
//!
//! T079, T081-T084: Implement default config generation
//! §3.2: Local-First & Offline-Capable

use crate::error::{NoaError, Result};
use crate::init::paths::NoaPaths;
use serde_json::json;
use std::fs;
use std::path::Path;
use tracing::{debug, info};

/// Configuration generator
pub struct ConfigGenerator;

impl ConfigGenerator {
    /// Generate all default configuration files
    pub fn generate_all(noa_root: &Path) -> Result<()> {
        info!("Generating default configuration files");

        Self::generate_ai_providers(noa_root)?;
        Self::generate_noa_server(noa_root)?;
        Self::generate_features(noa_root)?;
        Self::generate_models(noa_root)?;

        info!("Default configuration files generated");
        Ok(())
    }

    /// Generate ai-providers.json default config
    pub fn generate_ai_providers(noa_root: &Path) -> Result<()> {
        let path = NoaPaths::config(noa_root).join("ai-providers.json");

        if path.exists() {
            debug!(path = %path.display(), "ai-providers.json already exists");
            return Ok(());
        }

        let config = json!({
            "$schema": "https://noa.local/schemas/providers.yaml",
            "version": "1.0.0",
            "providerPriority": ["local", "hybrid", "ide", "cloud"],
            "providers": {
                "local": {
                    "enabled": true,
                    "priority": 1,
                    "types": ["llama.cpp", "ollama", "git-cli"],
                    "configPath": "${NOA_ROOT}/ai/providers/local"
                },
                "hybrid": {
                    "enabled": true,
                    "priority": 2,
                    "types": ["cursor"],
                    "configPath": "${NOA_ROOT}/ai/providers/hybrid"
                },
                "ide": {
                    "enabled": true,
                    "priority": 4,
                    "types": ["vscode-copilot"],
                    "configPath": "${NOA_ROOT}/ai/providers/ide"
                },
                "cloud": {
                    "enabled": true,
                    "priority": 3,
                    "types": ["claude-code", "codex", "abacus"],
                    "configPath": "${NOA_ROOT}/ai/providers/cloud"
                }
            }
        });

        let content = serde_json::to_string_pretty(&config)
            .map_err(|e| NoaError::Serialization(e.to_string()))?;
        fs::write(&path, content)?;

        info!(path = %path.display(), "Generated ai-providers.json");
        Ok(())
    }

    /// Generate noa-server.json default config
    pub fn generate_noa_server(noa_root: &Path) -> Result<()> {
        let path = NoaPaths::config(noa_root).join("noa-server.json");

        if path.exists() {
            debug!(path = %path.display(), "noa-server.json already exists");
            return Ok(());
        }

        let config = json!({
            "$schema": "https://noa.local/schemas/noa-server.json",
            "version": "1.0.0",
            "server": {
                "host": "127.0.0.1",
                "port": 8080,
                "timeout_secs": 30,
                "max_connections": 100
            },
            "database": {
                "primary": {
                    "driver": "sqlite",
                    "path": "${NOA_ROOT}/data/noa.db",
                    "max_connections": 10
                }
            },
            "logging": {
                "level": "info",
                "format": "json",
                "output": "${NOA_ROOT}/logs/noa.log",
                "rotate": {
                    "enabled": true,
                    "max_size_mb": 100,
                    "max_files": 10
                }
            },
            "observability": {
                "metrics": {
                    "enabled": true,
                    "endpoint": "/metrics"
                },
                "tracing": {
                    "enabled": true,
                    "endpoint": "http://localhost:4318"
                }
            }
        });

        let content = serde_json::to_string_pretty(&config)
            .map_err(|e| NoaError::Serialization(e.to_string()))?;
        fs::write(&path, content)?;

        info!(path = %path.display(), "Generated noa-server.json");
        Ok(())
    }

    /// Generate features.json default feature flags
    pub fn generate_features(noa_root: &Path) -> Result<()> {
        let path = NoaPaths::config(noa_root).join("features.json");

        if path.exists() {
            debug!(path = %path.display(), "features.json already exists");
            return Ok(());
        }

        let config = json!({
            "$schema": "https://noa.local/schemas/features.json",
            "version": "1.0.0",
            "features": [
                {
                    "name": "offline_mode",
                    "enabled": true,
                    "description": "Enable offline operation",
                    "scope": "global"
                },
                {
                    "name": "experimental_agents",
                    "enabled": false,
                    "description": "Enable experimental agent features",
                    "scope": "agent"
                },
                {
                    "name": "p2p_federation",
                    "enabled": false,
                    "description": "Enable P2P device federation",
                    "scope": "network"
                },
                {
                    "name": "multi_modal",
                    "enabled": false,
                    "description": "Enable multi-modal interaction (voice, vision)",
                    "scope": "ui"
                },
                {
                    "name": "self_improvement",
                    "enabled": false,
                    "description": "Enable autonomous self-improvement",
                    "scope": "autonomy"
                }
            ]
        });

        let content = serde_json::to_string_pretty(&config)
            .map_err(|e| NoaError::Serialization(e.to_string()))?;
        fs::write(&path, content)?;

        info!(path = %path.display(), "Generated features.json");
        Ok(())
    }

    /// Generate models.json default model registry
    pub fn generate_models(noa_root: &Path) -> Result<()> {
        let path = NoaPaths::config(noa_root).join("models.json");

        if path.exists() {
            debug!(path = %path.display(), "models.json already exists");
            return Ok(());
        }

        let config = json!({
            "$schema": "https://noa.local/schemas/models.json",
            "version": "1.0.0",
            "models": [
                {
                    "id": "llama-3.2-1b",
                    "name": "Llama 3.2 1B",
                    "type": "llm",
                    "format": "gguf",
                    "path": "${NOA_ROOT}/opt/models/llama-3.2-1b-instruct-q4_k_m.gguf",
                    "size_gb": 0.7,
                    "context_length": 128000,
                    "enabled": true
                },
                {
                    "id": "llama-3.2-3b",
                    "name": "Llama 3.2 3B",
                    "type": "llm",
                    "format": "gguf",
                    "path": "${NOA_ROOT}/opt/models/llama-3.2-3b-instruct-q4_k_m.gguf",
                    "size_gb": 2.0,
                    "context_length": 128000,
                    "enabled": true
                },
                {
                    "id": "phi-3-mini",
                    "name": "Phi-3 Mini",
                    "type": "llm",
                    "format": "gguf",
                    "path": "${NOA_ROOT}/opt/models/phi-3-mini-4k-instruct-q4_k_m.gguf",
                    "size_gb": 2.3,
                    "context_length": 4096,
                    "enabled": true
                }
            ]
        });

        let content = serde_json::to_string_pretty(&config)
            .map_err(|e| NoaError::Serialization(e.to_string()))?;
        fs::write(&path, content)?;

        info!(path = %path.display(), "Generated models.json");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_generate_all_configs() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(NoaPaths::config(root)).unwrap();

        ConfigGenerator::generate_all(root).unwrap();

        assert!(NoaPaths::config(root).join("ai-providers.json").exists());
        assert!(NoaPaths::config(root).join("noa-server.json").exists());
        assert!(NoaPaths::config(root).join("features.json").exists());
        assert!(NoaPaths::config(root).join("models.json").exists());
    }
}
