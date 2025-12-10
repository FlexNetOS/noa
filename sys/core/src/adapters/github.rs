//! GitHub adapter toggle implementation.
//!
//! Performs lightweight checks for GitHub CLI availability or token presence to
//! help decide when GitHub-backed features can be enabled.

use super::toggle::{Adapter, AdapterHealth};
use std::process::Command;

/// Adapter for GitHub integrations.
#[derive(Debug, Default)]
pub struct GithubAdapter;

impl GithubAdapter {
    fn check_token_or_cli(&self) -> AdapterHealth {
        if std::env::var("GITHUB_TOKEN").is_ok() {
            return AdapterHealth::healthy();
        }

        match Command::new("gh").arg("--version").output() {
            Ok(output) => {
                if output.status.success() {
                    AdapterHealth::healthy()
                } else {
                    AdapterHealth::unhealthy("gh CLI returned non-zero status")
                }
            }
            Err(_) => AdapterHealth::unhealthy("gh CLI not available and GITHUB_TOKEN not set"),
        }
    }
}

impl Adapter for GithubAdapter {
    fn id(&self) -> &'static str {
        "github"
    }

    fn description(&self) -> &'static str {
        "GitHub adapter"
    }

    fn check_health(&self) -> AdapterHealth {
        self.check_token_or_cli()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn github_health_check_handles_missing_cli() {
        let adapter = GithubAdapter;
        let health = adapter.check_health();
        assert!(
            health.healthy || health.message.is_some(),
            "health should be reported even if CLI is missing"
        );
    }
}
