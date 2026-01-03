//! AI provider configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub default_provider: String,
    pub providers: AIProviders,
    pub cache: CacheConfig,
    pub embeddings: EmbeddingsConfig,
    pub fallback_strategy: FallbackStrategy,
}

/// AI provider configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviders {
    pub openai: OpenAIConfig,
    pub anthropic: AnthropicConfig,
    pub local: LocalAIConfig,
    pub custom: Vec<CustomProviderConfig>,
}

/// OpenAI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    pub enabled: bool,
    pub api_key: Option<String>,
    pub base_url: String,
    pub default_model: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

/// Anthropic provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    pub enabled: bool,
    pub api_key: Option<String>,
    pub base_url: String,
    pub default_model: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

/// Local AI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalAIConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub default_model: String,
    pub timeout_seconds: u64,
}

/// Custom provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomProviderConfig {
    pub name: String,
    pub enabled: bool,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub headers: HashMap<String, String>,
    pub default_model: String,
}

/// Cache configuration for AI responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub backend: CacheBackend,
    pub ttl_seconds: u64,
    pub max_entries: usize,
    pub compression_enabled: bool,
}

/// Cache backend types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CacheBackend {
    Memory,
    Redis { url: String },
    Disk { path: String },
}

/// Embeddings configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsConfig {
    pub enabled: bool,
    pub model: String,
    pub dimensions: usize,
    pub batch_size: usize,
    pub cache_enabled: bool,
}

/// Fallback strategy for AI providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackStrategy {
    pub enabled: bool,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub fallback_order: Vec<String>,
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            default_provider: "local".to_string(),
            providers: AIProviders::default(),
            cache: CacheConfig::default(),
            embeddings: EmbeddingsConfig::default(),
            fallback_strategy: FallbackStrategy::default(),
        }
    }
}

impl Default for AIProviders {
    fn default() -> Self {
        Self {
            openai: OpenAIConfig::default(),
            anthropic: AnthropicConfig::default(),
            local: LocalAIConfig::default(),
            custom: Vec::new(),
        }
    }
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_key: None,
            base_url: "https://api.openai.com/v1".to_string(),
            default_model: "gpt-4".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
        }
    }
}

impl Default for AnthropicConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            api_key: None,
            base_url: "https://api.anthropic.com".to_string(),
            default_model: "claude-3-5-sonnet-20241022".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
        }
    }
}

impl Default for LocalAIConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "http://localhost:11434".to_string(),
            default_model: "llama3.2".to_string(),
            timeout_seconds: 120,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            backend: CacheBackend::Memory,
            ttl_seconds: 3600,
            max_entries: 10000,
            compression_enabled: true,
        }
    }
}

impl Default for EmbeddingsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            model: "nomic-embed-text".to_string(),
            dimensions: 768,
            batch_size: 32,
            cache_enabled: true,
        }
    }
}

impl Default for FallbackStrategy {
    fn default() -> Self {
        Self {
            enabled: true,
            max_retries: 3,
            retry_delay_ms: 1000,
            fallback_order: vec!["local".to_string(), "openai".to_string(), "anthropic".to_string()],
        }
    }
}
