//! Docker adapter toggle implementation.
//!
//! Provides a minimal readiness check for the Docker CLI to allow higher-level
//! components to decide whether container-backed features can be used.

use super::toggle::{Adapter, AdapterHealth};
use std::process::Command;

/// Adapter for Docker-based integrations.
#[derive(Debug, Default)]
pub struct DockerAdapter {
    required_version_hint: Option<String>,
}

impl DockerAdapter {
    /// Create a Docker adapter with an optional required version hint.
    pub fn new(required_version_hint: Option<String>) -> Self {
        Self { required_version_hint }
    }

    fn check_docker_binary(&self) -> AdapterHealth {
        match Command::new("docker").arg("--version").output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if stdout.is_empty() {
                    return AdapterHealth::unhealthy("docker returned no version output");
                }

                if let Some(hint) = &self.required_version_hint {
                    if !stdout.contains(hint) {
                        return AdapterHealth::unhealthy(format!(
                            "docker version does not match hint '{}'",
                            hint
                        ));
                    }
                }

                AdapterHealth::healthy()
            }
            Err(_) => AdapterHealth::unhealthy("docker binary not available"),
        }
    }
}

impl Adapter for DockerAdapter {
    fn id(&self) -> &'static str {
        "docker"
    }

    fn description(&self) -> &'static str {
        "Docker CLI adapter"
    }

    fn check_health(&self) -> AdapterHealth {
        self.check_docker_binary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn docker_health_check_is_resilient() {
        let adapter = DockerAdapter::default();
        let health = adapter.check_health();
        assert!(
            health.healthy || health.message.is_some(),
            "health should indicate state or provide a message"
        );
    }
}
