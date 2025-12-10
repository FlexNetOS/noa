use async_trait::async_trait;
use chrono::Duration;

use crate::connectors::base::{Connector, ConnectorContext};
use crate::connectors::cache::ConnectorCache;
use crate::connectors::network;
use crate::connectors::ConnectorState;
use crate::error::Result;

/// Cloud storage connector (S3/GCS/MinIO)
pub struct CloudStorageConnector;

impl CloudStorageConnector {
    pub fn new() -> Self {
        Self
    }

    fn has_credentials(&self) -> bool {
        let aws = std::env::var("AWS_ACCESS_KEY_ID").is_ok()
            && std::env::var("AWS_SECRET_ACCESS_KEY").is_ok();
        let gcs = std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok();
        let minio = std::env::var("MINIO_ENDPOINT").is_ok()
            && std::env::var("MINIO_ACCESS_KEY").is_ok()
            && std::env::var("MINIO_SECRET_KEY").is_ok();
        aws || gcs || minio
    }
}

#[async_trait]
impl Connector for CloudStorageConnector {
    fn name(&self) -> &str {
        "cloud_storage"
    }

    fn feature_flag(&self) -> &str {
        "connectors.cloud_storage"
    }

    async fn authorize_url(&self, _ctx: &ConnectorContext) -> Result<Option<String>> {
        Ok(None)
    }

    async fn status(&self, ctx: &ConnectorContext) -> Result<ConnectorState> {
        let cache = ConnectorCache::new(None)?;
        if !ctx.is_enabled(self.feature_flag()) {
            let state = ConnectorState::disabled(self.name());
            cache.store(&state)?;
            return Ok(state);
        }

        let net = network::check_connectivity();
        if !net.available {
            if let Some(cached) = cache.get(self.name(), Duration::minutes(10)) {
                return Ok(cached);
            }
            let state = ConnectorState::offline(self.name(), "Network unavailable");
            cache.store(&state)?;
            return Ok(state);
        }

        let state = if self.has_credentials() {
            ConnectorState::ready(self.name())
        } else {
            ConnectorState::degraded(
                self.name(),
                "Missing S3/GCS/MinIO credentials in environment",
            )
        };
        cache.store(&state)?;
        Ok(state)
    }
}
