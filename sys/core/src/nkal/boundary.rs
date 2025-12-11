use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{ConfigError, NoaError, Result, ValidationError};
use crate::kernel::nkal::KernelMode;

/// Capability policy loaded from `config/nkal-capabilities.json`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CapabilityPolicy {
    /// Schema version (informational)
    pub version: Option<String>,
    /// Capability mapping keyed by kernel mode name.
    #[serde(default)]
    pub modes: HashMap<String, ModeCapability>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ModeCapability {
    /// Capabilities granted for this mode.
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// Explicit denials that override grants.
    #[serde(default)]
    pub denies: Vec<String>,
    /// Per-provider overrides.
    #[serde(default)]
    pub providers: HashMap<String, ProviderOverride>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ProviderOverride {
    /// Capabilities granted to the provider in this mode.
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// Explicit denials for the provider in this mode.
    #[serde(default)]
    pub denies: Vec<String>,
}

/// Result of a boundary enforcement check.
#[derive(Debug, Clone)]
pub struct BoundaryDecision {
    pub allowed: bool,
    pub reason: String,
}

/// Validates NKAL boundary operations against capability policy.
pub struct BoundaryValidator {
    policy: CapabilityPolicy,
    source: PathBuf,
}

impl BoundaryValidator {
    /// Load capability policy from disk.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let data = std::fs::read_to_string(path).map_err(|e| {
            NoaError::Config(ConfigError::ParseError {
                path: path.display().to_string(),
                error: e.to_string(),
            })
        })?;

        let policy: CapabilityPolicy = serde_json::from_str(&data).map_err(|e| {
            NoaError::Config(ConfigError::ParseError {
                path: path.display().to_string(),
                error: e.to_string(),
            })
        })?;

        Ok(Self {
            policy,
            source: path.to_path_buf(),
        })
    }

    /// Check whether an action is allowed for the given mode/provider.
    pub fn allows(&self, mode: KernelMode, action: &str, provider: Option<&str>) -> bool {
        self.enforce(mode, action, provider)
            .map(|d| d.allowed)
            .unwrap_or(false)
    }

    /// Enforce the boundary policy and return a decision.
    pub fn enforce(
        &self,
        mode: KernelMode,
        action: &str,
        provider: Option<&str>,
    ) -> Result<BoundaryDecision> {
        let mode_key = mode.to_string();
        let Some(mode_caps) = self.policy.modes.get(&mode_key) else {
            return Err(NoaError::Validation(ValidationError::new(
                "mode",
                format!("No capability grants for mode {}", mode_key),
                "NKAL_MODE_UNCONFIGURED",
            )));
        };

        // Provider-specific overrides take precedence over mode-level grants.
        if let Some(provider_name) = provider {
            if let Some(p_override) = mode_caps.providers.get(provider_name) {
                if p_override.denies.iter().any(|d| d == action) {
                    return Ok(BoundaryDecision {
                        allowed: false,
                        reason: format!(
                            "Denied by provider override '{}' in mode {}",
                            provider_name, mode_key
                        ),
                    });
                }

                if p_override.capabilities.iter().any(|c| c == action) {
                    return Ok(BoundaryDecision {
                        allowed: true,
                        reason: format!(
                            "Allowed by provider override '{}' in mode {}",
                            provider_name, mode_key
                        ),
                    });
                }
            }
        }

        if mode_caps.denies.iter().any(|d| d == action) {
            return Ok(BoundaryDecision {
                allowed: false,
                reason: format!(
                    "Denied by mode-level rule for {} in {} (source: {})",
                    action,
                    mode_key,
                    self.source.display()
                ),
            });
        }

        if mode_caps.capabilities.iter().any(|c| c == action) {
            return Ok(BoundaryDecision {
                allowed: true,
                reason: format!(
                    "Allowed by mode-level rule for {} in {} (source: {})",
                    action,
                    mode_key,
                    self.source.display()
                ),
            });
        }

        Ok(BoundaryDecision {
            allowed: false,
            reason: format!(
                "Capability '{}' not granted for mode {} (source: {})",
                action,
                mode_key,
                self.source.display()
            ),
        })
    }

    /// Expose underlying policy (useful for inspection in status/debug output).
    pub fn policy(&self) -> &CapabilityPolicy {
        &self.policy
    }
}
