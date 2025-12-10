//! Prometheus Metrics Export
//!
//! Provides metrics collection and Prometheus export for NOA.
//! §3.5: Observability
//! FR-155: Observability - tracing, metrics, logging

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use lazy_static::lazy_static;
use prometheus::{
    self, Counter, CounterVec, Encoder, Gauge, GaugeVec, Histogram, HistogramOpts, HistogramVec,
    IntCounter, IntCounterVec, IntGauge, IntGaugeVec, Opts, Registry, TextEncoder,
};

use crate::error::Result;

lazy_static! {
    /// Global metrics registry
    pub static ref REGISTRY: Registry = Registry::new();

    // HTTP metrics
    pub static ref HTTP_REQUESTS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("noa_http_requests_total", "Total HTTP requests"),
        &["method", "path", "status"]
    ).expect("metric can be created");

    pub static ref HTTP_REQUEST_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new("noa_http_request_duration_seconds", "HTTP request duration in seconds")
            .buckets(vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]),
        &["method", "path"]
    ).expect("metric can be created");

    // Database metrics
    pub static ref DB_QUERIES_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("noa_db_queries_total", "Total database queries"),
        &["operation", "table"]
    ).expect("metric can be created");

    pub static ref DB_QUERY_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new("noa_db_query_duration_seconds", "Database query duration in seconds")
            .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]),
        &["operation"]
    ).expect("metric can be created");

    pub static ref DB_CONNECTIONS_ACTIVE: IntGauge = IntGauge::new(
        "noa_db_connections_active", "Active database connections"
    ).expect("metric can be created");

    pub static ref DB_CONNECTIONS_IDLE: IntGauge = IntGauge::new(
        "noa_db_connections_idle", "Idle database connections"
    ).expect("metric can be created");

    // Agent metrics
    pub static ref AGENT_ACTIONS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("noa_agent_actions_total", "Total agent actions"),
        &["agent", "action", "status"]
    ).expect("metric can be created");

    pub static ref AGENT_ACTION_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new("noa_agent_action_duration_seconds", "Agent action duration")
            .buckets(vec![0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0, 120.0]),
        &["agent", "action"]
    ).expect("metric can be created");

    pub static ref AGENTS_ACTIVE: IntGauge = IntGauge::new(
        "noa_agents_active", "Currently active agents"
    ).expect("metric can be created");

    // Provider metrics
    pub static ref PROVIDER_REQUESTS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("noa_provider_requests_total", "Total provider API requests"),
        &["provider", "model", "status"]
    ).expect("metric can be created");

    pub static ref PROVIDER_TOKENS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("noa_provider_tokens_total", "Total tokens processed"),
        &["provider", "model", "type"]
    ).expect("metric can be created");

    pub static ref PROVIDER_LATENCY: HistogramVec = HistogramVec::new(
        HistogramOpts::new("noa_provider_latency_seconds", "Provider API latency")
            .buckets(vec![0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 30.0, 60.0]),
        &["provider", "model"]
    ).expect("metric can be created");

    // Task metrics
    pub static ref TASKS_TOTAL: IntCounterVec = IntCounterVec::new(
        Opts::new("noa_tasks_total", "Total tasks processed"),
        &["type", "status"]
    ).expect("metric can be created");

    pub static ref TASKS_QUEUE_SIZE: IntGauge = IntGauge::new(
        "noa_tasks_queue_size", "Current task queue size"
    ).expect("metric can be created");

    // Memory metrics
    pub static ref MEMORY_ENTRIES: IntGauge = IntGauge::new(
        "noa_memory_entries_total", "Total memory entries"
    ).expect("metric can be created");

    pub static ref EMBEDDINGS_TOTAL: IntGauge = IntGauge::new(
        "noa_embeddings_total", "Total embeddings stored"
    ).expect("metric can be created");

    // System metrics
    pub static ref UPTIME_SECONDS: Gauge = Gauge::new(
        "noa_uptime_seconds", "NOA uptime in seconds"
    ).expect("metric can be created");

    pub static ref BUILD_INFO: IntGaugeVec = IntGaugeVec::new(
        Opts::new("noa_build_info", "NOA build information"),
        &["version", "commit", "rust_version"]
    ).expect("metric can be created");
}

