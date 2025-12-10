use crate::connectors::base::Connector;
use crate::connectors::ConnectorState;

#[derive(Debug, Default)]
pub struct EmailConnector;

impl Connector for EmailConnector {
    fn name(&self) -> &str {
        "email"
    }

    fn state(&self) -> ConnectorState {
        ConnectorState::ready("email")
    }
}

