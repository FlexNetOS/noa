# Ollama Integration

Connect NOA to Ollama for local LLM inference.

## Overview

Ollama provides a simple API for running LLMs locally with automatic GPU detection and model management.

## Installation

### Windows

```powershell
winget install Ollama.Ollama
```

### Linux

```bash
curl -fsSL https://ollama.com/install.sh | sh
```

### macOS

```bash
brew install ollama
```

## Configuration

Edit `~/.noa/config/config.toml`:

```toml
[neural]
backend = "ollama"

[neural.ollama]
base_url = "http://localhost:11434"
default_model = "qwen2.5-coder:7b"
timeout_seconds = 120
```

## Pulling Models

```bash
# Code generation
ollama pull qwen2.5-coder:7b

# General purpose
ollama pull llama3.2:3b

# Embeddings
ollama pull nomic-embed-text
```

## API Endpoints

| Endpoint | Purpose |
|----------|---------|
| `/api/generate` | Text completion |
| `/api/chat` | Chat completion |
| `/api/embed` | Generate embeddings |
| `/api/tags` | List models |

## Usage in NOA

```rust
use noa_core::providers::OllamaProvider;

let provider = OllamaProvider::new("http://localhost:11434");
let response = provider.chat(&messages, "qwen2.5-coder:7b").await?;
```

## GPU Support

Ollama automatically detects and uses:
- NVIDIA GPUs (CUDA)
- AMD GPUs (ROCm)
- Apple Silicon (Metal)

## Troubleshooting

### Model Not Found

```bash
ollama list  # Check available models
ollama pull <model>  # Download model
```

### Connection Refused

```bash
ollama serve  # Start server
```

### Out of Memory

Use smaller quantization:
```bash
ollama pull qwen2.5-coder:7b-instruct-q4_k_m
```

## See Also

- [Configure ML Backend](../how-tos/configure-ml.md)
- [Neural Module](../../wiki/internal-crates/sys-core/neural.md)
