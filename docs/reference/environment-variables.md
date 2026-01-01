# Environment Variables

Environment variables for configuring NOA.

## Core Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `NOA_DATA_DIR` | Data directory path | `~/.noa` |
| `NOA_CONFIG_FILE` | Config file path | `~/.noa/config/config.toml` |
| `NOA_LOG_LEVEL` | Log level | `info` |
| `NOA_LOG_FORMAT` | Log format: `text`, `json` | `text` |

## API Server

| Variable | Description | Default |
|----------|-------------|---------|
| `NOA_API_HOST` | Server host | `127.0.0.1` |
| `NOA_API_PORT` | Server port | `8080` |
| `NOA_API_TIMEOUT` | Request timeout (seconds) | `30` |

## Database

| Variable | Description | Default |
|----------|-------------|---------|
| `NOA_DATABASE_URL` | Database URL | `sqlite://~/.noa/data/noa.db` |
| `NOA_DATABASE_POOL_SIZE` | Connection pool size | `5` |

## Neural / ML

| Variable | Description | Default |
|----------|-------------|---------|
| `NOA_NEURAL_BACKEND` | ML backend | `llamacpp` |
| `NOA_MODEL_PATH` | Model storage path | `~/.noa/models` |
| `NOA_DEFAULT_MODEL` | Default model | - |

### llama.cpp

| Variable | Description | Default |
|----------|-------------|---------|
| `NOA_LLAMACPP_GPU_LAYERS` | GPU layers | `35` |
| `NOA_LLAMACPP_THREADS` | CPU threads | auto |
| `NOA_LLAMACPP_CONTEXT_SIZE` | Context size | `4096` |

### Ollama

| Variable | Description | Default |
|----------|-------------|---------|
| `NOA_OLLAMA_BASE_URL` | Ollama API URL | `http://localhost:11434` |
| `NOA_OLLAMA_MODEL` | Default model | - |

### OpenAI

| Variable | Description | Default |
|----------|-------------|---------|
| `NOA_OPENAI_API_KEY` | OpenAI API key | - |
| `NOA_OPENAI_BASE_URL` | API base URL | `https://api.openai.com/v1` |
| `NOA_OPENAI_MODEL` | Default model | `gpt-4o-mini` |

## Hugging Face

| Variable | Description | Default |
|----------|-------------|---------|
| `NOA_HF_TOKEN` | Hugging Face token | - |
| `HF_HOME` | HF cache directory | `~/.cache/huggingface` |

## P2P Networking

| Variable | Description | Default |
|----------|-------------|---------|
| `NOA_P2P_ENABLED` | Enable P2P | `false` |
| `NOA_P2P_LISTEN_ADDR` | Listen address | `/ip4/0.0.0.0/tcp/4001` |
| `NOA_P2P_EXTERNAL_ADDR` | External address | - |

## Observability

| Variable | Description | Default |
|----------|-------------|---------|
| `NOA_METRICS_ENABLED` | Enable metrics | `true` |
| `NOA_METRICS_PORT` | Metrics port | `9090` |
| `NOA_TRACING_ENABLED` | Enable tracing | `true` |
| `NOA_TRACING_BACKEND` | Tracing backend | `stdout` |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | OTLP endpoint | - |

## Cache

| Variable | Description | Default |
|----------|-------------|---------|
| `NOA_CACHE_DIR` | Cache directory | `~/.noa/cache` |
| `NOA_CACHE_MAX_SIZE` | Max cache size | `10GB` |

## Development

| Variable | Description | Default |
|----------|-------------|---------|
| `NOA_DEV_MODE` | Development mode | `false` |
| `NOA_DEBUG` | Debug mode | `false` |
| `RUST_BACKTRACE` | Rust backtraces | `0` |
| `RUST_LOG` | Rust log filter | - |

## Examples

### Minimal Setup

```bash
export NOA_DATA_DIR="$HOME/.noa"
export NOA_LOG_LEVEL="info"
```

### With Ollama

```bash
export NOA_NEURAL_BACKEND="ollama"
export NOA_OLLAMA_BASE_URL="http://localhost:11434"
export NOA_OLLAMA_MODEL="qwen2.5-coder:7b"
```

### With OpenAI

```bash
export NOA_NEURAL_BACKEND="openai"
export NOA_OPENAI_API_KEY="sk-..."
export NOA_OPENAI_MODEL="gpt-4o-mini"
```

### Production

```bash
export NOA_LOG_LEVEL="warn"
export NOA_LOG_FORMAT="json"
export NOA_METRICS_ENABLED="true"
export NOA_TRACING_BACKEND="otlp"
export OTEL_EXPORTER_OTLP_ENDPOINT="http://jaeger:4317"
```

## See Also

- [Configuration Schema](config-schema.md)
- [CLI Reference](cli.md)
