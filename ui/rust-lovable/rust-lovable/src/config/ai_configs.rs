use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub default_provider: String,
    pub providers: AIProviders,
    pub prompts: PromptLibrary,
    pub cache: CacheConfig,
    pub embeddings: EmbeddingsConfig,
    pub rate_limits: HashMap<String, RateLimitConfig>,
    pub fallback_strategy: FallbackStrategy,
    pub context_management: ContextManagementConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviders {
    pub openai: OpenAIConfig,
    pub anthropic: AnthropicConfig,
    pub groq: GroqConfig,
    pub local: LocalAIConfig,
    pub custom: Vec<CustomProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    pub enabled: bool,
    pub api_key: Option<String>,
    pub base_url: String,
    pub models: HashMap<String, ModelConfig>,
    pub default_model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    pub enabled: bool,
    pub api_key: Option<String>,
    pub base_url: String,
    pub models: HashMap<String, ModelConfig>,
    pub default_model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqConfig {
    pub enabled: bool,
    pub api_key: Option<String>,
    pub base_url: String,
    pub models: HashMap<String, ModelConfig>,
    pub default_model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalAIConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub models: HashMap<String, ModelConfig>,
    pub default_model: String,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomProviderConfig {
    pub name: String,
    pub enabled: bool,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub headers: HashMap<String, String>,
    pub models: HashMap<String, ModelConfig>,
    pub default_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub max_tokens: u32,
    pub context_window: u32,
    pub input_cost_per_1k_tokens: f64,
    pub output_cost_per_1k_tokens: f64,
    pub supports_functions: bool,
    pub supports_vision: bool,
    pub supports_streaming: bool,
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub tokens_per_minute: u32,
    pub burst_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptLibrary {
    pub system_prompts: HashMap<String, String>,
    pub user_prompts: HashMap<String, String>,
    pub templates: HashMap<String, PromptTemplate>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub name: String,
    pub template: String,
    pub variables: Vec<String>,
    pub category: String,
    pub version: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub backend: CacheBackend,
    pub ttl_seconds: u64,
    pub max_entries: usize,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CacheBackend {
    Memory,
    Redis {
        url: String,
        pool_size: u32,
    },
    Disk {
        path: String,
    },
    Hybrid {
        memory_size: usize,
        disk_path: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsConfig {
    pub enabled: bool,
    pub provider: String,
    pub model: String,
    pub dimensions: usize,
    pub batch_size: usize,
    pub vector_database: VectorDatabaseConfig,
    pub indexing: IndexingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDatabaseConfig {
    pub provider: String,
    pub connection_string: String,
    pub collection_name: String,
    pub distance_metric: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingConfig {
    pub enabled: bool,
    pub index_type: String,
    pub m: usize,
    pub ef_construction: usize,
    pub ef_search: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FallbackStrategy {
    RoundRobin,
    Priority,
    CostOptimized,
    PerformanceOptimized,
    ReliabilityOptimized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextManagementConfig {
    pub max_context_tokens: u32,
    pub context_compression_enabled: bool,
    pub summarization_threshold: u32,
    pub context_pruning_strategy: String,
    pub conversation_memory_enabled: bool,
    pub memory_retention_days: u32,
}

impl Default for AIConfig {
    fn default() -> Self {
        let mut openai_models = HashMap::new();
        openai_models.insert(
            "gpt-4".to_string(),
            ModelConfig {
                name: "gpt-4".to_string(),
                max_tokens: 8192,
                context_window: 128000,
                input_cost_per_1k_tokens: 0.03,
                output_cost_per_1k_tokens: 0.06,
                supports_functions: true,
                supports_vision: true,
                supports_streaming: true,
                rate_limit: RateLimitConfig {
                    requests_per_minute: 10000,
                    tokens_per_minute: 1000000,
                    burst_size: 500,
                },
            },
        );

        openai_models.insert(
            "gpt-4-turbo".to_string(),
            ModelConfig {
                name: "gpt-4-turbo".to_string(),
                max_tokens: 4096,
                context_window: 128000,
                input_cost_per_1k_tokens: 0.01,
                output_cost_per_1k_tokens: 0.03,
                supports_functions: true,
                supports_vision: true,
                supports_streaming: true,
                rate_limit: RateLimitConfig {
                    requests_per_minute: 10000,
                    tokens_per_minute: 1000000,
                    burst_size: 500,
                },
            },
        );

        let mut anthropic_models = HashMap::new();
        anthropic_models.insert(
            "claude-3-5-sonnet".to_string(),
            ModelConfig {
                name: "claude-3-5-sonnet".to_string(),
                max_tokens: 8192,
                context_window: 200000,
                input_cost_per_1k_tokens: 0.003,
                output_cost_per_1k_tokens: 0.015,
                supports_functions: true,
                supports_vision: true,
                supports_streaming: true,
                rate_limit: RateLimitConfig {
                    requests_per_minute: 1000,
                    tokens_per_minute: 40000,
                    burst_size: 100,
                },
            },
        );

        let mut prompts = HashMap::new();
        prompts.insert(
            "system".to_string(),
            "You are Rust Lovable AI, an expert Rust developer and UI designer. You help users build beautiful, functional, and performant applications using Rust and Dioxus.".to_string(),
        );

        prompts.insert(
            "ui_generation".to_string(),
            "Generate a Dioxus UI component based on the user's request. Focus on clean, idiomatic Rust code with proper error handling and accessibility.".to_string(),
        );

        let mut templates = HashMap::new();
        templates.insert(
            "component".to_string(),
            PromptTemplate {
                name: "component".to_string(),
                template: "Create a Dioxus component called {{component_name}} that {{description}}. The component should be responsive and accessible.".to_string(),
                variables: vec!["component_name".to_string(), "description".to_string()],
                category: "ui_generation".to_string(),
                version: "1.0".to_string(),
                tags: vec!["dioxus".to_string(), "component".to_string(), "ui".to_string()],
            },
        );

        let mut rate_limits = HashMap::new();
        rate_limits.insert(
            "default".to_string(),
            RateLimitConfig {
                requests_per_minute: 60,
                tokens_per_minute: 10000,
                burst_size: 10,
            },
        );

        Self {
            default_provider: "openai".to_string(),
            providers: AIProviders {
                openai: OpenAIConfig {
                    enabled: true,
                    api_key: None,
                    base_url: "https://api.openai.com/v1".to_string(),
                    models: openai_models,
                    default_model: "gpt-4-turbo".to_string(),
                    max_tokens: 4096,
                    temperature: 0.7,
                    top_p: 0.9,
                    frequency_penalty: 0.0,
                    presence_penalty: 0.0,
                },
                anthropic: AnthropicConfig {
                    enabled: true,
                    api_key: None,
                    base_url: "https://api.anthropic.com".to_string(),
                    models: anthropic_models,
                    default_model: "claude-3-5-sonnet".to_string(),
                    max_tokens: 4096,
                    temperature: 0.7,
                    top_p: 0.9,
                    top_k: None,
                },
                groq: GroqConfig {
                    enabled: false,
                    api_key: None,
                    base_url: "https://api.groq.com/openai/v1".to_string(),
                    models: HashMap::new(),
                    default_model: "".to_string(),
                    max_tokens: 4096,
                    temperature: 0.7,
                    top_p: 0.9,
                },
                local: LocalAIConfig {
                    enabled: false,
                    endpoint: "http://localhost:8080".to_string(),
                    models: HashMap::new(),
                    default_model: "".to_string(),
                    timeout_seconds: 300,
                },
                custom: Vec::new(),
            },
            prompts: PromptLibrary {
                system_prompts: prompts,
                user_prompts: HashMap::new(),
                templates,
                version: "1.0".to_string(),
            },
            cache: CacheConfig {
                enabled: true,
                backend: CacheBackend::Memory,
                ttl_seconds: 3600,
                max_entries: 10000,
                compression_enabled: true,
                encryption_enabled: false,
            },
            embeddings: EmbeddingsConfig {
                enabled: false,
                provider: "openai".to_string(),
                model: "text-embedding-3-small".to_string(),
                dimensions: 1536,
                batch_size: 100,
                vector_database: VectorDatabaseConfig {
                    provider: "pgvector".to_string(),
                    connection_string: "postgresql://localhost:5432/rust_lovable".to_string(),
                    collection_name: "embeddings".to_string(),
                    distance_metric: "cosine".to_string(),
                },
                indexing: IndexingConfig {
                    enabled: true,
                    index_type: "hnsw".to_string(),
                    m: 16,
                    ef_construction: 64,
                    ef_search: 40,
                },
            },
            rate_limits,
            fallback_strategy: FallbackStrategy::ReliabilityOptimized,
            context_management: ContextManagementConfig {
                max_context_tokens: 128000,
                context_compression_enabled: true,
                summarization_threshold: 100000,
                context_pruning_strategy: "smart".to_string(),
                conversation_memory_enabled: true,
                memory_retention_days: 30,
            },
        }
    }
}
