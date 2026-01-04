//! AI provider configsuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIconfigs {
    pub default_provider: String,
    pub providers: AIProviders,
    pub cache: Cacheconfigs,
    pub embeddings: Embeddingsconfigs,
    pub fallback_strategy: FallbackStrategy,
}

/// AI provider configsurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviders {
    pub openai: OpenAIconfigs,
    pub anthropic: Anthropicconfigs,
    pub local: LocalAIconfigs,
    pub custom: Vec<CustomProviderconfigs>,
}

/// OpenAI provider configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIconfigs {
    pub enabled: bool,
    pub api_key: Option<String>,
    pub base_url: String,
    pub default_model: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

/// Anthropic provider configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anthropicconfigs {
    pub enabled: bool,
    pub api_key: Option<String>,
    pub base_url: String,
    pub default_model: String,
    pub max_tokens: u32,
    pub temperature: f32,
}

/// Local AI provider configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalAIconfigs {
    pub enabled: bool,
    pub endpoint: String,
    pub default_model: String,
    pub timeout_seconds: u64,
}

/// Custom provider configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomProviderconfigs {
    pub name: String,
    pub enabled: bool,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub headers: HashMap<String, String>,
    pub default_model: String,
}

/// Cache configsuration for AI responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cacheconfigs {
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

/// Embeddings configsuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embeddingsconfigs {
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

impl Default for AIconfigs {
    fn default() -> Self {
        Self {
            default_provider: "local".to_string(),
            providers: AIProviders::default(),
            cache: Cacheconfigs::default(),
            embeddings: Embeddingsconfigs::default(),
            fallback_strategy: FallbackStrategy::default(),
        }
    }
}

impl Default for AIProviders {
    fn default() -> Self {
        Self {
            openai: OpenAIconfigs::default(),
            anthropic: Anthropicconfigs::default(),
            local: LocalAIconfigs::default(),
            custom: Vec::new(),
        }
    }
}

impl Default for OpenAIconfigs {
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

impl Default for Anthropicconfigs {
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

impl Default for LocalAIconfigs {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "http://localhost:11434".to_string(),
            default_model: "llama3.2".to_string(),
            timeout_seconds: 120,
        }
    }
}

impl Default for Cacheconfigs {
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

impl Default for Embeddingsconfigs {
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
