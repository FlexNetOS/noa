# Task List: NOA Unified Bootstrap - Complete Environment Setup

**Feature**: 002-unified-bootstrap
**Spec**: [spec.md](./spec.md)
**Branch**: `002-unified-bootstrap`
**Created**: 2025-12-08
**Updated**: 2025-12-08

---

## Overview

This is the **SINGLE setup script** that installs and configures the **COMPLETE NOA development environment**:

1. **Toolchains**: Rust, Go, Node.js, Python, protoc (all with package managers working)
2. **Dev Tools**: Cursor, VS Code, Docker Desktop, ChatGPT Desktop, Claude Desktop
3. **CLI Tools**: jq, ripgrep, fd, bat, fzf, gh, git-lfs
4. **Security Tools**: gitleaks, trivy, grype, semgrep
5. **Quality Tools**: rustfmt, clippy, golangci-lint, eslint, ruff
6. **Cache & Logs**: All caches and logs centralized in noa_root
7. **AI Provider Sync**: Provider caches synced and optimized

**Key Features**:
- ✅ Verification before download (skip if present, update if outdated)
- ✅ Relocation from wrong paths to correct noa_root location
- ✅ All dev tools in .gitignore
- ✅ Platform-specific shell configuration

---

## Directory Structure (Complete)

```
noa_root/
├── bin/                              # Standalone executables (in PATH)
│   ├── jq.exe, rg.exe, fd.exe, bat.exe, fzf.exe
│   ├── protoc.exe
│   ├── gitleaks.exe, trivy.exe, grype.exe
│   └── [symlinks or wrappers for tools]
│
├── opt/                              # Installed toolchains & apps
│   ├── rust/                         # Rust toolchain
│   │   ├── rustup/                   # RUSTUP_HOME
│   │   └── cargo/                    # CARGO_HOME + registry
│   │       └── bin/                  # rustc, cargo, rustfmt, clippy
│   ├── go/                           # Go toolchain
│   │   ├── bin/                      # go, gofmt
│   │   ├── pkg/mod/                  # Module cache (GOMODCACHE)
│   │   ├── cache/                    # Build cache (GOCACHE)
│   │   └── workspace/bin/            # go install output (GOBIN)
│   ├── node/                         # Node.js runtime
│   │   ├── node.exe, npm.cmd
│   │   └── node_modules/             # Global npm packages
│   ├── python/                       # Python runtime (if portable)
│   ├── venv/                         # Python virtual environment
│   │   └── Scripts/                  # pip-installed tools (ruff, semgrep)
│   ├── npm-cache/                    # npm cache directory
│   ├── protobuf/include/             # protoc includes
│   └── dev-tools/                    # Development applications (gitignored)
│       ├── cursor/                   # Cursor IDE (portable)
│       ├── vscode/                   # VS Code (portable)
│       ├── docker/                   # Docker binaries
│       ├── chatgpt/                  # ChatGPT Desktop
│       └── claude/                   # Claude Desktop
│
├── cache/                            # Centralized cache (synced)
│   ├── rust/                         # Cargo registry cache
│   ├── go/                           # Go module cache (symlinked)
│   ├── npm/                          # npm cache (symlinked)
│   ├── pip/                          # pip cache
│   ├── docker/                       # Docker layer cache
│   └── providers/                    # AI provider caches
│       ├── ollama/                   # Ollama model cache
│       ├── llama-cpp/                # llama.cpp model cache
│       └── huggingface/              # HuggingFace cache
│
├── logs/                             # Centralized logs
│   ├── bootstrap/                    # Bootstrap installation logs
│   ├── providers/                    # AI provider logs
│   │   ├── ollama/
│   │   ├── cursor/
│   │   └── claude/
│   └── tools/                        # Tool execution logs
│
├── config/                           # Configuration files
│   ├── noa.json                      # Main NOA config
│   ├── bootstrap-state.json          # Installation state tracking
│   ├── .npmrc                        # npm configuration
│   ├── .cargo/config.toml            # Cargo configuration
│   └── providers/                    # AI provider configs
│
├── lib/                              # Shared libraries
├── tmp/                              # Temporary files
│
└── [other noa directories...]
```

