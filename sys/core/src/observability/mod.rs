//! NOA Observability Module
//!
//! Provides logging, tracing, and metrics collection.
//! §3.5: Observability
//! FR-155: Observability - tracing, metrics, logging

pub mod logging;
pub mod metrics;
pub mod telemetry;

pub use logging::init_logging;
pub use metrics::{get_metrics, init_metrics, metrics_handler};
pub use telemetry::{init_telemetry, shutdown_telemetry};
