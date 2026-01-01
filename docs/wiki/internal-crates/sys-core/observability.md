# observability Module

Metrics, tracing, and monitoring.

**Location**: `sys/core/src/observability/`  
**Feature**: `full`

## Overview

Comprehensive observability stack:

- Prometheus metrics
- OpenTelemetry tracing
- Health checks
- Alerting integration

## Key Types

### Metrics

Prometheus metrics registry.

```rust
pub struct Metrics {
    registry: Registry,
    
    // Counters
    pub requests_total: IntCounter,
    pub errors_total: IntCounter,
    pub tasks_completed: IntCounter,
    
    // Gauges
    pub agents_active: IntGauge,
    pub memory_usage: IntGauge,
    pub model_loaded: IntGauge,
    
    // Histograms
    pub request_duration: Histogram,
    pub inference_duration: Histogram,
}
```

### TracingConfig

OpenTelemetry configuration.

```rust
pub struct TracingConfig {
    pub service_name: String,
    pub otlp_endpoint: Option<String>,
    pub sample_rate: f64,
}
```

## Metrics Endpoint

```
GET /metrics
```

Example output:
```prometheus
# HELP noa_requests_total Total HTTP requests
# TYPE noa_requests_total counter
noa_requests_total{method="GET",path="/api/v1/agents"} 42

# HELP noa_agents_active Currently active agents
# TYPE noa_agents_active gauge
noa_agents_active 3

# HELP noa_request_duration_seconds Request duration histogram
# TYPE noa_request_duration_seconds histogram
noa_request_duration_seconds_bucket{le="0.01"} 100
noa_request_duration_seconds_bucket{le="0.1"} 150
```

## Tracing

Distributed tracing with spans:

```rust
use tracing::{instrument, info_span};

#[instrument(name = "process_task", skip(db))]
async fn process_task(task_id: &str, db: &Database) -> NoaResult<()> {
    let span = info_span!("validation", task_id);
    let _guard = span.enter();
    
    // Processing...
    
    Ok(())
}
```

## Health Checks

```rust
pub struct HealthChecker {
    checks: Vec<Box<dyn HealthCheck>>,
}

#[async_trait]
pub trait HealthCheck {
    fn name(&self) -> &str;
    async fn check(&self) -> HealthStatus;
}

pub enum HealthStatus {
    Healthy,
    Degraded { reason: String },
    Unhealthy { reason: String },
}
```

## Dashboards

Recommended Grafana dashboards:

| Dashboard | Purpose |
|-----------|---------|
| NOA Overview | Request rates, errors, latency |
| Agent Performance | Task completion, failures |
| Model Inference | Inference latency, throughput |
| System Resources | Memory, CPU, disk |

## Usage

```rust
use noa_core::observability::{Metrics, setup_tracing};

async fn main() -> NoaResult<()> {
    // Setup tracing
    setup_tracing(TracingConfig::default())?;
    
    // Create metrics
    let metrics = Metrics::new();
    
    // Record metric
    metrics.requests_total.inc();
    
    Ok(())
}
```

## See Also

- [logging module](logging.md) — Structured logging
- [events module](events.md) — Event system
- [healing module](healing.md) — Self-healing
