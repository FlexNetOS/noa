use crate::connectors::base::Connector;
use crate::connectors::ConnectorState;

/// Stub Claude connector used for placeholder integration.
#[derive(Debug, Default)]
pub struct ClaudeConnector;

impl Connector for ClaudeConnector {
    fn name(&self) -> &str {
        "claude"
    }

    fn state(&self) -> ConnectorState {
        ConnectorState::ready("claude")
    }
}
