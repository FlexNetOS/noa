# NOA Bootstrap Tools Reference

Complete list of tools installed by the NOA bootstrap system.

## Toolchains

### Rust

| Component | Version | Description |
|-----------|---------|-------------|
| rustc | 1.83.0+ | Rust compiler |
| cargo | 1.83.0+ | Package manager |
| rustfmt | latest | Code formatter |
| clippy | latest | Linter |

**Environment Variables:**
- `CARGO_HOME`: `$NOA_ROOT/opt/rust/cargo`
- `RUSTUP_HOME`: `$NOA_ROOT/opt/rust/rustup`

### Go

| Component | Version | Description |
|-----------|---------|-------------|
| go | 1.23.4+ | Go compiler and tools |

**Environment Variables:**
- `GOROOT`: `$NOA_ROOT/opt/go`
- `GOPATH`: `$NOA_ROOT/opt/go/workspace`
- `GOBIN`: `$NOA_ROOT/opt/go/workspace/bin`
- `GOCACHE`: `$NOA_ROOT/cache/go`
- `GOMODCACHE`: `$NOA_ROOT/opt/go/pkg/mod`

### Node.js

| Component | Version | Description |
|-----------|---------|-------------|
| node | 20.x LTS | JavaScript runtime |
| npm | 10.x | Package manager |
| npx | 10.x | Package runner |

**Environment Variables:**
- `npm_config_cache`: `$NOA_ROOT/cache/npm`

### Python

| Component | Version | Description |
|-----------|---------|-------------|
| python | 3.12+ | Python interpreter |
| pip | latest | Package installer |

**Environment Variables:**
- `PIP_CACHE_DIR`: `$NOA_ROOT/cache/pip`

### Protocol Buffers

| Component | Version | Description |
|-----------|---------|-------------|
| protoc | 28.x | Protocol buffer compiler |

## CLI Utilities

| Tool | Version | Description | Installer |
|------|---------|-------------|-----------|
| jq | 1.7.1 | JSON processor | `jq.ps1` |
| ripgrep (rg) | 14.1.1 | Fast search | `ripgrep.ps1` |
| fd | 10.2.0 | Fast finder | `fd.ps1` |
| bat | 0.24.0 | Better cat | `bat.ps1` |
| fzf | 0.56.3 | Fuzzy finder | `fzf.ps1` |
| delta | 0.18.2 | Git diff viewer | `delta.ps1` |

## Quality Tools

### Rust

| Tool | Description |
|------|-------------|
| rustfmt | Code formatter |
| clippy | Linter |

### Go

| Tool | Description |
|------|-------------|
| golangci-lint | Linter aggregator |
| staticcheck | Static analyzer |

### TypeScript/JavaScript

| Tool | Description |
|------|-------------|
| eslint | Linter |
| prettier | Formatter |

### Python

| Tool | Description |
|------|-------------|
| ruff | Fast linter |
| black | Formatter |
| mypy | Type checker |

## Security Tools

| Tool | Version | Description |
|------|---------|-------------|
| gitleaks | 8.21.x | Secret scanner |
| trivy | 0.58.x | Vulnerability scanner |
| grype | 0.85.x | Container scanner |

## AI Provider CLIs

### Local Providers

| Tool | Type | Description |
|------|------|-------------|
| ollama | Local | Local model server |
| llama-server | Local | llama.cpp server |
| git | Local | Version control |

### Cloud Providers

| Tool | Package | Description |
|------|---------|-------------|
| claude | @anthropic-ai/claude-code | Claude Code CLI |
| codex | @openai/codex | OpenAI Codex CLI |
| abacusai | @abacus-ai/cli | Abacus AI CLI |

### IDE Integrations

| Tool | Type | Description |
|------|------|-------------|
| cursor | Hybrid | Cursor IDE |
| code | IDE | VS Code |
| gh copilot | IDE | GitHub Copilot |

## Dev Tools (Optional)

| Tool | Description | Installer |
|------|-------------|-----------|
| Cursor | AI-first IDE | `cursor.ps1` |
| VS Code | Code editor | `vscode.ps1` |
| Docker | Containerization | `docker.ps1` |
| Postman | API client | `postman.ps1` |
| DBeaver | Database client | `dbeaver.ps1` |
| ChatGPT Desktop | OpenAI desktop app | `chatgpt-desktop.ps1` |
| Claude Desktop | Anthropic desktop app | `claude-desktop.ps1` |

## Cache Locations

| Cache | Path | Description |
|-------|------|-------------|
| Rust registry | `cache/rust/` | Cargo packages |
| Go modules | `cache/go/` | Go modules |
| npm packages | `cache/npm/` | npm cache |
| pip packages | `cache/pip/` | pip cache |
| Ollama models | `cache/ollama/` | Ollama models |
| HuggingFace | `cache/huggingface/` | HF models |
| llama.cpp | `cache/models/` | GGUF models |
| Downloads | `cache/downloads/` | Downloaded archives |

## Verification

After installation, verify with:

```bash
# Check all tools
./scripts/bootstrap/verify/verify-all.sh

# Run smoke tests
./scripts/bootstrap/verify/smoke-test.sh

# Generate report
./scripts/bootstrap/report/generate-report.ps1
```

