//! NOA Structured Logging
//!
//! Provides structured logging with tracing for observability.
//! §3.5: Audit logging
//! FR-155: Observability - tracing, metrics, logging

use std::path::Path;

use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    prelude::*,
    EnvFilter,
};

use crate::config::{LogFormat, LogLevel, LoggingConfig};
use crate::error::Result;

/// Initialize logging subsystem
pub fn init(config: &LoggingConfig) -> Result<()> {
    let filter = match config.level {
        LogLevel::Trace => EnvFilter::new("trace"),
        LogLevel::Debug => EnvFilter::new("debug"),
        LogLevel::Info => EnvFilter::new("info"),
        LogLevel::Warn => EnvFilter::new("warn"),
        LogLevel::Error => EnvFilter::new("error"),
    };

    let subscriber = tracing_subscriber::registry().with(filter);

    match config.format {
        LogFormat::Json => {
            let layer = fmt::layer()
                .json()
                .with_span_events(FmtSpan::CLOSE)
                .with_current_span(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_file(true)
                .with_line_number(true);

            subscriber.with(layer).try_init().map_err(|e| {
                crate::error::NoaError::Internal {
                    message: format!("Failed to initialize logging: {}", e),
                    source: None,
                }
            })?;
        }
        LogFormat::Text => {
            let layer = fmt::layer()
                .with_span_events(FmtSpan::CLOSE)
                .compact();

            subscriber.with(layer).try_init().map_err(|e| {
                crate::error::NoaError::Internal {
                    message: format!("Failed to initialize logging: {}", e),
                    source: None,
                }
            })?;
        }
        LogFormat::Pretty => {
            let layer = fmt::layer()
                .with_span_events(FmtSpan::CLOSE)
                .pretty();

            subscriber.with(layer).try_init().map_err(|e| {
                crate::error::NoaError::Internal {
                    message: format!("Failed to initialize logging: {}", e),
                    source: None,
                }
            })?;
        }
    }

    Ok(())
}

/// Initialize logging with file output
pub fn init_with_file(config: &LoggingConfig) -> Result<()> {
    use tracing_appender::rolling::{RollingFileAppender, Rotation};

    // Create parent directory if needed
    if let Some(parent) = config.output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        config.output.parent().unwrap_or(Path::new(".")),
        config.output.file_name().unwrap_or_default(),
    );

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let filter = match config.level {
        LogLevel::Trace => EnvFilter::new("trace"),
        LogLevel::Debug => EnvFilter::new("debug"),
        LogLevel::Info => EnvFilter::new("info"),
        LogLevel::Warn => EnvFilter::new("warn"),
        LogLevel::Error => EnvFilter::new("error"),
    };

    let file_layer = fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::CLOSE)
        .with_current_span(true);

    let console_layer = fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .compact();

    tracing_subscriber::registry()
        .with(filter)
        .with(file_layer)
        .with(console_layer)
        .try_init()
        .map_err(|e| {
            crate::error::NoaError::Internal {
                message: format!("Failed to initialize logging: {}", e),
                source: None,
            }
        })?;

    Ok(())
}

/// Structured log event for audit trail
#[derive(Debug)]
pub struct AuditEvent {
    pub action: String,
    pub actor: String,
    pub resource: String,
    pub outcome: AuditOutcome,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy)]
pub enum AuditOutcome {
    Success,
    Failure,
    Denied,
}

impl AuditEvent {
    pub fn new(action: impl Into<String>, actor: impl Into<String>, resource: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            actor: actor.into(),
            resource: resource.into(),
            outcome: AuditOutcome::Success,
            details: None,
        }
    }

    pub fn with_outcome(mut self, outcome: AuditOutcome) -> Self {
        self.outcome = outcome;
        self
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    /// Log this audit event
    pub fn log(&self) {
        let outcome = match self.outcome {
            AuditOutcome::Success => "success",
            AuditOutcome::Failure => "failure",
            AuditOutcome::Denied => "denied",
        };

        tracing::info!(
            target: "audit",
            action = %self.action,
            actor = %self.actor,
            resource = %self.resource,
            outcome = outcome,
            details = ?self.details,
            "audit_event"
        );
    }
}

/// Log a database query for audit
#[macro_export]
macro_rules! log_query {
    ($query:expr, $duration_ms:expr) => {
        tracing::debug!(
            target: "database",
            query = %$query,
            duration_ms = $duration_ms,
            "database_query"
        );
    };
}

/// Log an agent action
#[macro_export]
macro_rules! log_agent_action {
    ($agent:expr, $action:expr, $status:expr) => {
        tracing::info!(
            target: "agent",
            agent = %$agent,
            action = %$action,
            status = %$status,
            "agent_action"
        );
    };
    ($agent:expr, $action:expr, $status:expr, $($field:tt)*) => {
        tracing::info!(
            target: "agent",
            agent = %$agent,
            action = %$action,
            status = %$status,
            $($field)*,
            "agent_action"
        );
    };
}

/// Log an API request
#[macro_export]
macro_rules! log_request {
    ($method:expr, $path:expr, $status:expr, $duration_ms:expr) => {
        tracing::info!(
            target: "api",
            method = %$method,
            path = %$path,
            status = $status,
            duration_ms = $duration_ms,
            "http_request"
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_event() {
        let event = AuditEvent::new("create", "user-123", "agent/TestAgent")
            .with_outcome(AuditOutcome::Success)
            .with_details(serde_json::json!({"version": "1.0.0"}));

        assert_eq!(event.action, "create");
        assert!(matches!(event.outcome, AuditOutcome::Success));
    }
}

