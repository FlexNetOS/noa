use crate::connectors::base::Connector;
use crate::connectors::ConnectorState;

#[derive(Debug, Default)]
pub struct CacheConnector;

impl Connector for CacheConnector {
    fn name(&self) -> &str {
        "cache"
    }

    fn state(&self) -> ConnectorState {
        ConnectorState::ready("cache")
    }
}

