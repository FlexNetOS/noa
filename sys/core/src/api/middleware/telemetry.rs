//! OpenTelemetry Tracing Middleware
//!
//! Implements distributed tracing with OpenTelemetry.
//! §3.5: Observability
//! FR-155: Observability - tracing, metrics, logging

use std::time::Instant;

use axum::{
    body::Body,
    http::{Request, Response},
    middleware::Next,
};
use opentelemetry::{
    trace::{Span, SpanKind, Status, Tracer},
    Context, KeyValue,
};
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// OpenTelemetry context key for request tracing
const TRACE_PARENT_HEADER: &str = "traceparent";
const TRACE_STATE_HEADER: &str = "tracestate";

/// OpenTelemetry tracing middleware
pub async fn trace_request(
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    let start = Instant::now();

    // Extract trace context from incoming request headers
    let parent_context = extract_context(&request);

    // Get request details for span attributes
    let method = request.method().to_string();
    let uri = request.uri().to_string();
    let path = request.uri().path().to_string();
    let scheme = request.uri().scheme_str().unwrap_or("http");

    // Get request ID
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(String::from)
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    // Create span for this request
    let span = tracing::info_span!(
        "http_request",
        otel.name = %format!("{} {}", method, path),
        otel.kind = ?SpanKind::Server,
        http.method = %method,
        http.url = %uri,
        http.target = %path,
        http.scheme = %scheme,
        http.request_id = %request_id,
    );

    // Set parent context if available
    span.set_parent(parent_context);

    let _guard = span.enter();

    // Process request
    let response = next.run(request).await;

    // Record response details
    let status_code = response.status().as_u16();
    let duration_ms = start.elapsed().as_millis() as i64;

    // Add response attributes to span
    tracing::Span::current().record("http.status_code", status_code);
    tracing::Span::current().record("http.duration_ms", duration_ms);

    // Log metrics for monitoring
    record_http_metrics(&method, &path, status_code, duration_ms);

    response
}

/// Extract OpenTelemetry context from request headers
fn extract_context(request: &Request<Body>) -> Context {
    use opentelemetry::propagation::TextMapPropagator;
    use opentelemetry_sdk::propagation::TraceContextPropagator;

    let propagator = TraceContextPropagator::new();

    let extractor = HeaderExtractor {
        headers: request.headers(),
    };

    propagator.extract(&extractor)
}

/// Header extractor for OpenTelemetry propagation
struct HeaderExtractor<'a> {
    headers: &'a axum::http::HeaderMap,
}

impl<'a> opentelemetry::propagation::Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.headers.get(key).and_then(|v| v.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.headers
            .keys()
            .map(|k| k.as_str())
            .collect()
    }
}

/// Record HTTP metrics for monitoring
fn record_http_metrics(method: &str, path: &str, status_code: u16, duration_ms: i64) {
    // This integrates with the metrics system
    // Using tracing for now, would integrate with Prometheus in production

    let status_class = match status_code {
        100..=199 => "1xx",
        200..=299 => "2xx",
        300..=399 => "3xx",
        400..=499 => "4xx",
        500..=599 => "5xx",
        _ => "unknown",
    };

    tracing::debug!(
        target: "metrics.http",
        method = %method,
        path = %path,
        status_code = status_code,
        status_class = status_class,
        duration_ms = duration_ms,
        "http_request_total"
    );

    // Record histogram for duration
    tracing::debug!(
        target: "metrics.http",
        method = %method,
        path = %path,
        duration_ms = duration_ms,
        "http_request_duration_ms"
    );
}

/// Initialize OpenTelemetry tracing
pub fn init_telemetry(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    use opentelemetry::global;
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::{
        runtime,
        trace::{RandomIdGenerator, Sampler, TracerProvider},
        Resource,
    };
    use opentelemetry_semantic_conventions::resource::SERVICE_NAME;

    // Configure OTLP exporter
    let otlp_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:4317".to_string());

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(&otlp_endpoint);

    // Build tracer provider
    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(
            opentelemetry_sdk::trace::Config::default()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(Resource::new(vec![
                    KeyValue::new(SERVICE_NAME, service_name.to_string()),
                ])),
        )
        .install_batch(runtime::Tokio)?;

    global::set_tracer_provider(tracer_provider);

    Ok(())
}

/// Shutdown OpenTelemetry gracefully
pub fn shutdown_telemetry() {
    opentelemetry::global::shutdown_tracer_provider();
}

/// Create a new span for a task
pub fn create_task_span(task_name: &str, task_id: &str) -> tracing::Span {
    tracing::info_span!(
        "task",
        otel.name = %task_name,
        task.id = %task_id,
        task.name = %task_name,
    )
}

/// Create a new span for an agent action
pub fn create_agent_span(agent_name: &str, action: &str) -> tracing::Span {
    tracing::info_span!(
        "agent_action",
        otel.name = %format!("{}.{}", agent_name, action),
        agent.name = %agent_name,
        agent.action = %action,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_http_metrics() {
        // Just verify it doesn't panic
        record_http_metrics("GET", "/api/v1/health", 200, 50);
        record_http_metrics("POST", "/api/v1/agents", 201, 100);
        record_http_metrics("GET", "/api/v1/not-found", 404, 10);
        record_http_metrics("POST", "/api/v1/error", 500, 200);
    }
}

