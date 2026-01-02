use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub metrics: MetricsConfig,
    pub tracing: TracingConfig,
    pub logging: LoggingConfig,
    pub alerting: AlertingConfig,
    pub dashboards: DashboardConfig,
    pub profiling: ProfilingConfig,
    pub health_checks: HealthCheckConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub provider: String,
    pub collection_interval_seconds: u64,
    pub retention_days: u32,
    pub labels: HashMap<String, String>,
    pub metrics: Vec<MetricDefinition>,
    pub histogram_buckets: Vec<f64>,
    pub aggregation_rules: Vec<AggregationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDefinition {
    pub name: String,
    pub metric_type: MetricType,
    pub description: String,
    pub unit: String,
    pub labels: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationRule {
    pub name: String,
    pub function: AggregationFunction,
    pub window_seconds: u64,
    pub group_by: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AggregationFunction {
    Sum,
    Average,
    Max,
    Min,
    Count,
    Percentile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    pub enabled: bool,
    pub provider: String,
    pub sampling_rate: f64,
    pub max_spans: u32,
    pub span_retention_seconds: u64,
    pub propagation: PropagationConfig,
    pub instrumentation: InstrumentationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropagationConfig {
    pub enabled: bool,
    pub headers: Vec<String>,
    pub baggage: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentationConfig {
    pub http_enabled: bool,
    pub database_enabled: bool,
    pub ai_enabled: bool,
    pub compression_enabled: bool,
    pub custom_instruments: Vec<CustomInstrument>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomInstrument {
    pub name: String,
    pub description: String,
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub level: String,
    pub format: LogFormat,
    pub outputs: Vec<LogOutput>,
    pub structured: StructuredLoggingConfig,
    pub sampling: LogSamplingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Json,
    Pretty,
    Structured,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogOutput {
    pub output_type: OutputType,
    pub config: HashMap<String, serde_json::Value>,
    pub filters: LogFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputType {
    Console,
    File,
    Syslog,
    Elasticsearch,
    Loki,
    CloudWatch,
    Stackdriver,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFilters {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub min_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredLoggingConfig {
    pub enabled: bool,
    pub fields: Vec<LogField>,
    pub context_propagation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogField {
    pub name: String,
    pub field_type: String,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogSamplingConfig {
    pub enabled: bool,
    pub rate: f64,
    pub policies: Vec<SamplingPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingPolicy {
    pub name: String,
    pub condition: String,
    pub rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    pub enabled: bool,
    pub provider: String,
    pub rules: Vec<AlertRule>,
    pub notification_channels: Vec<NotificationChannel>,
    pub suppression: AlertSuppressionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub name: String,
    pub condition: AlertCondition,
    pub severity: AlertSeverity,
    pub duration_seconds: u64,
    pub notification_channels: Vec<String>,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCondition {
    pub metric: String,
    pub operator: crate::config::compression_configs::ComparisonOperator,
    pub threshold: f64,
    pub comparison_window_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlertSeverity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    pub name: String,
    pub channel_type: NotificationType,
    pub config: HashMap<String, serde_json::Value>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationType {
    Email,
    Slack,
    PagerDuty,
    Webhook,
    SMS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertSuppressionConfig {
    pub enabled: bool,
    pub maintenance_windows: Vec<MaintenanceWindow>,
    pub dependency_rules: Vec<DependencyRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    pub name: String,
    pub start_time: String,
    pub end_time: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyRule {
    pub alert_name: String,
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub enabled: bool,
    pub provider: String,
    pub dashboards: Vec<DashboardDefinition>,
    pub refresh_interval_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardDefinition {
    pub name: String,
    pub description: String,
    pub panels: Vec<PanelDefinition>,
    pub variables: Vec<VariableDefinition>,
    pub time_range: TimeRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelDefinition {
    pub name: String,
    pub panel_type: String,
    pub query: String,
    pub visualization: VisualizationConfig,
    pub alerts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationConfig {
    pub chart_type: String,
    pub axes: AxesConfig,
    pub legend: LegendConfig,
    pub thresholds: Vec<Threshold>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxesConfig {
    pub x_axis: AxisConfig,
    pub y_axis: AxisConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxisConfig {
    pub label: String,
    pub unit: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub scale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegendConfig {
    pub enabled: bool,
    pub position: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threshold {
    pub value: f64,
    pub color: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDefinition {
    pub name: String,
    pub variable_type: String,
    pub query: String,
    pub default_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingConfig {
    pub enabled: bool,
    pub provider: String,
    pub sampling_rate: f64,
    pub max_profile_duration_seconds: u64,
    pub cpu_profiling: CPUProfilingConfig,
    pub memory_profiling: MemoryProfilingConfig,
    pub io_profiling: IOProfilingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUProfilingConfig {
    pub enabled: bool,
    pub sample_frequency_hz: u32,
    pub call_graph_depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProfilingConfig {
    pub enabled: bool,
    pub allocation_tracking: bool,
    pub leak_detection: bool,
    pub garbage_collection_tracking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOProfilingConfig {
    pub enabled: bool,
    pub track_file_operations: bool,
    pub track_network_operations: bool,
    pub track_database_operations: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub interval_seconds: u64,
    pub timeout_seconds: u64,
    pub checks: Vec<HealthCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub check_type: HealthCheckType,
    pub config: HashMap<String, serde_json::Value>,
    pub critical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthCheckType {
    HTTP,
    Database,
    Cache,
    DiskSpace,
    Memory,
    Custom,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics: MetricsConfig {
                enabled: true,
                provider: "prometheus".to_string(),
                collection_interval_seconds: 15,
                retention_days: 30,
                labels: HashMap::from([
                    ("service".to_string(), "rust-lovable".to_string()),
                    ("environment".to_string(), "development".to_string()),
                ]),
                metrics: vec![
                    MetricDefinition {
                        name: "http_requests_total".to_string(),
                        metric_type: MetricType::Counter,
                        description: "Total number of HTTP requests".to_string(),
                        unit: "requests".to_string(),
                        labels: vec![
                            "method".to_string(),
                            "path".to_string(),
                            "status".to_string(),
                        ],
                        enabled: true,
                    },
                    MetricDefinition {
                        name: "ai_request_duration_seconds".to_string(),
                        metric_type: MetricType::Histogram,
                        description: "AI request duration in seconds".to_string(),
                        unit: "seconds".to_string(),
                        labels: vec!["provider".to_string(), "model".to_string()],
                        enabled: true,
                    },
                ],
                histogram_buckets: vec![
                    0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
                ],
                aggregation_rules: vec![],
            },
            tracing: TracingConfig {
                enabled: true,
                provider: "jaeger".to_string(),
                sampling_rate: 0.1,
                max_spans: 10000,
                span_retention_seconds: 3600,
                propagation: PropagationConfig {
                    enabled: true,
                    headers: vec!["traceparent".to_string(), "baggage".to_string()],
                    baggage: true,
                },
                instrumentation: InstrumentationConfig {
                    http_enabled: true,
                    database_enabled: true,
                    ai_enabled: true,
                    compression_enabled: true,
                    custom_instruments: vec![],
                },
            },
            logging: LoggingConfig {
                enabled: true,
                level: "info".to_string(),
                format: LogFormat::Json,
                outputs: vec![LogOutput {
                    output_type: OutputType::Console,
                    config: HashMap::new(),
                    filters: LogFilters {
                        include: vec![],
                        exclude: vec![],
                        min_level: "info".to_string(),
                    },
                }],
                structured: StructuredLoggingConfig {
                    enabled: true,
                    fields: vec![
                        LogField {
                            name: "timestamp".to_string(),
                            field_type: "datetime".to_string(),
                            required: true,
                            default_value: None,
                        },
                        LogField {
                            name: "level".to_string(),
                            field_type: "string".to_string(),
                            required: true,
                            default_value: None,
                        },
                        LogField {
                            name: "message".to_string(),
                            field_type: "string".to_string(),
                            required: true,
                            default_value: None,
                        },
                    ],
                    context_propagation: true,
                },
                sampling: LogSamplingConfig {
                    enabled: false,
                    rate: 1.0,
                    policies: vec![],
                },
            },
            alerting: AlertingConfig {
                enabled: false,
                provider: "alertmanager".to_string(),
                rules: vec![],
                notification_channels: vec![],
                suppression: AlertSuppressionConfig {
                    enabled: true,
                    maintenance_windows: vec![],
                    dependency_rules: vec![],
                },
            },
            dashboards: DashboardConfig {
                enabled: false,
                provider: "grafana".to_string(),
                dashboards: vec![],
                refresh_interval_seconds: 30,
            },
            profiling: ProfilingConfig {
                enabled: false,
                provider: "pyroscope".to_string(),
                sampling_rate: 0.1,
                max_profile_duration_seconds: 300,
                cpu_profiling: CPUProfilingConfig {
                    enabled: true,
                    sample_frequency_hz: 100,
                    call_graph_depth: 128,
                },
                memory_profiling: MemoryProfilingConfig {
                    enabled: true,
                    allocation_tracking: true,
                    leak_detection: true,
                    garbage_collection_tracking: false,
                },
                io_profiling: IOProfilingConfig {
                    enabled: false,
                    track_file_operations: true,
                    track_network_operations: true,
                    track_database_operations: true,
                },
            },
            health_checks: HealthCheckConfig {
                enabled: true,
                endpoint: "/health".to_string(),
                interval_seconds: 30,
                timeout_seconds: 10,
                checks: vec![
                    HealthCheck {
                        name: "database".to_string(),
                        check_type: HealthCheckType::Database,
                        config: HashMap::new(),
                        critical: true,
                    },
                    HealthCheck {
                        name: "cache".to_string(),
                        check_type: HealthCheckType::Cache,
                        config: HashMap::new(),
                        critical: false,
                    },
                ],
            },
        }
    }
}
