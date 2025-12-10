use async_trait::async_trait;

use crate::connectors::base::{Connector, ConnectorContext};
use crate::connectors::ConnectorState;
use crate::error::Result;

/// Stub GitHub connector for buildability; full OAuth integration to be added later.
#[derive(Debug, Default)]
pub struct GithubConnector;

impl GithubConnector {
    pub fn new(_config: crate::connectors::oauth::client::OAuthClientConfig) -> Result<Self> {
        Ok(Self::default())
    }
}

#[async_trait]
impl Connector for GithubConnector {
    fn name(&self) -> &str {
        "github"
    }

    fn feature_flag(&self) -> &str {
        "connectors.github"
    }

    async fn status(&self, _ctx: &ConnectorContext) -> Result<ConnectorState> {
        Ok(ConnectorState::ready(self.name()))
    }
}
