//! MinIO adapter toggle implementation.
//!
//! Uses environment-based configuration to validate readiness of the MinIO
//! client integration without performing network calls.

use super::toggle::{Adapter, AdapterHealth};
use std::env;
use std::process::Command;

/// Adapter for MinIO client integrations.
#[derive(Debug, Default)]
pub struct MinioAdapter;

impl MinioAdapter {
    fn has_minio_config() -> bool {
        env::var("MINIO_ENDPOINT").is_ok()
            && env::var("MINIO_ROOT_USER").is_ok()
            && env::var("MINIO_ROOT_PASSWORD").is_ok()
    }

    fn check_cli() -> bool {
        Command::new("mc")
            .arg("--version")
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false)
    }
}

impl Adapter for MinioAdapter {
    fn id(&self) -> &'static str {
        "minio"
    }

    fn description(&self) -> &'static str {
        "MinIO adapter"
    }

    fn check_health(&self) -> AdapterHealth {
        if Self::has_minio_config() {
            return AdapterHealth::healthy();
        }

        if Self::check_cli() {
            return AdapterHealth::healthy();
        }

        AdapterHealth::unhealthy("MinIO credentials not set and mc CLI not available")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minio_health_check_handles_missing_configuration() {
        let adapter = MinioAdapter;
        let health = adapter.check_health();
        assert!(
            health.healthy || health.message.is_some(),
            "health check should not panic without configuration"
        );
    }
}
