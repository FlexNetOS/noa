# How-To: Configure ML Backend

Set up local ML inference with different backends.

## Available Backends

| Backend | Format | GPU Support | Notes |
|---------|--------|-------------|-------|
| llama.cpp | GGUF | Yes | Default, recommended |
| ONNX Runtime | ONNX | Yes | Requires `ml-devops` feature |
| Ollama | Any | Yes | External service |

## llama.cpp (Default)

### 1. Download Model

```bash
# Using NOA CLI
noa model download qwen2.5-coder-7b

# Or manually
wget https://huggingface.co/Qwen/Qwen2.5-Coder-7B-Instruct-GGUF/resolve/main/qwen2.5-coder-7b-instruct-q4_k_m.gguf \
  -O ~/.noa/models/qwen2.5-coder-7b.gguf
```

### 2. Configure

Edit `~/.noa/config/config.toml`:

```toml
[neural]
backend = "llamacpp"
model_path = "~/.noa/models"
default_model = "qwen2.5-coder-7b"

[neural.llamacpp]
context_size = 4096
gpu_layers = 35  # Set to 0 for CPU only
threads = 8
```

### 3. Verify

```bash
noa model test qwen2.5-coder-7b
```

## ONNX Runtime

### 1. Build with ml-devops

```bash
cargo build --features "full,ml-devops"
```

### 2. Download ONNX Model

```bash
noa model download --format onnx nomic-embed-text
```

### 3. Configure

```toml
[neural.ort]
execution_providers = ["CUDA", "CPU"]
```

## Ollama (External)

### 1. Install Ollama

```bash
# Windows
winget install Ollama.Ollama

# Linux
curl -fsSL https://ollama.com/install.sh | sh
```

### 2. Pull Model

```bash
ollama pull qwen2.5-coder:7b
```

### 3. Configure NOA

```toml
[neural]
backend = "ollama"

[neural.ollama]
base_url = "http://localhost:11434"
default_model = "qwen2.5-coder:7b"
```

## See Also

- [Bootstrap NOA](bootstrap.md)
- [Neural Module](../../wiki/internal-crates/sys-core/neural.md)
