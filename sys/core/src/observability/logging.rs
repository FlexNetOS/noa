//! Structured Logging with tracing-subscriber
//!
//! Sets up logging infrastructure for NOA.
//! §3.5: Audit logging

use std::path::Path;

use tracing_subscriber::{
    fmt::{self, format::FmtSpan, time::UtcTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

use crate::config::{LogFormat, LogLevel, LoggingConfig};
use crate::error::Result;

/// Initialize the logging subsystem
pub fn init_logging(config: &LoggingConfig) -> Result<()> {
    // Build env filter
    let env_filter = build_env_filter(config.level);

    // Console layer
    let console_layer = build_console_layer(config.format);

    // Initialize subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .try_init()
        .map_err(|e| crate::error::NoaError::Internal {
            message: format!("Failed to initialize logging: {}", e),
            source: None,
        })?;

    Ok(())
}

/// Initialize logging with file output
pub fn init_logging_with_file(config: &LoggingConfig) -> Result<LogGuard> {
    // Ensure log directory exists
    if let Some(parent) = config.output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Build env filter
    let env_filter = build_env_filter(config.level);

    // File appender
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        config.output.parent().unwrap_or(Path::new(".")),
        config.output.file_name().unwrap_or_default(),
    );
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // File layer (always JSON for structured logs)
    let file_layer = fmt::layer()
        .json()
        .with_timer(UtcTime::rfc_3339())
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::CLOSE)
        .with_current_span(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    // Console layer
    let console_layer = build_console_layer(config.format);

    // Initialize subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(console_layer)
        .try_init()
        .map_err(|e| crate::error::NoaError::Internal {
            message: format!("Failed to initialize logging: {}", e),
            source: None,
        })?;

    Ok(LogGuard { _guard: guard })
}

/// Guard to keep the file writer alive
pub struct LogGuard {
    _guard: tracing_appender::non_blocking::WorkerGuard,
}

fn build_env_filter(level: LogLevel) -> EnvFilter {
    let level_str = match level {
        LogLevel::Trace => "trace",
        LogLevel::Debug => "debug",
        LogLevel::Info => "info",
        LogLevel::Warn => "warn",
        LogLevel::Error => "error",
    };

    // Allow override via RUST_LOG env var
    EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level_str))
}

fn build_console_layer<S>(format: LogFormat) -> Box<dyn Layer<S> + Send + Sync + 'static>
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    match format {
        LogFormat::Json => Box::new(
            fmt::layer()
                .json()
                .with_timer(UtcTime::rfc_3339())
                .with_span_events(FmtSpan::CLOSE)
        ),
        LogFormat::Text => Box::new(
            fmt::layer()
                .with_timer(UtcTime::rfc_3339())
                .compact()
        ),
        LogFormat::Pretty => Box::new(
            fmt::layer()
                .with_timer(UtcTime::rfc_3339())
                .pretty()
        ),
    }
}

/// Log targets for different subsystems
pub mod targets {
    pub const HTTP: &str = "http";
    pub const DATABASE: &str = "database";
    pub const AGENT: &str = "agent";
    pub const PROVIDER: &str = "provider";
    pub const AUDIT: &str = "audit";
    pub const METRICS: &str = "metrics";
    pub const SYSTEM: &str = "system";
}

/// Audit log macro for important operations
#[macro_export]
macro_rules! audit_log {
    ($action:expr, $actor:expr, $resource:expr) => {
        tracing::info!(
            target: "audit",
            action = %$action,
            actor = %$actor,
            resource = %$resource,
            timestamp = %chrono::Utc::now().to_rfc3339(),
            "audit_event"
        );
    };
    ($action:expr, $actor:expr, $resource:expr, $($field:tt)*) => {
        tracing::info!(
            target: "audit",
            action = %$action,
            actor = %$actor,
            resource = %$resource,
            timestamp = %chrono::Utc::now().to_rfc3339(),
            $($field)*,
            "audit_event"
        );
    };
}

/// Database query log macro
#[macro_export]
macro_rules! db_log {
    ($query:expr, $duration_ms:expr) => {
        tracing::debug!(
            target: "database",
            query = %$query,
            duration_ms = $duration_ms,
            "db_query"
        );
    };
}

/// Agent action log macro
#[macro_export]
macro_rules! agent_log {
    ($agent:expr, $action:expr, $status:expr) => {
        tracing::info!(
            target: "agent",
            agent = %$agent,
            action = %$action,
            status = %$status,
            "agent_action"
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_env_filter() {
        let filter = build_env_filter(LogLevel::Info);
        // Just ensure it doesn't panic
        assert!(true);
    }
}