---

## Script Logic Flow

```
┌─────────────────────────────────────────────────────────────────┐
│ STEP 1: Platform Detection & Initialization                    │
├─────────────────────────────────────────────────────────────────┤
│ - Detect OS (Windows/macOS/Linux/WSL)                          │
│ - Detect architecture (x64/arm64)                              │
│ - Detect shell (PowerShell/Bash/Zsh)                           │
│ - Initialize noa_root directory structure                      │
│ - Load or create bootstrap-state.json                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ STEP 2: Verification & Inventory                               │
├─────────────────────────────────────────────────────────────────┤
│ For each tool/toolchain:                                       │
│   - Check if installed in CORRECT location (noa_root)          │
│   - Check if installed in WRONG location (system-wide)         │
│   - Check version meets minimum requirement                    │
│   - Determine action: SKIP | UPDATE | INSTALL | RELOCATE       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ STEP 3: Prerequisites Installation                             │
├─────────────────────────────────────────────────────────────────┤
│ - Git (required for everything)                                │
│ - Git LFS (required for large files)                           │
│ - GitHub CLI (required for workflows)                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ STEP 4: Toolchain Installation (Parallel where possible)       │
├─────────────────────────────────────────────────────────────────┤
│ - Rust + rustup + cargo (→ opt/rust/)                          │
│ - Go (→ opt/go/)                                               │
│ - Node.js + npm (→ opt/node/)                                  │
│ - Python + venv (→ opt/python/, opt/venv/)                     │
│ - protoc (→ bin/)                                              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ STEP 5: Quality & Security Tools (via package managers)        │
├─────────────────────────────────────────────────────────────────┤
│ - rustfmt, clippy (rustup component add)                       │
│ - golangci-lint (go install)                                   │
│ - eslint (npm install -g)                                      │
│ - ruff, semgrep (pip install in venv)                          │
│ - gitleaks, trivy, grype (direct binary download)              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ STEP 6: CLI Utilities (direct download to bin/)                │
├─────────────────────────────────────────────────────────────────┤
│ - jq, ripgrep, fd, bat, fzf                                    │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ STEP 7: Dev Tools (→ opt/dev-tools/, gitignored)               │
├─────────────────────────────────────────────────────────────────┤
│ - Cursor IDE (portable)                                        │
│ - VS Code (portable)                                           │
│ - Docker Desktop                                               │
│ - ChatGPT Desktop                                              │
│ - Claude Desktop                                               │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ STEP 8: Cache & Log Configuration                              │
├─────────────────────────────────────────────────────────────────┤
│ - Create symlinks for caches to cache/                         │
│ - Configure provider cache locations                           │
│ - Set up log rotation                                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ STEP 9: Shell Configuration & Environment                      │
├─────────────────────────────────────────────────────────────────┤
│ - Generate noa-env.ps1 / .noa-env                              │
│ - Configure PATH with all tool locations                       │
│ - Set all HOME/CACHE environment variables                     │
│ - Integrate with shell profile (optional)                      │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│ STEP 10: Verification & Report                                 │
├─────────────────────────────────────────────────────────────────┤
│ - Verify all tools accessible                                  │
│ - Run smoke tests                                              │
│ - Generate installation report                                 │
│ - Save state to bootstrap-state.json                           │
└─────────────────────────────────────────────────────────────────┘
```

---

## Phase 1: Foundation - Core Infrastructure

**Purpose**: Create the unified bootstrap script foundation

