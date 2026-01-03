//! Fallback Chain Implementation
//!
//! Manages provider fallback: llama.cpp → copilot → anthropic → openai → git

use super::{LithoConfig, LithoError, ProviderPriority};

/// Provider status in the fallback chain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderStatus {
    Available,
    RateLimited,
    Unavailable,
    Failed,
}

/// A provider in the fallback chain
#[derive(Debug, Clone)]
pub struct FallbackProvider {
    pub priority: ProviderPriority,
    pub status: ProviderStatus,
    pub endpoint: Option<String>,
    pub model: String,
    pub retry_count: u32,
    pub last_error: Option<String>,
}

impl FallbackProvider {
    pub fn new(priority: ProviderPriority, model: String, endpoint: Option<String>) -> Self {
        Self {
            priority,
            status: ProviderStatus::Available,
            endpoint,
            model,
            retry_count: 0,
            last_error: None,
        }
    }

    pub fn mark_failed(&mut self, error: String) {
        self.retry_count += 1;
        self.last_error = Some(error);
        if self.retry_count >= 3 {
            self.status = ProviderStatus::Failed;
        }
    }

    pub fn mark_rate_limited(&mut self) {
        self.status = ProviderStatus::RateLimited;
    }

    pub fn reset(&mut self) {
        self.status = ProviderStatus::Available;
        self.retry_count = 0;
        self.last_error = None;
    }
}

/// Fallback chain manager
#[derive(Debug, Clone)]
pub struct FallbackChain {
    providers: Vec<FallbackProvider>,
    current_index: usize,
}

impl Default for FallbackChain {
    fn default() -> Self {
        Self {
            providers: vec![
                FallbackProvider::new(
                    ProviderPriority::LlamaCpp,
                    "qwen2.5-coder:1.5b".to_string(),
                    Some("http://localhost:8080/v1".to_string()),
                ),
                FallbackProvider::new(
                    ProviderPriority::Copilot,
                    "gpt-4".to_string(),
                    None, // Uses IDE integration
                ),
                FallbackProvider::new(
                    ProviderPriority::Anthropic,
                    "claude-3-haiku".to_string(),
                    Some("https://api.anthropic.com/v1".to_string()),
                ),
                FallbackProvider::new(
                    ProviderPriority::OpenAI,
                    "gpt-4o-mini".to_string(),
                    Some("https://api.openai.com/v1".to_string()),
                ),
                FallbackProvider::new(
                    ProviderPriority::Git,
                    "template-based".to_string(),
                    None, // Local template generation
                ),
            ],
            current_index: 0,
        }
    }
}

impl FallbackChain {
    /// Create fallback chain from configuration
    pub fn from_config(config: &LithoConfig) -> Result<Self, LithoError> {
        let mut providers = Vec::new();

        // Primary provider
        let primary = match config.model.provider.primary.as_str() {
            "llama.cpp" => FallbackProvider::new(
                ProviderPriority::LlamaCpp,
                config.model.name.clone(),
                Some(config.model.provider.llm_api_base_url.clone()),
            ),
            other => {
                return Err(LithoError::Config(format!(
                    "Unknown primary provider: {}",
                    other
                )))
            }
        };
        providers.push(primary);

        // Fallback providers
        for fallback in &config.model.provider.fallback {
            let provider = match fallback.as_str() {
                "copilot" => FallbackProvider::new(
                    ProviderPriority::Copilot,
                    "gpt-4".to_string(),
                    None,
                ),
                "anthropic" => FallbackProvider::new(
                    ProviderPriority::Anthropic,
                    "claude-3-haiku".to_string(),
                    Some("https://api.anthropic.com/v1".to_string()),
                ),
                "openai" => FallbackProvider::new(
                    ProviderPriority::OpenAI,
                    "gpt-4o-mini".to_string(),
                    Some("https://api.openai.com/v1".to_string()),
                ),
                "git" => FallbackProvider::new(
                    ProviderPriority::Git,
                    "template-based".to_string(),
                    None,
                ),
                other => {
                    return Err(LithoError::Config(format!(
                        "Unknown fallback provider: {}",
                        other
                    )))
                }
            };
            providers.push(provider);
        }

        Ok(Self {
            providers,
            current_index: 0,
        })
    }

    /// Get the current active provider
    pub fn current(&self) -> Option<&FallbackProvider> {
        self.providers.get(self.current_index)
    }

    /// Get mutable reference to current provider
    pub fn current_mut(&mut self) -> Option<&mut FallbackProvider> {
        self.providers.get_mut(self.current_index)
    }

    /// Try to get the next available provider
    pub fn next_available(&mut self) -> Option<&FallbackProvider> {
        while self.current_index < self.providers.len() {
            let provider = &self.providers[self.current_index];
            if provider.status == ProviderStatus::Available {
                return Some(provider);
            }
            self.current_index += 1;
        }
        None
    }

    /// Advance to next provider after failure
    pub fn advance(&mut self) -> bool {
        if self.current_index + 1 < self.providers.len() {
            self.current_index += 1;
            true
        } else {
            false
        }
    }

    /// Reset all providers to available
    pub fn reset_all(&mut self) {
        for provider in &mut self.providers {
            provider.reset();
        }
        self.current_index = 0;
    }

    /// Check if any provider is still available
    pub fn has_available(&self) -> bool {
        self.providers
            .iter()
            .any(|p| p.status == ProviderStatus::Available)
    }

    /// Get all providers
    pub fn all_providers(&self) -> &[FallbackProvider] {
        &self.providers
    }

    /// Handle provider error and potentially advance
    pub fn handle_error(&mut self, error: &str) -> Result<&FallbackProvider, LithoError> {
        if let Some(current) = self.current_mut() {
            current.mark_failed(error.to_string());
        }

        if self.advance() {
            self.next_available()
                .ok_or(LithoError::AllProvidersFailed)
        } else {
            Err(LithoError::AllProvidersFailed)
        }
    }

    /// Handle rate limiting
    pub fn handle_rate_limit(&mut self) -> Result<&FallbackProvider, LithoError> {
        if let Some(current) = self.current_mut() {
            current.mark_rate_limited();
        }

        if self.advance() {
            self.next_available()
                .ok_or(LithoError::AllProvidersFailed)
        } else {
            Err(LithoError::AllProvidersFailed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_chain() {
        let chain = FallbackChain::default();
        assert_eq!(chain.providers.len(), 5);
        assert_eq!(chain.current().unwrap().priority, ProviderPriority::LlamaCpp);
    }

    #[test]
    fn test_advance() {
        let mut chain = FallbackChain::default();
        assert!(chain.advance());
        assert_eq!(chain.current().unwrap().priority, ProviderPriority::Copilot);
    }

    #[test]
    fn test_handle_error() {
        let mut chain = FallbackChain::default();
        let result = chain.handle_error("connection failed");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().priority, ProviderPriority::Copilot);
    }
}
