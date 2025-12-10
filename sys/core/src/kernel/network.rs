//! Network helpers for kernel independence.
//!
//! Provides safe probes to check whether network capabilities are available
//! without assuming platform-specific tooling.

use crate::error::{NoaError, Result};
use std::net::TcpListener;

/// Network utilities.
#[derive(Debug, Default)]
pub struct NetworkOps;

impl NetworkOps {
    /// Check whether a TCP port can be bound on loopback.
    pub fn can_bind(&self, port: u16) -> bool {
        TcpListener::bind(("127.0.0.1", port)).is_ok()
    }

    /// Find an available ephemeral port on loopback.
    pub fn find_ephemeral_port(&self) -> Result<u16> {
        let listener = TcpListener::bind("127.0.0.1:0").map_err(NoaError::from)?;
        let port = listener.local_addr().map_err(NoaError::from)?.port();
        Ok(port)
    }

    /// Quick check for loopback availability.
    pub fn loopback_available(&self) -> bool {
        TcpListener::bind("127.0.0.1:0").is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_ephemeral_port() {
        let ops = NetworkOps::default();
        let port = ops.find_ephemeral_port().unwrap();
        assert!(port > 0);
    }
}