- [ ] T001 Create `scripts/bootstrap/bootstrap.ps1` - main unified entry point
- [ ] T002 Create `scripts/bootstrap/bootstrap.sh` - Unix equivalent
- [ ] T003 [P] Create `scripts/bootstrap/lib/logging.ps1` - centralized logging to `logs/bootstrap/`
- [ ] T004 [P] Create `scripts/bootstrap/lib/logging.sh`
- [ ] T005 [P] Create `scripts/bootstrap/lib/platform.ps1` - OS, arch, shell detection
- [ ] T006 [P] Create `scripts/bootstrap/lib/platform.sh`
- [ ] T007 [P] Create `scripts/bootstrap/lib/state.ps1` - bootstrap-state.json management
- [ ] T008 [P] Create `scripts/bootstrap/lib/state.sh`
- [ ] T009 Create `scripts/bootstrap/lib/verification.ps1` - tool verification logic
  - Check if tool exists at expected path
  - Check version against minimum
  - Detect tool in wrong location (system vs noa_root)
  - Return action: SKIP | UPDATE | INSTALL | RELOCATE
- [ ] T010 Create `scripts/bootstrap/lib/verification.sh`
- [ ] T011 [P] Create `scripts/bootstrap/lib/download.ps1` - download with caching
- [ ] T012 [P] Create `scripts/bootstrap/lib/download.sh`
- [ ] T013 Create `scripts/bootstrap/config/tools.json` - all tool definitions

---

## Phase 2: Directory Structure & State Management

**Purpose**: Create noa_root directory structure and state tracking

- [ ] T014 Create `scripts/bootstrap/lib/directories.ps1` - create all directories
  - Create bin/, opt/, cache/, logs/, config/, lib/, tmp/
  - Create all subdirectories per structure above
  - Create opt/dev-tools/ (gitignored)
- [ ] T015 Create `scripts/bootstrap/lib/directories.sh`
- [ ] T016 Update `.gitignore` to include dev-tools exclusions:
  ```
  # Dev tools (installed by bootstrap, not committed)
  opt/dev-tools/
  opt/dev-tools/**

  # IDE settings (user-specific)
  .vscode/
  .idea/
  .cursor/

  # Caches (regenerated)
  cache/

  # Logs (local)
  logs/
  ```
- [ ] T017 Create `config/bootstrap-state.json` schema and initial file
  - Track each tool: name, version, location, installed_at, status
  - Track last full run timestamp
  - Track platform info

---

## Phase 3: Prerequisites - Git & GitHub CLI

**Purpose**: Install core prerequisites needed by other tools

- [ ] T018 [US1] Create `scripts/bootstrap/installers/git.ps1`
  - Verify: Check if git exists
  - Install: Download Git portable or use winget
  - Configure: Set git config for noa (user, email, editor)
- [ ] T019 [US1] Create `scripts/bootstrap/installers/git.sh`
- [ ] T020 [US1] Create `scripts/bootstrap/installers/git-lfs.ps1`
- [ ] T021 [US1] Create `scripts/bootstrap/installers/git-lfs.sh`
- [ ] T022 [US1] Create `scripts/bootstrap/installers/gh.ps1` - GitHub CLI
- [ ] T023 [US1] Create `scripts/bootstrap/installers/gh.sh`

---

## Phase 4: Toolchains - Language Runtimes

**Purpose**: Install all language toolchains to noa_root/opt/

### US1: Rust Toolchain

