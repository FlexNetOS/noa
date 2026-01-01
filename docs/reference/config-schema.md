# Configuration Schema

NOA uses TOML configuration files.

## File Locations

| File | Purpose |
|------|---------|
| `~/.noa/config/config.toml` | Main configuration |
| `~/.noa/config/agents.toml` | Agent configuration |
| `~/.noa/config/policies.toml` | Governance policies |

## Main Configuration

```toml
# NOA Main Configuration
# Location: ~/.noa/config/config.toml

# ─────────────────────────────────────────────────────────────────
# CORE SETTINGS
# ─────────────────────────────────────────────────────────────────

[core]
# Data directory (default: ~/.noa)
data_dir = "~/.noa"

# Log level: trace, debug, info, warn, error
log_level = "info"

# ─────────────────────────────────────────────────────────────────
# DATABASE
# ─────────────────────────────────────────────────────────────────

[database]
# SQLite database URL
url = "sqlite://~/.noa/data/noa.db"

# Connection pool size
pool_size = 5

# Enable WAL mode (recommended)
wal_mode = true

# ─────────────────────────────────────────────────────────────────
# API SERVER
# ─────────────────────────────────────────────────────────────────

[api]
# Server host
host = "127.0.0.1"

# Server port
port = 8080

# Enable CORS
cors_enabled = true

# Allowed origins (empty = all)
cors_origins = []

# Request timeout (seconds)
timeout = 30

# ─────────────────────────────────────────────────────────────────
# NEURAL / ML
# ─────────────────────────────────────────────────────────────────

[neural]
# Backend: llamacpp, ollama, openai
backend = "llamacpp"

# Model storage path
model_path = "~/.noa/models"

# Default model
default_model = "qwen2.5-coder-7b"

# Embedding model
embedding_model = "nomic-embed-text"

# llama.cpp settings
[neural.llamacpp]
context_size = 4096
gpu_layers = 35
threads = 8
batch_size = 512

# Ollama settings
[neural.ollama]
base_url = "http://localhost:11434"
timeout_seconds = 120

# OpenAI settings
[neural.openai]
api_key = ""  # Use env: NOA_OPENAI_API_KEY
base_url = "https://api.openai.com/v1"
max_retries = 3

# ─────────────────────────────────────────────────────────────────
# P2P NETWORKING
# ─────────────────────────────────────────────────────────────────

[p2p]
# Enable P2P
enabled = false

# Listen addresses
listen_addresses = ["/ip4/0.0.0.0/tcp/4001"]

# Bootstrap peers
[p2p.bootstrap]
peers = []

# NAT traversal
[p2p.nat]
upnp = true
relay = true
relay_servers = []

# ─────────────────────────────────────────────────────────────────
# AGENTS
# ─────────────────────────────────────────────────────────────────

[agents]
# Enable agent system
enabled = true

# Max concurrent agents
max_concurrent = 10

# Default timeout (seconds)
default_timeout = 300

# ─────────────────────────────────────────────────────────────────
# OBSERVABILITY
# ─────────────────────────────────────────────────────────────────

[observability]
# Enable metrics
metrics_enabled = true

# Metrics port
metrics_port = 9090

# Enable tracing
tracing_enabled = true

# Tracing backend: stdout, jaeger, otlp
tracing_backend = "stdout"

# ─────────────────────────────────────────────────────────────────
# CACHE
# ─────────────────────────────────────────────────────────────────

[cache]
# Cache root directory
root = "~/.noa/cache"

# Max cache size (bytes)
max_size = 10737418240  # 10GB

# Auto-cleanup enabled
auto_cleanup = true

# Cleanup older than (days)
cleanup_days = 30
```

## Agent Configuration

```toml
# Agent Configuration
# Location: ~/.noa/config/agents.toml

# Commander-Chief (orchestration)
[[agents]]
id = "commander-chief"
kind = "commander-chief"
enabled = true
priority = 100

# File-IO Agent
[[agents]]
id = "file-io"
kind = "file-io"
enabled = true
allowed_paths = ["~", "."]
denied_paths = ["/etc", "/var"]

# Terminal Agent
[[agents]]
id = "terminal"
kind = "terminal"
enabled = true
allowed_commands = ["cargo", "npm", "git"]
timeout = 60
```

## See Also

- [CLI Reference](cli.md)
- [Environment Variables](environment-variables.md)
