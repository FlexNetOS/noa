# How-To: Bootstrap NOA

Step-by-step guide to initialize a new NOA instance.

## Prerequisites

- Rust 1.83.0 or later
- 4GB RAM minimum
- 10GB disk space

## Steps

### 1. Clone Repository

```bash
git clone https://github.com/FlexNetOS/noa.git
cd noa
```

### 2. Build NOA

```bash
# Minimal build (bootstrap only)
cargo build --release

# Full build
cargo build --release --features full
```

### 3. Initialize

```bash
# Run initialization
./target/release/noa init

# Or with custom data directory
./target/release/noa init --data-dir ~/.noa
```

### 4. Configure

Edit `~/.noa/config/config.toml`:

```toml
[database]
url = "sqlite://~/.noa/data/noa.db"

[api]
host = "127.0.0.1"
port = 8080

[neural]
model_path = "~/.noa/models"
default_model = "qwen2.5-coder-7b"
```

### 5. Download Models

```bash
# Download default model
noa model download qwen2.5-coder-7b
```

### 6. Start NOA

```bash
# Foreground
noa run

# Background (daemon)
noa run --daemon
```

### 7. Verify

```bash
# Check health
curl http://localhost:8080/health

# List agents
noa agent list
```

## Next Steps

- [Add Custom Agent](add-agent.md)
- [Configure ML Backend](configure-ml.md)
- [Setup P2P Network](setup-p2p.md)
