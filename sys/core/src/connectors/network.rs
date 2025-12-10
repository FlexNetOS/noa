use crate::connectors::base::Connector;
use crate::connectors::ConnectorState;

#[derive(Debug, Default)]
pub struct NetworkConnector;

impl Connector for NetworkConnector {
    fn name(&self) -> &str {
        "network"
    }

    fn state(&self) -> ConnectorState {
        ConnectorState::ready("network")
    }
}

