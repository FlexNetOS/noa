use serde::Serialize;

use super::{ConnectorHealth, ConnectorState};

/// Aggregated connector health snapshot
#[derive(Debug, Serialize)]
pub struct ConnectorStatusReport {
    pub ready: Vec<String>,
    pub degraded: Vec<String>,
    pub offline: Vec<String>,
    pub unauthorized: Vec<String>,
    pub disabled: Vec<String>,
}

pub fn build_report(states: &[ConnectorState]) -> ConnectorStatusReport {
    let mut report = ConnectorStatusReport {
        ready: Vec::new(),
        degraded: Vec::new(),
        offline: Vec::new(),
        unauthorized: Vec::new(),
        disabled: Vec::new(),
    };

    for state in states {
        let bucket = match state.health {
            ConnectorHealth::Ready => &mut report.ready,
            ConnectorHealth::Degraded => &mut report.degraded,
            ConnectorHealth::Offline => &mut report.offline,
            ConnectorHealth::Unauthorized => &mut report.unauthorized,
            ConnectorHealth::Disabled => &mut report.disabled,
        };
        bucket.push(state.name.clone());
    }

    report
}

/// Render a concise text table for CLI use
pub fn print_table(states: &[ConnectorState]) {
    println!("{:<16} {:<12} {}", "connector", "health", "message");
    for state in states {
        let message = state.message.clone().unwrap_or_default();
        println!(
            "{:<16} {:<12} {}",
            state.name,
            format!("{:?}", state.health),
            message
        );
    }
}
