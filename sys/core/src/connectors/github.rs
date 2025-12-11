use crate::connectors::base::Connector;
use crate::connectors::ConnectorState;

/// Stub GitHub connector.
#[derive(Debug, Default)]
pub struct GithubConnector;

impl GithubConnector {
    pub fn new() -> Self {
        Self
    }
}

impl Connector for GithubConnector {
    fn name(&self) -> &str {
        "github"
    }

    fn state(&self) -> ConnectorState {
        ConnectorState::ready(self.name())
    }
}
