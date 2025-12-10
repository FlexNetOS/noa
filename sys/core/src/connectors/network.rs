use std::net::TcpStream;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};

/// Lightweight network availability snapshot
#[derive(Debug, Clone)]
pub struct NetworkStatus {
    pub available: bool,
    pub latency_ms: Option<u128>,
    pub last_checked: DateTime<Utc>,
}

impl NetworkStatus {
    pub fn offline() -> Self {
        Self {
            available: false,
            latency_ms: None,
            last_checked: Utc::now(),
        }
    }
}

/// Check whether outbound network connectivity is available.
/// Falls back to the `NOA_OFFLINE` override to disable probing.
pub fn check_connectivity() -> NetworkStatus {
    if std::env::var("NOA_OFFLINE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        return NetworkStatus::offline();
    }

    let start = Instant::now();
    let target = "1.1.1.1:80";
    let available = TcpStream::connect_timeout(
        &target
            .parse()
            .unwrap_or_else(|_| "8.8.8.8:53".parse().unwrap()),
        Duration::from_millis(500),
    )
    .is_ok();

    NetworkStatus {
        available,
        latency_ms: Some(start.elapsed().as_millis()),
        last_checked: Utc::now(),
    }
}
