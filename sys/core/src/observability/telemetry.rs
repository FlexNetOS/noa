//! OpenTelemetry OTLP Export
//!
//! Sets up OpenTelemetry tracing and metrics export.
//! §3.5: Observability
//! FR-155: Observability - tracing, metrics, logging

use std::time::Duration;

use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    runtime,
    trace::{BatchConfig, RandomIdGenerator, Sampler, TracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_VERSION};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, Registry};

use crate::error::Result;

/// Telemetry configuration
#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    /// Service name for traces
    pub service_name: String,

    /// Service version
    pub service_version: String,

    /// OTLP endpoint (e.g., http://localhost:4317)
    pub otlp_endpoint: String,

    /// Enable tracing
    pub tracing_enabled: bool,

    /// Enable metrics
    pub metrics_enabled: bool,

    /// Sampling ratio (0.0 to 1.0)
    pub sampling_ratio: f64,

    /// Batch export timeout in milliseconds
    pub export_timeout_ms: u64,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            service_name: "noa-core".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            otlp_endpoint: std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:4317".to_string()),
            tracing_enabled: true,
            metrics_enabled: true,
            sampling_ratio: 1.0,
            export_timeout_ms: 30000,
        }
    }
}

/// Initialize OpenTelemetry with OTLP export
pub fn init_telemetry(config: &TelemetryConfig) -> Result<()> {
    if !config.tracing_enabled {
        tracing::info!("OpenTelemetry tracing disabled");
        return Ok(());
    }

    // Create resource with service info
    let resource = Resource::new(vec![
        KeyValue::new(SERVICE_NAME, config.service_name.clone()),
        KeyValue::new(SERVICE_VERSION, config.service_version.clone()),
        KeyValue::new("deployment.environment",
            std::env::var("NOA_ENV").unwrap_or_else(|_| "development".to_string())),
    ]);

    // Create OTLP exporter
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(&config.otlp_endpoint)
        .with_timeout(Duration::from_millis(config.export_timeout_ms));

    // Create tracer provider
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            opentelemetry_sdk::trace::Config::default()
                .with_sampler(Sampler::TraceIdRatioBased(config.sampling_ratio))
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(resource),
        )
        .with_batch_config(BatchConfig::default())
        .install_batch(runtime::Tokio)
        .map_err(|e| crate::error::NoaError::Internal {
            message: format!("Failed to initialize OpenTelemetry: {}", e),
            source: None,
        })?;

    // Create OpenTelemetry layer for tracing
    let telemetry_layer = OpenTelemetryLayer::new(tracer);

    // Get existing subscriber and add telemetry layer
    // Note: This should be called after logging is initialized
    tracing::subscriber::set_global_default(
        Registry::default().with(telemetry_layer)
    ).map_err(|e| crate::error::NoaError::Internal {
        message: format!("Failed to set subscriber: {}", e),
        source: None,
    })?;

    tracing::info!(
        endpoint = %config.otlp_endpoint,
        service = %config.service_name,
        "OpenTelemetry initialized"
    );

    Ok(())
}

/// Shutdown OpenTelemetry gracefully
pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
    tracing::info!("OpenTelemetry shutdown complete");
}

/// Create a traced span for a task
pub fn task_span(task_id: &str, task_name: &str) -> tracing::Span {
    tracing::info_span!(
        "task",
        otel.name = %task_name,
        task.id = %task_id,
        task.name = %task_name,
    )
}

/// Create a traced span for an agent action
pub fn agent_span(agent_name: &str, action: &str) -> tracing::Span {
    tracing::info_span!(
        "agent_action",
        otel.name = %format!("{}.{}", agent_name, action),
        agent.name = %agent_name,
        agent.action = %action,
    )
}

/// Create a traced span for a database operation
pub fn db_span(operation: &str, table: &str) -> tracing::Span {
    tracing::info_span!(
        "db_operation",
        otel.name = %format!("db.{}", operation),
        db.operation = %operation,
        db.table = %table,
    )
}

/// Create a traced span for an HTTP request
pub fn http_span(method: &str, path: &str) -> tracing::Span {
    tracing::info_span!(
        "http_request",
        otel.name = %format!("{} {}", method, path),
        http.method = %method,
        http.target = %path,
    )
}

/// Create a traced span for a provider call
pub fn provider_span(provider: &str, model: &str, operation: &str) -> tracing::Span {
    tracing::info_span!(
        "provider_call",
        otel.name = %format!("{}.{}", provider, operation),
        provider.name = %provider,
        provider.model = %model,
        provider.operation = %operation,
    )
}

/// Record metrics for a completed operation
pub fn record_operation_metrics(
    operation: &str,
    duration_ms: u64,
    success: bool,
    attributes: &[(String, String)],
) {
    tracing::debug!(
        target: "metrics",
        operation = %operation,
        duration_ms = duration_ms,
        success = success,
        attributes = ?attributes,
        "operation_metrics"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_config_default() {
        let config = TelemetryConfig::default();
        assert_eq!(config.service_name, "noa-core");
        assert!(config.tracing_enabled);
        assert!(config.metrics_enabled);
    }

    #[test]
    fn test_span_creation() {
        let _span = task_span("task-123", "test-task");
        let _span = agent_span("TestAgent", "execute");
        let _span = db_span("SELECT", "agents");
        // Verify they don't panic
    }
}

