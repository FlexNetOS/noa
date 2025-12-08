# Quickstart: NOA Seed Foundation

Get NOA running on your system in under 10 minutes.

## Prerequisites

### Hardware Tiers
| Tier | RAM | CPU | GPU | Storage | Use Case |
|------|-----|-----|-----|---------|----------|
| **Minimum** | 8GB | 4-core | None | 20GB | Baseline operation, single model |
| **Standard** | 16GB | 8-core | Integrated | 100GB | Multi-SLM operation |
| **High-Performance** | 64GB+ | 16+ core | RTX 3080+ | 500GB | Optimal local inference |
| **Development** | 512GB+ | 24+ core | 2x RTX 5090+ | 2TB+ | Full development, tensor parallelism |

### Minimum Requirements
- 8GB RAM minimum (see tiers above for recommendations)
- 20GB free disk space (100GB+ for models, 2TB+ for development)
- 64-bit operating system
- Network access for initial setup
- For GPU acceleration: CUDA 13.1+ toolkit with tiles support

### Supported Platforms
- Windows 11 (x64)
- macOS 13+ (x64, arm64)
- Ubuntu 22.04+ (x64)

---

## Installation

### Option 1: Quick Install (Recommended)

#### Windows (PowerShell as Administrator)
```powershell
# Download and run installer
irm https://get.noa.dev/install.ps1 | iex
```

#### macOS / Linux
```bash
# Download and run installer
curl -fsSL https://get.noa.dev/install.sh | bash
```

### Option 2: Manual Installation

1. **Download the release**
   - Visit https://github.com/noa/noa/releases
   - Download the appropriate archive for your platform

2. **Extract and set up**
   ```bash
   # Extract to your preferred location
   tar -xzf noa-v1.0.0-linux-x64.tar.gz -C ~/.noa

   # Add to PATH
   export NOA_ROOT="$HOME/.noa"
   export PATH="$NOA_ROOT/bin:$PATH"
   ```

3. **Initialize NOA**
   ```bash
   noa init
   ```

---

## First Run

### 1. Initialize NOA

```bash
# Initialize with default settings
noa init

# Or specify a custom directory
noa init --root /path/to/noa
```

This creates:
- `noa_root/` directory structure
- Default configuration
- Local database
- System prompts

### 2. Download a Model

```bash
# List available models
noa models list --available

# Download recommended model (Qwen2.5-1.5B, ~1GB)
noa models download qwen2.5-1.5b-instruct

# Or download a smaller model for testing (~500MB)
noa models download tinyllama-1.1b
```

### 3. Start NOA

```bash
# Start in foreground
noa start

# Or start as background service
noa start --daemon

# Check status
noa status
```

### 4. Open the UI

Once started, open your browser to:
```
http://localhost:8080
```

---

## Basic Usage

### Interact via CLI

```bash
# Ask NOA a question
noa ask "What can you help me with?"

# Create a memory
noa memory create "Today I learned about NOA's architecture"

# Search memories
noa memory search "architecture"

# List agents
noa agents list

# View activity log
noa logs --follow
```

### Interact via API

```bash
# Health check
curl http://localhost:8080/api/v1/health

# Create a memory
curl -X POST http://localhost:8080/api/v1/memories \
  -H "Content-Type: application/json" \
  -d '{"type": "learning", "content": "NOA is running!"}'

# Search memories
curl -X POST http://localhost:8080/api/v1/memories/search \
  -H "Content-Type: application/json" \
  -d '{"query": "NOA"}'
```

---

## Digest Your First Repository

NOA can analyze and understand any codebase:

```bash
# Digest a repository
noa digest https://github.com/example/repo

# Check digest status
noa digest status

# View the knowledge graph
noa knowledge search "main function"

# Get security report
noa digest security-report
```

---

## Connect Devices (P2P)

To create a hive-mind across your devices:

### On your first device:
```bash
# Start NOA with P2P enabled
noa start --p2p

# Get your peer ID
noa p2p info
```

### On additional devices:
```bash
# Initialize and start NOA
noa init
noa start --p2p

# Connect to first device (they'll discover each other on same network)
# Or manually connect:
noa p2p connect <peer-id>
```

---

## Configuration

### Key Configuration Files

| File | Purpose |
|------|---------|
| `config/noa-server.json` | Server settings |
| `config/ai-providers.json` | Model configuration |
| `config/features.json` | Feature flags |
| `config/device-orchestration.json` | P2P settings |

### Environment Variables

```bash
# Override root directory
export NOA_ROOT="/custom/path"

# Set log level
export NOA_LOG_LEVEL="debug"

# Disable telemetry (if implemented)
export NOA_TELEMETRY="false"
```

### Example Configuration

```json
// config/noa-server.json
{
  "host": "127.0.0.1",
  "port": 8080,
  "database": {
    "path": "${NOA_ROOT}/data/memory/noa.db"
  },
  "models": {
    "default": "qwen2.5-1.5b-instruct",
    "context_length": 4096,
    "gpu_layers": "auto"
  },
  "p2p": {
    "enabled": true,
    "discovery": "mdns"
  }
}
```

---

## Troubleshooting

### NOA won't start

```bash
# Check for port conflicts
netstat -an | grep 8080

# Check logs
noa logs --level error

# Reset to defaults (preserves data)
noa reset --config-only
```

### Model loading fails

```bash
# Verify model integrity
noa models verify qwen2.5-1.5b-instruct

# Check available memory
noa system info

# Try a smaller model
noa models download tinyllama-1.1b
```

### P2P connection issues

```bash
# Check P2P status
noa p2p status

# Verify network connectivity
noa p2p ping <peer-id>

# Reset P2P identity (loses peer connections)
noa p2p reset
```

### Database issues

```bash
# Check database integrity
noa db check

# Repair if needed (creates backup first)
noa db repair

# Export data
noa db export --output backup.json
```

---

## Next Steps

1. **Explore the UI**: Navigate different views and familiarize yourself with the interface
2. **Add more models**: Download specialized models for different tasks
3. **Digest your codebases**: Let NOA understand your projects
4. **Connect devices**: Build your personal hive-mind
5. **Customize agents**: Configure agent behaviors for your workflow

---

## Getting Help

- **Documentation**: https://docs.noa.dev
- **GitHub Issues**: https://github.com/noa/noa/issues
- **Community**: https://discord.gg/noa

---

## Uninstalling

```bash
# Stop NOA
noa stop

# Remove NOA (preserves data by default)
noa uninstall

# Full removal including data
noa uninstall --all
```

To manually remove:
1. Delete the `noa_root` directory
2. Remove NOA from your PATH
