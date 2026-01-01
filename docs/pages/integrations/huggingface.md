# Hugging Face Integration

Connect NOA to Hugging Face for models and datasets.

## Overview

NOA integrates with Hugging Face for:
- Model downloads
- Inference API
- Datasets

## Configuration

Edit `~/.noa/config/config.toml`:

```toml
[huggingface]
cache_dir = "~/.noa/cache/huggingface"
token = "hf_..."  # Or use NOA_HF_TOKEN env var

[huggingface.inference]
base_url = "https://api-inference.huggingface.co"
timeout_seconds = 120
```

## Environment Variables

```bash
export NOA_HF_TOKEN="hf_..."
export HF_HOME="~/.noa/cache/huggingface"
```

## Downloading Models

```bash
# Using NOA CLI
noa model download --source hf Qwen/Qwen2.5-Coder-7B-Instruct-GGUF

# Specific file
noa model download --source hf \
  Qwen/Qwen2.5-Coder-7B-Instruct-GGUF \
  --file qwen2.5-coder-7b-instruct-q4_k_m.gguf
```

## Inference API

For serverless inference:

```toml
[huggingface.inference]
enabled = true
default_model = "Qwen/Qwen2.5-Coder-7B-Instruct"
```

```rust
use noa_core::providers::HuggingFaceProvider;

let provider = HuggingFaceProvider::from_env()?;
let response = provider.generate(prompt, model).await?;
```

## Model Types

| Type | Use Case |
|------|----------|
| GGUF | llama.cpp inference |
| ONNX | ONNX Runtime |
| SafeTensors | Python/Transformers |

## Recommended Models

### Code Generation

- `Qwen/Qwen2.5-Coder-7B-Instruct-GGUF`
- `deepseek-ai/DeepSeek-Coder-V2-Lite-Instruct-GGUF`

### Embeddings

- `nomic-ai/nomic-embed-text-v1.5`
- `BAAI/bge-small-en-v1.5`

### Reranking

- `BAAI/bge-reranker-base`

## Cache Management

```bash
# Check cache size
noa cache info huggingface

# Clean old models
noa cache clean huggingface --older-than 30d
```

## See Also

- [Configure ML Backend](../how-tos/configure-ml.md)
- [Models Cache](../../wiki/internal-crates/sys-core/memory.md)