- [ ] T024 [US1] [§3.1] Create `scripts/bootstrap/installers/rust.ps1`
  - **Verify**: Check `$NOA_ROOT/opt/rust/cargo/bin/rustc.exe` exists and version >= 1.83
  - **Verify wrong location**: Check if rustc exists in `%USERPROFILE%\.cargo\bin\`
  - **Relocate**: If in wrong location, export crates list, remove old, install new
  - **Install**: Download rustup-init, set RUSTUP_HOME/CARGO_HOME, install
  - **Update**: If version outdated, run `rustup update`
  - **Configure**: Set all env vars to noa_root paths
- [ ] T025 [US1] [§3.1] Create `scripts/bootstrap/installers/rust.sh`
- [ ] T026 [US1] Create Rust cache symlink: `cache/rust/ → opt/rust/cargo/registry/`

### US1: Go Toolchain

- [ ] T027 [US1] [§3.1] Create `scripts/bootstrap/installers/go.ps1`
  - **Verify**: Check `$NOA_ROOT/opt/go/bin/go.exe` exists and version >= 1.23
  - **Verify wrong location**: Check if go exists in `C:\Program Files\Go\`
  - **Install**: Download official zip, extract to opt/go/
  - **Update**: If version outdated, download new version
  - **Configure**: Set GOROOT, GOPATH, GOBIN, GOCACHE, GOMODCACHE
- [ ] T028 [US1] [§3.1] Create `scripts/bootstrap/installers/go.sh`
- [ ] T029 [US1] Create Go cache symlink: `cache/go/ → opt/go/pkg/mod/`

### US1: Node.js Toolchain

- [ ] T030 [US1] [§3.1] Create `scripts/bootstrap/installers/node.ps1`
  - **Verify**: Check `$NOA_ROOT/opt/node/node.exe` exists and version >= 22
  - **Verify wrong location**: Check if node exists in `C:\Program Files\nodejs\`
  - **Install**: Download official zip, extract to opt/node/
  - **Update**: If version outdated, download new version
  - **Configure**: Set npm_config_prefix, npm_config_cache, NODE_PATH
  - **Create**: .npmrc in noa_root with portable settings
- [ ] T031 [US1] [§3.1] Create `scripts/bootstrap/installers/node.sh`
- [ ] T032 [US1] Create npm cache symlink: `cache/npm/ → opt/npm-cache/`

### US1: Python Toolchain

- [ ] T033 [US1] [§3.1] Create `scripts/bootstrap/installers/python.ps1`
  - **Verify**: Check Python 3.12+ available (system or portable)
  - **Verify venv**: Check `$NOA_ROOT/opt/venv/` exists
  - **Install venv**: Create venv at opt/venv/
  - **Configure**: Set VIRTUAL_ENV, pip cache to cache/pip/
- [ ] T034 [US1] [§3.1] Create `scripts/bootstrap/installers/python.sh`
- [ ] T035 [US1] Create pip cache symlink: `cache/pip/` with PIP_CACHE_DIR

### US1: Protocol Buffers

- [ ] T036 [US1] [§3.1] Create `scripts/bootstrap/installers/protoc.ps1`
  - **Verify**: Check `$NOA_ROOT/bin/protoc.exe` exists and version >= 28
  - **Install**: Download from GitHub releases, extract binary to bin/
  - **Configure**: Extract includes to opt/protobuf/include/
- [ ] T037 [US1] [§3.1] Create `scripts/bootstrap/installers/protoc.sh`

---

## Phase 5: Quality & Security Tools

**Purpose**: Install code quality and security tools using package managers

### Via Rust (rustup/cargo)

- [ ] T038 [US1] Create `scripts/bootstrap/installers/rust-tools.ps1`
  - Install rustfmt: `rustup component add rustfmt`
  - Install clippy: `rustup component add clippy`
  - Verify: Check cargo-fmt and cargo-clippy exist
- [ ] T039 [US1] Create `scripts/bootstrap/installers/rust-tools.sh`

### Via Go (go install)

- [ ] T040 [US1] Create `scripts/bootstrap/installers/go-tools.ps1`
  - Install golangci-lint: `go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest`
  - Verify: Check golangci-lint in GOBIN
- [ ] T041 [US1] Create `scripts/bootstrap/installers/go-tools.sh`

### Via npm (npm install -g)

- [ ] T042 [US1] Create `scripts/bootstrap/installers/npm-tools.ps1`
  - Install eslint: `npm install -g eslint`
  - Verify: Check eslint in npm bin
- [ ] T043 [US1] Create `scripts/bootstrap/installers/npm-tools.sh`

### Via pip (in venv)

- [ ] T044 [US1] Create `scripts/bootstrap/installers/pip-tools.ps1`
  - Install ruff: `pip install ruff`
  - Install semgrep: `pip install semgrep`
  - Verify: Check ruff.exe and semgrep.exe in venv/Scripts/
- [ ] T045 [US1] Create `scripts/bootstrap/installers/pip-tools.sh`

### Direct Binary Downloads (Security Tools)

- [ ] T046 [US1] [§3.6] [P] Create `scripts/bootstrap/installers/gitleaks.ps1`
- [ ] T047 [US1] [§3.6] [P] Create `scripts/bootstrap/installers/trivy.ps1`
- [ ] T048 [US1] [§3.6] [P] Create `scripts/bootstrap/installers/grype.ps1`

---

## Phase 6: CLI Utilities

**Purpose**: Install standalone CLI utilities to bin/

- [ ] T049 [US1] [P] Create `scripts/bootstrap/installers/jq.ps1`
- [ ] T050 [US1] [P] Create `scripts/bootstrap/installers/ripgrep.ps1`
- [ ] T051 [US1] [P] Create `scripts/bootstrap/installers/fd.ps1`
- [ ] T052 [US1] [P] Create `scripts/bootstrap/installers/bat.ps1`
- [ ] T053 [US1] [P] Create `scripts/bootstrap/installers/fzf.ps1`
- [ ] T054 [US1] [P] Create `scripts/bootstrap/installers/delta.ps1` (git-delta)
- [ ] T055 [US1] [P] Create unified `scripts/bootstrap/installers/cli-tools.sh` for all Unix

---

## Phase 7: Dev Tools (Gitignored)

**Purpose**: Install development applications to opt/dev-tools/

### US2: IDE & Editor Installation

- [ ] T056 [US2] Create `scripts/bootstrap/installers/dev-tools/cursor.ps1`
  - **Verify**: Check opt/dev-tools/cursor/ exists
  - **Install**: Download Cursor portable/installer
  - **Configure**: Set settings to use noa_root paths
  - **Log location**: logs/providers/cursor/
- [ ] T057 [US2] Create `scripts/bootstrap/installers/dev-tools/vscode.ps1`
  - **Verify**: Check opt/dev-tools/vscode/ exists
  - **Install**: Download VS Code portable
  - **Configure**: Set settings.json with noa_root paths
- [ ] T058 [US2] Create `scripts/bootstrap/installers/dev-tools/cursor.sh`
- [ ] T059 [US2] Create `scripts/bootstrap/installers/dev-tools/vscode.sh`

### US2: Docker Installation

- [ ] T060 [US2] Create `scripts/bootstrap/installers/dev-tools/docker.ps1`
  - **Verify**: Check Docker available
  - **Install Windows**: Download Docker Desktop or configure WSL2 Docker
  - **Configure**: Set DOCKER_CONFIG to config/docker/
  - **Cache**: Set docker cache to cache/docker/
  - **Log location**: logs/tools/docker/
- [ ] T061 [US2] Create `scripts/bootstrap/installers/dev-tools/docker.sh`

### US2: AI Desktop Apps

- [ ] T062 [US2] Create `scripts/bootstrap/installers/dev-tools/chatgpt-desktop.ps1`
  - **Verify**: Check if ChatGPT Desktop installed
  - **Install**: Download from OpenAI
  - **Configure**: Set cache/log locations if configurable
- [ ] T063 [US2] Create `scripts/bootstrap/installers/dev-tools/claude-desktop.ps1`
  - **Verify**: Check if Claude Desktop installed
  - **Install**: Download from Anthropic
  - **Configure**: Set config location to config/providers/claude/
  - **Log location**: logs/providers/claude/
- [ ] T064 [US2] Create `scripts/bootstrap/installers/dev-tools/ai-apps.sh`

### US2: Additional Dev Tools

- [ ] T065 [US2] [P] Create `scripts/bootstrap/installers/dev-tools/postman.ps1`
- [ ] T066 [US2] [P] Create `scripts/bootstrap/installers/dev-tools/insomnia.ps1`
- [ ] T067 [US2] [P] Create `scripts/bootstrap/installers/dev-tools/dbeaver.ps1`

---

## Phase 8: Cache & Log Configuration

**Purpose**: Centralize all caches and logs in noa_root

- [ ] T068 [US3] Create `scripts/bootstrap/config/cache-setup.ps1`
  - Create cache/ directory structure
  - Create symlinks for tool caches:
    - `cache/rust/` ← cargo registry
    - `cache/go/` ← go modules
    - `cache/npm/` ← npm cache
    - `cache/pip/` ← pip cache
    - `cache/docker/` ← docker layers
  - Configure environment variables for cache locations
- [ ] T069 [US3] Create `scripts/bootstrap/config/cache-setup.sh`

### US3: AI Provider Cache Configuration

- [ ] T070 [US3] Create `scripts/bootstrap/config/provider-cache.ps1`
  - **Ollama**: Set OLLAMA_MODELS to cache/providers/ollama/
  - **llama.cpp**: Set model directory to cache/providers/llama-cpp/
  - **HuggingFace**: Set HF_HOME to cache/providers/huggingface/
  - **LangChain**: Set cache to cache/providers/langchain/
- [ ] T071 [US3] Create `scripts/bootstrap/config/provider-cache.sh`

### US3: Log Configuration

- [ ] T072 [US3] Create `scripts/bootstrap/config/log-setup.ps1`
  - Create logs/ directory structure
  - Configure log rotation (keep last 7 days)
  - Set tool log locations:
    - Bootstrap logs → logs/bootstrap/
    - Provider logs → logs/providers/{provider}/
    - Tool logs → logs/tools/
- [ ] T073 [US3] Create `scripts/bootstrap/config/log-setup.sh`

---

## Phase 9: Environment Configuration

**Purpose**: Generate shell configuration files

- [ ] T074 [US1] Create `scripts/bootstrap/generators/noa-env.ps1`
  - Generate `noa-env.ps1` with ALL environment variables:
  ```powershell
  # NOA Environment Configuration
  # Generated by bootstrap - DO NOT EDIT MANUALLY

  $env:NOA_ROOT = "N:\noa"

  # Toolchain Homes
  $env:RUSTUP_HOME = "$env:NOA_ROOT\opt\rust\rustup"
  $env:CARGO_HOME = "$env:NOA_ROOT\opt\rust\cargo"
  $env:GOROOT = "$env:NOA_ROOT\opt\go"
  $env:GOPATH = "$env:NOA_ROOT\opt\go\workspace"
  $env:GOBIN = "$env:NOA_ROOT\opt\go\workspace\bin"
  $env:GOCACHE = "$env:NOA_ROOT\cache\go\build"
  $env:GOMODCACHE = "$env:NOA_ROOT\cache\go\mod"
  $env:NODE_PATH = "$env:NOA_ROOT\opt\node\node_modules"
  $env:npm_config_prefix = "$env:NOA_ROOT\opt\node"
  $env:npm_config_cache = "$env:NOA_ROOT\cache\npm"
  $env:VIRTUAL_ENV = "$env:NOA_ROOT\opt\venv"
  $env:PIP_CACHE_DIR = "$env:NOA_ROOT\cache\pip"

  # AI Provider Caches
  $env:OLLAMA_MODELS = "$env:NOA_ROOT\cache\providers\ollama"
  $env:HF_HOME = "$env:NOA_ROOT\cache\providers\huggingface"

  # Docker
  $env:DOCKER_CONFIG = "$env:NOA_ROOT\config\docker"

  # Unified PATH
  $env:PATH = @(
      "$env:NOA_ROOT\bin",
      "$env:NOA_ROOT\opt\rust\cargo\bin",
      "$env:NOA_ROOT\opt\go\bin",
      "$env:NOA_ROOT\opt\go\workspace\bin",
      "$env:NOA_ROOT\opt\node",
      "$env:NOA_ROOT\opt\node\node_modules\.bin",
      "$env:NOA_ROOT\opt\venv\Scripts",
      $env:PATH
  ) -join ";"
  ```
- [ ] T075 [US1] Create `scripts/bootstrap/generators/noa-env.sh`
- [ ] T076 [US1] Create `scripts/bootstrap/generators/shell-integration.ps1`
  - Add to PowerShell profile (optional, user consent)
- [ ] T077 [US1] Create `scripts/bootstrap/generators/shell-integration.sh`
  - Add to .bashrc/.zshrc (optional, user consent)

---

## Phase 10: Main Orchestrator

**Purpose**: Create the unified entry point that orchestrates all phases

- [ ] T078 Create main orchestration in `scripts/bootstrap/bootstrap.ps1`:
  ```powershell
  # Usage:
  #   .\bootstrap.ps1                    # Full install
  #   .\bootstrap.ps1 -SkipDevTools      # Skip IDE installation
  #   .\bootstrap.ps1 -UpdateOnly        # Only update existing tools
  #   .\bootstrap.ps1 -Verify            # Only verify, no install
  #   .\bootstrap.ps1 -Force             # Force reinstall everything

  param(
      [string]$NoaRoot = "N:\noa",
      [switch]$SkipDevTools,
      [switch]$SkipProfile,
      [switch]$UpdateOnly,
      [switch]$Verify,
      [switch]$Force,
      [switch]$Verbose
  )
  ```
- [ ] T079 Create main orchestration in `scripts/bootstrap/bootstrap.sh`
- [ ] T080 Implement phase execution order in bootstrap.ps1:
  1. Platform detection
  2. Directory creation
  3. Load/create state
  4. Verification pass (determine actions for each tool)
  5. Prerequisites (Git, etc.)
  6. Toolchains (parallel where possible)
  7. Quality tools
  8. CLI utilities
  9. Dev tools (if not skipped)
  10. Cache/log configuration
  11. Environment generation
  12. Final verification
  13. Save state and report

---

## Phase 11: Verification & Reporting

**Purpose**: Comprehensive verification and installation report

- [ ] T081 [§3.12] Create `scripts/bootstrap/verify/verify-all.ps1`
  - Verify each tool is accessible via PATH
  - Verify each tool meets minimum version
  - Verify all caches point to correct locations
  - Verify all logs can be written
  - Output JSON report
- [ ] T082 [§3.12] Create `scripts/bootstrap/verify/verify-all.sh`
- [ ] T083 [§3.12] Create `scripts/bootstrap/verify/smoke-test.ps1`
  - Compile and run minimal Rust program
  - Compile and run minimal Go program
  - Run minimal Node.js script
  - Run minimal Python script
  - Run protoc on test .proto
  - Run each security tool on test file
- [ ] T084 [§3.12] Create `scripts/bootstrap/verify/smoke-test.sh`
- [ ] T085 Create `scripts/bootstrap/report/generate-report.ps1`
  - Generate human-readable installation report
  - List all installed tools with versions
  - List all configured paths
  - List any warnings or issues
  - Save to logs/bootstrap/report-{timestamp}.md

---

## Phase 12: Integration & Migration

**Purpose**: Integrate with existing scripts and migrate old setups

- [ ] T086 Update `scripts/setup.ps1` to delegate to bootstrap.ps1
- [ ] T087 Deprecate old scripts with redirect messages:
  - scripts/setup/setup-noa.ps1
  - scripts/setup/install-prereqs.ps1
  - scripts/download-static-binaries.ps1
- [ ] T088 Create migration script for existing installations:
  - Detect tools in old locations
  - Offer to relocate to noa_root
  - Export package lists before migration
  - Re-import packages after migration
- [ ] T089 Update `noa-env.ps1` to source generated environment
- [ ] T090 Update `scripts/noa.ps1` validate command to use bootstrap verification

---

## Phase 13: Documentation

**Purpose**: Document the unified bootstrap system

- [ ] T091 [P] Create `scripts/bootstrap/README.md`
  - Quick start guide
  - All available options
  - Troubleshooting common issues
- [ ] T092 [P] Create `docs/setup/bootstrap-complete-guide.md`
  - Full documentation of all features
  - Directory structure explanation
  - Environment variable reference
  - Cache and log management
- [ ] T093 [P] Update main `README.md` with new bootstrap instructions
- [ ] T094 [P] Create `scripts/bootstrap/TOOLS.md` - list of all tools with versions

---

## Phase 14: Constitutional Verification

**Purpose**: Validate compliance with NOA constitution

- [ ] T095 [§3.1] Verify all paths resolve under noa_root (except explicit system tools)
- [ ] T096 [§3.2] Test offline functionality with pre-cached archives
- [ ] T097 [§3.5] Verify all actions logged to logs/bootstrap/
- [ ] T098 [§3.6] Security review: HTTPS downloads, checksum verification
- [ ] T099 [§3.12] Run full verification suite
- [ ] T100 Final sign-off: All tools working, all caches configured, all logs centralized

---

## Tool Inventory Summary

### Toolchains (opt/)
| Tool | Location | Min Version | Package Manager |
|------|----------|-------------|-----------------|
| Rust | opt/rust/ | 1.83.0 | cargo install |
| Go | opt/go/ | 1.23.0 | go install |
| Node.js | opt/node/ | 22.0.0 | npm install |
| Python | opt/venv/ | 3.12.0 | pip install |
| protoc | bin/ | 28.0.0 | N/A |

### Quality Tools (via package managers)
| Tool | Installer | Location |
|------|-----------|----------|
| rustfmt | rustup | opt/rust/cargo/bin/ |
| clippy | rustup | opt/rust/cargo/bin/ |
| golangci-lint | go install | opt/go/workspace/bin/ |
| eslint | npm install -g | opt/node/node_modules/.bin/ |
| ruff | pip install | opt/venv/Scripts/ |
| semgrep | pip install | opt/venv/Scripts/ |

### Security Tools (bin/)
| Tool | Source | Min Version |
|------|--------|-------------|
| gitleaks | GitHub releases | 8.21.0 |
| trivy | GitHub releases | 0.57.0 |
| grype | GitHub releases | 0.84.0 |

### CLI Utilities (bin/)
| Tool | Source |
|------|--------|
| jq | GitHub releases |
| ripgrep (rg) | GitHub releases |
| fd | GitHub releases |
| bat | GitHub releases |
| fzf | GitHub releases |
| delta | GitHub releases |

### Dev Tools (opt/dev-tools/, gitignored)
| Tool | Type |
|------|------|
| Cursor | IDE |
| VS Code | IDE |
| Docker Desktop | Container Runtime |
| ChatGPT Desktop | AI Assistant |
| Claude Desktop | AI Assistant |
| Postman | API Testing |
| DBeaver | Database GUI |

---

## Cache & Log Locations

### Caches (cache/)
| Cache | Environment Variable | Symlinked From |
|-------|---------------------|----------------|
| Rust registry | CARGO_HOME | opt/rust/cargo/registry |
| Go modules | GOMODCACHE | opt/go/pkg/mod |
| npm | npm_config_cache | opt/npm-cache |
| pip | PIP_CACHE_DIR | - |
| Docker | DOCKER_CONFIG | - |
| Ollama models | OLLAMA_MODELS | - |
| HuggingFace | HF_HOME | - |

### Logs (logs/)
| Log Category | Location |
|--------------|----------|
| Bootstrap | logs/bootstrap/ |
| AI Providers | logs/providers/{provider}/ |
| Tool execution | logs/tools/ |

---

## Notes

- **Single Entry Point**: `.\scripts\bootstrap\bootstrap.ps1` handles everything
- **Verification First**: Always check before downloading
- **Relocate vs Reinstall**: Offer to move from wrong location
- **Dev Tools Gitignored**: Never commit IDE/app installations
- **Centralized Cache**: All caches in `cache/` for easy backup/sync
- **Centralized Logs**: All logs in `logs/` for easy debugging

---

**Total Tasks**: 100
**Parallelizable Tasks**: 35 (35%)
**Estimated Duration**: 5-6 weeks (1-2 developers)
