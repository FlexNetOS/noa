# OpenAI API Integration

Connect NOA to OpenAI or compatible APIs.

## Overview

NOA supports OpenAI-compatible APIs for cloud inference.

## Configuration

Edit `~/.noa/config/config.toml`:

```toml
[neural]
backend = "openai"

[neural.openai]
api_key = "sk-..."  # Or use NOA_OPENAI_API_KEY env var
base_url = "https://api.openai.com/v1"
default_model = "gpt-4o-mini"
timeout_seconds = 60
max_retries = 3
```

## Environment Variables

```bash
export NOA_OPENAI_API_KEY="sk-..."
export NOA_OPENAI_BASE_URL="https://api.openai.com/v1"
```

## Compatible Providers

Works with OpenAI-compatible APIs:

| Provider | Base URL |
|----------|----------|
| OpenAI | `https://api.openai.com/v1` |
| Azure OpenAI | `https://{name}.openai.azure.com/` |
| OpenRouter | `https://openrouter.ai/api/v1` |
| Groq | `https://api.groq.com/openai/v1` |
| Together AI | `https://api.together.xyz/v1` |

## Models

### Chat Models

- `gpt-4o` - Most capable
- `gpt-4o-mini` - Fast and efficient
- `gpt-4-turbo` - Previous generation

### Embedding Models

- `text-embedding-3-small` - Fast
- `text-embedding-3-large` - High quality

## Usage

```rust
use noa_core::providers::OpenAIProvider;

let provider = OpenAIProvider::from_env()?;
let response = provider.chat(&messages, "gpt-4o-mini").await?;
```

## Rate Limiting

NOA handles rate limits automatically:
- Exponential backoff
- Request queuing
- Retry on 429 responses

## Cost Control

```toml
[neural.openai]
max_tokens_per_request = 4096
max_requests_per_minute = 60
monthly_budget_usd = 50.0
```

## See Also

- [Configure ML Backend](../how-tos/configure-ml.md)
- [Providers Module](../../wiki/internal-crates/sys-core/providers.md)
