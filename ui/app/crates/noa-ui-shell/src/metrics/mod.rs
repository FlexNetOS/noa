//! Metrics feature module.
//!
//! Provides system metrics and monitoring.

mod metrics_page;
mod status_card;
mod charts;

pub use metrics_page::MetricsPage;
pub use status_card::StatusCard;
pub use charts::MetricChart;
