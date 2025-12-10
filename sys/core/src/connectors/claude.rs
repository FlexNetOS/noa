use crate::connectors::base::Connector;
use crate::connectors::ConnectorState;

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

