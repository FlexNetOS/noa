use crate::connectors::base::Connector;
use crate::connectors::ConnectorState;

/// Stub Google connector used for placeholder integration.
#[derive(Debug, Default)]
pub struct GoogleConnector;

impl Connector for GoogleConnector {
    fn name(&self) -> &str {
        "google"
    }

    fn state(&self) -> ConnectorState {
        ConnectorState::ready("google")
    }
}
