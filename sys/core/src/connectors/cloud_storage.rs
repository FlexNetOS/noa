use crate::connectors::base::Connector;
use crate::connectors::ConnectorState;

#[derive(Debug, Default)]
pub struct CloudStorageConnector;

impl Connector for CloudStorageConnector {
    fn name(&self) -> &str {
        "cloud_storage"
    }

    fn state(&self) -> ConnectorState {
        ConnectorState::ready("cloud_storage")
    }
}

