//! Feature flag service
//!
//! Responsible for loading, querying, and persisting boolean feature flags
//! defined in `config/features.json`. Flags are stored as flat names with
//! optional categories (e.g., `connectors.github`).

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::expand_env_vars;
use crate::error::{NoaError, Result};

/// Individual feature flag definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlag {
    pub name: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
}

impl FeatureFlag {
    pub fn new(name: impl Into<String>, enabled: bool, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            enabled,
            description: Some(description.into()),
            category: None,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct FeatureFile {
    #[serde(rename = "$schema")]
    schema: Option<String>,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    feature_flags: Vec<FeatureFlag>,
    #[serde(default)]
    providers: HashMap<String, bool>,
    #[serde(default)]
    connectors: HashMap<String, bool>,
}

/// In-memory representation of loaded flags
#[derive(Debug, Clone)]
pub struct FeatureFlagStore {
    path: PathBuf,
    flags: Vec<FeatureFlag>,
}

impl FeatureFlagStore {
    /// Load feature flags from config/features.json, creating the file with defaults if missing.
    pub fn load(noa_root: Option<&Path>) -> Result<Self> {
        let root = noa_root
            .map(|p| p.to_path_buf())
            .or_else(|| std::env::var("NOA_ROOT").ok().map(PathBuf::from))
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        let path = root.join("config/features.json");
        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(NoaError::from)?;
            }
            let defaults = FeatureFlagStore {
                path: path.clone(),
                flags: default_flags(),
            };
            defaults.persist()?;
            return Ok(defaults);
        }

        let content = fs::read_to_string(&path).map_err(NoaError::from)?;
        let expanded = expand_env_vars(&content);
        let parsed: serde_json::Value =
            serde_json::from_str(&expanded).map_err(|e| NoaError::Serialization(e.to_string()))?;

        let mut flags = collect_flags(&parsed);
        if flags.is_empty() {
            flags = default_flags();
        }

        Ok(Self { path, flags })
    }

    /// List all feature flags
    pub fn list(&self) -> &[FeatureFlag] {
        &self.flags
    }

    /// Check if a flag is enabled
    pub fn is_enabled(&self, name: &str) -> bool {
        self.flags.iter().find(|f| f.name == name).map(|f| f.enabled).unwrap_or(false)
    }

    /// Enable or disable a flag and persist to disk
    pub fn set(&mut self, name: &str, enabled: bool) -> Result<()> {
        if let Some(flag) = self.flags.iter_mut().find(|f| f.name == name) {
            flag.enabled = enabled;
        } else {
            self.flags.push(FeatureFlag::new(
                name.to_string(),
                enabled,
                format!("Dynamically added flag {}", name),
            ));
        }
        self.persist()
    }

    fn persist(&self) -> Result<()> {
        let file = build_feature_file(&self.flags);
        let json = serde_json::to_string_pretty(&file)
            .map_err(|e| NoaError::Serialization(e.to_string()))?;
        fs::write(&self.path, json).map_err(NoaError::from)
    }
}

fn collect_flags(parsed: &serde_json::Value) -> Vec<FeatureFlag> {
    let mut flags: Vec<FeatureFlag> = Vec::new();

    if let Ok(file) = serde_json::from_value::<FeatureFile>(parsed.clone()) {
        flags.extend(file.feature_flags);

        for (name, enabled) in file.providers {
            flags.push(FeatureFlag {
                name: format!("providers.{}", name),
                enabled,
                description: Some("Provider-level toggle".to_string()),
                category: Some("providers".to_string()),
            });
        }

        for (name, enabled) in file.connectors {
            flags.push(FeatureFlag {
                name: format!("connectors.{}", name),
                enabled,
                description: Some("Connector toggle".to_string()),
                category: Some("connectors".to_string()),
            });
        }
    }

    // If feature_flags array not present, attempt to derive from top-level object of booleans
    if flags.is_empty() {
        if let Some(obj) = parsed.as_object() {
            for (key, value) in obj {
                if let Some(enabled) = value.as_bool() {
                    flags.push(FeatureFlag {
                        name: key.clone(),
                        enabled,
                        description: None,
                        category: None,
                    });
                }
            }
        }
    }

    flags
}

fn build_feature_file(flags: &[FeatureFlag]) -> FeatureFileSerializable {
    let mut feature_flags = Vec::new();
    let mut providers = HashMap::new();
    let mut connectors = HashMap::new();

    for flag in flags {
        if let Some(stripped) = flag.name.strip_prefix("providers.") {
            providers.insert(stripped.to_string(), flag.enabled);
        } else if let Some(stripped) = flag.name.strip_prefix("connectors.") {
            connectors.insert(stripped.to_string(), flag.enabled);
        } else {
            feature_flags.push(flag.clone());
        }
    }

    FeatureFileSerializable {
        schema: "https://noa.local/schemas/features.json".to_string(),
        version: "1.0.0".to_string(),
        feature_flags,
        providers,
        connectors,
    }
}

fn default_flags() -> Vec<FeatureFlag> {
    vec![
        FeatureFlag::new(
            "connectors.enabled",
            true,
            "Master switch for all external connectors",
        ),
        FeatureFlag::new(
            "connectors.github",
            true,
            "Enable GitHub connector for repositories and issues",
        ),
        FeatureFlag::new("connectors.google", true, "Enable Google/Gmail connector"),
        FeatureFlag::new("connectors.openai", true, "Enable OpenAI connector"),
        FeatureFlag::new(
            "connectors.claude",
            true,
            "Enable Anthropic Claude connector",
        ),
        FeatureFlag::new(
            "connectors.cloud_storage",
            true,
            "Enable cloud storage connectors (S3/GCS)",
        ),
        FeatureFlag::new(
            "connectors.email",
            false,
            "Enable SMTP/IMAP email connector",
        ),
        FeatureFlag::new(
            "connectors.offline_cache",
            true,
            "Allow cached data when offline",
        ),
        FeatureFlag::new(
            "connectors.network_monitor",
            true,
            "Monitor connectivity for graceful degradation",
        ),
        FeatureFlag::new(
            "connectors.status_dashboard",
            true,
            "Expose connector health/status to UI",
        ),
        FeatureFlag::new(
            "connectors.oauth",
            true,
            "Enable OAuth authentication flows for connectors",
        ),
    ]
}

#[derive(Debug, Serialize)]
struct FeatureFileSerializable {
    #[serde(rename = "$schema")]
    schema: String,
    version: String,
    feature_flags: Vec<FeatureFlag>,
    providers: HashMap<String, bool>,
    connectors: HashMap<String, bool>,
}
