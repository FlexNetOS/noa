# Tasks: NOA Seed Foundation (Unified)

**Input**: Design documents from `/specs/001-noa-seed-foundation/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅
**Includes**: Bootstrap installer (formerly 002-unified-bootstrap) - MERGED

**Organization**: Tasks grouped by phase, with Phase 0 (Bootstrap) running FIRST.

## Format: `[ID] [P?] [Story?] [Principle?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: US1-US10 from spec.md (or BOOT for bootstrap-specific)
- **[Principle]**: Constitutional principle tag (§3.1-§4.6)
- **Task Prefix**: B = Bootstrap (Phase 0), T = Core (Phase 1+)

## Constitutional Principle Tags

| Tag | Principle |
|-----|-----------|
| §3.1 | Self-Contained & Autonomous |
| §3.2 | Local-First & Offline-Capable |
| §3.3 | Agentic Orchestration |
| §3.4 | Adaptive & Self-Improving |
| §3.5 | Transparent & Auditable |
| §3.6 | Security & Privacy |
| §3.7 | Total Memory Sovereignty |
| §3.8 | P2P Hive-Mind |
| §3.9 | Truth & Knowledge Seeking |
| §3.10 | Biblical Governance |
| §3.12 | Test Everything, Trust Nothing |
| §4.6 | Goals-Policy-Rules Flow |

## User Story Summary

| Story | Title | Priority | MVP |
|-------|-------|----------|-----|
| BOOT | Bootstrap - Unified Environment Setup | P0 | ✅ |
| US1 | Initialize NOA Seed Environment | P1 | ✅ |
| US2 | Multi-SLM Neural Runtime | P1 | ✅ |
| US3 | Total Memory Sovereignty | P1 | ✅ |
| US4 | Digest Everything Pipeline | P2 | |
| US5 | Dynamic Context-Aware UI | P2 | |
| US6 | P2P Hive-Mind Device Federation | P2 | |
| US7 | Autonomous Agent Orchestration | P2 | |
| US8 | Self-Improvement & Code Modification | P3 | |
| US9 | Cross-Platform Deployment | P3 | |
| US10 | Connectors & External Integration | P3 | |

---

## ═══════════════════════════════════════════════════════════════
## PHASE 0: UNIFIED BOOTSTRAP (CRITICAL - RUN FIRST)
## ═══════════════════════════════════════════════════════════════

**Purpose**: Complete environment setup - ONE SCRIPT installs EVERYTHING

**Script Chaining Architecture:**

```
setup-noa.ps1 / setup-noa.sh
    │
    ├── Creates directory structure (noa_root/*)
    │
    ├── Creates AI provider directories (ai/providers/cloud/*, ai/providers/hybrid/*)
    │
    ├── Creates shared resource directories (ai/shared/*)
    │
    ├── [--install-all-tools] ──► install-all-tools.ps1 / .sh
    │       │
    │       ├── rust, go, protoc, node, python (toolchains)
    │       ├── golangci-lint, eslint, ruff (quality)
    │       ├── gitleaks, trivy, grype, semgrep (security)
    │       │
    │       └── [ai-providers] ──► Install ALL AI Provider CLIs
    │               │
    │               ├── claude-code (npm/@anthropic-ai/claude-code OR git clone)
    │               ├── codex-cli (npm/@openai/codex OR git clone)
    │               ├── cursor-cli (manual download from cursor.com)
    │               └── abacus-cli (npm/@abacus-ai/cli)
    │               │
    │               └── Uses bootstrap/installers/ai-providers/*.ps1/.sh
    │
    ├── [--install-shared-resources] ──► Initialize ai/shared/ resources
    │       │
    │       ├── Create execution-memory.db (shared memory bus)
    │       ├── Create context store, reasoning state tables
    │       └── Configure provider state sync
    │
    ├── Generates noa-profile.ps1 / .sh
    │
    └── Generates config/noa.json + config/shared-resources.json
```

**Entry Points:**
- **Quick Setup (directories only)**:
  - Windows: `.\scripts\setup\setup-noa.ps1`
  - Unix: `./scripts/setup/setup-noa.sh`

- **Full Setup (with all toolchains + AI providers)**:
  - Windows: `.\scripts\setup\setup-noa.ps1 -InstallAllTools -InstallAiProviders`
  - Unix: `./scripts/setup/setup-noa.sh --install-all-tools --install-ai-providers`

- **Bootstrap (alternative full setup)**:
  - Windows: `.\scripts\bootstrap\bootstrap.ps1`
  - Unix: `./scripts/bootstrap/bootstrap.sh`

**Features:**
- ✅ All toolchains portable to `noa_root/opt/`
- ✅ Cross-platform parity (PS1 ↔ Bash)
- ✅ Kernel independence (can run without host kernel)
- ✅ Idempotent (safe to re-run)
- ✅ Dev tools gitignored
- ✅ AI Provider CLIs integrated (FR-039): claude-code, codex, cursor, abacus
- ✅ Shared Provider Resources (FR-037 to FR-042): ai/shared/
- ✅ Shared Execution Memory Bus for collaborative reasoning
- ✅ Proper script chaining (setup → install-all → individual installers)

---

### Phase 0.1: Foundation - Core Infrastructure

**Purpose**: Create unified bootstrap script foundation

- [X] B001 Create `scripts/bootstrap/bootstrap.ps1` - main unified Windows entry point
- [X] B002 Create `scripts/bootstrap/bootstrap.sh` - main unified Unix entry point
- [X] B003 [P] Create `scripts/bootstrap/lib/logging.ps1` - centralized logging
- [X] B004 [P] Create `scripts/bootstrap/lib/logging.sh`
- [X] B005 [P] Create `scripts/bootstrap/lib/platform.ps1` - OS, arch, shell detection
- [X] B006 [P] Create `scripts/bootstrap/lib/platform.sh`
- [X] B007 [P] Create `scripts/bootstrap/lib/state.ps1` - bootstrap-state.json management
- [X] B008 [P] Create `scripts/bootstrap/lib/state.sh`
- [X] B009 Create `scripts/bootstrap/lib/verification.ps1` - tool verification (SKIP|UPDATE|INSTALL|RELOCATE)
- [X] B010 Create `scripts/bootstrap/lib/verification.sh`
- [X] B011 [P] Create `scripts/bootstrap/lib/download.ps1` - download with caching
- [X] B012 [P] Create `scripts/bootstrap/lib/download.sh`
- [X] B013 Create `scripts/bootstrap/config/tools.json` - all tool definitions

---

### Phase 0.2: Directory Structure & State Management

**Purpose**: Create noa_root directory structure and state tracking

- [X] B014 Create `scripts/bootstrap/lib/directories.ps1` - create bin/, opt/, cache/, logs/, config/, lib/, tmp/
- [X] B015 Create `scripts/bootstrap/lib/directories.sh`
- [X] B016 Update `.gitignore` with dev-tools exclusions, caches, logs
- [X] B017 Create `config/bootstrap-state.json` schema (tool tracking)

---

### Phase 0.3: Prerequisites - Git & GitHub CLI

**Purpose**: Install core prerequisites needed by other tools

- [X] B018 [BOOT] Create `scripts/bootstrap/installers/git.ps1`
- [X] B019 [BOOT] Create `scripts/bootstrap/installers/git.sh`
- [X] B020 [BOOT] Create `scripts/bootstrap/installers/git-lfs.ps1`
- [X] B021 [BOOT] Create `scripts/bootstrap/installers/git-lfs.sh`
- [X] B022 [BOOT] Create `scripts/bootstrap/installers/gh.ps1` - GitHub CLI
- [X] B023 [BOOT] Create `scripts/bootstrap/installers/gh.sh`

---

### Phase 0.4: Portable Toolchains (FR-081)

**Purpose**: Install ALL language toolchains to noa_root/opt/ - NO system-wide installs

#### Rust Toolchain → noa_root/opt/rust/

- [X] B024 [BOOT] [§3.1] Create `scripts/bootstrap/installers/rust-portable.ps1`
  - Download rustup-init, install to opt/rust/
  - Set RUSTUP_HOME, CARGO_HOME
  - Verify: rustc >= 1.83
- [X] B025 [BOOT] [§3.1] Create `scripts/bootstrap/installers/rust-portable.sh`
- [X] B026 [BOOT] Create Rust cache symlink: `cache/rust/ → opt/rust/cargo/registry/` (via cache-setup.ps1)

#### Go Toolchain → noa_root/opt/go/

- [X] B027 [BOOT] [§3.1] Create `scripts/bootstrap/installers/go-portable.ps1`
  - Download go.zip, extract to opt/go/
  - Set GOROOT, GOPATH, GOBIN, GOCACHE, GOMODCACHE
  - Verify: go >= 1.23
- [X] B028 [BOOT] [§3.1] Create `scripts/bootstrap/installers/go-portable.sh`
- [X] B029 [BOOT] Create Go cache symlink: `cache/go/ → opt/go/pkg/mod/` (via cache-setup.ps1)

#### Node.js Toolchain → noa_root/opt/node/

- [X] B030 [BOOT] [§3.1] Create `scripts/bootstrap/installers/node-portable.ps1`
  - Download node.zip, extract to opt/node/
  - Set npm_config_prefix, npm_config_cache, NODE_PATH
  - Verify: node >= 22
- [X] B031 [BOOT] [§3.1] Create `scripts/bootstrap/installers/node-portable.sh`
- [X] B032 [BOOT] Create npm cache symlink: `cache/npm/ → opt/npm-cache/` (via cache-setup.ps1)

#### Python Toolchain → noa_root/opt/python/ + opt/venv/

- [X] B033 [BOOT] [§3.1] Create `scripts/bootstrap/installers/python-portable.ps1`
  - Download embed zip or use python-build-standalone
  - Create venv at opt/venv/
  - Set VIRTUAL_ENV, PIP_CACHE_DIR
  - Verify: python >= 3.12
- [X] B034 [BOOT] [§3.1] Create `scripts/bootstrap/installers/python-portable.sh`
- [X] B035 [BOOT] Create pip cache directory: `cache/pip/` (via cache-setup.ps1)

#### Protocol Buffers → noa_root/bin/

- [X] B036 [BOOT] [§3.1] Create `scripts/bootstrap/installers/protoc-portable.ps1`
  - Download from GitHub releases
  - Extract binary to bin/, includes to opt/protobuf/include/
  - Verify: protoc >= 28
- [X] B037 [BOOT] [§3.1] Create `scripts/bootstrap/installers/protoc-portable.sh`

---

### Phase 0.5: Quality & Security Tools

**Purpose**: Install code quality and security tools via package managers

#### Via Rust (rustup/cargo)

- [X] B038 [BOOT] Create `scripts/bootstrap/installers/rust-tools.ps1` (rustfmt, clippy)
- [X] B039 [BOOT] Create `scripts/bootstrap/installers/rust-tools.sh`

#### Via Go (go install)

- [X] B040 [BOOT] Create `scripts/bootstrap/installers/go-tools.ps1` (golangci-lint)
- [X] B041 [BOOT] Create `scripts/bootstrap/installers/go-tools.sh`

#### Via npm (npm install -g)

- [X] B042 [BOOT] Create `scripts/bootstrap/installers/npm-tools.ps1` (eslint)
- [X] B043 [BOOT] Create `scripts/bootstrap/installers/npm-tools.sh`

#### Via pip (in venv)

- [X] B044 [BOOT] Create `scripts/bootstrap/installers/pip-tools.ps1` (ruff, semgrep)
- [X] B045 [BOOT] Create `scripts/bootstrap/installers/pip-tools.sh`

#### Direct Binary Downloads (Security Tools → bin/)

- [X] B046 [BOOT] [§3.6] [P] Create `scripts/bootstrap/installers/gitleaks.ps1`
- [X] B047 [BOOT] [§3.6] [P] Create `scripts/bootstrap/installers/trivy.ps1`
- [X] B048 [BOOT] [§3.6] [P] Create `scripts/bootstrap/installers/grype.ps1`
- [X] B049 [BOOT] [§3.6] [P] Create `scripts/bootstrap/installers/security-tools.sh` (all Unix)

---

### Phase 0.6: CLI Utilities

**Purpose**: Install standalone CLI utilities to bin/

- [X] B050 [BOOT] [P] Create `scripts/bootstrap/installers/jq.ps1`
- [X] B051 [BOOT] [P] Create `scripts/bootstrap/installers/ripgrep.ps1`
- [X] B052 [BOOT] [P] Create `scripts/bootstrap/installers/fd.ps1`
- [X] B053 [BOOT] [P] Create `scripts/bootstrap/installers/bat.ps1`
- [X] B054 [BOOT] [P] Create `scripts/bootstrap/installers/fzf.ps1`
- [X] B055 [BOOT] [P] Create `scripts/bootstrap/installers/delta.ps1` (git-delta)
- [X] B056 [BOOT] [P] Create unified `scripts/bootstrap/installers/cli-tools.sh` (all Unix)

---

### Phase 0.6.5: AI Provider CLIs (FR-039)

**Purpose**: Install ALL AI provider CLI tools to opt/ for shared execution memory

**Priority Order (Local > Hybrid > Cloud)**:
1. llama.cpp (Priority 1 - Local)
2. Cursor CLI (Priority 2 - Hybrid/IDE)
3. Claude Code CLI (Priority 3 - Cloud/CLI/IDE)
4. Codex CLI (Priority 4 - Cloud/CLI)
5. VS Code Copilot (Priority 5 - IDE only)
6. Git CLI (Priority 6 - Local)
7. Abacus CLI (Priority 7 - Cloud/CLI)

#### Claude Code CLI (Priority 3 - Cloud/CLI/IDE)

- [X] B057a [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/ai-providers/claude-code.ps1`
  - Install @anthropic-ai/claude-code via npm to opt/node/node_modules/
  - Alternative: Clone https://github.com/FlexNetOS/claude-code.git to opt/claude-code/
  - Create symlink: bin/claude → opt/node/node_modules/.bin/claude
  - Create provider config at ai/providers/cloud/claude-code/config.json
  - Configure shared resources path: ai/shared/
  - Verify: claude --version
- [X] B057b [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/ai-providers/claude-code.sh`

#### Cursor CLI (Priority 2 - Hybrid/IDE/CLI)

- [X] B057c [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/ai-providers/cursor-cli.ps1`
  - Download cursor CLI from cursor.com/docs/cli/headless
  - Install to opt/cursor-cli/
  - Create symlink: bin/cursor → opt/cursor-cli/cursor
  - Create provider config at ai/providers/hybrid/cursor/config.json
  - Configure shared resources path: ai/shared/
- [X] B057d [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/ai-providers/cursor-cli.sh`

#### Codex CLI (Priority 4 - Cloud/CLI)

- [X] B057e [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/ai-providers/codex-cli.ps1`
  - Install @openai/codex via npm to opt/node/node_modules/
  - Alternative: Clone https://github.com/FlexNetOS/codex.git to opt/codex/
  - Create symlink: bin/codex → opt/node/node_modules/.bin/codex
  - Create provider config at ai/providers/cloud/codex/config.json
  - Configure shared resources path: ai/shared/
  - Verify: codex --version
- [X] B057f [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/ai-providers/codex-cli.sh`

#### VS Code with Copilot (Priority 5 - IDE)

- [X] B057g [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/ai-providers/vscode-copilot.ps1`
  - Download VS Code portable to opt/dev-tools/vscode/
  - Create portable mode marker (data/ directory)
  - Install GitHub.copilot and GitHub.copilot-chat extensions
  - Create symlink: bin/code → opt/dev-tools/vscode/bin/code
  - Create provider config at ai/providers/ide/vscode-copilot/config.json
  - Configure shared resources path: ai/shared/
  - Verify: code --version && code --list-extensions | grep copilot
- [X] B057h [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/ai-providers/vscode-copilot.sh`

#### Git CLI Provider (Priority 6 - Local)

- [X] B057i [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/ai-providers/git-cli-provider.ps1`
  - Ensure PortableGit is installed to opt/dev-tools/git/
  - Create symlinks: bin/git → opt/dev-tools/git/cmd/git
  - Create provider config at ai/providers/local/git-cli/config.json
  - Configure shared resources path: ai/shared/
  - Verify: git --version
- [X] B057j [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/ai-providers/git-cli-provider.sh`

#### Abacus CLI (Priority 7 - Cloud/CLI)

- [X] B057k [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/ai-providers/abacus-cli.ps1`
  - Install @abacus-ai/cli via npm to opt/node/node_modules/
  - Create symlink: bin/abacus → opt/node/node_modules/.bin/abacus
  - Create provider config at ai/providers/cloud/abacus/config.json
  - Configure shared resources path: ai/shared/
  - Note: Requires Abacus Desktop app for initial auth
  - Verify: abacus --version
- [X] B057l [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/ai-providers/abacus-cli.sh`

#### Provider Setup Verification

- [X] B057m [BOOT] §3.12 Create `scripts/bootstrap/verify-ai-providers.ps1` - verify all provider CLIs
  - Check: claude, codex, cursor, code (vscode), git, abacusai
  - Verify shared resources access for each provider
  - Output: JSON report with status for each provider
- [X] B057n [BOOT] §3.12 Create `scripts/bootstrap/verify-ai-providers.sh`

---

### Phase 0.6.6: Shared Provider Resources (FR-037 to FR-042)

**Purpose**: Create shared resource directories and execution memory bus for ALL providers

**Shared Resources Architecture:**
```
ai/shared/
├── agents/           # Shared agent definitions
├── workflows/        # Shared workflow definitions
├── prompts/          # Shared prompt templates
├── skills/           # Shared skill definitions
├── tools/            # Shared MCP tools and functions
├── models/           # Shared model configs/adapters
├── commands/         # Shared command definitions
└── resources/        # Execution memory and state
    ├── execution-memory.db   # Shared execution memory bus
    ├── context/              # Shared context store
    └── state/                # Provider state sync
```

#### Shared Directory Structure

- [X] B058a [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/shared-resources/create-directories.ps1`
  - Create all ai/shared/ subdirectories
  - Set appropriate permissions for provider access
- [X] B058b [BOOT] §3.3 [P] Create `scripts/bootstrap/installers/shared-resources/create-directories.sh`

#### Shared Resource Templates

- [X] B058c [BOOT] §3.3 [P] Create `ai/shared/agents/README.md` - agent definition template
- [X] B058d [BOOT] §3.3 [P] Create `ai/shared/workflows/README.md` - workflow definition template
- [X] B058e [BOOT] §3.3 [P] Create `ai/shared/prompts/README.md` - prompt template guide
- [X] B058f [BOOT] §3.3 [P] Create `ai/shared/skills/README.md` - skill definition template
- [X] B058g [BOOT] §3.3 [P] Create `ai/shared/tools/README.md` - MCP tool definition template
- [X] B058h [BOOT] §3.3 [P] Create `ai/shared/models/README.md` - model adapter template
- [X] B058i [BOOT] §3.3 [P] Create `ai/shared/commands/README.md` - command definition template

#### Shared Execution Memory (FR-037)

- [X] B058j [BOOT] §3.7 Create `scripts/bootstrap/installers/shared-resources/execution-memory.ps1`
  - Initialize execution-memory.db (SQLite)
  - Create tables: context_store, reasoning_state, task_queue, provider_state
  - Enable shared access for all providers
- [X] B058k [BOOT] §3.7 Create `scripts/bootstrap/installers/shared-resources/execution-memory.sh`

#### Shared Execution Memory Schema

- [X] B058l [BOOT] §3.7 Create `ai/shared/resources/schema/execution-memory.sql`
  - context_store: id, provider, context_type, content, metadata, created_at, updated_at
  - reasoning_state: id, session_id, provider, state_json, checkpoint_at
  - task_queue: id, task_type, provider, priority, payload, status, created_at
  - provider_state: id, provider_name, state_key, state_value, synced_at

#### Provider Configuration Integration

- [X] B058m [BOOT] §3.3 Update all provider configs to reference ai/shared/ path
- [X] B058n [BOOT] §3.3 Create `config/shared-resources.json` - centralized shared resource config
- [X] B058o [BOOT] §3.3 Create `scripts/bootstrap/verify-shared-resources.ps1`
- [X] B058p [BOOT] §3.3 Create `scripts/bootstrap/verify-shared-resources.sh`

#### Collaborative Reasoning Support (FR-038)

- [X] B058q [BOOT] §3.3 Create `ai/shared/resources/collaborative-reasoning.yaml`
  - Define collaborative reasoning protocols
  - Configure multi-provider task distribution
  - Set up reasoning state synchronization

#### Parallel Task Distribution (FR-041)

- [X] B058r [BOOT] §3.3 Create `ai/shared/resources/task-distribution.yaml`
  - Configure parallel task queue
  - Define provider capability mapping
  - Set up load balancing rules

#### Provider State Sync (FR-042)

- [X] B058s [BOOT] §3.8 Create `scripts/bootstrap/installers/shared-resources/provider-sync.ps1`
  - Initialize provider state sync mechanism
  - Configure sync intervals and conflict resolution
- [X] B058t [BOOT] §3.8 Create `scripts/bootstrap/installers/shared-resources/provider-sync.sh`

---

### Phase 0.7: Dev Tools (Gitignored → opt/dev-tools/)

**Purpose**: Install development applications (gitignored, user-specific)

- [X] B059 [BOOT] Create `scripts/bootstrap/installers/dev-tools/cursor.ps1`
- [X] B060 [BOOT] Create `scripts/bootstrap/installers/dev-tools/vscode.ps1`
- [X] B061 [BOOT] Create `scripts/bootstrap/installers/dev-tools/cursor.sh`
- [X] B062 [BOOT] Create `scripts/bootstrap/installers/dev-tools/vscode.sh`
- [X] B063 [BOOT] Create `scripts/bootstrap/installers/dev-tools/docker.ps1`
- [X] B064 [BOOT] Create `scripts/bootstrap/installers/dev-tools/docker.sh`
- [X] B065 [BOOT] Create `scripts/bootstrap/installers/dev-tools/chatgpt-desktop.ps1`
- [X] B066 [BOOT] Create `scripts/bootstrap/installers/dev-tools/claude-desktop.ps1`
- [X] B067 [BOOT] Create `scripts/bootstrap/installers/dev-tools/ai-apps.sh`
- [X] B067a [BOOT] [P] Create `scripts/bootstrap/installers/dev-tools/postman.ps1`
- [X] B067b [BOOT] [P] Create `scripts/bootstrap/installers/dev-tools/dbeaver.ps1`

---

### Phase 0.8: Cache & Log Configuration

**Purpose**: Centralize all caches and logs in noa_root

- [X] B068 [BOOT] Create `scripts/bootstrap/config/cache-setup.ps1` (symlinks for tool caches)
- [X] B069 [BOOT] Create `scripts/bootstrap/config/cache-setup.sh`
- [X] B070 [BOOT] Create `scripts/bootstrap/config/provider-cache.ps1` (Ollama, HuggingFace, llama.cpp)
- [X] B071 [BOOT] Create `scripts/bootstrap/config/provider-cache.sh`
- [X] B072 [BOOT] Create `scripts/bootstrap/config/log-setup.ps1` (rotation, centralized logs)
- [X] B073 [BOOT] Create `scripts/bootstrap/config/log-setup.sh`

---

### Phase 0.9: Environment Configuration

**Purpose**: Generate shell configuration files

- [X] B074 [BOOT] Create `scripts/bootstrap/generators/noa-env.ps1` - generate noa-env.ps1
- [X] B075 [BOOT] Create `scripts/bootstrap/generators/noa-env.sh` - generate .noa-env
- [X] B076 [BOOT] Create `scripts/bootstrap/generators/shell-integration.ps1` (add to $PROFILE)
- [X] B077 [BOOT] Create `scripts/bootstrap/generators/shell-integration.sh` (add to .bashrc/.zshrc)

---

### Phase 0.10: Main Orchestrator

**Purpose**: Create unified entry point that orchestrates all phases

- [X] B078 Implement main orchestration in `scripts/bootstrap/bootstrap.ps1`
- [X] B079 Implement main orchestration in `scripts/bootstrap/bootstrap.sh`
- [X] B080 Implement phase execution order (platform → directories → state → verification → install → verify → report)

---

### Phase 0.11: Verification & Reporting

**Purpose**: Comprehensive verification and installation report

- [X] B081 [§3.12] Create `scripts/bootstrap/verify/verify-all.ps1`
- [X] B082 [§3.12] Create `scripts/bootstrap/verify/verify-all.sh`
- [X] B083 [§3.12] Create `scripts/bootstrap/verify/smoke-test.ps1` (compile/run minimal programs)
- [X] B084 [§3.12] Create `scripts/bootstrap/verify/smoke-test.sh`
- [X] B085 Create `scripts/bootstrap/report/generate-report.ps1`
- [X] B085a FR-088 Create `scripts/bootstrap/report/generate-report.sh` - Cross-platform parity

---

### Phase 0.12: Integration & Migration

**Purpose**: Integrate with existing scripts

- [X] B086 Update `scripts/setup.ps1` to delegate to bootstrap.ps1
- [X] B087 Deprecate old scripts with redirect messages
- [X] B088 Create migration script for existing installations
- [X] B089 Update `noa-env.ps1` to source generated environment
- [X] B090 Update `scripts/noa.ps1` validate command to use bootstrap verification

---

### Phase 0.13: Documentation

**Purpose**: Document the unified bootstrap system

- [X] B091 [P] Create `scripts/bootstrap/README.md`
- [X] B092 [P] Create `docs/setup/bootstrap-complete-guide.md`
- [X] B093 [P] Update main `README.md` with bootstrap instructions
- [X] B094 [P] Create `scripts/bootstrap/TOOLS.md` - list of all tools with versions

---

### Phase 0.14: Constitutional Verification

**Purpose**: Validate compliance with NOA constitution

- [X] B095 [§3.1] Verify all paths resolve under noa_root
- [X] B096 [§3.2] Test offline functionality with pre-cached archives
- [X] B097 [§3.5] Verify all actions logged to logs/bootstrap/
- [X] B098 [§3.6] Security review: HTTPS downloads, checksum verification
- [X] B099 [§3.12] Run full verification suite
- [X] B100 Final sign-off: All tools working, all caches configured

---

### Phase 0.15: Cross-Platform Script Parity (FR-088)

**Purpose**: Ensure ALL scripts have mirrored Windows (PS1) and Unix (Bash) versions

#### Service Scripts (ALL MIRRORED)
- [x] B101 [P] docker-service.ps1 / docker-service ✅
- [x] B102 [P] ollama-service.ps1 / ollama-service ✅
- [x] B103 [P] ssh-service.ps1 / ssh-service ✅
- [x] B104 [P] gitea-service.ps1 / gitea-service ✅

#### Kernel/System Scripts (ALL MIRRORED)
- [x] B105 [P] noa-kernel-params.ps1 / noa-kernel-params ✅
- [x] B106 [P] noa-kmod.ps1 / noa-kmod ✅
- [x] B107 [P] noa-namespace.ps1 / noa-namespace ✅

#### Library Bundling Scripts (ALL MIRRORED)
- [x] B108 [P] bundle-libraries.ps1 / bundle-libraries ✅
- [x] B109 [P] bundle-all-libs.ps1 / bundle-all-libs ✅
- [X] B110 [P] patch-binary-libs.ps1 (Windows equivalent for rpath patching)

#### Git Workflow Scripts (ALL MIRRORED)
- [x] B111 [P] git-ci.ps1 / git-ci ✅
- [x] B112 [P] git-conflict.ps1 / git-conflict ✅
- [x] B113 [P] git-pr.ps1 / git-pr ✅

#### Bootstrap Scripts (ALL MIRRORED)
- [x] B114 [P] bootstrap/bootstrap.ps1 / bootstrap/bootstrap.sh ✅
- [x] B115 [P] setup/check-prereqs.ps1 / init/check-prereqs.sh ✅
- [x] B116 [P] setup/install-prereqs.ps1 / (via bootstrap.sh) ✅

#### Verification
- [X] B117 Create cross-platform test suite
- [X] B118 Verify identical exit codes for all mirrored scripts
- [X] B119 Verify identical argument handling
- [X] B120 Create scripts/README.md with complete mapping table

---

### Phase 0.16: Kernel Independence Layer (FR-091 to FR-094)

**Purpose**: Enable NOA to operate independently of host kernel on ALL platforms

**Principle**: NOA CAN use host kernel but MUST be able to run independently.

#### Kernel Abstraction Design
- [X] B121 Design NKAL (NOA Kernel Abstraction Layer) interface in `sys/core/src/kernel/`
- [X] B122 Document kernel independence modes: native, vm, container, sandbox

#### Windows Kernel Independence
- [X] B123 Implement Hyper-V VM management in `sys/kernel/windows/hyperv/noa-vm.ps1`
- [X] B124 Configure TAP adapter for network isolation
- [X] B125 Create Windows Sandbox profile for lightweight isolation
- [X] B126 Build Hyper-V VM image (noa-linux.vhdx) with custom kernel

#### Linux Kernel Independence
- [X] B127 Implement KVM/QEMU VM launcher in `sys/kernel/linux/vm/`
- [X] B128 Configure namespace isolation in `sys/kernel/linux/namespaces/`
- [X] B129 Integrate container runtime (rootless Docker/Podman)
- [X] B130 Build QCOW2 VM image (noa-linux.qcow2) with custom kernel

#### macOS Kernel Independence
- [X] B131 Implement Virtualization.framework wrapper in `sys/kernel/macos/vm/`
- [X] B132 Build macOS VM image (noa-linux.dmg)
- [X] B133 Document limitations (Darwin kernel differences)

#### VM Image Build System
- [X] B134 Create Alpine-based minimal Linux image builder
- [X] B135 Include P2P kernel modules (tun, bridge, wireguard, nf_tables)
- [X] B136 Configure < 3 second boot time
- [X] B137 Package NOA runtime binaries in image
- [X] B138 Create image update/sync mechanism

#### Kernel Mode Switching
- [X] B139 Implement `noa-kernel-params set kernel_mode {native|vm|container}`
- [X] B140 Auto-detect available isolation methods per platform
- [X] B141 Graceful fallback when requested mode unavailable

#### Testing
- [X] B142 Test VM boot on Windows (Hyper-V)
- [X] B143 Test VM boot on Linux (KVM)
- [X] B144 Test VM boot on macOS (Virtualization.framework)
- [X] B145 Benchmark native vs VM performance overhead

---

### Phase 0.17: Platform Testing Matrix

**Purpose**: Verify all scripts work on all supported platforms

- [X] B146 Create GitHub Actions workflow for Windows testing
- [X] B147 Create GitHub Actions workflow for Ubuntu testing
- [X] B148 Create GitHub Actions workflow for macOS testing
- [X] B149 Create WSL2 testing in Windows runner
- [X] B150 Document manual test procedures for WSL1

---

## ═══════════════════════════════════════════════════════════════
## PHASE 1+: CORE NOA IMPLEMENTATION
## ═══════════════════════════════════════════════════════════════

**Prerequisites**: Phase 0 (Bootstrap) must be complete.
**Entry Point**: After bootstrap, run `noa init` to verify environment.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and monorepo structure

### Prerequisite Check (CRITICAL - Run First)

- Shimmed entrypoints (platform-aware):
  - Bash/WSL/macOS: `scripts/bash/check-prerequisites.sh --json`
  - PowerShell/Windows: `scripts/powershell/check-prerequisites.ps1 -Json`

- [x] T673 **CRITICAL** ✅ DONE: Implement prerequisite check script (bash) in `init/check-prereqs.sh`
  - Check: Rust 1.83+, Go 1.23+, Node 20+, Python 3.12+, protoc 28+
  - Check: rustfmt, clippy, golangci-lint 1.62+, eslint 9+, ruff 0.8+
  - Check: Gitleaks 8.21+, Trivy 0.57+, Grype 0.84+, Semgrep 1.97+
  - Check: Self-contained tools in noa_root/bin/ (per §3.1)
  - Output: ✅/❌/⚠️ per tool with install commands
- [x] T674 [P] **CRITICAL** ✅ DONE: Implement prerequisite check script (PowerShell) in `scripts/setup/check-prereqs.ps1`
  - Unified with setup scripts
  - Constitution-compliant (§3.1 self-contained check)
  - Use winget/choco install commands
- [ ] T675 [P] Add prerequisite check to CI pipeline in `.github/workflows/ci.yml`

- [ ] T001 §3.1 Create monorepo directory structure per plan.md in `noa_root/`

### Directory Structure (FR-029 to FR-036)

- [ ] T002 [P] §3.1 FR-029 Create and populate `noa_root/sys/` with system-level components: `core/`, `ui/`, `digest/`, `kernel/`
- [ ] T003 [P] §3.1 FR-030 Create and populate `noa_root/p2p/` with P2P networking: `discovery/`, `sync/`, `compute/`, `storage/`
- [ ] T004 [P] §3.1 FR-031 Create and populate `noa_root/opt/` with optional packages: `llama.cpp/`, `llama-cpp-rs/`, `ollama/`
- [ ] T005 [P] §3.1 FR-032 Create and populate `noa_root/init/` with init scripts: `migrations/`, `seeds/`, `noa-init`
- [ ] T006 [P] §3.1 FR-033 Create and populate `noa_root/containers/` with container definitions: `oci/`, `compose/`, `Dockerfile`
- [ ] T007 [P] §3.1 FR-034 Create and populate `noa_root/config/` with configuration files: `noa-server.json`, `ai-providers.json`, `features.json`
- [ ] T008 [P] §3.1 FR-035 Create and populate `noa_root/bin/` with executables: `noa`, `noa-server`, wrappers for `llama-cli`, `ollama`
- [ ] T009 [P] §3.1 FR-036 Create and populate `noa_root/ai/` with AI assets: `providers/`, `models/`, `prompts/`, `grammars/`

### Project Initialization

- [ ] T010 [P] §3.1 Initialize Rust workspace with Cargo.toml (all crates: api, embedder, trainer, indexer, ui, agent, common) in `noa_root/sys/core/Cargo.toml`
- [ ] T011 [P] §3.1 Initialize Go module for P2P services in `noa_root/p2p/go.mod`
- [ ] T012 [P] §3.1 Initialize TypeScript/Next.js project for UI in `noa_root/sys/ui/package.json`
- [ ] T013 [P] §3.1 Initialize Python project for digest pipeline in `noa_root/sys/digest/pyproject.toml`
- [ ] T014 [P] Configure linting: rustfmt, golangci-lint, eslint, ruff in `.config/`
- [ ] T015 [P] Create cross-platform build scripts in `scripts/bash/build.sh` and `scripts/powershell/build.ps1`
- [ ] T016 §3.1 Create environment configuration templates in `config/`
- [ ] T017 [P] Setup GitHub Actions CI pipeline in `.github/workflows/ci.yml`
- [ ] T018 Create README.md with quickstart instructions in `noa_root/README.md`

---

## Phase 2: Foundational - Database & Storage Infrastructure

**Purpose**: Core storage infrastructure required by ALL user stories

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Storage Components Setup

- [ ] T018a §3.2 Create data directory structure (`data/memory/`, `data/knowledge/`, `data/embeddings/`, `data/artifacts/`) in `noa_root/data/`
- [ ] T018b [P] Setup Private OCI Registry configuration in `containers/oci/registry.yaml`
- [ ] T018c [P] Setup MinIO S3-compatible storage config in `config/minio.yaml`
- [ ] T018d [P] Setup Postgres/SQLite configuration in `config/database.yaml`
- [ ] T018e [P] Setup Qdrant vector store configuration in `config/qdrant.yaml`
- [ ] T018f [P] Setup Quickwit hybrid search configuration in `config/quickwit.yaml`

### Database Schema (SQLite Primary)

- [ ] T018g §3.2 Create complete SQLite schema with all 14 entities from data-model.md in `init/migrations/001_initial.sql`
- [ ] T019 [P] §3.7 Create Memory table with JSON metadata and checksum in `init/migrations/001_initial.sql`
- [ ] T020 [P] §3.7 Create Embedding table with vector column (384-dim) in `init/migrations/001_initial.sql`
- [ ] T021 [P] §3.3 Create Agent table with stack_code support (mas_*, gen_mas) in `init/migrations/001_initial.sql`
- [ ] T022 [P] §3.5 Create AgentLog table (append-only audit) in `init/migrations/001_initial.sql`
- [ ] T023 [P] §3.3 Create Task table with priority, retry_count in `init/migrations/001_initial.sql`
- [ ] T024 [P] Create TaskEvent table with lifecycle events in `init/migrations/001_initial.sql`
- [ ] T025 [P] §3.3 Create MicroAgentStack table with 5-stage lifecycle in `init/migrations/001_initial.sql`
- [ ] T026 [P] Create Capsule table with manifest checksum in `init/migrations/001_initial.sql`
- [ ] T027 [P] Create KnowledgeNode table (type: function/class/module/file/repo/concept) in `init/migrations/001_initial.sql`
- [ ] T028 [P] Create KnowledgeEdge table (relationship: calls/imports/extends/implements/contains/references) in `init/migrations/001_initial.sql`
- [ ] T029 [P] Create DigestSource table (type: repository/file/api/document) in `init/migrations/001_initial.sql`
- [ ] T030 [P] Create Model table with provider, parameters, context_length in `init/migrations/001_initial.sql`
- [ ] T031 [P] §3.8 Create Device table with platform, peer_id, resources in `init/migrations/001_initial.sql`
- [ ] T032 [P] §3.8 Create SyncState table with vector clocks in `init/migrations/001_initial.sql`

### Additional Database Tables (from plan.md)

- [ ] T033 [P] Create traces table (run_id, action, input, output, duration_ms) in `init/migrations/001_initial.sql`
- [ ] T034 [P] Create claims table (statement, evidence, verified, timestamp) in `init/migrations/001_initial.sql`
- [ ] T035 [P] Create evidence table (claim_id, source, hash, timestamp) in `init/migrations/001_initial.sql`
- [ ] T036 [P] Create metrics table (name, value, unit, timestamp) in `init/migrations/001_initial.sql`
- [ ] T037 Create database indexes per data-model.md in `init/migrations/002_indexes.sql`

### Vector Storage Setup

- [ ] T038 §3.7 Integrate sqlite-vss extension for vector search in `init/migrations/003_vectors.sql`
- [ ] T039 [P] Create HNSW index on embeddings vector column in `init/migrations/003_vectors.sql`
- [ ] T040 [P] Setup pgvector extension (optional PostgreSQL scale-up) in `init/migrations/pg/001_pgvector.sql`

### CSV & Data Table Standards

- [ ] T041 §3.5 Implement CSV export service for all entities in `sys/core/src/export/csv_export.rs`
- [ ] T042 [P] Define CSV schema for Agent Directory in `config/schemas/csv/agent_directory.yaml`
- [ ] T043 [P] Define CSV schema for Task Tables in `config/schemas/csv/task_tables.yaml`
- [ ] T044 [P] Define CSV schema for Claims/Evidence Ledger in `config/schemas/csv/claims_evidence.yaml`
- [ ] T045 [P] Define CSV schema for Metrics/Traces in `config/schemas/csv/metrics_traces.yaml`

### Configuration Standards

- [ ] T046 Define unified JSON/YAML config schema in `config/schemas/config_schema.json`
- [ ] T047 [P] Implement config validation with rich metadata support in `sys/core/src/config/validator.rs`
- [ ] T048 [P] Implement config lineage/provenance tracking in `sys/core/src/config/lineage.rs`
- [ ] T049 [P] Create default config templates in `config/templates/`

### Rust Core Foundation

- [ ] T050 §3.1 Define core error types and Result wrapper in `sys/core/src/error.rs`
- [ ] T051 [P] §3.2 Implement configuration loader from JSON/YAML in `sys/core/src/config/mod.rs`
- [ ] T052 [P] §3.5 Implement structured logging with tracing in `sys/core/src/logging.rs`
- [ ] T053 §3.2 Implement SQLite connection pool with rusqlite in `sys/core/src/db/pool.rs`
- [ ] T054 §3.1 Define repository trait pattern in `sys/core/src/db/repository.rs`
- [ ] T055 §3.2 Implement database migration runner in `sys/core/src/db/migrations.rs`

### API Foundation

- [ ] T056 §3.2 Implement HTTP server with axum in `sys/core/src/api/server.rs`
- [ ] T057 [P] Implement health check endpoint GET /api/v1/health in `sys/core/src/api/routes/health.rs`
- [ ] T058 [P] §3.6 Implement request validation middleware in `sys/core/src/api/middleware/validation.rs`
- [ ] T059 [P] §3.5 Implement request logging middleware in `sys/core/src/api/middleware/logging.rs`
- [ ] T060 [P] §3.5 Implement OpenTelemetry tracing middleware in `sys/core/src/api/middleware/telemetry.rs`

### CLI Foundation

- [ ] T061 §3.1 Create CLI entry point with clap in `sys/core/src/main.rs`
- [ ] T062 [P] Implement `noa init` command in `sys/core/src/cli/init.rs`
- [ ] T063 [P] Implement `noa start` command in `sys/core/src/cli/start.rs`
- [ ] T064 [P] Implement `noa status` command in `sys/core/src/cli/status.rs`
- [ ] T065 [P] Implement `noa stop` command in `sys/core/src/cli/stop.rs`
- [ ] T066 [P] Implement `noa db check` command in `sys/core/src/cli/db.rs`
- [ ] T067 [P] Implement `noa db export` command in `sys/core/src/cli/db.rs`

### Observability Foundation

- [ ] T068 §3.5 Setup tracing-subscriber for log collection in `sys/core/src/observability/logging.rs`
- [ ] T069 [P] Setup opentelemetry OTLP export in `sys/core/src/observability/telemetry.rs`
- [ ] T070 [P] Setup prometheus metrics export in `sys/core/src/observability/metrics.rs`
- [ ] T071 [P] Create observability config in `config/observability.yaml`

**Checkpoint**: Foundation ready - Continue to 3-Plane Control Fabric ✅

---

## Phase 2.5: 3-Plane Control Fabric Architecture (Critical)

**Purpose**: Implement 3-plane architecture for zero-downtime self-updates with long-term memory persistence

**⚠️ CRITICAL**: This phase implements FR-056 through FR-060 and FR-071 through FR-075

**Reference**: See `project-mgmt/docs/07-plans/autonomous-system.md` for architecture details

### Autonomous Operation Entity Tables (data-model.md Entities 18-22)

- [ ] T545 §3.4 Create Goal table (unified goal queue: user, self_generated, constitutional) in `init/migrations/005_autonomous.sql`
- [ ] T546 [P] Create Plane table (sandbox/deployed/coordinator with role and status) in `init/migrations/005_autonomous.sql`
- [ ] T547 [P] Create PlaneTransition table (promotion/rollback audit trail) in `init/migrations/005_autonomous.sql`
- [ ] T548 [P] Create HealingEvent table (5-stage self-healing audit) in `init/migrations/005_autonomous.sql`
- [ ] T549 [P] Create HealthMetric table (continuous health monitoring) in `init/migrations/005_autonomous.sql`
- [ ] T550 [P] Create autonomous operation indexes in `init/migrations/005_autonomous.sql`

### 3-Plane Directory Structure

- [ ] T551 §3.1 Create sandbox-plane directory structure per plan.md in `noa_root/sandbox-plane/`
- [ ] T552 [P] Create deployed-plane directory structure per plan.md in `noa_root/deployed-plane/`
- [ ] T553 [P] Create coordinator-plane directory structure per plan.md in `noa_root/coordinator-plane/`
- [ ] T554 [P] Create shared cross-plane directory structure in `noa_root/shared/`
- [ ] T555 [P] Create run_sandbox.sh entry point in `sandbox-plane/bin/run_sandbox.sh`
- [ ] T556 [P] Create run_coordinator.sh entry point in `coordinator-plane/bin/run_coordinator.sh`
- [ ] T557 [P] Create run_deployed.sh entry point in `deployed-plane/bin/run_deployed.sh`

### 14 Components per Plane

- [ ] T558 Create agents component structure (bin/, manifests/, policy/, spec/, tests/) in `sandbox-plane/components/agents/`
- [ ] T559 [P] Create memory component structure in `sandbox-plane/components/memory/`
- [ ] T560 [P] Create models component structure in `sandbox-plane/components/models/`
- [ ] T561 [P] Create orchestrator component structure in `sandbox-plane/components/orchestrator/`
- [ ] T562 [P] Create dataplane component structure in `sandbox-plane/components/dataplane/`
- [ ] T563 [P] Create networking component structure in `sandbox-plane/components/networking/`
- [ ] T564 [P] Create observability component structure in `sandbox-plane/components/observability/`
- [ ] T565 [P] Create packaging component structure in `sandbox-plane/components/packaging/`
- [ ] T566 [P] Create perception component structure in `sandbox-plane/components/perception/`
- [ ] T567 [P] Create security component structure in `sandbox-plane/components/security/`
- [ ] T568 [P] Create system component structure in `sandbox-plane/components/system/`
- [ ] T569 [P] Create ui component structure in `sandbox-plane/components/ui/`
- [ ] T570 [P] Create update component structure in `sandbox-plane/components/update/`
- [ ] T571 [P] Create workflow component structure in `sandbox-plane/components/workflow/`

### Coordinator Plane Configuration

- [ ] T572 §3.4 Create analytics.yaml (llama.cpp swarm config) in `coordinator-plane/config/analytics.yaml`
- [ ] T573 [P] Create promotion-policy.yaml (risk tiers, canary cohorts, abort gates) in `coordinator-plane/config/promotion-policy.yaml`
- [ ] T574 [P] Create channels.yml (update channels) in `coordinator-plane/config/channels.yml`
- [ ] T575 [P] Create router.yml (traffic routing) in `coordinator-plane/config/router.yml`
- [ ] T576 [P] Create global.yaml (constitutional policy) in `coordinator-plane/policy/global.yaml`

### Coordinator Analytics Prompts

- [ ] T577 Create constitutional-court.md prompt in `coordinator-plane/prompts/constitutional-court.md`
- [ ] T578 [P] Create deployment-advisor.md prompt in `coordinator-plane/prompts/deployment-advisor.md`
- [ ] T579 [P] Create risk-analyst.md prompt in `coordinator-plane/prompts/risk-analyst.md`
- [ ] T580 [P] Create telemetry-forecaster.md prompt in `coordinator-plane/prompts/telemetry-forecaster.md`

### Biblical Governance ML Pipeline (FR-026) - Reference Implementation

**Purpose**: Transform biblical texts to ML-compatible format for constitutional governance *(P2+ scope)*

- [ ] T580a [P] §3.10 Create lexical analysis schema for biblical texts in `ai/governance/schema/lexical-schema.json`
- [ ] T580b [P] §3.10 Create placeholder for Greek/Hebrew text ingestion in `ai/governance/ingest/text-loader.py`
- [ ] T580c [P] §3.10 Create semantic embedding pipeline stub in `ai/governance/embeddings/biblical-embedder.py`
- [ ] T580d [P] §3.10 Create knowledge graph integration interface in `ai/governance/kg/biblical-kg.py`
- [ ] T580e §3.10 Document biblical governance data flow in `ai/governance/README.md`

### Shared Infrastructure

- [ ] T581 §3.8 Create shared/state/registry.db schema (capability metadata, deps, history) in `shared/state/registry_schema.sql`
- [ ] T582 [P] Create shared/artifacts/ structure for build outputs in `shared/artifacts/`
- [ ] T583 [P] Create shared/releases/ structure for promoted releases in `shared/releases/`
- [ ] T584 [P] Create shared/logs/ structure with plane subdirs in `shared/logs/`
- [ ] T585 [P] Create capability-pack-schema.yaml in `shared/config/capability-pack-schema.yaml`
- [ ] T586 [P] Create message-bus.toml (Redis/NATS config) in `shared/config/message-bus.toml`
- [ ] T587 [P] Create compose.yaml for container orchestration in `shared/runtime/compose.yaml`

### Promotion Policy Engine (FR-057)

- [ ] T588 §3.4 Implement promotion policy parser in `coordinator-plane/src/policy/parser.rs`
- [ ] T589 [P] Implement risk tier classifier (low/medium/high/critical) in `coordinator-plane/src/policy/risk_tier.rs`
- [ ] T590 [P] Implement canary cohort calculator in `coordinator-plane/src/policy/canary.rs`
- [ ] T591 [P] Implement abort gate checker (latency, error_rate, violations) in `coordinator-plane/src/policy/abort_gates.rs`
- [ ] T592 [P] Implement constitutional compliance checker in `coordinator-plane/src/policy/constitutional.rs`

### Coordinator Analytics Engine

- [ ] T593 §3.4 Implement llama.cpp analytics swarm runner in `coordinator-plane/src/analytics/swarm.rs`
- [ ] T594 [P] Implement artifact ingestion from sandbox in `coordinator-plane/src/analytics/ingest.rs`
- [ ] T595 [P] Implement capability evaluation in `coordinator-plane/src/analytics/evaluate.rs`
- [ ] T596 [P] Implement decision trace logging in `coordinator-plane/src/analytics/decision_trace.rs`

### Promotion Flow (FR-057, FR-058)

- [ ] T597 §3.4 Implement promotion orchestrator in `coordinator-plane/src/promotion/orchestrator.rs`
- [ ] T598 [P] Implement sandbox → coordinator artifact transfer in `coordinator-plane/src/promotion/transfer.rs`
- [ ] T599 [P] Implement coordinator → deployed release push in `coordinator-plane/src/promotion/push.rs`
- [ ] T600 [P] Implement rollback trigger on gate breach in `coordinator-plane/src/promotion/rollback.rs`

### Canary Controller (FR-058)

- [ ] T601 §3.4 Implement canary deployment controller in `deployed-plane/src/canary/controller.rs`
- [ ] T602 [P] Implement traffic splitting (1-10% cohort) in `deployed-plane/src/canary/traffic.rs`
- [ ] T603 [P] Implement SLO monitoring in `deployed-plane/src/canary/slo_monitor.rs`
- [ ] T604 [P] Implement autopilot rollback on SLO violation in `deployed-plane/src/canary/autopilot.rs`

### State Synchronization (FR-059)

- [ ] T605 §3.8 Implement shared state sync between planes in `shared/src/sync/state_sync.rs`
- [ ] T606 [P] Implement registry.db sync in `shared/src/sync/registry_sync.rs`
- [ ] T607 [P] Implement knowledge base version sync in `shared/src/sync/knowledge_sync.rs`
- [ ] T608 [P] Implement memory sync across planes in `shared/src/sync/memory_sync.rs`

### Plane Transition Audit (FR-060)

- [ ] T609 §3.5 Implement transition logger (before/after state) in `coordinator-plane/src/audit/transition_logger.rs`
- [ ] T610 [P] Implement decision rationale recorder in `coordinator-plane/src/audit/decision_record.rs`
- [ ] T611 [P] Implement transition query API in `coordinator-plane/src/audit/query.rs`

### Self-Healing Loop (FR-071-075)

- [ ] T612 §3.4 Implement continuous health monitoring in `sys/core/src/healing/monitor.rs`
- [ ] T613 [P] Implement anomaly detection in `sys/core/src/healing/anomaly.rs`
- [ ] T614 [P] Implement root cause analysis engine in `sys/core/src/healing/diagnose.rs`
- [ ] T615 §3.4 Implement auto-fix executor (restart, reconfig, rollback, redistribute) in `sys/core/src/healing/fix.rs`
- [ ] T616 [P] Implement fix validation in `sys/core/src/healing/validate.rs`
- [ ] T617 [P] Implement retry counter (≥3 attempts before escalate) in `sys/core/src/healing/retry.rs`
- [ ] T618 [P] Implement user escalation notification in `sys/core/src/healing/escalate.rs`
- [ ] T619 §3.5 Implement healing audit logger in `sys/core/src/healing/audit.rs`
- [ ] T620 [P] Implement plane swap for component recovery in `sys/core/src/healing/plane_swap.rs`

### Autonomous Continuous Loop (FR-051-055)

- [ ] T621 §3.4 Implement always-on continuous loop in `sys/core/src/autonomy/continuous_loop.rs`
- [ ] T622 [P] Implement user goal queue manager in `sys/core/src/autonomy/goal_queue.rs`
- [ ] T623 [P] Implement goal decomposition engine in `sys/core/src/autonomy/decompose.rs`
- [ ] T624 [P] Implement resource optimization monitor in `sys/core/src/autonomy/resource_optimizer.rs`
- [ ] T625 [P] Implement performance self-monitoring in `sys/core/src/autonomy/self_monitor.rs`

### Full Autonomy Operation (FR-061-065)

- [ ] T626 §3.4 Implement autonomous execution mode in `sys/core/src/autonomy/autonomous_mode.rs`
- [ ] T627 [P] Implement co-improvement goal intake in `sys/core/src/autonomy/co_improve.rs`
- [ ] T628 [P] Implement 3-plane rollback safety net in `sys/core/src/autonomy/safety_net.rs`
- [ ] T629 §3.5 Implement real-time activity log (observation only) in `sys/core/src/autonomy/activity_log.rs`
- [ ] T630 [P] Implement constitutional decision boundary in `sys/core/src/autonomy/constitutional_boundary.rs`

### Autonomous Goal Generation (FR-066-070)

- [ ] T631 §3.4 Implement self-generated goal engine in `sys/core/src/autonomy/goal_generator.rs`
- [ ] T632 [P] Implement constitutional goal boundary checker in `sys/core/src/autonomy/goal_boundary.rs`
- [ ] T633 §3.5 Implement goal rationale logger in `sys/core/src/autonomy/goal_rationale.rs`
- [ ] T634 [P] Implement unified priority queue in `sys/core/src/autonomy/priority_queue.rs`
- [ ] T635 [P] Implement pattern analysis for improvement opportunities in `sys/core/src/autonomy/pattern_analyzer.rs`
- [ ] T635a [P] §3.12 Create unit tests for pattern_analyzer in `sys/core/src/autonomy/pattern_analyzer_test.rs`

### 3-Plane CLI Commands

- [ ] T636 Implement `noa plane status` command in `sys/core/src/cli/plane.rs`
- [ ] T637 [P] Implement `noa plane switch` command (emergency) in `sys/core/src/cli/plane.rs`
- [ ] T638 [P] Implement `noa plane rollback` command in `sys/core/src/cli/plane.rs`
- [ ] T639 [P] Implement `noa promotion status` command in `sys/core/src/cli/promotion.rs`
- [ ] T640 [P] Implement `noa promotion approve` command (manual) in `sys/core/src/cli/promotion.rs`
- [ ] T641 [P] Implement `noa healing status` command in `sys/core/src/cli/healing.rs`

### 3-Plane API Endpoints

- [ ] T642 Implement GET /api/v1/planes endpoint in `sys/core/src/api/routes/planes.rs`
- [ ] T643 [P] Implement GET /api/v1/planes/{name}/health endpoint in `sys/core/src/api/routes/planes.rs`
- [ ] T644 [P] Implement POST /api/v1/planes/switch endpoint (emergency) in `sys/core/src/api/routes/planes.rs`
- [ ] T645 [P] Implement GET /api/v1/promotions endpoint in `sys/core/src/api/routes/promotions.rs`
- [ ] T646 [P] Implement GET /api/v1/promotions/{id}/status endpoint in `sys/core/src/api/routes/promotions.rs`
- [ ] T647 [P] Implement GET /api/v1/healing/status endpoint in `sys/core/src/api/routes/healing.rs`
- [ ] T648 [P] Implement GET /api/v1/healing/events endpoint in `sys/core/src/api/routes/healing.rs`
- [ ] T649 [P] Implement GET /api/v1/goals endpoint in `sys/core/src/api/routes/goals.rs`
- [ ] T650 [P] Implement POST /api/v1/goals endpoint in `sys/core/src/api/routes/goals.rs`
- [ ] T651 [P] Implement GET /api/v1/activity/stream WebSocket in `sys/core/src/api/routes/activity.rs`

**3-Plane Checkpoint**: Architecture complete - Continue to Shared Provider setup ✅

---

## Phase 2.6: Shared Provider Execution Memory (Critical)

**Purpose**: Enable multiple model providers to share memory, context, and execute tasks collaboratively

**⚠️ CRITICAL**: This phase implements FR-037 through FR-042 for multi-provider collaborative execution

**Prerequisite**: Phase 2.5 (3-Plane Control Fabric) must be complete

### Database Schema (Provider Entities)

- [ ] T417 §3.3 Create Provider table in `init/migrations/004_providers.sql`
- [ ] T418 [P] Create SharedExecutionContext table in `init/migrations/004_providers.sql`
- [ ] T419 [P] Create ProviderTask table in `init/migrations/004_providers.sql`
- [ ] T420 [P] Create provider indexes (name, priority, status) in `init/migrations/004_providers.sql`

### Shared Memory Bus

- [ ] T421 §3.3 Create SharedProviderMemory bus architecture in `sys/core/src/providers/shared_memory.rs`
- [ ] T422 [P] Implement context creation and lifecycle in `sys/core/src/providers/context_manager.rs`
- [ ] T423 [P] Implement context synchronization across providers in `sys/core/src/providers/sync.rs`
- [ ] T424 §3.7 Implement shared execution memory persistence in `sys/core/src/providers/persistence.rs`

### Provider Framework

- [ ] T425 §3.3 Define ProviderTrait base interface in `sys/core/src/providers/base.rs`
- [ ] T426 [P] Implement provider registry and discovery in `sys/core/src/providers/registry.rs`
- [ ] T427 [P] Implement provider health checking in `sys/core/src/providers/health.rs`
- [ ] T428 [P] Implement provider priority selection in `sys/core/src/providers/selector.rs`

### Local Providers (llama.cpp - 5+ models)

- [ ] T429 §3.2 Implement llama.cpp multi-model shared context in `sys/core/src/providers/llama/shared.rs`
- [ ] T430 [P] Implement llama.cpp model pool (5+ concurrent) in `sys/core/src/providers/llama/pool.rs`
- [ ] T431 [P] Implement llama.cpp context forwarding in `sys/core/src/providers/llama/context.rs`

### Cloud/CLI Providers

- [ ] T432 §3.3 Implement Claude Code provider (CLI + Cloud + IDE) in `sys/core/src/providers/claude/mod.rs`
- [ ] T433 [P] Implement Claude CLI wrapper in `sys/core/src/providers/claude/cli.rs`
- [ ] T434 [P] Implement Claude Cloud API client in `sys/core/src/providers/claude/cloud.rs`
- [ ] T435 [P] Implement Claude IDE extension bridge in `sys/core/src/providers/claude/ide.rs`

- [ ] T436 §3.3 Implement Codex provider (CLI + Cloud + IDE) in `sys/core/src/providers/codex/mod.rs`
- [ ] T437 [P] Implement Codex CLI wrapper in `sys/core/src/providers/codex/cli.rs`
- [ ] T438 [P] Implement Codex Cloud API client in `sys/core/src/providers/codex/cloud.rs`

- [ ] T439 §3.3 Implement VS Code Copilot IDE provider in `sys/core/src/providers/copilot/mod.rs`
- [ ] T440 [P] Implement Copilot extension communication in `sys/core/src/providers/copilot/extension.rs`

- [ ] T441 §3.3 Implement Git CLI provider in `sys/core/src/providers/git/mod.rs`
- [ ] T442 [P] Implement Git command execution in `sys/core/src/providers/git/commands.rs`

- [ ] T443 §3.3 Implement Cursor provider (IDE + CLI + Cloud) in `sys/core/src/providers/cursor/mod.rs`
- [ ] T444 [P] Implement Cursor CLI wrapper in `sys/core/src/providers/cursor/cli.rs`
- [ ] T445 [P] Implement Cursor Cloud API client in `sys/core/src/providers/cursor/cloud.rs`
- [ ] T446 [P] Implement Cursor IDE extension bridge in `sys/core/src/providers/cursor/ide.rs`
- [ ] T446a §3.3 Implement Cursor provider orchestration mode - coordinate ALL available providers for parallel task execution in `sys/core/src/providers/cursor/orchestrator.rs`
- [ ] T446b [P] Implement task-to-provider routing (reasoning→Claude, code→Codex, local→llama.cpp) in `sys/core/src/providers/cursor/router.rs`

- [ ] T447 §3.3 Implement Abacus provider (CLI + Cloud) in `sys/core/src/providers/abacus/mod.rs`
- [ ] T448 [P] Implement Abacus CLI wrapper in `sys/core/src/providers/abacus/cli.rs`
- [ ] T449 [P] Implement Abacus Cloud API client in `sys/core/src/providers/abacus/cloud.rs`

### Collaborative Reasoning Engine

- [ ] T450 §3.3 Implement collaborative reasoning orchestrator in `sys/core/src/providers/collaborative/orchestrator.rs`
- [ ] T451 [P] Implement parallel task distribution in `sys/core/src/providers/collaborative/distributor.rs`
- [ ] T452 [P] Implement result aggregation and consensus in `sys/core/src/providers/collaborative/aggregator.rs`
- [ ] T453 [P] Implement provider state synchronization in `sys/core/src/providers/collaborative/state_sync.rs`

### Provider Configuration

- [ ] T454 Create provider configuration schema in `config/schemas/providers.yaml`
- [ ] T455 [P] Create default provider configs in `config/providers/`
- [ ] T456 [P] Implement provider feature flags in `config/features.json`

### CLI Commands (Providers)

- [ ] T457 Implement `noa providers list` command in `sys/core/src/cli/providers.rs`
- [ ] T458 [P] Implement `noa providers status` command in `sys/core/src/cli/providers.rs`
- [ ] T459 [P] Implement `noa providers enable/disable` command in `sys/core/src/cli/providers.rs`
- [ ] T460 [P] Implement `noa providers test` command in `sys/core/src/cli/providers.rs`

### API Endpoints (Providers)

- [ ] T461 Implement GET /api/v1/providers endpoint in `sys/core/src/api/routes/providers.rs`
- [ ] T462 [P] Implement POST /api/v1/providers/{id}/enable endpoint in `sys/core/src/api/routes/providers.rs`
- [ ] T463 [P] Implement GET /api/v1/providers/context endpoint in `sys/core/src/api/routes/providers.rs`
- [ ] T464 [P] Implement POST /api/v1/providers/execute endpoint in `sys/core/src/api/routes/providers.rs`

### Spec-Kit Shared Provider Integration (§3.13, FR-037)

**Purpose**: Wire spec-kit into shared provider access so ALL providers access the same spec simultaneously

- [ ] SK001 §3.13 Create `connect_provider()` function in `project-mgmt/spec-kit/src/specify_cli/providers.py`
- [ ] SK002 [P] Implement spec-distribution.json schema validation in `project-mgmt/spec-kit/src/specify_cli/schemas.py`
- [ ] SK003 §3.3 Add provider registration to execution-memory.db in `project-mgmt/spec-kit/src/specify_cli/memory.py`
- [ ] SK004 [P] Implement parallel spec broadcast to all connected providers in `project-mgmt/spec-kit/src/specify_cli/broadcast.py`
- [ ] SK005 [P] Add spec locking mechanism for write coordination in `project-mgmt/spec-kit/src/specify_cli/locks.py`
- [ ] SK006 [P] §3.5 Create audit logging for provider spec access in `project-mgmt/spec-kit/src/specify_cli/audit.py`
- [ ] SK007 [P] Update AGENT_CONFIG in spec-kit to use shared resources in `project-mgmt/spec-kit/src/specify_cli/__init__.py`
- [ ] SK008 [P] Add `--connect-providers` flag to `specify init` command in `project-mgmt/spec-kit/src/specify_cli/__init__.py`
- [ ] SK009 [P] Implement provider health check before spec distribution in `project-mgmt/spec-kit/src/specify_cli/health.py`
- [ ] SK010 §3.3 Create spec-kit MCP tool for provider orchestration in `project-mgmt/spec-kit/src/specify_cli/mcp.py`

**Spec-Kit Provider Integration Acceptance Criteria**:
- [ ] All registered providers can access the same spec simultaneously
- [ ] No duplicate spec copies created (single source of truth via shared memory bus)
- [ ] Providers coordinate via execution-memory.db
- [ ] Audit trail logs all provider spec access

**Phase 2.6 Acceptance Criteria**:
- [ ] Minimum 5 llama.cpp models running concurrently with shared context
- [ ] All 8 provider types integrated (llama.cpp, Claude, Codex, Copilot, Git, Cursor, Abacus)
- [ ] Providers can reason together on complex tasks
- [ ] Shared execution memory persists across sessions
- [ ] Spec-kit connected to shared provider access (SK001-SK010)

**Checkpoint**: Provider infrastructure ready - User story implementation can now begin ✅

---

## Phase 3: User Story 1 - Initialize NOA Seed Environment (P1) 🎯 MVP

**Goal**: Create self-contained environment under `noa_root` with all components

**Independent Test**: Run `noa init` and verify directory structure, database, and binary accessibility

### Directory Creation (US1)

- [ ] T072 [US1] §3.1 Implement directory structure creation in `sys/core/src/init/structure.rs`
- [ ] T073 [P] [US1] §3.1 Define directory constants (sys/, p2p/, opt/, init/, containers/, config/, bin/, ai/, data/) in `sys/core/src/init/paths.rs`
- [ ] T074 [P] [US1] Create sys/core/ subdirectory structure in `sys/core/src/init/structure.rs`
- [ ] T075 [P] [US1] Create sys/services/ subdirectory structure in `sys/core/src/init/structure.rs`
- [ ] T076 [P] [US1] Create sys/ui/ subdirectory structure in `sys/core/src/init/structure.rs`
- [ ] T077 [P] [US1] Create sys/digest/ subdirectory structure in `sys/core/src/init/structure.rs`
- [ ] T078 [P] [US1] Create sys/kernel/ subdirectory structure in `sys/core/src/init/structure.rs`

### Config & Database Init (US1)

- [ ] T079 [US1] §3.2 Implement default config generation in `sys/core/src/init/config.rs`
- [ ] T080 [US1] §3.2 Implement database initialization in `sys/core/src/init/database.rs`
- [ ] T081 [P] [US1] Generate ai-providers.json default config in `sys/core/src/init/config.rs`
- [ ] T082 [P] [US1] Generate noa-server.json default config in `sys/core/src/init/config.rs`
- [ ] T083 [P] [US1] Generate features.json default feature flags in `sys/core/src/init/config.rs`
- [ ] T084 [P] [US1] Generate models.json default model registry in `sys/core/src/init/config.rs`

### Services (US1)

- [ ] T085 [US1] §3.1 Implement InitService with full initialization flow in `sys/core/src/services/init_service.rs`
- [ ] T086 [P] [US1] §3.6 Implement permission checking and setup in `sys/core/src/services/init_service.rs`
- [ ] T087 [P] [US1] §3.1 Implement binary path registration in `sys/core/src/services/init_service.rs`

### Bootstrap Scripts (US1)

- [ ] T088 [US1] Create bootstrap dirs.sh script in `init/bootstrap/dirs.sh`
- [ ] T089 [P] [US1] Create bootstrap deps.sh script in `init/bootstrap/deps.sh`
- [ ] T090 [P] [US1] Create bootstrap models.sh script in `init/bootstrap/models.sh`
- [ ] T091 [P] [US1] Create Windows init script noa-init.ps1 in `init/noa-init.ps1`

### CLI Commands (US1)

- [ ] T092 [US1] Enhance `noa init` with --root and --force flags in `sys/core/src/cli/init.rs`
- [ ] T093 [P] [US1] Implement initialization progress display in `sys/core/src/cli/init.rs`
- [ ] T094 [P] [US1] Implement verification output in `sys/core/src/cli/init.rs`

### API Endpoints (US1)

- [ ] T095 [P] [US1] Implement GET /api/v1/system/info endpoint in `sys/core/src/api/routes/system.rs`
- [ ] T096 [P] [US1] Implement GET /api/v1/system/health with database check in `sys/core/src/api/routes/system.rs`

**US1 Acceptance Criteria**:
- [ ] Directory structure created with correct permissions
- [ ] Local database operational
- [ ] Works fully offline

---

## Phase 4: User Story 2 - Multi-SLM Neural Runtime (P1) 🎯 MVP

**Goal**: Run multiple SLMs locally with <2s response on standard hardware

**Independent Test**: Load model, send prompt, verify response within latency target

### Rust ML Stack Setup (US2)

- [ ] T097 [US2] §3.2 Add burn-rs/burn to Cargo.toml for training in `sys/core/Cargo.toml`
- [ ] T098 [P] [US2] Add candle-core and candle-transformers for inference in `sys/core/Cargo.toml`
- [ ] T099 [P] [US2] Add tokenizers crate for HuggingFace tokenization in `sys/core/Cargo.toml`
- [ ] T100 [P] [US2] Add qdrant-client for vector store in `sys/core/Cargo.toml`
- [ ] T101 [P] [US2] Configure tch backend for CUDA 13.1+ in `sys/core/Cargo.toml`
- [ ] T102 [P] [US2] Configure wgpu backend for cross-platform GPU in `sys/core/Cargo.toml`

### Models & Data Layer (US2)

- [ ] T103 [US2] §3.2 Implement Model entity repository in `sys/core/src/db/repositories/model_repository.rs`
- [ ] T104 [P] [US2] Define model configuration schema in `config/ai-providers.json`
- [ ] T105 [P] [US2] Create GGUF model loader interface in `sys/core/src/neural/model_loader.rs`

### Neural Runtime Core (US2)

- [ ] T106 [US2] §3.2 Integrate llama-cpp-rs bindings in `sys/core/src/neural/llama_backend.rs`
- [ ] T107 [US2] Implement model loading with GPU layer auto-detection in `sys/core/src/neural/model_loader.rs`
- [ ] T108 [P] [US2] Implement context management for inference in `sys/core/src/neural/context.rs`
- [ ] T109 [US2] Implement inference engine with streaming in `sys/core/src/neural/inference.rs`
- [ ] T110 [P] [US2] Implement quantization detection and layer offloading in `sys/core/src/neural/hardware.rs`

### Multi-GPU Support (US2) - FR-047 to FR-050

- [ ] T478 §3.2 [US2] Implement CUDA device enumeration in `sys/core/src/neural/cuda_devices.rs`
- [ ] T479 [P] [US2] Implement multi-GPU layer distribution in `sys/core/src/neural/multi_gpu.rs`
- [ ] T480 [P] [US2] Implement tensor parallelism for model sharding in `sys/core/src/neural/tensor_parallel.rs`
- [ ] T481 [P] [US2] Implement NVLink detection and utilization in `sys/core/src/neural/nvlink.rs`
- [ ] T482 [P] [US2] Add GPU memory pooling across devices in `sys/core/src/neural/gpu_pool.rs`
- [ ] T483 §3.2 [US2] Implement CUDA 13.1+ tiles configuration in `sys/core/src/neural/cuda_tiles.rs`
- [ ] T484 [P] [US2] Add multi-GPU load balancing in `sys/core/src/neural/gpu_scheduler.rs`
- [ ] T485 [P] [US2] Implement GPU health monitoring in `sys/core/src/neural/gpu_health.rs`

### Advanced Learning Techniques (US2) - FR-043 to FR-046 (SHOULD)

**Note**: These are SHOULD requirements - implement after MVP if resources allow.

- [ ] T657 [US2] §3.4 Design ToolkenGPT integration architecture in `docs/architecture/toolkengpt.md`
- [ ] T658 [P] [US2] Implement ToolkenGPT token registry in `sys/core/src/learning/toolkengpt/registry.rs`
- [ ] T659 [P] [US2] Implement tool token embedding pre-training in `sys/core/src/learning/toolkengpt/pretrain.rs`
- [ ] T660 [P] [US2] Implement tool token plugin loader in `sys/core/src/learning/toolkengpt/plugin.rs`
- [ ] T661 [US2] §3.7 Design Replay Memory Cache architecture in `docs/architecture/replay_memory.md`
- [ ] T662 [P] [US2] Implement short-term memory buffer in `sys/core/src/learning/replay/buffer.rs`
- [ ] T663 [P] [US2] Implement external knowledge base connector in `sys/core/src/learning/replay/knowledge_base.rs`
- [ ] T664 [P] [US2] Implement experience replay sampling in `sys/core/src/learning/replay/sampler.rs`
- [ ] T665 [US2] §3.4 Design EWC (Elastic Weight Consolidation) module in `docs/architecture/ewc.md`
- [ ] T666 [P] [US2] Implement Fisher Information computation in `sys/core/src/learning/ewc/fisher.rs`
- [ ] T667 [P] [US2] Implement importance-weighted parameter consolidation in `sys/core/src/learning/ewc/consolidate.rs`
- [ ] T668 [P] [US2] Implement dynamic architecture adapter modules in `sys/core/src/learning/ewc/adapters.rs`
- [ ] T669 [US2] §3.4 Design MAML (Model-Agnostic Meta-Learning) module in `docs/architecture/maml.md`
- [ ] T670 [P] [US2] Implement inner-loop task adaptation in `sys/core/src/learning/maml/inner_loop.rs`
- [ ] T671 [P] [US2] Implement outer-loop meta-optimization in `sys/core/src/learning/maml/outer_loop.rs`
- [ ] T672 [P] [US2] Implement few-shot learning interface in `sys/core/src/learning/maml/few_shot.rs`

### Candle Inference Layer (US2)

- [ ] T111 [US2] Implement Candle embedding service in `sys/core/crates/embedder/src/lib.rs`
- [ ] T112 [P] [US2] Support models: Llama, Mistral, Qwen, BGE/E5 in `sys/core/crates/embedder/src/models.rs`
- [ ] T113 [P] [US2] Implement safetensors/GGUF export in `sys/core/src/neural/export.rs`

### Model Selection (US2)

- [ ] T114 [US2] §3.3 Implement ModelSelectorAgent base logic in `sys/core/src/agents/model_selector.rs`
- [ ] T115 [P] [US2] Define model selection criteria (task type, resources) in `sys/core/src/agents/model_selector.rs`
- [ ] T116 [P] [US2] Implement model benchmarking utility in `sys/core/src/neural/benchmark.rs`

### 13 Specialized ModelSelectorAgents (US2)

- [ ] T465 [P] [US2] Implement ModelSelectorAgent_Audit in `ai/agents/model_selectors/audit.ts`
- [ ] T466 [P] [US2] Implement ModelSelectorAgent_DataStack in `ai/agents/model_selectors/data_stack.ts`
- [ ] T467 [P] [US2] Implement ModelSelectorAgent_DevOps in `ai/agents/model_selectors/devops.ts`
- [ ] T468 [P] [US2] Implement ModelSelectorAgent_Ethics in `ai/agents/model_selectors/ethics.ts`
- [ ] T469 [P] [US2] Implement ModelSelectorAgent_Finance in `ai/agents/model_selectors/finance.ts`
- [ ] T470 [P] [US2] Implement ModelSelectorAgent_HR in `ai/agents/model_selectors/hr.ts`
- [ ] T471 [P] [US2] Implement ModelSelectorAgent_LegalCompliance in `ai/agents/model_selectors/legal.ts`
- [ ] T472 [P] [US2] Implement ModelSelectorAgent_Marketing in `ai/agents/model_selectors/marketing.ts`
- [ ] T473 [P] [US2] Implement ModelSelectorAgent_Operations in `ai/agents/model_selectors/operations.ts`
- [ ] T474 [P] [US2] Implement ModelSelectorAgent_Security in `ai/agents/model_selectors/security.ts`
- [ ] T475 [P] [US2] Implement ModelSelectorAgent_Strategy in `ai/agents/model_selectors/strategy.ts`
- [ ] T476 [P] [US2] Implement ModelSelectorAgent_Technology in `ai/agents/model_selectors/technology.ts`
- [ ] T477 [P] [US2] Implement ModelSelectorAgent_Vision in `ai/agents/model_selectors/vision.ts`

### Services (US2)

- [ ] T118 [US2] Implement NeuralService with model management in `sys/core/src/services/neural_service.rs`
- [ ] T119 [P] [US2] Implement model download with progress in `sys/core/src/services/model_download.rs`

### CLI Commands (US2)

- [ ] T120 [US2] Implement `noa models list` command in `sys/core/src/cli/models.rs`
- [ ] T121 [P] [US2] Implement `noa models download` command in `sys/core/src/cli/models.rs`
- [ ] T122 [P] [US2] Implement `noa models verify` command in `sys/core/src/cli/models.rs`
- [ ] T123 [US2] Implement `noa ask` command for inference in `sys/core/src/cli/ask.rs`
- [ ] T124 [P] [US2] Implement `noa models benchmark` command in `sys/core/src/cli/models.rs`

### API Endpoints (US2)

- [ ] T125 [P] [US2] Implement GET /api/v1/models endpoint in `sys/core/src/api/routes/models.rs`
- [ ] T126 [P] [US2] Implement POST /api/v1/models/download endpoint in `sys/core/src/api/routes/models.rs`
- [ ] T127 [US2] Implement POST /api/v1/inference endpoint in `sys/core/src/api/routes/inference.rs`
- [ ] T128 [P] [US2] Implement POST /api/v1/inference/stream SSE endpoint in `sys/core/src/api/routes/inference.rs`
- [ ] T129 [P] [US2] Implement POST /api/v1/models/benchmark endpoint in `sys/core/src/api/routes/models.rs`
- [ ] T130 [P] [US2] Implement POST /api/v1/models/ingest endpoint in `sys/core/src/api/routes/models.rs`
- [ ] T527 [P] [US2] Implement POST /api/v1/models/{id}/load endpoint in `sys/core/src/api/routes/models.rs`
- [ ] T528 [P] [US2] Implement POST /api/v1/models/{id}/unload endpoint in `sys/core/src/api/routes/models.rs`
- [ ] T529 [P] [US2] Implement GET /api/v1/models/{id}/status endpoint in `sys/core/src/api/routes/models.rs`

**US2 Acceptance Criteria**:
- [ ] Response within 2 seconds on standard hardware
- [ ] ModelSelectorAgent routes tasks to optimal model
- [ ] Dynamic quantization based on available resources

---

## Phase 5: User Story 3 - Total Memory Sovereignty (P1) 🎯 MVP

**Goal**: Remember everything with instant recall (<500ms)

**Independent Test**: Create memory, close system, reopen, verify instant recall

### Models & Data Layer (US3)

- [ ] T131 [US3] §3.7 Implement Memory entity repository in `sys/core/src/db/repositories/memory_repository.rs`
- [ ] T132 [P] [US3] §3.7 Implement Embedding entity repository in `sys/core/src/db/repositories/embedding_repository.rs`
- [ ] T133 [US3] §3.7 Integrate sqlite-vss for vector search in `sys/core/src/db/vector_search.rs`

### Embedding Pipeline (US3)

- [ ] T134 [US3] §3.7 Implement embedding generation with Candle in `sys/core/src/memory/embeddings.rs`
- [ ] T135 [P] [US3] Create embedding model loader (MiniLM-384 or BGE/E5) in `sys/core/src/memory/embedding_model.rs`
- [ ] T136 [US3] §3.7 Implement semantic search with HNSW index in `sys/core/src/memory/semantic_search.rs`
- [ ] T137 [P] [US3] Implement embedding cache (model_version + input_hash + params_hash) in `sys/core/src/memory/cache.rs`
- [ ] T138 [P] [US3] Implement batch embedding requests in `sys/core/src/memory/embeddings.rs`

### Vector Store Integration (US3)

- [ ] T139 [US3] §3.7 Implement Qdrant client wrapper in `sys/core/src/vector/qdrant_client.rs`
- [ ] T140 [P] [US3] Implement vector upsert with metadata in `sys/core/src/vector/qdrant_client.rs`
- [ ] T141 [P] [US3] Implement vector search with filters in `sys/core/src/vector/qdrant_client.rs`

### Services (US3)

- [ ] T142 [US3] §3.7 Implement MemoryService with CRUD operations in `sys/core/src/services/memory_service.rs`
- [ ] T143 [P] [US3] Implement memory checksum validation in `sys/core/src/services/memory_service.rs`
- [ ] T144 [US3] §3.7 Implement SearchService with semantic + keyword in `sys/core/src/services/search_service.rs`

### CLI Commands (US3)

- [ ] T145 [US3] Implement `noa memory create` command in `sys/core/src/cli/memory.rs`
- [ ] T146 [P] [US3] Implement `noa memory search` command in `sys/core/src/cli/memory.rs`
- [ ] T147 [P] [US3] Implement `noa memory list` command in `sys/core/src/cli/memory.rs`
- [ ] T148 [P] [US3] Implement `noa memory get` command in `sys/core/src/cli/memory.rs`

### API Endpoints (US3)

- [ ] T149 [US3] §3.7 Implement POST /api/v1/memories endpoint in `sys/core/src/api/routes/memories.rs`
- [ ] T150 [P] [US3] Implement GET /api/v1/memories/{id} endpoint in `sys/core/src/api/routes/memories.rs`
- [ ] T151 [P] [US3] Implement GET /api/v1/memories endpoint with pagination in `sys/core/src/api/routes/memories.rs`
- [ ] T152 [US3] §3.7 Implement POST /api/v1/memories/search endpoint in `sys/core/src/api/routes/memories.rs`

**US3 Acceptance Criteria**:
- [ ] Instant recall of past conversations
- [ ] Search results in <500ms
- [ ] Memory syncs across P2P devices (when US6 complete)

---

## Phase 6: User Story 4 - Digest Everything Pipeline (P2)

**Goal**: Ingest and understand any codebase or data source

**Independent Test**: Point digest at GitHub repo, verify output artifacts

### Backend Pipeline Services (US4)

- [ ] T153 [US4] Implement IntakeService (Stage 1) in `sys/core/src/services/digest/intake.rs`
- [ ] T154 [P] [US4] Implement ClassifierService (Stage 2) - languages, licenses in `sys/core/src/services/digest/classifier.rs`
- [ ] T155 [P] [US4] Implement GraphExtractService (Stage 3) - kg.json, system_card in `sys/core/src/services/digest/graph_extract.rs`
- [ ] T156 [P] [US4] Implement EmbeddingsService (Stage 4) - pgvector/Qdrant in `sys/core/src/services/digest/embeddings.rs`
- [ ] T157 [P] [US4] Implement EnvSynthesisService (Stage 5) - Dockerfiles, compose, K8s in `sys/core/src/services/digest/env_synthesis.rs`
- [ ] T158 [P] [US4] Implement SafetyService (Stage 6) - SBOM, Grype, Gitleaks in `sys/core/src/services/digest/safety.rs`
- [ ] T159 [P] [US4] Implement RunnerService (Stage 7) - build, test, demo in `sys/core/src/services/digest/runner.rs`
- [ ] T160 [P] [US4] Implement IntegratorService (Stage 8) - SDKs, telemetry in `sys/core/src/services/digest/integrator.rs`
- [ ] T161 [P] [US4] Implement RegistrarService (Stage 9) - storage, registry in `sys/core/src/services/digest/registrar.rs`

### Models & Data Layer (US4)

- [ ] T162 [US4] Implement DigestSource repository in `sys/core/src/db/repositories/digest_repository.rs`
- [ ] T163 [P] [US4] Implement KnowledgeNode repository in `sys/core/src/db/repositories/knowledge_node_repository.rs`
- [ ] T164 [P] [US4] Implement KnowledgeEdge repository in `sys/core/src/db/repositories/knowledge_edge_repository.rs`

### Digest Pipeline (Python) (US4)

- [ ] T165 [US4] §3.4 Implement Discover stage in `sys/digest/src/stages/discover.py`
- [ ] T166 [P] [US4] Implement Fetch stage with git clone in `sys/digest/src/stages/fetch.py`
- [ ] T167 [US4] Implement Parse stage with tree-sitter in `sys/digest/src/stages/parse.py`
- [ ] T168 [P] [US4] §3.4 Implement Analyze stage for embeddings in `sys/digest/src/stages/analyze.py`
- [ ] T169 [P] [US4] Implement Summarize stage in `sys/digest/src/stages/summarize.py`
- [ ] T170 [US4] Implement Surface stage for output generation in `sys/digest/src/stages/surface.py`
- [ ] T171 [US4] §3.6 Implement Secure stage with Gitleaks/Trivy in `sys/digest/src/stages/secure.py`

### Multi-Language Parsing (US4)

- [ ] T172 [P] [US4] Implement Python AST parser in `sys/digest/src/parsers/python_parser.py`
- [ ] T173 [P] [US4] Implement TypeScript parser with ts-morph in `sys/digest/src/parsers/typescript_parser.py`
- [ ] T174 [P] [US4] Implement Rust parser with syn in `sys/digest/src/parsers/rust_parser.py`
- [ ] T175 [P] [US4] Implement Go parser with go/ast in `sys/digest/src/parsers/go_parser.py`
- [ ] T176 [P] [US4] Implement Java parser in `sys/digest/src/parsers/java_parser.py`

### Output Generation (US4)

- [ ] T177 [US4] Generate profile.json output in `sys/digest/src/output/profile.py`
- [ ] T178 [P] [US4] Generate system_card.md output in `sys/digest/src/output/system_card.py`
- [ ] T179 [P] [US4] Generate kg.json knowledge graph in `sys/digest/src/output/knowledge_graph.py`
- [ ] T180 [P] [US4] §3.6 Generate SBOM with Syft in `sys/digest/src/output/sbom.py`

### Event Bus (US4)

- [ ] T181 [US4] Implement Redis Streams event bus in `sys/core/src/events/redis_streams.rs`
- [ ] T182 [P] [US4] Implement workflow DAG engine in `sys/core/src/events/workflow_engine.rs`
- [ ] T183 [P] [US4] Implement digest job queue in `sys/core/src/services/digest_queue.rs`

### Services (US4)

- [ ] T184 [US4] Implement DigestService orchestrator in `sys/core/src/services/digest_service.rs`

### CLI Commands (US4)

- [ ] T185 [US4] Implement `noa digest <url>` command in `sys/core/src/cli/digest.rs`
- [ ] T186 [P] [US4] Implement `noa digest status` command in `sys/core/src/cli/digest.rs`
- [ ] T187 [P] [US4] Implement `noa digest security-report` command in `sys/core/src/cli/digest.rs`
- [ ] T188 [P] [US4] Implement `noa knowledge search` command in `sys/core/src/cli/knowledge.rs`

### API Endpoints (US4)

- [ ] T189 [US4] Implement POST /api/v1/digest endpoint per OpenAPI in `sys/core/src/api/routes/digest.rs`
- [ ] T190 [P] [US4] Implement GET /api/v1/digest/{jobId} endpoint in `sys/core/src/api/routes/digest.rs`
- [ ] T191 [P] [US4] Implement GET /api/v1/digest/{jobId}/artifacts endpoint in `sys/core/src/api/routes/digest.rs`

### Digest Source Endpoints (US4) - per digest-pipeline.openapi.yaml

- [ ] T509 [US4] Implement GET /api/v1/digest/sources endpoint in `sys/core/src/api/routes/digest_sources.rs`
- [ ] T510 [P] [US4] Implement POST /api/v1/digest/sources endpoint in `sys/core/src/api/routes/digest_sources.rs`
- [ ] T511 [P] [US4] Implement GET /api/v1/digest/sources/{id} endpoint in `sys/core/src/api/routes/digest_sources.rs`
- [ ] T512 [P] [US4] Implement GET /api/v1/digest/sources/{id}/profile endpoint in `sys/core/src/api/routes/digest_sources.rs`
- [ ] T513 [P] [US4] Implement GET /api/v1/digest/sources/{id}/system-card endpoint in `sys/core/src/api/routes/digest_sources.rs`
- [ ] T536 [P] [US4] Implement GET /api/v1/digest/sources/{id}/sbom endpoint in `sys/core/src/api/routes/digest_sources.rs`
- [ ] T537 [P] [US4] Implement GET /api/v1/digest/sources/{id}/security endpoint in `sys/core/src/api/routes/digest_sources.rs`

### Knowledge Graph Endpoints (US4) - per digest-pipeline.openapi.yaml

- [ ] T538 [US4] Implement GET /api/v1/knowledge/nodes endpoint in `sys/core/src/api/routes/knowledge.rs`
- [ ] T539 [P] [US4] Implement GET /api/v1/knowledge/nodes/{id} endpoint in `sys/core/src/api/routes/knowledge.rs`
- [ ] T540 [P] [US4] Implement GET /api/v1/knowledge/edges endpoint in `sys/core/src/api/routes/knowledge.rs`
- [ ] T541 [P] [US4] Implement POST /api/v1/knowledge/query endpoint in `sys/core/src/api/routes/knowledge.rs`

**US4 Acceptance Criteria**:
- [ ] Produces profile.json, system_card.md, kg.json, SBOM, security report
- [ ] Multi-language parsing (Python, TypeScript, Rust, Go, Java)
- [ ] Vulnerabilities flagged with severity

---

## Phase 7: User Story 5 - Dynamic Context-Aware UI (P2)

**Goal**: Fluid, agent-driven interface that adapts to context

**Independent Test**: Switch tasks and verify UI reconfigures appropriately

### UI Foundation (US5)

- [ ] T192 [US5] Initialize Next.js app with TypeScript in `sys/ui/`
- [ ] T193 [P] [US5] Setup Tailwind CSS and shadcn/ui components in `sys/ui/`
- [ ] T194 [P] [US5] Implement API client for backend in `sys/ui/src/lib/api.ts`
- [ ] T195 [US5] Implement WebSocket connection for real-time updates in `sys/ui/src/lib/websocket.ts`

### Core Components (US5)

- [ ] T196 [US5] Implement main layout with sidebar in `sys/ui/src/components/layout/MainLayout.tsx`
- [ ] T197 [P] [US5] Implement navigation component in `sys/ui/src/components/layout/Navigation.tsx`
- [ ] T198 [P] [US5] §3.5 Implement activity log component in `sys/ui/src/components/ActivityLog.tsx`
- [ ] T199 [US5] Implement context detection service in `sys/ui/src/services/contextDetector.ts`

### Admin Console Features (US5)

- [ ] T200 [US5] Implement Jobs Dashboard in `sys/ui/src/pages/admin/Jobs.tsx`
- [ ] T201 [P] [US5] Implement Capsules View in `sys/ui/src/pages/admin/Capsules.tsx`
- [ ] T202 [P] [US5] Implement Artifacts Explorer in `sys/ui/src/pages/admin/Artifacts.tsx`
- [ ] T203 [P] [US5] Implement SBOM & Security view in `sys/ui/src/pages/admin/Security.tsx`
- [ ] T204 [P] [US5] Implement Model Registry view in `sys/ui/src/pages/admin/Models.tsx`
- [ ] T205 [P] [US5] Implement CRM Controls view in `sys/ui/src/pages/admin/CRM.tsx`

### Settings Dashboard (US5)

- [ ] T206 [US5] Implement Settings Dashboard architecture in `sys/ui/src/pages/Settings.tsx`
- [ ] T207 [P] [US5] Implement AI/LLM Settings panel in `sys/ui/src/components/settings/AISettings.tsx`
- [ ] T208 [P] [US5] Implement Providers Settings panel in `sys/ui/src/components/settings/ProvidersSettings.tsx`
- [ ] T209 [P] [US5] Implement IDE/Editor Settings panel in `sys/ui/src/components/settings/IDESettings.tsx`
- [ ] T210 [P] [US5] Implement CLI Settings panel in `sys/ui/src/components/settings/CLISettings.tsx`
- [ ] T211 [P] [US5] Implement Sync Settings panel in `sys/ui/src/components/settings/SyncSettings.tsx`
- [ ] T212 [P] [US5] Implement Memory Settings panel in `sys/ui/src/components/settings/MemorySettings.tsx`
- [ ] T213 [P] [US5] Implement Security Settings panel in `sys/ui/src/components/settings/SecuritySettings.tsx`
- [ ] T214 [P] [US5] Implement Privacy Settings panel in `sys/ui/src/components/settings/PrivacySettings.tsx`
- [ ] T215 [P] [US5] Implement Theme Settings panel in `sys/ui/src/components/settings/ThemeSettings.tsx`

### Cross-Platform Settings Sync (US5)

- [ ] T216 [US5] §3.8 Implement settings sync service in `sys/ui/src/services/settingsSync.ts`
- [ ] T217 [P] [US5] Implement sync scope (global/per-device/per-project) in `sys/ui/src/services/settingsSync.ts`
- [ ] T218 [P] [US5] Implement conflict resolution strategy in `sys/ui/src/services/settingsSync.ts`

### Dynamic UI Features (US5)

- [ ] T219 [US5] Implement widget registry in `sys/ui/src/components/widgets/WidgetRegistry.ts`
- [ ] T220 [P] [US5] Implement drag-and-drop widget layout in `sys/ui/src/components/widgets/WidgetGrid.tsx`
- [ ] T221 [P] [US5] Implement widget persistence in `sys/ui/src/services/widgetPersistence.ts`
- [ ] T222 [P] [US5] Implement contextual UI auto-adapt in `sys/ui/src/services/contextualUI.ts`
- [ ] T223 [P] [US5] Implement user presets save/switch in `sys/ui/src/services/presets.ts`
- [ ] T224 [P] [US5] Implement AI-assisted insights in `sys/ui/src/services/insights.ts`

### Model-Agnostic Chat Interface (US5)

- [ ] T225 [US5] Implement chat interface component in `sys/ui/src/components/Chat.tsx`
- [ ] T226 [P] [US5] Implement streaming response display in `sys/ui/src/components/ChatMessage.tsx`
- [ ] T227 [P] [US5] Implement markdown rendering in `sys/ui/src/components/MarkdownRenderer.tsx`
- [ ] T228 [US5] Implement provider abstraction for model switching in `sys/ui/src/services/providerClient.ts`
- [ ] T229 [P] [US5] Implement context persistence across devices in `sys/ui/src/services/contextPersistence.ts`

### Multi-Modal Interaction (US5) - FR-023 (deferred: voice/vision where hardware permits)

- [ ] T760 [US5] §3.3 Implement multi-modal input abstraction in `sys/ui/src/services/multiModal.ts`
- [ ] T761 [P] [US5] Implement voice input stub (Web Speech API) in `sys/ui/src/components/VoiceInput.tsx`
- [ ] T762 [P] [US5] Implement vision input stub (camera/screen capture) in `sys/ui/src/components/VisionInput.tsx`
- [ ] T763 [P] [US5] Create hardware capability detection for multi-modal in `sys/ui/src/services/hardwareCapabilities.ts`
- [ ] T764 [P] [US5] Implement graceful degradation when hardware unavailable in `sys/ui/src/services/multiModal.ts`

**US5 Acceptance Criteria**:
- [ ] UI surfaces relevant tools based on context
- [ ] Live scrollable activity log
- [ ] Real-time updates via WebSocket
- [ ] Settings sync across IDE, CLI, web

---

## Phase 8: User Story 6 - P2P Hive-Mind Device Federation (P2)

**Goal**: User-owned P2P cloud sharing compute, memory, storage

**Independent Test**: Connect two devices, verify discovery and resource sharing

### P2P Core (Go) (US6)

- [ ] T230 [US6] §3.8 Initialize libp2p host in `p2p/pkg/node/host.go`
- [ ] T231 [P] [US6] §3.8 Implement mDNS discovery in `p2p/pkg/discovery/mdns.go`
- [ ] T232 [P] [US6] Implement DHT discovery in `p2p/pkg/discovery/dht.go`
- [ ] T233 [P] [US6] §3.6 Implement TLS encryption for connections in `p2p/pkg/security/tls.go`
- [ ] T234 [US6] §3.8 Implement peer connection management in `p2p/pkg/node/peers.go`

### Device Management (US6)

- [ ] T235 [US6] §3.8 Implement Device repository in `sys/core/src/db/repositories/device_repository.rs`
- [ ] T236 [P] [US6] Implement device registration in `sys/core/src/services/device_service.rs`
- [ ] T237 [US6] §3.8 Implement resource capability detection in `p2p/pkg/resources/capabilities.go`

### P2P Discovery Service (US6) - per p2p-protocol.proto

- [ ] T501 [US6] §3.8 Implement Discovery.Announce RPC in `p2p/pkg/discovery/announce.go`
- [ ] T502 [P] [US6] Implement Discovery.FindPeers RPC in `p2p/pkg/discovery/find_peers.go`
- [ ] T503 [P] [US6] Implement Discovery.Ping RPC in `p2p/pkg/discovery/ping.go`
- [ ] T504 [P] [US6] Generate Go code from p2p-protocol.proto in `p2p/pkg/protocol/`

### Sync Protocol (US6) - per p2p-protocol.proto

- [ ] T238 [US6] §3.8 Implement SyncState repository in `sys/core/src/db/repositories/sync_repository.rs`
- [ ] T239 [US6] §3.8 Implement vector clock sync in `p2p/pkg/sync/vector_clock.go`
- [ ] T240 [P] [US6] Implement CRDT data types in `p2p/pkg/sync/crdt.go`
- [ ] T241 [P] [US6] Implement delta sync protocol in `p2p/pkg/sync/delta.go`
- [ ] T242 [P] [US6] Implement conflict resolution in `p2p/pkg/sync/conflict.go`
- [ ] T243 [US6] §3.7 Implement memory sync protocol in `p2p/pkg/sync/memory_sync.go`
- [ ] T505 [US6] Implement Sync.GetState RPC in `p2p/pkg/sync/get_state.go`
- [ ] T506 [P] [US6] Implement Sync.PushChanges RPC in `p2p/pkg/sync/push_changes.go`
- [ ] T507 [P] [US6] Implement Sync.PullChanges RPC in `p2p/pkg/sync/pull_changes.go`
- [ ] T508 [P] [US6] Implement Sync.ResolveConflict RPC in `p2p/pkg/sync/resolve_conflict.go`

### Distributed Storage (US6)

- [ ] T244 [US6] §3.8 Implement CAS (Content-Addressable Storage) in `p2p/pkg/storage/cas.go`
- [ ] T245 [P] [US6] Implement replication protocol in `p2p/pkg/storage/replication.go`

### P2P Storage Service (US6) - per p2p-protocol.proto

- [ ] T652 [US6] §3.8 Implement Storage.Store RPC in `p2p/pkg/storage/store.go`
- [ ] T653 [P] [US6] Implement Storage.Retrieve RPC in `p2p/pkg/storage/retrieve.go`
- [ ] T654 [P] [US6] Implement Storage.Exists RPC in `p2p/pkg/storage/exists.go`
- [ ] T655 [P] [US6] Implement Storage.List RPC in `p2p/pkg/storage/list.go`
- [ ] T656 [P] [US6] Implement Storage.Replicate RPC in `p2p/pkg/storage/replicate.go`

### Task Distribution (US6)

- [ ] T246 [US6] §3.3 Implement task offloading protocol in `p2p/pkg/tasks/offload.go`
- [ ] T247 [P] [US6] Implement resource-based routing in `p2p/pkg/tasks/router.go`
- [ ] T248 [P] [US6] Implement distributed compute scheduler in `p2p/pkg/compute/scheduler.go`
- [ ] T249 [P] [US6] Implement worker node in `p2p/pkg/compute/worker.go`

### P2P Compute Service (US6) - per p2p-protocol.proto

- [ ] T514 [US6] §3.3 Implement Compute.SubmitTask RPC in `p2p/pkg/compute/submit_task.go`
- [ ] T515 [P] [US6] Implement Compute.GetTaskStatus RPC in `p2p/pkg/compute/get_task_status.go`
- [ ] T516 [P] [US6] Implement Compute.CancelTask RPC in `p2p/pkg/compute/cancel_task.go`
- [ ] T517 [P] [US6] Implement Compute.StreamOutput RPC (streaming) in `p2p/pkg/compute/stream_output.go`

### CLI Commands (US6)

- [ ] T250 [US6] Implement `noa p2p info` command in `sys/core/src/cli/p2p.rs`
- [ ] T251 [P] [US6] Implement `noa p2p connect` command in `sys/core/src/cli/p2p.rs`
- [ ] T252 [P] [US6] Implement `noa p2p status` command in `sys/core/src/cli/p2p.rs`
- [ ] T253 [P] [US6] Implement `noa p2p ping` command in `sys/core/src/cli/p2p.rs`
- [ ] T254 [P] [US6] Implement `noa p2p reset` command in `sys/core/src/cli/p2p.rs`

### API Endpoints (US6)

- [ ] T255 [US6] Implement GET /api/v1/p2p/info endpoint in `sys/core/src/api/routes/p2p.rs`
- [ ] T256 [P] [US6] Implement GET /api/v1/p2p/peers endpoint in `sys/core/src/api/routes/p2p.rs`
- [ ] T257 [P] [US6] Implement POST /api/v1/p2p/connect endpoint in `sys/core/src/api/routes/p2p.rs`

**US6 Acceptance Criteria**:
- [ ] Devices discover each other on same network
- [ ] Tasks distributed based on available resources
- [ ] Graceful degradation when device goes offline

---

## Phase 9: User Story 7 - Autonomous Agent Orchestration (P2)

**Goal**: Coordinate specialized agents for complex multi-step problems

**Independent Test**: Submit complex goal, observe decomposition and agent collaboration

### Agent Framework (US7)

- [ ] T258 [US7] §3.3 Implement Agent repository in `sys/core/src/db/repositories/agent_repository.rs`
- [ ] T259 [P] [US7] §3.5 Implement AgentLog repository in `sys/core/src/db/repositories/agentlog_repository.rs`
- [ ] T260 [US7] §3.3 Implement Task repository in `sys/core/src/db/repositories/task_repository.rs`
- [ ] T261 [P] [US7] Implement TaskEvent repository in `sys/core/src/db/repositories/task_event_repository.rs`

### Core Agents (US7)

- [ ] T262 [US7] §3.3 Implement BaseAgent trait in `sys/core/src/agents/base.rs`
- [ ] T263 [P] [US7] Implement FileIOAgent in `sys/core/src/agents/file_io.rs`
- [ ] T264 [P] [US7] Implement TerminalAgent in `sys/core/src/agents/terminal.rs`
- [ ] T265 [P] [US7] Implement RAGAgent in `sys/core/src/agents/rag.rs`
- [ ] T266 [P] [US7] Implement MicroserviceManagementAgent in `sys/core/src/agents/microservice_mgmt.rs`

### Executive Board Agents (US7)

- [ ] T267 [US7] §3.3 Implement ExecutiveCommanderChiefAgent in `sys/core/src/agents/executive/commander.rs`
- [ ] T268 [P] [US7] Implement EA_HR Agent in `sys/core/src/agents/executive/hr.rs`
- [ ] T269 [P] [US7] Implement EA_Finance Agent in `sys/core/src/agents/executive/finance.rs`
- [ ] T270 [P] [US7] Implement EA_Audit Agent in `sys/core/src/agents/executive/audit.rs`
- [ ] T271 [P] [US7] Implement EA_Ethics Agent in `sys/core/src/agents/executive/ethics.rs`
- [ ] T272 [P] [US7] Implement EA_Operations Agent in `sys/core/src/agents/executive/operations.rs`
- [ ] T273 [P] [US7] Implement EA_Security Agent in `sys/core/src/agents/executive/security.rs`
- [ ] T274 [P] [US7] Implement EA_Technology Agent in `sys/core/src/agents/executive/technology.rs`

### MicroAgentStack (US7)

- [ ] T275 [US7] §3.3 Implement MicroAgentStack repository in `sys/core/src/db/repositories/stack_repository.rs`
- [ ] T276 [US7] §3.3 Implement 5-stage lifecycle (bootstrap→execute→validate→package→archive) in `sys/core/src/orchestration/stack_lifecycle.rs`
- [ ] T277 [P] [US7] Implement CommanderChiefAgent in `sys/core/src/agents/commander.rs`
- [ ] T278 [P] [US7] Implement stack naming (mas_* reusable, gen_mas disposable) in `sys/core/src/orchestration/stack_naming.rs`
- [ ] T279 [P] [US7] Implement stack workspace (in/work/out/logs) in `sys/core/src/orchestration/stack_workspace.rs`

### Orchestration (US7)

- [ ] T280 [US7] §3.3 Implement goal decomposition in `sys/core/src/orchestration/decomposer.rs`
- [ ] T281 [P] [US7] Implement task scheduler with priority queue in `sys/core/src/orchestration/scheduler.rs`
- [ ] T282 [US7] §3.3 Implement orchestration engine (CECCA) in `sys/core/src/orchestration/engine.rs`
- [ ] T283 [P] [US7] Implement retry and escalation logic in `sys/core/src/orchestration/retry.rs`

### Capsule Architecture (US7)

- [ ] T284 [US7] Implement Capsule sidecars (build-proxy, service-mirror, policy-agent, telemetry) in `containers/capsules/sidecars/`
- [ ] T285 [P] [US7] Create noa-core.yaml capsule spec in `containers/capsules/noa-core.yaml`
- [ ] T286 [P] [US7] Create noa-digest.yaml capsule spec in `containers/capsules/noa-digest.yaml`

### CRM Strangler Pattern (US7)

- [ ] T287 [US7] Implement CRM Strangler Proxy service in `sys/core/src/services/crm_strangler.rs`
- [ ] T288 [P] [US7] Implement shadow mode in `sys/core/src/services/crm_strangler.rs`
- [ ] T289 [P] [US7] Implement write-through mode in `sys/core/src/services/crm_strangler.rs`
- [ ] T290 [P] [US7] Implement instant rollback in `sys/core/src/services/crm_strangler.rs`

### Services (US7)

- [ ] T291 [US7] §3.3 Implement AgentService in `sys/core/src/services/agent_service.rs`
- [ ] T292 [P] [US7] Implement TaskService in `sys/core/src/services/task_service.rs`
- [ ] T293 [US7] §3.3 Implement OrchestrationService in `sys/core/src/services/orchestration_service.rs`

### CLI Commands (US7)

- [ ] T294 [US7] Implement `noa agents list` command in `sys/core/src/cli/agents.rs`
- [ ] T295 [P] [US7] Implement `noa tasks list` command in `sys/core/src/cli/tasks.rs`
- [ ] T296 [P] [US7] Implement `noa goal submit` command in `sys/core/src/cli/goal.rs`
- [ ] T297 [P] [US7] §3.5 Implement `noa logs` command in `sys/core/src/cli/logs.rs`
- [ ] T298 [P] [US7] Implement `noa capsule spawn` command in `sys/core/src/cli/capsule.rs`
- [ ] T299 [P] [US7] Implement `noa crm toggle` command in `sys/core/src/cli/crm.rs`

### API Endpoints (US7)

- [ ] T300 [US7] Implement GET /api/v1/agents endpoint in `sys/core/src/api/routes/agents.rs`
- [ ] T520 [P] [US7] Implement GET /api/v1/agents/{id} endpoint in `sys/core/src/api/routes/agents.rs`
- [ ] T521 [P] [US7] Implement POST /api/v1/agents/{id}/start endpoint in `sys/core/src/api/routes/agents.rs`
- [ ] T522 [P] [US7] Implement POST /api/v1/agents/{id}/stop endpoint in `sys/core/src/api/routes/agents.rs`
- [ ] T523 [P] [US7] Implement GET /api/v1/agents/{id}/logs endpoint in `sys/core/src/api/routes/agents.rs`
- [ ] T301 [P] [US7] Implement GET /api/v1/tasks endpoint in `sys/core/src/api/routes/tasks.rs`
- [ ] T524 [P] [US7] Implement POST /api/v1/tasks endpoint in `sys/core/src/api/routes/tasks.rs`
- [ ] T525 [P] [US7] Implement GET /api/v1/tasks/{id} endpoint in `sys/core/src/api/routes/tasks.rs`
- [ ] T526 [P] [US7] Implement PATCH /api/v1/tasks/{id} endpoint in `sys/core/src/api/routes/tasks.rs`
- [ ] T302 [US7] Implement POST /api/v1/goals endpoint in `sys/core/src/api/routes/goals.rs`
- [ ] T303 [P] [US7] Implement GET /api/v1/orchestration/status endpoint in `sys/core/src/api/routes/orchestration.rs`
- [ ] T304 [P] [US7] Implement POST /api/v1/capsule/spawn endpoint in `sys/core/src/api/routes/capsule.rs`
- [ ] T305 [P] [US7] Implement POST /api/v1/crm/toggle endpoint in `sys/core/src/api/routes/crm.rs`
- [ ] T306 [P] [US7] Implement POST /api/v1/crm/rollback endpoint in `sys/core/src/api/routes/crm.rs`

### Board Agents (US7) - FR-142

- [ ] T306a [US7] §3.3 Implement LegalAgent in `sys/core/src/agents/board/legal.rs`
  - Contract review, IP protection, compliance checking
  - Governance decision escalation
- [ ] T306b [P] [US7] §3.3 Implement QAAgent in `sys/core/src/agents/board/qa.rs`
  - Code quality validation, test coverage monitoring
  - Release gate approval
- [ ] T306c [P] [US7] §3.3 Implement ArchitectureAgent in `sys/core/src/agents/board/architecture.rs`
  - Design review, dependency analysis
  - Technical debt assessment

**US7 Acceptance Criteria**:
- [ ] Complex goals decomposed into tasks
- [ ] 98% task completion rate at 200 concurrent
- [ ] Proper escalation on failure
- [ ] Board agents operational for governance decisions

---

## Phase 10: User Story 8 - Self-Improvement & Code Modification (P3)

**Goal**: Continuously improve by analyzing performance and modifying own code

**Independent Test**: Trigger self-analysis, verify improvement proposals with rollback capability

### Dynamic Graphs Framework (US8)

- [ ] T307 [US8] §3.4 Implement Dynamic Graph base framework in `sys/core/src/graphs/base.rs`
- [ ] T308 [P] [US8] Implement EFG (Environment Function Graph) in `sys/core/src/graphs/efg.rs`
- [ ] T309 [P] [US8] Implement DSG (Dynamic Software Graph) in `sys/core/src/graphs/dsg.rs`
- [ ] T310 [P] [US8] Implement DHG (Dynamic Hardware Graph) in `sys/core/src/graphs/dhg.rs`
- [ ] T311 [P] [US8] Implement DPG (Dynamic Process Graph) in `sys/core/src/graphs/dpg.rs`
- [ ] T312 [P] [US8] Implement DRG (Dynamic Resource Graph) in `sys/core/src/graphs/drg.rs`
- [ ] T313 [P] [US8] Implement DSeCG (Dynamic Security Graph) in `sys/core/src/graphs/dsecg.rs`
- [ ] T314 [P] [US8] Implement DPeG (Dynamic Performance Graph) in `sys/core/src/graphs/dpeg.rs`
- [ ] T315 [P] [US8] Implement DEG (Dynamic Error Graph) in `sys/core/src/graphs/deg.rs`
- [ ] T316 [P] [US8] Implement DKG (Dynamic Knowledge Graph) in `sys/core/src/graphs/dkg.rs`

### Specialized CECCA Cells (US8)

- [ ] T317 [US8] §3.4 Implement CC_CONST (Constitution/Signer) in `sys/core/src/cecca/cells/constitution.rs`
- [ ] T318 [P] [US8] Implement CC_QUORUM (Virtual Board) in `sys/core/src/cecca/cells/quorum.rs`
- [ ] T319 [P] [US8] Implement CC_TRUTH (Truth-Gate) in `sys/core/src/cecca/cells/truth_gate.rs`
- [ ] T320 [US8] §3.4 Implement CC_STEM_REPL (Replicator) in `sys/core/src/cecca/cells/stem_repl.rs`
- [ ] T321 [P] [US8] Implement CC_STEM_DIFF (Differentiator) in `sys/core/src/cecca/cells/stem_diff.rs`
- [ ] T322 [US8] §3.4 Implement CC_CHOP (Capsule Surgeon) in `sys/core/src/cecca/cells/chop.rs`
- [ ] T323 [P] [US8] Implement CC_ARBITER (Promotion Arbiter) in `sys/core/src/cecca/cells/arbiter.rs`
- [ ] T324 [P] [US8] Implement CC_AUDIT (Global Auditor) in `sys/core/src/cecca/cells/audit.rs`
- [ ] T325 [P] [US8] Implement CC_LICENSE (License Gate) in `sys/core/src/cecca/cells/license.rs`

### Knowledge Capsules (KPLANE) (US8)

- [ ] T326 [US8] Implement KIDX_CAP (CAS Index & Blob Store) in `sys/core/src/knowledge/kidx.rs`
- [ ] T327 [P] [US8] Implement KSCHEMA_CAP (Schema Registry) in `sys/core/src/knowledge/kschema.rs`
- [ ] T328 [P] [US8] Implement KMETRICS_CAP (Metrics & Events) in `sys/core/src/knowledge/kmetrics.rs`
- [ ] T329 [P] [US8] Implement KDIR_CAP (Canonical Directory) in `sys/core/src/knowledge/kdir.rs`
- [ ] T330 [P] [US8] Implement KSNAP_CAP (Snapshot & Restore) in `sys/core/src/knowledge/ksnap.rs`
- [ ] T331 [P] [US8] Implement KCRASH_CAP (Crash Forensics) in `sys/core/src/knowledge/kcrash.rs`

### Self-Analysis (US8)

> **NOTE**: Advanced Learning tasks (ToolkenGPT, Replay Memory, EWC, MAML) are in Phase 4 (US2) as T657-T672 per FR-043-046.

- [ ] T338 [US8] §3.4 Implement performance metrics collection in `sys/core/src/self_improve/metrics.rs`
- [ ] T339 [P] [US8] Implement efficiency analysis in `sys/core/src/self_improve/analyzer.rs`
- [ ] T340 [US8] §3.4 Implement improvement proposal generation in `sys/core/src/self_improve/proposals.rs`

### Safety & Rollback (US8)

- [ ] T341 [US8] §3.12 Implement pre-modification snapshot in `sys/core/src/self_improve/snapshot.rs`
- [ ] T342 [P] [US8] §3.12 Implement test runner for modifications in `sys/core/src/self_improve/test_runner.rs`
- [ ] T343 [US8] §3.12 Implement automatic rollback on test failure in `sys/core/src/self_improve/rollback.rs`

### Evidence-Based Execution Policy (US8)

- [ ] T344 [US8] §3.12 Implement Order of Truth Sources in `sys/core/src/policy/truth_sources.rs`
- [ ] T345 [P] [US8] Implement Hard Stop Rule in `sys/core/src/policy/hard_stop.rs`
- [ ] T346 [P] [US8] Implement Triple-Verify Rule in `sys/core/src/policy/triple_verify.rs`
- [ ] T347 [P] [US8] Implement Gap Hunt Rule in `sys/core/src/policy/gap_hunt.rs`

### AMPK Mode (Resource Scarcity) (US8)

- [ ] T348 [US8] §3.4 Implement AMPK mode detection in `sys/core/src/autonomy/ampk.rs`
- [ ] T349 [P] [US8] Implement throttle and quiesce logic in `sys/core/src/autonomy/ampk.rs`
- [ ] T350 [P] [US8] Implement prioritize and resume logic in `sys/core/src/autonomy/ampk.rs`

### Autonomy Loop (US8)

- [ ] T351 [US8] §3.4 Implement SENSE→DECIDE→UPDATE loop in `sys/core/src/autonomy/autonomy_loop.rs`
- [ ] T352 [P] [US8] Implement hourly self-reinvention scheduler in `sys/core/src/autonomy/scheduler.rs`

### Human Co-Improvement (US8)

- [ ] T353 [US8] §3.4 Implement improvement approval workflow in `sys/core/src/self_improve/approval.rs`
- [ ] T354 [P] [US8] Implement improvement audit log in `sys/core/src/self_improve/audit.rs`

### VHDX Integration (US8)

- [ ] T355 [US8] Implement VHDX stack packaging in `sys/core/src/vhdx/packaging.rs`
- [ ] T356 [P] [US8] Implement nested VHDX support in `sys/core/src/vhdx/nested.rs`
- [ ] T357 [P] [US8] Implement VHDX snapshot/rollback in `sys/core/src/vhdx/snapshot.rs`

### CLI Commands (US8)

- [ ] T358 [US8] Implement `noa improve analyze` command in `sys/core/src/cli/improve.rs`
- [ ] T359 [P] [US8] Implement `noa improve propose` command in `sys/core/src/cli/improve.rs`
- [ ] T360 [P] [US8] Implement `noa improve apply` command in `sys/core/src/cli/improve.rs`
- [ ] T361 [P] [US8] Implement `noa improve rollback` command in `sys/core/src/cli/improve.rs`

### Predictive Problem Solving (US8) - §3.11 Constitution Principle

- [ ] T765 [US8] §3.11 Implement pattern recognition engine in `sys/core/src/predict/patterns.rs`
- [ ] T766 [P] [US8] §3.11 Implement failure pattern analysis (own + external) in `sys/core/src/predict/failure_analysis.rs`
- [ ] T767 [P] [US8] §3.11 Implement input/output value evaluator (max fruit/output) in `sys/core/src/predict/value_evaluator.rs`
- [ ] T768 [P] [US8] §3.11 Implement resource allocation predictor in `sys/core/src/predict/resource_predictor.rs`
- [ ] T769 [P] [US8] §3.11 Implement task prioritization predictor in `sys/core/src/predict/priority_predictor.rs`
- [ ] T770 [P] [US8] §3.11 Implement risk assessment engine in `sys/core/src/predict/risk_assessment.rs`

**US8 Acceptance Criteria**:
- [ ] 25+ Dynamic Graphs operational
- [ ] CECCA cells functional
- [ ] Improvement proposals include before/after comparison
- [ ] Automatic rollback on test failure
- [ ] All changes logged with rationale

---

## Phase 11: Project Management Integration

**Purpose**: BMAD, PRP, Spec-Kit, DSPy, Ruler integrations

### BMAD Integration

- [ ] T362 Implement BMAD workflow orchestrator in `ai/agents/bmad/orchestrator.ts`
- [ ] T363 [P] Implement BMAD analyst agent in `ai/agents/bmad/analyst.ts`
- [ ] T364 [P] Implement BMAD architect agent in `ai/agents/bmad/architect.ts`
- [ ] T365 [P] Implement BMAD po (product owner) agent in `ai/agents/bmad/po.ts`
- [ ] T366 [P] Implement BMAD dev agent in `ai/agents/bmad/dev.ts`
- [ ] T367 [P] Create BMAD document templates in `ai/agents/bmad/templates/`

### PRP Integration

- [ ] T368 Implement PRP workflow engine in `ai/agents/prp/workflow.ts`
- [ ] T369 [P] Implement PRP signal system (30+ signals) in `ai/agents/prp/signals.ts`
- [ ] T370 [P] Implement PRP LOOP MODE execution in `ai/agents/prp/loop_mode.ts`
- [ ] T371 [P] Implement robo-system-analyst agent in `ai/agents/prp/system_analyst.ts`
- [ ] T372 [P] Implement robo-aqa agent in `ai/agents/prp/aqa.ts`

### Spec-Kit Integration

- [ ] T373 Implement Spec-Kit provider auto-detection in `ai/agents/speckit/provider_detect.ts`
- [ ] T374 [P] Implement Spec-Kit command generation in `ai/agents/speckit/commands.ts`
- [ ] T375 [P] Create Spec-Kit CLI wrapper in `sys/core/src/cli/speckit.rs`

### DSPy Training Integration

- [ ] T376 Implement DSPy training pipeline in `sys/digest/src/training/dspy_trainer.py`
- [ ] T377 [P] Implement DSPy modules (Predict, ChainOfThought, ReAct) in `sys/digest/src/training/modules.py`
- [ ] T378 [P] Implement DSPy optimizers (MIPROv2, COPRO) in `sys/digest/src/training/optimizers.py`
- [ ] T379 [P] Implement PRD-based training data loader in `sys/digest/src/training/prd_loader.py`

### Ruler Integration

- [ ] T380 Create unified .ruler/AGENTS.md for all providers in `ruler/AGENTS.md`
- [ ] T381 [P] Implement TDD enforcement rules in `ruler/rules/tdd.md`
- [ ] T382 [P] Implement code quality rules in `ruler/rules/quality.md`
- [ ] T383 [P] Implement CI integration rules in `ruler/rules/ci.md`
- [ ] T384 [P] Implement AI commit attribution in `ruler/rules/attribution.md`

---

## Phase 12: Polish & Cross-Cutting Concerns

**Purpose**: Final integration, documentation, and quality assurance

### External Connectors (Feature-Flagged)

> **NOTE**: Connector tasks moved to Phase 14 (US10) with full OAuth support and feature flags.
> See T739-T759 for complete connector implementation.

### Secondary Layer Adapters

- [ ] T390 Implement adapter toggle framework in `sys/core/src/adapters/toggle.rs`
- [ ] T391 [P] Implement AD_DKR (Docker) adapter in `sys/core/src/adapters/docker.rs`
- [ ] T392 [P] Implement AD_GH (GitHub) adapter in `sys/core/src/adapters/github.rs`
- [ ] T393 [P] Implement AD_MINIO (MinIO) adapter in `sys/core/src/adapters/minio.rs`

### Kernel Independence Layer

- [ ] T394 Implement KernelAbstraction trait in `sys/core/src/kernel/abstraction.rs`
- [ ] T395 [P] Implement file operations (cross-platform) in `sys/core/src/kernel/file.rs`
- [ ] T396 [P] Implement process operations in `sys/core/src/kernel/process.rs`
- [ ] T397 [P] Implement network operations in `sys/core/src/kernel/network.rs`
- [ ] T398 [P] Implement platform detection in `sys/core/src/kernel/platform.rs`

### System Prompts & Grammars

- [ ] T399 Create noa-system.md system prompt in `ai/shared/prompts/noa-system.md`
- [ ] T400 [P] Create agent-coordinator.md prompt in `ai/shared/prompts/agent-coordinator.md`
- [ ] T401 [P] Create digest-analyst.md prompt in `ai/shared/prompts/digest-analyst.md`
- [ ] T402 [P] Create json.gbnf grammar in `ai/grammars/json.gbnf`
- [ ] T403 [P] Create structured-output.gbnf grammar in `ai/grammars/structured-output.gbnf`

### Documentation

- [ ] T404 [P] Update README.md with final quickstart in `noa_root/README.md`
- [ ] T405 [P] Generate API documentation from OpenAPI in `docs/api/`
- [ ] T406 [P] Create architecture documentation in `docs/architecture.md`
- [ ] T407 [P] Create contributing guide in `CONTRIBUTING.md`
- [ ] T408 [P] Create setup guides (windows.md, macos.md, linux.md) in `docs/setup/`

### Integration Testing

- [ ] T409 §3.12 Create end-to-end test suite for US1-US3 (MVP) in `tests/e2e/`
- [ ] T410 [P] §3.12 Create integration tests for US4-US7 in `tests/integration/`
- [ ] T411 [P] §3.12 Create performance benchmarks in `tests/benchmarks/`

### Universal Task Execution Policy Artifacts (§9)

Per [universal_task_execution_policy.md](../../project-mgmt/docs/05-policy/universal_task_execution_policy.md):

- [ ] T486 Create HASHES.txt generation script in `scripts/bash/generate-hashes.sh`
- [ ] T487 [P] Create SHA-256 hash generation for all key artifacts in `noa_root/test-results/HASHES.txt`
- [ ] T488 [P] Create FINAL_REPORT.md template in `noa_root/test-results/FINAL_REPORT.md`
- [ ] T489 [P] Create COVERAGE.md template (requirements → artifacts → tests map) in `noa_root/test-results/COVERAGE.md`
- [ ] T490 [P] Create REPRO.md template (exact environment, commands) in `noa_root/test-results/REPRO.md`
- [ ] T491 [P] Create EVIDENCE_LEDGER.md template in `noa_root/test-results/EVIDENCE_LEDGER.md`
- [ ] T492 Implement Triple-Verification Protocol (Pass A/B/C) automation in `scripts/bash/triple-verify.sh`
- [ ] T493 [P] Create Truth Gate checklist automation in `scripts/bash/truth-gate.sh`
- [ ] T494 [P] Implement Gap Hunt scan automation in `scripts/bash/gap-scan.sh`
- [ ] T495 [P] Create Claims Table generator in `scripts/bash/claims-table.sh`

### Rich Metadata & Schema Verification

- [ ] T496 Implement metadata validator (id, created_at, updated_at, checksum) in `sys/core/src/validation/metadata.rs`
- [ ] T497 [P] Implement embedding validator (model, vector, source_type, source_id) in `sys/core/src/validation/embedding.rs`
- [ ] T498 [P] Implement config schema validation against `config/schemas/` in `sys/core/src/validation/config.rs`
- [ ] T499 [P] Implement index verification for all database tables in `sys/core/src/validation/indexes.rs`
- [ ] T500 [P] Implement checksum generation and verification in `sys/core/src/validation/checksum.rs`

### Open Questions Resolution (from research.md)

- [ ] T530 Implement model download progress UI in `sys/ui/src/components/models/DownloadProgress.tsx`
- [ ] T531 [P] Implement P2P first-time device pairing wizard in `sys/ui/src/components/p2p/PairingWizard.tsx`
- [ ] T532 [P] Implement memory retention policy configuration in `config/memory-retention.json`
- [ ] T533 [P] Implement memory auto-prune service in `sys/core/src/services/memory_retention.rs`
- [ ] T534 [P] Create agent marketplace stub API in `sys/core/src/api/routes/marketplace.rs`
- [ ] T535 [P] Create agent marketplace stub UI in `sys/ui/src/components/marketplace/`

### Build & Release

- [ ] T412 Create cross-platform build scripts in `scripts/bash/release.sh`
- [ ] T413 [P] Create installer scripts (PowerShell, Bash) in `scripts/install/`
- [ ] T414 [P] Setup release automation in `.github/workflows/release.yml`

### Configuration Validation

- [ ] T415 §3.1 Implement config validation CLI command in `sys/core/src/cli/config.rs`
- [ ] T416 [P] Create default configuration templates in `config/templates/`

---

## Phase 13: US9 - Cross-Platform Deployment (P3)

**Purpose**: Enable NOA to run on Windows, macOS, Linux, mobile, tablets, and XR devices

**User Story**: As a user, I want NOA to run on all my devices so I can use it everywhere.

**Acceptance Criteria**:
- Core functionality works identically across Windows 11, macOS, Ubuntu
- Mobile companion app participates in P2P hive-mind
- Resource usage adapts to available hardware

### Cross-Platform Build Infrastructure (US9)

- [ ] T722 [US9] Create platform detection module in `sys/core/src/platform/detect.rs`
- [ ] T723 [P] [US9] Implement Windows-specific adaptations in `sys/core/src/platform/windows.rs`
- [ ] T724 [P] [US9] Implement macOS-specific adaptations in `sys/core/src/platform/macos.rs`
- [ ] T725 [P] [US9] Implement Linux-specific adaptations in `sys/core/src/platform/linux.rs`
- [ ] T726 [P] [US9] Create cross-platform path resolver in `sys/core/src/platform/paths.rs`

### CI/CD Matrix (US9)

- [ ] T727 [US9] Create cross-platform CI matrix in `.github/workflows/cross-platform.yml`
- [ ] T728 [P] [US9] Add Windows build job (x64) in `.github/workflows/cross-platform.yml`
- [ ] T729 [P] [US9] Add macOS build jobs (x64, arm64) in `.github/workflows/cross-platform.yml`
- [ ] T730 [P] [US9] Add Ubuntu build job (x64) in `.github/workflows/cross-platform.yml`
- [ ] T731 [P] [US9] Create platform-specific artifact packaging in `scripts/package/`

### Mobile Companion (US9)

- [ ] T732 [US9] Create mobile companion project structure in `sys/mobile/` (stub: connects to P2P only)
- [ ] T733 [P] [US9] Implement P2P client for mobile in `sys/mobile/src/p2p_client.rs`
- [ ] T734 [P] [US9] Create minimal mobile UI (companion mode) in `sys/mobile/src/ui/`
- [ ] T735 [US9] Implement hardware capability detection in `sys/core/src/platform/capabilities.rs`

### Resource Adaptation (US9)

- [ ] T736 [US9] Implement dynamic resource allocation based on hardware tier in `sys/core/src/resources/allocator.rs`
- [ ] T737 [P] [US9] Implement model size selection per hardware in `sys/core/src/resources/model_selector.rs`
- [ ] T738 [P] [US9] Implement graceful degradation on low-resource systems in `sys/core/src/resources/degradation.rs`

**US9 Acceptance Criteria**:
- [ ] CI passes on Windows, macOS (x64+arm64), Ubuntu
- [ ] Mobile companion connects to desktop NOA
- [ ] Resource usage scales with hardware tier

---

## Phase 14: US10 - Connectors & External Integration (P3)

**Purpose**: Enable NOA to connect to external services (Gmail, GitHub, cloud storage)

**User Story**: As a user, I want NOA to integrate with my existing accounts and services.

**Acceptance Criteria**:
- OAuth connectors authenticate within 5 seconds
- Connectors degrade gracefully when network unavailable
- All connectors feature-flagged and optional

### Feature Flag Framework (US10)

- [ ] T739 [US10] Implement feature flag service in `sys/core/src/features/flags.rs`
- [ ] T740 [P] [US10] Create feature flag configuration in `config/features.json`
- [ ] T741 [P] [US10] Implement feature flag CLI commands in `sys/core/src/cli/features.rs`

### OAuth Framework (US10)

- [ ] T742 [US10] Implement OAuth2 base client in `sys/core/src/connectors/oauth/client.rs`
- [ ] T743 [P] [US10] Implement OAuth callback handler in `sys/core/src/connectors/oauth/callback.rs`
- [ ] T744 [P] [US10] Implement token exchange flow in `sys/core/src/connectors/oauth/token_exchange.rs`
- [ ] T745 [P] [US10] Implement token refresh mechanism in `sys/core/src/connectors/oauth/refresh.rs`
- [ ] T746 [P] [US10] Implement secure token storage in `sys/core/src/connectors/oauth/storage.rs`

### Connector Implementations (US10)

- [ ] T747 [US10] Implement connector base trait in `sys/core/src/connectors/base.rs`
- [ ] T748 [P] [US10] Implement GitHub connector in `sys/core/src/connectors/github.rs`
- [ ] T749 [P] [US10] Implement Gmail/Google connector in `sys/core/src/connectors/google.rs`
- [ ] T750 [P] [US10] Implement OpenAI connector in `sys/core/src/connectors/openai.rs`
- [ ] T751 [P] [US10] Implement Claude connector in `sys/core/src/connectors/claude.rs`
- [ ] T752 [P] [US10] Implement cloud storage connector (S3/GCS) in `sys/core/src/connectors/cloud_storage.rs`
- [ ] T753 [P] [US10] Implement Email (SMTP/IMAP) connector in `sys/core/src/connectors/email.rs`

### Connector Graceful Degradation (US10)

- [ ] T754 [US10] Implement offline cache for connectors in `sys/core/src/connectors/cache.rs`
- [ ] T755 [P] [US10] Implement network availability detection in `sys/core/src/connectors/network.rs`
- [ ] T756 [P] [US10] Implement connector status reporting in `sys/core/src/connectors/status.rs`

### Connector UI (US10)

- [ ] T757 [US10] Create connector settings page in `sys/ui/src/pages/settings/connectors.tsx`
- [ ] T758 [P] [US10] Create OAuth authorization flow UI in `sys/ui/src/components/connectors/OAuthFlow.tsx`
- [ ] T759 [P] [US10] Create connector status dashboard in `sys/ui/src/components/connectors/StatusDashboard.tsx`

**US10 Acceptance Criteria**:
- [ ] OAuth authentication completes within 5 seconds
- [ ] Connectors work with cached data when offline
- [ ] All connectors can be disabled via feature flags

---

## Phase 15: Governance & Safety (FR-025 to FR-028)

**Purpose**: Implement constitutional governance, biblical truth pipeline, and safety mechanisms

### Constitutional Governance (FR-025)

- [ ] T690 §3.10 Implement constitutional governance engine in `sys/core/src/governance/engine.rs`
- [ ] T691 [P] §3.5 Implement governance audit trail in `sys/core/src/governance/audit.rs`
- [ ] T692 [P] Create governance decision schema in `config/schemas/governance.json`

### Biblical Governance Pipeline (FR-026) - §3.9 §3.10

- [ ] T693 §3.10 Create biblical source text storage in `data/governance/biblical/sources/`
- [ ] T694 [P] §3.10 Implement Greek/Hebrew text ingestion in `sys/core/src/governance/biblical/ingest.rs`
- [ ] T695 [P] §3.10 Implement lexical analysis module in `sys/core/src/governance/biblical/lexical.rs`
- [ ] T696 §3.10 Implement semantic embedding pipeline in `sys/core/src/governance/biblical/embedding.rs`
- [ ] T697 [P] §3.10 Implement biblical knowledge graph integration in `sys/core/src/governance/biblical/knowledge_graph.rs`
- [ ] T698 [P] §3.10 Create biblical governance prompts in `ai/shared/prompts/biblical-governance.md`
- [ ] T699 §3.10 Implement ethical decision boundary from biblical principles in `sys/core/src/governance/biblical/ethics.rs`

### Reward/Correction Mechanisms (FR-027)

- [ ] T700 §3.4 Implement agent compliance reward system in `sys/core/src/governance/rewards.rs`
- [ ] T701 [P] §3.12 Implement drift detection testing loop in `sys/core/src/governance/drift_detection.rs`
- [ ] T702 [P] §3.4 Implement compliance correction mechanism in `sys/core/src/governance/correction.rs`

### Self-Modification Rollback (FR-028)

- [ ] T703 §3.4 Implement modification snapshot service in `sys/core/src/governance/snapshot.rs`
- [ ] T704 [P] §3.4 Implement rollback path validator in `sys/core/src/governance/rollback_validator.rs`
- [ ] T705 [P] §3.4 Implement modification reversal executor in `sys/core/src/governance/rollback_executor.rs`

**Governance Acceptance Criteria**:
- [ ] All agent decisions logged with rationale
- [ ] Biblical text transformation pipeline operational
- [ ] 100% of self-modifications have valid rollback path

---

## Phase 16: Success Criteria Verification (SC-001 to SC-012) - §3.12

**Purpose**: Implement benchmarks and verification for all Success Criteria

### Performance Benchmarks (SC-001, SC-002, SC-011, SC-012)

- [ ] T706 §3.12 SC-001 Implement system initialization benchmark (<60s on standard hardware) in `tests/benchmarks/sc001_init.rs`
- [ ] T707 [P] §3.12 SC-002 Implement CPU inference benchmark (<2s on CPU-only) in `tests/benchmarks/sc002_cpu_inference.rs`
- [ ] T708 [P] §3.12 SC-011 Implement single GPU inference benchmark (<500ms) in `tests/benchmarks/sc011_gpu_inference.rs`
- [ ] T709 [P] §3.12 SC-012 Implement multi-GPU inference benchmark (<300ms with tensor parallelism) in `tests/benchmarks/sc012_multi_gpu.rs`

### Memory & Database Benchmarks (SC-003, SC-004)

- [ ] T710 §3.12 SC-003 Implement memory recall benchmark (<500ms for any interaction) in `tests/benchmarks/sc003_memory_recall.rs`
- [ ] T711 [P] §3.12 SC-004 Implement digest pipeline benchmark (<30min for 10K files) in `tests/benchmarks/sc004_digest.rs`

### Concurrency & Scalability Benchmarks (SC-005, SC-006)

- [ ] T712 §3.12 SC-005 Implement concurrent agent task benchmark (200 tasks, ≥98% success) in `tests/benchmarks/sc005_concurrent_tasks.rs`
- [ ] T713 [P] §3.12 SC-006 Implement P2P sync benchmark (<5s for <1MB delta) in `tests/benchmarks/sc006_p2p_sync.rs`

### UI & Responsiveness Benchmarks (SC-007)

- [ ] T714 §3.12 SC-007 Implement UI context switch benchmark (<200ms) in `tests/benchmarks/sc007_ui_switch.rs`

### Stability & Reliability Benchmarks (SC-008, SC-010)

- [ ] T715 §3.12 SC-008 Implement 7-day continuous operation test in `tests/stability/sc008_long_running.rs`
- [ ] T716 [P] §3.12 SC-010 Implement rollback path validation (100% coverage) in `tests/stability/sc010_rollback.rs`

### Cross-Platform Verification (SC-009)

- [ ] T717 §3.12 SC-009 Implement cross-platform consistency test suite in `tests/cross_platform/sc009_consistency.rs`
- [ ] T718 [P] §3.12 SC-009 Create platform comparison report generator in `scripts/bash/platform-report.sh`

### SC Verification CI Integration

- [ ] T719 §3.12 Create SC verification workflow in `.github/workflows/sc-verification.yml`
- [ ] T720 [P] §3.12 Create SC dashboard in `sys/ui/src/pages/admin/sc-dashboard.tsx`
- [ ] T721 [P] §3.12 Create SC report generator in `scripts/bash/sc-report.sh`

**SC Verification Acceptance Criteria**:
- [ ] All 12 SCs have automated benchmarks
- [ ] CI runs SC verification on release
- [ ] SC dashboard shows current compliance status

---

## Phase 18: Kernel Independence & Self-Containment (FR-159 to FR-165) - §3.1

**Purpose**: Implement explicit kernel selection precedence, tool isolation, and NKAL trust boundary per /clarify 2025-12-09

### Phase 18.1: Bootstrap Integration - Kernel Selection (T827-T834)

**Goal**: Implement "NOA kernels first" policy with explicit precedence and tool isolation

- [ ] T827 [P] §3.1 Add `config/kernel-selection-policy.json` schema with precedence rules (VM > Container > Sandbox > Native)
- [ ] T828 §3.1 Implement kernel selection logic in `scripts/noa-kernel-params.ps1` based on FR-160
- [ ] T829 [P] §3.1 Implement kernel selection logic in `scripts/noa-kernel-params` (bash) matching T828
- [ ] T830 §3.1 Add `--allow-global` flag to all tool detection scripts in `scripts/setup/check-prereqs.ps1`
- [ ] T831 [P] §3.1 Create `config/bootstrap-tools.json` with version pinning schema (FR-163)
- [ ] T832 §3.1 Implement `-UpdateExisting` flag in `scripts/setup/install-all-tools.ps1`
- [ ] T833 §3.1 Implement tool archival to `noa_root/opt/archive/` before upgrade
- [ ] T834 §3.1 Add upgrade rollback via `scripts/setup/install-all-tools.ps1 -Rollback -Tool <name>`

### Phase 18.2: Foundation - NKAL Trust Boundary (T835-T842)

**Goal**: Implement NKAL as trust boundary with capability grants and state persistence

- [ ] T835 §3.1 Create NKAL capability grant schema `config/nkal-capabilities.json`
- [ ] T836 §3.1 Implement NKAL trust boundary validation in `sys/core/src/nkal/boundary.rs`
- [ ] T837 [P] §3.1 Add input sanitization layer at NKAL boundary in `sys/core/src/nkal/sanitize.rs`
- [ ] T838 [P] §3.1 Add output verification layer at NKAL boundary in `sys/core/src/nkal/verify.rs`
- [ ] T839 §3.1 Create `.kernel-switch-state.json` checkpoint on mode change in `sys/core/src/nkal/checkpoint.rs`
- [ ] T840 §3.1 Implement state verification after kernel mode switch in `sys/core/src/nkal/state.rs`
- [ ] T841 [P] §3.1 Add shared volume mount configuration for VM/container modes in `config/kernel-mounts.json`
- [ ] T842 §3.1 Implement graceful shutdown requirement before mode switch in `sys/core/src/nkal/shutdown.rs`

### Phase 18.3: Documentation & Verification (T843-T845)

**Goal**: Document kernel independence architecture and add status output

- [ ] T843 §3.1 Document kernel selection precedence in `docs/architecture/kernel-independence.md`
- [ ] T844 [P] §3.1 Document external vs internal dependency boundary in `docs/architecture/self-containment.md`
- [ ] T845 §3.1 Add kernel mode and tool version to `noa status` output in `scripts/noa.ps1`

### Phase 18.4: Checklist Gap Resolution (T846-T858)

**Goal**: Address remaining high-priority checklist gaps for kernel independence

- [ ] T846 §3.1 Document kernel initialization startup sequence in `docs/architecture/kernel-startup.md` (CHK005)
- [ ] T847 [P] §3.1 Add "NOA kernel" vs "host kernel" terminology definitions to spec.md Glossary (CHK006)
- [ ] T848 §3.1 Document performance trade-offs between kernel modes in `docs/architecture/kernel-performance.md` (CHK007)
- [ ] T849 [P] §3.1 Document consistency verification between kernel-independence.md and §3.1 (CHK010)
- [ ] T850 §3.1 Verify FR-091 to FR-094 alignment with kernel-independence.md strategy (CHK011)
- [ ] T851 [P] §3.1 Ensure kernel modes consistently named across all documentation (CHK012)
- [ ] T852 §3.1 Document all required tools with internal installation paths (CHK013)
- [ ] T853 [P] §3.1 Document permitted host OS interactions explicitly (CHK020)
- [ ] T854 §3.1 Define measurable criteria for "self-contained" (CHK021)
- [ ] T855 [P] §3.1 Verify consistency between spec.md and CONSTITUTION.md definitions (CHK022)
- [ ] T856 §3.1 Document NKAL interface contracts in kernel-independence.md (CHK031)
- [ ] T857 [P] §3.1 Define acceptable performance overhead for each kernel mode (CHK032)
- [ ] T858 §3.1 Document specific host kernel features that CAN be leveraged (CHK033)

**Kernel Independence Acceptance Criteria**:
- [ ] Kernel selection policy enforced on startup
- [ ] NKAL validates all boundary crossings
- [ ] State persistence verified across mode switches
- [ ] All checklist gaps CHK005-CHK033 addressed

---

## Dependency Graph

```
Phase 1 (Setup)
    │  ├─ Directory Structure (FR-029-036): T002-T009
    │  └─ Project Initialization: T010-T018
    ↓
Phase 2 (Foundation - Database & Storage)
    ↓
Phase 2.5 (3-Plane Control Fabric) ← FR-056-060, FR-071-075
    │  ├─ Autonomous Entity Tables (T545-T550)
    │  ├─ 3-Plane Directory Structure (T551-T557)
    │  ├─ 14 Components per Plane (T558-T571)
    │  ├─ Promotion Policy Engine (T588-T592)
    │  ├─ Self-Healing Loop (T612-T620)
    │  └─ Autonomous Operation Loop (T621-T635)
    ↓
Phase 2.6 (Shared Provider Execution Memory) ← FR-037-042
    ↓
┌───────────────────────────────────────┐
│         MVP (P1 User Stories)          │
│  ┌─────────────────────────────────┐   │
│  │ Phase 3: US1 (Init)             │   │
│  │            ↓                    │   │
│  │ Phase 4: US2 (Neural) ←────────┐│   │
│  │    + 13 ModelSelectorAgents    ││   │
│  │            ↓                   ││   │
│  │ Phase 5: US3 (Memory) ←────────┘│   │
│  └─────────────────────────────────┘   │
└───────────────────────────────────────┘
    ↓ MVP Complete
┌───────────────────────────────────────┐
│       P2 User Stories (Parallel)       │
│  Phase 6: US4 (Digest)    ←─┐          │
│  Phase 7: US5 (UI)        ←─┤ Independent│
│  Phase 8: US6 (P2P)       ←─┤          │
│  Phase 9: US7 (Agents)    ←─┘          │
└───────────────────────────────────────┘
    ↓
Phase 10: US8 (Self-Improve) ← Requires US7
    ↓
Phase 11 (Project Mgmt Integration)
    ↓
Phase 12 (Polish)
    ↓
┌───────────────────────────────────────┐
│       P3 User Stories (Parallel)       │
│  Phase 13: US9 (Cross-Platform)  ←─┐   │
│  Phase 14: US10 (Connectors)     ←─┘   │
└───────────────────────────────────────┘
    ↓
Phase 15 (Governance & Safety) ← FR-025-028, Biblical Pipeline
    ↓
Phase 16 (SC Verification) ← SC-001 to SC-012
```

### 3-Plane Control Fabric Flow

```
Phase 2.5 (3-Plane Architecture):
    ┌──────────────────────────────────────────────────────────────┐
    │  SANDBOX PLANE      COORDINATOR PLANE      DEPLOYED PLANE    │
    │      (Blue)            (Analytics)             (Green)       │
    │                                                              │
    │  T551 dirs           T553 dirs               T552 dirs      │
    │  T558-571 comps      T572-580 config         T601-604 canary│
    │                      T588-596 analytics                      │
    │                      T609-611 audit                          │
    │                                                              │
    │  Artifacts ──────▶   Evaluate ──────▶       Promote         │
    │  (selftest)          (llama.cpp)            (canary)        │
    │                                                              │
    │  ◀────────────────   Rollback   ◀──────────────────────     │
    │                      (T600)                  (T604)         │
    └──────────────────────────────────────────────────────────────┘
```

### Provider Integration Flow

```
Phase 2.6 Provider Setup:
    llama.cpp (5+ models) ───┐
    Claude Code (CLI/Cloud/IDE) ───┤
    Codex (CLI/Cloud/IDE) ───┤
    VS Code Copilot (IDE) ───┼──> Shared Execution Memory Bus ──> Collaborative Reasoning
    Git CLI ───┤
    Cursor (IDE/CLI/Cloud) ───┤
    Abacus (CLI/Cloud) ───┘
```

### Autonomous Operation Flow

```
Continuous Loop (T621):
    ┌──────────────────────────────────────────────────────────────┐
    │                                                              │
    │   Goal Queue ──▶ Decompose ──▶ Execute ──▶ Validate         │
    │   (T622)         (T623)                                      │
    │       ▲                                                      │
    │       │                                                      │
    │   Self-Generated Goals (T631-T635)                          │
    │       │                                                      │
    │   Self-Healing Loop (T612-T620):                            │
    │   Detect → Diagnose → Auto-Fix → Validate → Escalate        │
    │                                                              │
    └──────────────────────────────────────────────────────────────┘
```

---

## Phase 17: New Requirements (from /clarify Session 2025-12-08)

### Rate Limiting & Throttling (FR-095-099) - Phase 2.6 Extension
- [ ] T771 [P] FR-095 Implement per-provider rate limit registry in `sys/core/src/providers/rate_limits.rs`
- [ ] T772 [P] FR-096 Implement exponential backoff handler (1s init, 60s max, 2x factor) in `sys/core/src/providers/backoff.rs`
- [ ] T773 [P] FR-097 Implement P2P throttling based on peer capacity in `p2p/src/throttle/mod.go`
- [ ] T774 FR-098 Implement self-generated goal rate limiter (max 10/hour) in `sys/core/src/autonomy/goal_limiter.rs`
- [ ] T775 FR-099 Add rate limit state to Shared Provider Execution Memory in `ai/shared/resources/execution-memory.db`

### Authentication & Identity (FR-100-109) - Phase 2 Extension
- [ ] T776 [P] FR-100 Implement Ed25519 keypair generation per device in `sys/core/src/auth/device_identity.rs`
- [ ] T777 [P] FR-101 Implement Argon2id key derivation for device key encryption in `sys/core/src/auth/key_derivation.rs`
- [ ] T778 [P] FR-102 Implement QR code pairing flow (5-min expiry token) in `sys/core/src/auth/pairing/qr.rs`
- [ ] T779 [P] FR-103 Implement 6-digit PIN pairing flow in `sys/core/src/auth/pairing/pin.rs`
- [ ] T780 [P] FR-104 Implement Bluetooth/NFC proximity pairing in `sys/core/src/auth/pairing/proximity.rs`
- [ ] T781 [P] FR-105 Implement encrypted file transfer pairing in `sys/core/src/auth/pairing/file_transfer.rs`
- [ ] T782 FR-106 Implement P2P mutual TLS with device keys in `p2p/src/security/mtls.go`
- [ ] T783 FR-107 Implement device revocation flow in `sys/core/src/auth/revocation.rs`
- [ ] T784 FR-108 Implement key rotation mechanism in `sys/core/src/auth/key_rotation.rs`
- [ ] T785 FR-109 Add browser password manager integration for web UI in `web/src/components/auth/PasswordManagerHint.tsx`

### Accessibility & Internationalization (FR-110-119) - Phase 7 (US5) Extension
- [ ] T786 [P] FR-110 Implement WCAG 2.1 AAA compliance audit in `web/src/utils/a11y/wcag-audit.ts`
- [ ] T787 [P] FR-111 Implement keyboard navigation with focus indicators (7:1 contrast) in `web/src/styles/focus.css`
- [ ] T788 [P] FR-112 Add ARIA labels to all interactive elements in `web/src/components/**/*.tsx`
- [ ] T789 [P] FR-113 Implement high contrast mode and OS preference detection in `web/src/hooks/useAccessibility.ts`
- [ ] T790 [P] FR-114 Create i18n string externalization system in `config/i18n/{locale}.json`
- [ ] T791 FR-115 Bundle local translations (no cloud dependency) in `web/src/i18n/bundled/`
- [ ] T792 FR-116 Add RTL layout support for Arabic/Hebrew in `web/src/styles/rtl.css`
- [ ] T793 [P] FR-117 Implement locale detection and switching in `web/src/i18n/locale-detector.ts`
- [ ] T794 [P] FR-118 Create translation contribution workflow in `docs/contributing/translations.md`
- [ ] T795 FR-119 Bundle English, Spanish, Chinese, Arabic, Hebrew translations in `config/i18n/`

### UI States & Feedback (FR-120-127) - Phase 7 (US5) Extension
- [ ] T796 [P] FR-120 Implement skeleton loader components in `web/src/components/ui/Skeleton.tsx`
- [ ] T797 [P] FR-121 Implement persistent status bar for background ops in `web/src/components/layout/StatusBar.tsx`
- [ ] T798 [P] FR-122 Implement toast notification system with retry actions in `web/src/components/ui/Toast.tsx`
- [ ] T799 [P] FR-123 Implement cached/partial data display with sync indicator in `web/src/hooks/useOfflineData.ts`
- [ ] T800 [P] FR-124 Design meaningful empty states with suggested actions in `web/src/components/ui/EmptyState.tsx`
- [ ] T801 [P] FR-125 Implement offline mode detection and indicator in `web/src/hooks/useNetworkStatus.ts`
- [ ] T802 [P] FR-126 Implement error boundary with recovery options in `web/src/components/ErrorBoundary.tsx`
- [ ] T803 FR-127 Add progress indicators for long-running ops (>2s) in `web/src/components/ui/Progress.tsx`

### Multi-Modal Interaction (FR-128-136) - Phase 7 (US5) - MVP for Glasses
- [ ] T804 [P] FR-128 Implement Whisper STT integration (<500ms latency) in `ai/voice/whisper_stt.py`
- [ ] T805 [P] FR-129 Implement Piper/Coqui TTS with voice selection in `ai/voice/piper_tts.py`
- [ ] T806 [P] FR-130 Implement camera input for real-time visual context in `web/src/components/vision/CameraInput.tsx`
- [ ] T807 [P] FR-131 Implement screen capture for screenshot queries in `sys/core/src/vision/screen_capture.rs`
- [ ] T808 [P] FR-132 Implement image file analysis via LLaVA in `ai/vision/llava_analyzer.py`
- [ ] T809 FR-133 Implement graceful degradation for missing hardware in `sys/core/src/multimodal/fallback.rs`
- [ ] T810 FR-134 Implement input method switching without restart in `sys/core/src/multimodal/switcher.rs`
- [ ] T811 FR-135 Add privacy controls for camera/mic in `web/src/components/settings/PrivacyControls.tsx`
- [ ] T812 FR-136 Implement multi-modal session persistence in `sys/core/src/multimodal/session.rs`

### Feature Flag System Infrastructure (FR-137-141) - Phase 2 Extension
- [ ] T739a FR-141 Implement feature flag runtime reload in `sys/core/src/config/feature_flags.rs`
  - Hot reload without restart
  - Flag state persistence to bootstrap-state.json
- [ ] T739b [P] FR-138 Implement per-scope flag management in `sys/core/src/config/flag_scopes.rs`
  - Global, device, user, and session scopes
  - Scope precedence rules
- [ ] T739c [P] FR-139 Implement flag change audit logging in `sys/core/src/config/flag_audit.rs`
  - Before/after state logging
  - Change attribution (user, agent, system)
- [ ] T739d [P] FR-137 Create feature flags config schema in `sys/core/src/config/schemas/feature_flags.json`
  - Define JSON schema for config/feature-flags.json
  - Include validation rules for flag names, scopes, and values
  - Add schema reference to bootstrap state
- [ ] T739e [P] FR-140 Implement graceful degradation UI in `web/src/components/ui/FeatureUnavailable.tsx`
  - Display clear feedback when feature-flagged capability is disabled
  - Provide guidance on how to enable the feature
  - Support dark/light themes

### Lifecycle Operations (FR-143-145) - Phase 0 Extension
- [X] B151 [BOOT] FR-146 Create `scripts/uninstall.ps1` - Clean NOA uninstall (Windows)
  - Remove toolchains, caches, generated files
  - Preserve user data with --keep-data flag
  - DryRun mode for preview
- [X] B152 [BOOT] FR-146 Create `scripts/uninstall.sh` - Clean NOA uninstall (Unix)
  - Mirror uninstall.ps1 functionality
  - Cross-platform parity per FR-088
- [ ] T813 FR-145 Implement upgrade detection in `sys/core/src/upgrade/detector.rs`
  - Version comparison (semantic versioning)
  - Breaking change detection
  - Migration path generator
- [ ] T814 [P] FR-145 Implement rollback-upgrade command in `sys/core/src/cli/upgrade.rs`
  - Pre-upgrade snapshot
  - Atomic upgrade with rollback on failure
  - Version history tracking

### Recovery Operations (FR-147-150) - Phase 2 Extension
- [ ] T815 FR-147 Implement download resume with progress tracking in `sys/core/src/download/resume.rs`
  - Range request support for partial downloads
  - Progress persistence across restarts
  - Bandwidth throttling option
- [ ] T816 [P] FR-148 Implement DB corruption detection and recovery in `sys/core/src/storage/recovery.rs`
  - SQLite integrity check on startup
  - WAL checkpoint management
  - Auto-repair for common issues
- [ ] T817 [P] FR-149 Implement OOM mitigation during inference in `sys/core/src/inference/memory_guard.rs`
  - Memory pressure monitoring
  - Graceful model unloading
  - Context size reduction
- [ ] T818 [P] FR-150 Implement proactive token refresh in `sys/core/src/auth/token_manager.rs`
  - Refresh before expiry (configurable margin)
  - Background refresh worker
  - Retry with exponential backoff

### Board Agent Conflict Resolution (FR-151) - Phase 9 Extension

- [ ] T819 FR-151 Implement Board Agent conflict resolution in `sys/core/src/agents/conflict_resolver.rs`
  - Constitutional arbitration for conflicting Board Agent recommendations
  - Staged deployment: Sandbox → fix issues → Deployed
  - SecurityAgent findings MUST be resolved before promotion
  - Log conflict details, resolution rationale, and deployment trace

### Offline Model Sideloading (FR-152) - Phase 4 Extension

- [ ] T820 FR-152 Implement model sideloading in `sys/core/src/models/sideload.rs`
  - Scan `noa_root/ai/models/` for new `.gguf` files on startup
  - Verify integrity via SHA-256 checksum in `model.sha256` companion file
  - Auto-register sideloaded models in model registry
  - Document sideloading procedure in quickstart.md

### Observability Stack (FR-153 to FR-158) - Phase 2 Extension

- [ ] T821 FR-153 Add tracing + tracing-subscriber to core Rust services in `sys/core/Cargo.toml`
  - Structured logging with spans
  - Configurable log levels and formats
  - Console and file output subscribers
- [ ] T822 [P] FR-153/FR-155 Add OpenTelemetry with OTLP exporter in `sys/core/src/observability/tracing.rs`
  - opentelemetry + opentelemetry-otlp crates
  - Export traces to configurable OTLP endpoint (Tempo, Jaeger)
  - Span context propagation across services
- [ ] T823 [P] FR-153/FR-154 Add Prometheus metrics exposition in `sys/core/src/observability/metrics.rs`
  - opentelemetry-prometheus crate
  - Expose `GET /metrics` endpoint in axum router
  - Standard NOA metrics: request_duration, active_connections, model_inference_time
- [ ] T824 [P] FR-156 Implement SQLite metrics store in `sys/core/src/observability/metrics_store.rs`
  - Persist metrics to `noa_root/data/metrics.db`
  - Rolling window retention (7 days default)
  - Query API for historical metrics analysis
- [ ] T825 FR-157 Ensure no Docker dependency for core observability
  - Document standalone observability setup
  - Local metrics available without external services
- [ ] T826 FR-158 Implement built-in metrics dashboard in `web/src/components/observability/MetricsDashboard.tsx`
  - Time-series charts for key metrics
  - Fallback when external Grafana unavailable
  - Real-time updates via WebSocket

---

## Parallel Execution Opportunities

### Within Foundation (Phase 2)
```
T018 (Schema) → T019-T036 (Tables) [PARALLEL]
T050 (Errors) → T051-T055 (Config/Logging/DB) [PARALLEL]
T056 (Server) → T057-T060 (Endpoints/Middleware) [PARALLEL]
T061 (CLI) → T062-T067 (Commands) [PARALLEL]
T776-T781 (Auth Pairing Methods) [PARALLEL]
```

### Within MVP Stories
```
US1: T072 → T073-T078 [PARALLEL] → T079
US2: T106-T110 [PARALLEL] → T114 → T118
US3: T131 → T132-T133 [PARALLEL] → T134-T138 [PARALLEL]
```

### P2 Stories (Fully Parallel)
```
US4 (Digest) ─────────┐
US5 (UI)    ─────────┼─── All execute in parallel
US6 (P2P)   ─────────┤
US7 (Agents) ────────┘
```

---

## Implementation Strategy

### MVP First (P1 Stories: US1, US2, US3)

**Goal**: Working seed environment with local intelligence and memory

1. Complete Phase 1-2 (Setup + Foundation)
2. Implement US1 (Init) - Self-contained environment
3. Implement US2 (Neural) - Local inference
4. Implement US3 (Memory) - Persistent recall

**MVP Deliverable**: `noa init && noa ask "Hello" && noa memory create/search`

### Incremental Delivery (P2 Stories)

After MVP, implement P2 stories in parallel:
- **US4**: Digest pipeline for knowledge acquisition
- **US5**: UI for user interaction
- **US6**: P2P for device federation
- **US7**: Agent orchestration for complex tasks

### Advanced Features (P3)

Finally, implement self-improvement (US8) which requires stable agent framework (US7).

Then implement P3 stories:
- **US9**: Cross-platform deployment (Windows, macOS, Linux, mobile)
- **US10**: External connectors with OAuth (GitHub, Gmail, cloud storage)

---

## Summary

| Metric | Count |
|--------|-------|
| **Total Tasks** | **1047** (187 Bootstrap + 850 T-tasks + 10 SK-tasks) |
| **Phase 0 (Bootstrap)** | **187** (B001-B152 + B057a-B058t AI/Shared) - UNIFIED |
| **Phase 1 (Setup)** | 21 (includes FR-029-036 directory structure + 3 prerequisite check tasks) |
| **Phase 2 (Foundation)** | 85 (60 original + 10 Auth/Identity T776-T785 + 5 Feature Flags T739a-e + 4 Recovery T815-T818 + 6 Observability T821-T826) |
| **Phase 2.5 (3-Plane Control Fabric)** | 107 (FR-056-060, FR-071-075) |
| **Phase 2.6 (Shared Providers)** | 63 (48 original + 5 Rate Limiting T771-T775 + 10 Spec-Kit Integration SK001-SK010) |
| **US1 Tasks** | 25 |
| **US2 Tasks** | 75 (includes 13 ModelSelectorAgents + 8 Multi-GPU + 3 model endpoints + 16 Advanced Learning + 1 Sideloading T820) |
| **US3 Tasks** | 22 |
| **US4 Tasks** | 50 (includes 11 digest source/knowledge endpoints) |
| **US5 Tasks** | 65 (38 original + 10 A11y/i18n T786-T795 + 8 UI States T796-T803 + 9 Multi-Modal T804-T812) |
| **US6 Tasks** | 49 (includes 21 P2P protocol service tasks: Discovery 4, Sync 4, Compute 4, Storage 5 + 4 infra) |
| **US7 Tasks** | 60 (includes 7 agent/task endpoints + 3 Board Agents T306a-c + 1 Conflict Resolution T819) |
| **US8 Tasks** | 55 |
| **US9 Tasks** | 17 (Cross-Platform Deployment) |
| **US10 Tasks** | 21 (Connectors & OAuth) |
| **Phase 11 (Project Mgmt)** | 23 |
| **Phase 12 (Polish)** | 48 (includes Execution Policy + Open Questions) |
| **Phase 15 (Governance)** | 16 (FR-025-028, Biblical Pipeline) |
| **Phase 16 (SC Verification)** | 16 (SC-001 to SC-012) |
| **Phase 17 (New Requirements)** | 65 (T771-T826 from /clarify sessions, T739a-e Feature Flags, T813-T818 Lifecycle/Recovery) |
| **Phase 18 (Kernel Independence)** | **32** (T827-T858, FR-159-165 + Checklist Gaps) |
| **Parallelizable [P]** | ~690 (66%) |

### Phase 0 Bootstrap Summary (MERGED from 002-unified-bootstrap)

| Subcategory | Tasks | Description |
|-------------|-------|-------------|
| Foundation | B001-B013 | Logging, platform detection, state management |
| Directory Structure | B014-B017 | Create noa_root directories |
| Prerequisites | B018-B023 | Git, Git LFS, GitHub CLI |
| Portable Toolchains | B024-B037 | Rust, Go, Node, Python, protoc → noa_root/opt/ |
| Quality Tools | B038-B049 | Linters, formatters, security scanners |
| CLI Utilities | B050-B056 | jq, rg, fd, bat, fzf → bin/ |
| Dev Tools | B057-B067 | IDEs, Docker, AI apps (gitignored) |
| Configuration | B068-B077 | Cache, logs, environment files |
| Orchestrator | B078-B085 | Main bootstrap script, verification |
| Integration | B086-B090 | Migrate from old scripts |
| Documentation | B091-B094 | README, guides |
| Constitutional | B095-B100 | Verify §3.1, §3.2, §3.5, §3.6, §3.12 |
| Lifecycle | B151-B152 | Uninstall scripts (PS1 + Bash) |
| Cross-Platform | B101-B120 | Script parity (PS1 ↔ Bash mirroring) |
| Kernel Independence | B121-B145 | NKAL, VM images, mode switching |
| Testing Matrix | B146-B150 | Platform-specific CI tests |

### Kernel Independence & Self-Containment (NEW - FR-159-165, /clarify 2025-12-09)
- **Kernel Selection Policy (T827-T834)**: ✅ T827-T834 (precedence rules, selection logic, tool isolation, upgrade rollback)
- **NKAL Trust Boundary (T835-T842)**: ✅ T835-T842 (capability grants, boundary validation, state persistence)
- **Documentation (T843-T845)**: ✅ T843-T845 (kernel precedence docs, self-containment docs, status output)
- **Checklist Gap Resolution (T846-T858)**: ✅ T846-T858 (CHK005-CHK033 gaps: startup sequence, terminology, performance, contracts)

### Prerequisite Check Tasks (NEW - CRITICAL)
- **Prerequisite Scripts**: ✅ T673-T675 (bash, PowerShell, CI integration)
  - Checks: Rust 1.83+, Go 1.23+, Node 20+, Python 3.12+, protoc 28+
  - Checks: rustfmt, clippy, golangci-lint 1.62+, eslint 9+, ruff 0.8+
  - Checks: Gitleaks 8.21+, Trivy 0.57+, Grype 0.84+, Semgrep 1.97+

### 3-Plane Control Fabric Architecture (NEW - FR-056-060)
- **Autonomous Entities**: ✅ T545-T550 (Goal, Plane, PlaneTransition, HealingEvent, HealthMetric tables)
- **3-Plane Directory Structure**: ✅ T551-T557 (sandbox-plane, deployed-plane, coordinator-plane)
- **14 Components per Plane**: ✅ T558-T571 (agents, memory, models, orchestrator, dataplane, networking, observability, packaging, perception, security, system, ui, update, workflow)
- **Coordinator Configuration**: ✅ T572-T580 (analytics.yaml, promotion-policy.yaml, constitutional prompts)
- **Shared Infrastructure**: ✅ T581-T587 (registry.db, artifacts/, releases/, logs/)
- **Promotion Policy Engine**: ✅ T588-T592 (risk tiers, canary cohorts, abort gates)
- **Coordinator Analytics**: ✅ T593-T596 (llama.cpp swarm, artifact ingestion, evaluation)
- **Promotion Flow**: ✅ T597-T600 (orchestrator, transfer, push, rollback)
- **Canary Controller**: ✅ T601-T604 (traffic splitting, SLO monitoring, autopilot)
- **State Synchronization**: ✅ T605-T608 (registry sync, knowledge sync, memory sync)
- **Transition Audit**: ✅ T609-T611 (before/after state logging)

### Autonomous Operation (NEW - FR-051-055, FR-061-075)
- **Self-Healing Loop**: ✅ T612-T620 (detect, diagnose, auto-fix, validate, escalate, plane swap)
- **Continuous Loop**: ✅ T621-T625 (always-on loop, goal queue, decomposition, optimization)
- **Full Autonomy**: ✅ T626-T630 (autonomous mode, co-improvement, safety net, activity log)
- **Goal Generation**: ✅ T631-T635 (self-generated goals, boundary checker, rationale logger, pattern analyzer)
- **3-Plane CLI**: ✅ T636-T641 (plane status, switch, rollback, promotion commands)
- **3-Plane API**: ✅ T642-T651 (planes, promotions, healing, goals, activity stream endpoints)

### Advanced Learning Techniques (NEW - FR-043 to FR-046 SHOULD)
- **ToolkenGPT (FR-043)**: ✅ T657-T660 (architecture, registry, pre-training, plugin loader)
- **Replay Memory Cache (FR-044)**: ✅ T661-T664 (architecture, buffer, knowledge base, sampler)
- **EWC Consolidation (FR-045)**: ✅ T665-T668 (architecture, Fisher info, consolidation, adapters)
- **MAML Meta-Learning (FR-046)**: ✅ T669-T672 (architecture, inner loop, outer loop, few-shot)

### Directory Structure Coverage (FR-029 to FR-036)
- **FR-029 noa_root/sys/**: ✅ T002 (system-level components)
- **FR-030 noa_root/p2p/**: ✅ T003 (P2P networking)
- **FR-031 noa_root/opt/**: ✅ T004 (optional packages)
- **FR-032 noa_root/init/**: ✅ T005 (init scripts)
- **FR-033 noa_root/containers/**: ✅ T006 (container definitions)
- **FR-034 noa_root/config/**: ✅ T007 (configuration files)
- **FR-035 noa_root/bin/**: ✅ T008 (executables)
- **FR-036 noa_root/ai/**: ✅ T009 (AI assets)

### Governance & Safety Coverage (FR-025 to FR-028)
- **FR-025 Constitutional Governance**: ✅ T690-T692 (engine, audit trail, schema)
- **FR-026 Biblical Governance Pipeline**: ✅ T693-T699 (Greek/Hebrew ingest, lexical analysis, embedding, knowledge graph)
- **FR-027 Reward/Correction Mechanisms**: ✅ T700-T702 (rewards, drift detection, correction)
- **FR-028 Self-Modification Rollback**: ✅ T703-T705 (snapshot, validator, executor)

### New Requirements Covered (FR-037 to FR-075)
- **Shared Provider Execution Memory**: ✅ 48 tasks (FR-037 to FR-042)
- **Collaborative Reasoning**: ✅ T450-T453
- **8 Provider Types**: ✅ llama.cpp, Claude, Codex, Copilot, Git, Cursor, Abacus
- **5+ Concurrent Local Models**: ✅ T429-T431
- **Multi-GPU Support**: ✅ 8 tasks T478-T485 (FR-047 to FR-050)
- **CUDA 13.1+ Tiles**: ✅ T483
- **3-Plane Control Fabric**: ✅ 107 tasks T545-T651 (FR-056-060, FR-071-075)
- **Autonomous Continuous Operation**: ✅ T621-T625 (FR-051-055)
- **Full Autonomy Operation**: ✅ T626-T630 (FR-061-065)
- **Autonomous Goal Generation**: ✅ T631-T635 (FR-066-070)
- **Self-Healing Loop**: ✅ T612-T620 (FR-071-075)
- **Multi-Modal Interaction (FR-023)**: ✅ T760-T764 (voice/vision stubs with graceful degradation)
- **Predictive Problem Solving (§3.11)**: ✅ T765-T770 (pattern recognition, failure analysis, risk assessment)

### Clarification Session Requirements (FR-095 to FR-136) - 2025-12-08
- **Rate Limiting & Throttling (FR-095-099)**: ✅ T771-T775 (per-provider limits, backoff, P2P throttle, goal limiter)
- **Authentication & Identity (FR-100-109)**: ✅ T776-T785 (Ed25519 keys, Argon2id, QR/PIN/NFC pairing, mTLS, revocation)
- **Accessibility & i18n (FR-110-119)**: ✅ T786-T795 (WCAG 2.1 AAA, keyboard nav, ARIA, RTL, bundled translations)
- **UI States & Feedback (FR-120-127)**: ✅ T796-T803 (skeleton loaders, status bar, toasts, empty states, offline)
- **Multi-Modal MVP (FR-128-136)**: ✅ T804-T812 (Whisper STT, Piper TTS, camera, screen capture, LLaVA, privacy)

### Contract Coverage (from contracts/)
- **noa-core.openapi.yaml**: ✅ Health, Memory, Agents, Tasks, Models, System, Planes, Promotions, Healing, Goals, Activity
- **digest-pipeline.openapi.yaml**: ✅ Sources, Jobs, Knowledge, Security endpoints
- **p2p-protocol.proto**: ✅ All 4 services covered:
  - Discovery (T501-T503): Announce, FindPeers, Ping
  - Sync (T505-T508): GetState, PushChanges, PullChanges, ResolveConflict
  - Storage (T652-T656): Store, Retrieve, Exists, List, Replicate
  - Compute (T514-T517): SubmitTask, GetTaskStatus, CancelTask, StreamOutput

### Data Model Coverage (22 Entities)
- **Core (1-14)**: Memory, Embedding, Agent, AgentLog, Task, TaskEvent, MicroAgentStack, Capsule, KnowledgeNode, KnowledgeEdge, DigestSource, Model, Device, SyncState
- **Providers (15-17)**: Provider, SharedExecutionContext, ProviderTask
- **Autonomous (18-22 NEW)**: Goal, Plane, PlaneTransition, HealingEvent, HealthMetric

### Universal Task Execution Policy Artifacts (§9)
- **Hash Generation**: ✅ T486-T487 (SHA-256 for all artifacts)
- **Verification Artifacts**: ✅ T488-T491 (FINAL_REPORT, COVERAGE, REPRO, EVIDENCE_LEDGER)
- **Triple-Verification**: ✅ T492 (Pass A/B/C automation)
- **Truth Gate**: ✅ T493 (§4 compliance automation)
- **Gap Hunt**: ✅ T494 (coverage scan)
- **Rich Metadata**: ✅ T496-T500 (validators for metadata, embeddings, configs, indexes, checksums)

### Open Questions Addressed (from research.md)
- **Model Download UX**: ✅ T530 (progress UI)
- **P2P Bootstrap**: ✅ T531 (first-time pairing wizard)
- **Memory Retention Policy**: ✅ T532-T533 (config + auto-prune)
- **Agent Marketplace**: ✅ T534-T535 (stub API + UI)

### Success Criteria Coverage (SC-001 to SC-012) - §3.12
- **SC-001 Init <60s**: ✅ T706 (benchmark)
- **SC-002 CPU Inference <2s**: ✅ T707 (benchmark)
- **SC-003 Memory Recall <500ms**: ✅ T710 (benchmark)
- **SC-004 Digest 10K <30min**: ✅ T711 (benchmark)
- **SC-005 200 Tasks 98%**: ✅ T712 (benchmark)
- **SC-006 P2P Sync <5s**: ✅ T713 (benchmark)
- **SC-007 UI Switch <200ms**: ✅ T714 (benchmark)
- **SC-008 7-Day Continuous**: ✅ T715 (stability test)
- **SC-009 Cross-Platform**: ✅ T717-T718 (consistency tests)
- **SC-010 100% Rollback**: ✅ T716 (validation)
- **SC-011 GPU <500ms**: ✅ T708 (benchmark)
- **SC-012 Multi-GPU <300ms**: ✅ T709 (benchmark)

### User Story Coverage (US1-US10)
- **US1 (Init)**: ✅ 25 tasks (Phase 3)
- **US2 (Neural)**: ✅ 58 tasks (Phase 4)
- **US3 (Memory)**: ✅ 22 tasks (Phase 5)
- **US4 (Digest)**: ✅ 50 tasks (Phase 6)
- **US5 (UI)**: ✅ 43 tasks (Phase 7) (includes FR-023 multi-modal T760-T764)
- **US6 (P2P)**: ✅ 49 tasks (Phase 8)
- **US7 (Agents)**: ✅ 56 tasks (Phase 9)
- **US8 (Self-Improve)**: ✅ 55 tasks (Phase 10) (includes §3.11 predictive T765-T770)
- **US9 (Cross-Platform)**: ✅ 17 tasks T722-T738 (Phase 13)
- **US10 (Connectors)**: ✅ 21 tasks T739-T759 (Phase 14)

### MVP Scope (US1 + US2 + US3 + 3-Plane Foundation)
- **Tasks**: 330 (Foundation + 3-Plane + Providers + 3 stories + Multi-GPU)
- **Estimated Duration**: 12-14 weeks with 2 developers

### Full Implementation
- **Tasks**: 770
- **Estimated Duration**: 26-30 weeks with 2-4 developers
