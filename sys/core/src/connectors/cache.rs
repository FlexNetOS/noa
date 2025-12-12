use crate::error::Result;
use crate::connectors::ConnectorState;
use chrono::Duration;

/// Placeholder connector cache.
#[derive(Debug, Default)]
pub struct ConnectorCache;

impl ConnectorCache {
    pub fn new(_path: Option<&str>) -> Result<Self> {
        Ok(Self)
    }

    pub fn store(&self, _state: &ConnectorState) -> Result<()> {
        Ok(())
    }

    pub fn get(&self, _name: &str, _max_age: Duration) -> Option<ConnectorState> {
        None
    }
}
