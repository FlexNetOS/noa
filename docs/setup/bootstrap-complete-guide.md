# NOA Bootstrap Complete Guide

This guide walks you through the complete NOA environment setup process.

## Prerequisites

Before running bootstrap, ensure you have:

- **Windows**: PowerShell 7+ (`pwsh`)
- **macOS/Linux**: Bash 4+
- **Git**: For version control
- **Internet**: For downloading toolchains (one-time)

## Quick Start

### Minimal Setup (Directories Only)

```powershell
# Windows
.\scripts\bootstrap\bootstrap.ps1

# Unix
./scripts/bootstrap/bootstrap.sh
```

### Full Setup (All Toolchains)

```powershell
# Windows - Install everything
.\scripts\bootstrap\bootstrap.ps1 -InstallAllTools -InstallAiProviders

# Unix
./scripts/bootstrap/bootstrap.sh --install-all-tools --install-ai-providers
```

## What Gets Installed

### Directory Structure

```
noa_root/
├── ai/
│   ├── providers/     # AI provider configurations
│   │   ├── local/     # Ollama, llama.cpp, git-cli
│   │   ├── cloud/     # Claude, Codex, Abacus
│   │   ├── hybrid/    # Cursor
│   │   └── ide/       # VS Code Copilot
│   └── shared/        # Shared resources across providers
│       ├── agents/    # Agent definitions
│       ├── workflows/ # Orchestration workflows
│       ├── prompts/   # Prompt templates
│       ├── tools/     # MCP tool definitions
│       ├── skills/    # Agent skills
│       ├── models/    # Model configurations
│       ├── commands/  # Shared commands
│       └── resources/ # Execution memory, configs
├── bin/               # NOA executables
├── cache/             # Centralized caches
├── config/            # Configuration files
├── logs/              # Centralized logging
├── opt/               # Portable toolchains
│   ├── rust/          # Rust/Cargo
│   ├── go/            # Go
│   ├── node/          # Node.js
│   └── python/        # Python
├── scripts/           # Setup and utility scripts
└── specs/             # Feature specifications
```

### Toolchains

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | 1.83+ | Core runtime development |
| Go | 1.23+ | P2P and networking |
| Node.js | 20.x | AI CLI tools, scripting |
| Python | 3.12+ | AI/ML integration |
| Protobuf | 28.x | Service definitions |

### CLI Utilities

| Tool | Purpose |
|------|---------|
| jq | JSON processing |
| ripgrep (rg) | Fast code search |
| fd | Fast file finder |
| bat | Syntax-highlighted cat |
| fzf | Fuzzy finder |
| delta | Better git diffs |

### AI Providers

| Provider | Type | Installation |
|----------|------|--------------|
| Ollama | Local | Automatic |
| llama-server | Local | Automatic |
| Claude Code | Cloud | npm install |
| Codex CLI | Cloud | npm install |
| Cursor | Hybrid | Manual (IDE) |
| VS Code Copilot | IDE | Manual (extension) |

## Post-Installation

### 1. Load Environment

```powershell
# Windows
. .\noa-env.ps1

# Unix
source ./.noa-env
```

### 2. Verify Installation

```powershell
# Full verification
.\scripts\bootstrap\verify\verify-all.ps1

# Smoke tests (compile & run test programs)
.\scripts\bootstrap\verify\smoke-test.ps1

# Generate report
.\scripts\bootstrap\report\generate-report.ps1
```

### 3. Add to Shell Profile (Optional)

```powershell
# Windows - adds to PowerShell profile
.\scripts\bootstrap\generators\shell-integration.ps1

# Unix - adds to .bashrc/.zshrc
./scripts/bootstrap/generators/shell-integration.sh
```

## Configuration

### Environment Variables

Set these in your environment:

```bash
# Required
export NOA_ROOT="/path/to/noa"

# Optional - AI Provider Keys
export ANTHROPIC_API_KEY="..."
export OPENAI_API_KEY="..."
export ABACUS_API_KEY="..."
```

### AI Provider Configuration

Edit `config/ai-providers.json` to:
- Enable/disable providers
- Set priority order
- Configure endpoints

### Shared Resources

The shared resource system (`ai/shared/`) enables:
- Unified agent definitions
- Cross-provider workflows
- Shared execution memory

See `config/shared-resources.json` for configuration.

## Troubleshooting

### Windows Issues

**PowerShell execution policy error:**
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

**Missing curl/wget:**
Bootstrap uses `Invoke-WebRequest` on Windows.

### Unix Issues

**Permission denied:**
```bash
chmod +x scripts/bootstrap/*.sh
chmod +x scripts/bootstrap/**/*.sh
```

**Missing bash:**
Install bash 4+ or use zsh.

### Network Issues

**Behind proxy:**
```bash
export HTTP_PROXY="http://proxy:port"
export HTTPS_PROXY="http://proxy:port"
```

**Offline mode:**
Pre-download archives to `cache/downloads/` and run with `--offline`.

## Re-running Bootstrap

Bootstrap is idempotent - running it again will:
- Skip already-installed tools
- Update outdated tools (with `--force`)
- Repair broken installations

```powershell
# Force reinstall everything
.\scripts\bootstrap\bootstrap.ps1 -InstallAllTools -Force
```

## Support

- Check `logs/bootstrap/` for detailed logs
- Run `verify-all.ps1` to diagnose issues
- See `specs/001-noa-seed-foundation/` for full specification

