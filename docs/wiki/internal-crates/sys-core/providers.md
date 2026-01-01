# providers Module

External service integrations.

**Location**: `sys/core/src/providers/`  
**Feature**: `full`

## Overview

Abstraction layer for external services:

- Provider trait for pluggable backends
- LLM providers (local, API)
- Storage providers
- Notification providers

## Key Types

### Provider Trait

```rust
#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn kind(&self) -> ProviderKind;
    async fn health(&self) -> HealthStatus;
}
```

### ProviderKind

```rust
pub enum ProviderKind {
    Llm,
    Storage,
    Notification,
    Search,
    Custom(String),
}
```

## LLM Providers

| Provider | Description |
|----------|-------------|
| `LlamaCppProvider` | Local llama.cpp |
| `OllamaProvider` | Ollama API |
| `OpenAIProvider` | OpenAI API |
| `AnthropicProvider` | Claude API |

## Provider Configuration

```toml
# config/ai-providers.json
{
    "default": "llamacpp",
    "providers": {
        "llamacpp": {
            "model_path": "~/.noa/models/qwen2.5-coder-7b.gguf"
        },
        "ollama": {
            "base_url": "http://localhost:11434"
        }
    }
}
```

## Usage

```rust
use noa_core::providers::{ProviderRegistry, LlmProvider};

async fn example(registry: &ProviderRegistry) -> NoaResult<()> {
    let llm = registry.get::<dyn LlmProvider>("llamacpp")?;
    
    let response = llm.generate("Hello, world!").await?;
    println!("{}", response);
    
    Ok(())
}
```

## See Also

- [neural module](neural.md) — Neural inference
- [config/ai-providers.json](../../../../config/ai-providers.json) — Provider config
