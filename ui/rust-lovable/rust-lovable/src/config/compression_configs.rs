use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    pub enabled: bool,
    pub default_algorithm: String,
    pub algorithms: HashMap<String, AlgorithmConfig>,
    pub strategies: Vec<CompressionStrategy>,
    pub thresholds: CompressionThresholds,
    pub monitoring: CompressionMonitoringConfig,
    pub ml_specific: MLCompressionConfig,
    pub streaming: StreamingCompressionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmConfig {
    pub enabled: bool,
    pub level: CompressionLevel,
    pub settings: HashMap<String, serde_json::Value>,
    pub use_cases: Vec<String>,
    pub performance: PerformanceProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionLevel {
    pub min: u32,
    pub max: u32,
    pub default: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    pub compression_speed: SpeedProfile,
    pub decompression_speed: SpeedProfile,
    pub compression_ratio: f64,
    pub memory_usage_mb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SpeedProfile {
    VeryFast,
    Fast,
    Medium,
    Slow,
    VerySlow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionStrategy {
    pub name: String,
    pub conditions: Vec<CompressionCondition>,
    pub algorithm: String,
    pub settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionCondition {
    pub condition_type: ConditionType,
    pub operator: ComparisonOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConditionType {
    DataSize,
    ContentType,
    DataType,
    Priority,
    TimeSensitivity,
    ResourceAvailability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComparisonOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    In,
    NotIn,
    Between,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionThresholds {
    pub min_size_bytes: usize,
    pub max_size_bytes: Option<usize>,
    pub compression_ratio_threshold: f64,
    pub time_limit_ms: u64,
    pub memory_limit_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionMonitoringConfig {
    pub enabled: bool,
    pub metrics: Vec<CompressionMetric>,
    pub sampling_rate: f64,
    pub alerting: CompressionAlertingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionMetric {
    pub name: String,
    pub metric_type: MetricType,
    pub labels: HashMap<String, String>,
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
pub struct CompressionAlertingConfig {
    pub enabled: bool,
    pub thresholds: HashMap<String, f64>,
    pub channels: Vec<AlertChannel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannel {
    pub channel_type: String,
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLCompressionConfig {
    pub enabled: bool,
    pub embeddings: EmbeddingsCompressionConfig,
    pub models: ModelCompressionConfig,
    pub prompts: PromptCompressionConfig,
    pub gradients: GradientCompressionConfig,
    pub activations: ActivationCompressionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsCompressionConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub quantization_bits: u8,
    pub dimensionality_reduction: bool,
    pub target_dimensions: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCompressionConfig {
    pub enabled: bool,
    pub pruning: PruningConfig,
    pub quantization: QuantizationConfig,
    pub knowledge_distillation: KnowledgeDistillationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PruningConfig {
    pub enabled: bool,
    pub method: String,
    pub sparsity_target: f64,
    pub structured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantizationConfig {
    pub enabled: bool,
    pub bits: u8,
    pub method: String,
    pub calibration_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeDistillationConfig {
    pub enabled: bool,
    pub teacher_student_ratio: f64,
    pub temperature: f64,
    pub alpha: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptCompressionConfig {
    pub enabled: bool,
    pub tokenization_optimization: bool,
    pub semantic_compression: bool,
    pub cache_compressed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientCompressionConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub error_feedback: bool,
    pub top_k_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationCompressionConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub checkpointing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingCompressionConfig {
    pub enabled: bool,
    pub chunk_size_bytes: usize,
    pub sliding_window_size: usize,
    pub dictionary_training: DictionaryTrainingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryTrainingConfig {
    pub enabled: bool,
    pub sample_size: usize,
    pub training_iterations: u32,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        let mut algorithms = HashMap::new();

        algorithms.insert(
            "zstd".to_string(),
            AlgorithmConfig {
                enabled: true,
                level: CompressionLevel {
                    min: 1,
                    max: 22,
                    default: 3,
                },
                settings: HashMap::from([
                    (
                        "window_log".to_string(),
                        serde_json::Value::Number(23.into()),
                    ),
                    ("hash_log".to_string(), serde_json::Value::Number(20.into())),
                ]),
                use_cases: vec![
                    "general_purpose".to_string(),
                    "ml_models".to_string(),
                    "embeddings".to_string(),
                ],
                performance: PerformanceProfile {
                    compression_speed: SpeedProfile::Fast,
                    decompression_speed: SpeedProfile::VeryFast,
                    compression_ratio: 3.5,
                    memory_usage_mb: 32.0,
                },
            },
        );

        algorithms.insert(
            "brotli".to_string(),
            AlgorithmConfig {
                enabled: true,
                level: CompressionLevel {
                    min: 0,
                    max: 11,
                    default: 6,
                },
                settings: HashMap::from([
                    ("window".to_string(), serde_json::Value::Number(22.into())),
                    ("quality".to_string(), serde_json::Value::Number(8.into())),
                ]),
                use_cases: vec![
                    "web_content".to_string(),
                    "text_data".to_string(),
                    "static_assets".to_string(),
                ],
                performance: PerformanceProfile {
                    compression_speed: SpeedProfile::Medium,
                    decompression_speed: SpeedProfile::VeryFast,
                    compression_ratio: 4.0,
                    memory_usage_mb: 16.0,
                },
            },
        );

        algorithms.insert(
            "gzip".to_string(),
            AlgorithmConfig {
                enabled: true,
                level: CompressionLevel {
                    min: 1,
                    max: 9,
                    default: 6,
                },
                settings: HashMap::new(),
                use_cases: vec![
                    "compatibility".to_string(),
                    "general_purpose".to_string(),
                    "legacy_systems".to_string(),
                ],
                performance: PerformanceProfile {
                    compression_speed: SpeedProfile::Medium,
                    decompression_speed: SpeedProfile::Fast,
                    compression_ratio: 2.8,
                    memory_usage_mb: 8.0,
                },
            },
        );

        algorithms.insert(
            "lz4".to_string(),
            AlgorithmConfig {
                enabled: true,
                level: CompressionLevel {
                    min: 1,
                    max: 16,
                    default: 9,
                },
                settings: HashMap::new(),
                use_cases: vec![
                    "real_time".to_string(),
                    "streaming".to_string(),
                    "low_latency".to_string(),
                ],
                performance: PerformanceProfile {
                    compression_speed: SpeedProfile::VeryFast,
                    decompression_speed: SpeedProfile::VeryFast,
                    compression_ratio: 2.2,
                    memory_usage_mb: 4.0,
                },
            },
        );

        Self {
            enabled: true,
            default_algorithm: "zstd".to_string(),
            algorithms,
            strategies: vec![
                CompressionStrategy {
                    name: "ml_embeddings".to_string(),
                    conditions: vec![CompressionCondition {
                        condition_type: ConditionType::DataType,
                        operator: ComparisonOperator::Equal,
                        value: serde_json::Value::String("embeddings".to_string()),
                    }],
                    algorithm: "zstd".to_string(),
                    settings: HashMap::from([(
                        "level".to_string(),
                        serde_json::Value::Number(6.into()),
                    )]),
                },
                CompressionStrategy {
                    name: "real_time_streaming".to_string(),
                    conditions: vec![CompressionCondition {
                        condition_type: ConditionType::TimeSensitivity,
                        operator: ComparisonOperator::Equal,
                        value: serde_json::Value::String("high".to_string()),
                    }],
                    algorithm: "lz4".to_string(),
                    settings: HashMap::new(),
                },
            ],
            thresholds: CompressionThresholds {
                min_size_bytes: 1024,
                max_size_bytes: Some(1024 * 1024 * 1024), // 1GB
                compression_ratio_threshold: 1.1,
                time_limit_ms: 5000,
                memory_limit_mb: 512,
            },
            monitoring: CompressionMonitoringConfig {
                enabled: true,
                metrics: vec![
                    CompressionMetric {
                        name: "compression_ratio".to_string(),
                        metric_type: MetricType::Histogram,
                        labels: HashMap::new(),
                    },
                    CompressionMetric {
                        name: "compression_time_ms".to_string(),
                        metric_type: MetricType::Histogram,
                        labels: HashMap::new(),
                    },
                ],
                sampling_rate: 1.0,
                alerting: CompressionAlertingConfig {
                    enabled: false,
                    thresholds: HashMap::new(),
                    channels: vec![],
                },
            },
            ml_specific: MLCompressionConfig {
                enabled: true,
                embeddings: EmbeddingsCompressionConfig {
                    enabled: true,
                    algorithm: "zstd".to_string(),
                    quantization_bits: 8,
                    dimensionality_reduction: false,
                    target_dimensions: None,
                },
                models: ModelCompressionConfig {
                    enabled: false,
                    pruning: PruningConfig {
                        enabled: false,
                        method: "magnitude".to_string(),
                        sparsity_target: 0.5,
                        structured: false,
                    },
                    quantization: QuantizationConfig {
                        enabled: false,
                        bits: 8,
                        method: "dynamic".to_string(),
                        calibration_method: "min_max".to_string(),
                    },
                    knowledge_distillation: KnowledgeDistillationConfig {
                        enabled: false,
                        teacher_student_ratio: 0.1,
                        temperature: 4.0,
                        alpha: 0.7,
                    },
                },
                prompts: PromptCompressionConfig {
                    enabled: true,
                    tokenization_optimization: true,
                    semantic_compression: false,
                    cache_compressed: true,
                },
                gradients: GradientCompressionConfig {
                    enabled: false,
                    algorithm: "top_k".to_string(),
                    error_feedback: true,
                    top_k_ratio: 0.1,
                },
                activations: ActivationCompressionConfig {
                    enabled: false,
                    algorithm: "checkpointing".to_string(),
                    checkpointing: true,
                },
            },
            streaming: StreamingCompressionConfig {
                enabled: true,
                chunk_size_bytes: 64 * 1024, // 64KB
                sliding_window_size: 64 * 1024,
                dictionary_training: DictionaryTrainingConfig {
                    enabled: false,
                    sample_size: 10000,
                    training_iterations: 100,
                },
            },
        }
    }
}
