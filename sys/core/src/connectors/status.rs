use crate::connectors::ConnectorState;
use serde::{Deserialize, Serialize};

/// Summary of connector health across providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorStatusReport {
    pub states: Vec<ConnectorState>,
}

impl ConnectorStatusReport {
    pub fn new(states: Vec<ConnectorState>) -> Self {
        Self { states }
    }
}