/// Initialize the metrics system
pub fn init_metrics() -> Result<()> {
    // Register all metrics with the registry
    REGISTRY.register(Box::new(HTTP_REQUESTS_TOTAL.clone()))?;
    REGISTRY.register(Box::new(HTTP_REQUEST_DURATION.clone()))?;
    REGISTRY.register(Box::new(DB_QUERIES_TOTAL.clone()))?;
    REGISTRY.register(Box::new(DB_QUERY_DURATION.clone()))?;
    REGISTRY.register(Box::new(DB_CONNECTIONS_ACTIVE.clone()))?;
    REGISTRY.register(Box::new(DB_CONNECTIONS_IDLE.clone()))?;
    REGISTRY.register(Box::new(AGENT_ACTIONS_TOTAL.clone()))?;
    REGISTRY.register(Box::new(AGENT_ACTION_DURATION.clone()))?;
    REGISTRY.register(Box::new(AGENTS_ACTIVE.clone()))?;
    REGISTRY.register(Box::new(PROVIDER_REQUESTS_TOTAL.clone()))?;
    REGISTRY.register(Box::new(PROVIDER_TOKENS_TOTAL.clone()))?;
    REGISTRY.register(Box::new(PROVIDER_LATENCY.clone()))?;
    REGISTRY.register(Box::new(TASKS_TOTAL.clone()))?;
    REGISTRY.register(Box::new(TASKS_QUEUE_SIZE.clone()))?;
    REGISTRY.register(Box::new(MEMORY_ENTRIES.clone()))?;
    REGISTRY.register(Box::new(EMBEDDINGS_TOTAL.clone()))?;
    REGISTRY.register(Box::new(UPTIME_SECONDS.clone()))?;
    REGISTRY.register(Box::new(BUILD_INFO.clone()))?;

    // Set build info
    BUILD_INFO
        .with_label_values(&[
            env!("CARGO_PKG_VERSION"),
            option_env!("GIT_COMMIT").unwrap_or("unknown"),
            option_env!("RUSTC_VERSION").unwrap_or("unknown"),
        ])
        .set(1);

    tracing::info!("Metrics system initialized");
    Ok(())
}

/// Get metrics in Prometheus text format
pub fn get_metrics() -> Result<String> {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer)?;
    Ok(String::from_utf8(buffer)?)
}

/// Record an HTTP request
pub fn record_http_request(method: &str, path: &str, status: u16, duration: Duration) {
    HTTP_REQUESTS_TOTAL
        .with_label_values(&[method, path, &status.to_string()])
        .inc();

    HTTP_REQUEST_DURATION
        .with_label_values(&[method, path])
        .observe(duration.as_secs_f64());
}

/// Record a database query
pub fn record_db_query(operation: &str, table: &str, duration: Duration) {
    DB_QUERIES_TOTAL.with_label_values(&[operation, table]).inc();

    DB_QUERY_DURATION
        .with_label_values(&[operation])
        .observe(duration.as_secs_f64());
}

/// Record an agent action
pub fn record_agent_action(agent: &str, action: &str, status: &str, duration: Duration) {
    AGENT_ACTIONS_TOTAL.with_label_values(&[agent, action, status]).inc();

    AGENT_ACTION_DURATION
        .with_label_values(&[agent, action])
        .observe(duration.as_secs_f64());
}

/// Record a provider request
pub fn record_provider_request(
    provider: &str,
    model: &str,
    status: &str,
    input_tokens: u64,
    output_tokens: u64,
    duration: Duration,
) {
    PROVIDER_REQUESTS_TOTAL.with_label_values(&[provider, model, status]).inc();

    PROVIDER_TOKENS_TOTAL
        .with_label_values(&[provider, model, "input"])
        .inc_by(input_tokens);

    PROVIDER_TOKENS_TOTAL
        .with_label_values(&[provider, model, "output"])
        .inc_by(output_tokens);

    PROVIDER_LATENCY
        .with_label_values(&[provider, model])
        .observe(duration.as_secs_f64());
}

/// Update database connection metrics
pub fn update_db_connections(active: i64, idle: i64) {
    DB_CONNECTIONS_ACTIVE.set(active);
    DB_CONNECTIONS_IDLE.set(idle);
}

/// Timer helper for measuring duration
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

/// Axum handler for /metrics endpoint
pub async fn metrics_handler() -> impl axum::response::IntoResponse {
    match get_metrics() {
        Ok(metrics) => (
            axum::http::StatusCode::OK,
            [(
                axum::http::header::CONTENT_TYPE,
                "text/plain; version=0.0.4",
            )],
            metrics,
        ),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            [(axum::http::header::CONTENT_TYPE, "text/plain")],
            format!("Error: {}", e),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        let timer = Timer::start();
        std::thread::sleep(Duration::from_millis(10));
        assert!(timer.elapsed() >= Duration::from_millis(10));
    }

    #[test]
    fn test_record_http_request() {
        // Just verify it doesn't panic
        record_http_request("GET", "/api/v1/health", 200, Duration::from_millis(50));
    }
}
