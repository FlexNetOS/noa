use crate::connectors::base::Connector;
use crate::connectors::ConnectorState;

/// Stub email connector.
#[derive(Debug, Default)]
pub struct EmailConnector;

impl EmailConnector {
    pub fn new() -> Self {
        Self
    }
}

impl Connector for EmailConnector {
    fn name(&self) -> &str {
        "email"
    }

    fn state(&self) -> ConnectorState {
        ConnectorState::ready(self.name())
    }
}
