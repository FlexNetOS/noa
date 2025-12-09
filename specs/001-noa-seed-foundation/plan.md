# Implementation Plan: NOA Seed Foundation

**Feature**: 001-noa-seed-foundation
**Spec**: [spec.md](./spec.md)
**Branch**: `001-noa-seed-foundation`
**Created**: 2025-12-08
**Updated**: 2025-12-08

---

## Technical Context

**Project Type**: Multi-language monorepo with autonomous agentic capabilities
**Language/Version**: Rust 1.83+, Go 1.23+, TypeScript 5.x, Python 3.12+
**Primary Dependencies**: llama.cpp, tokio, axum, libp2p, React, Next.js
**Storage**: SQLite/PostgreSQL + pgvector 0.5.0+, Qdrant 1.8+, Redis 7.0+

### Cross-Platform Code Quality Requirement (CRITICAL)

**Every script MUST have platform-mirrored versions:**

| Platform | Extension | Shell | Notes |
|----------|-----------|-------|-------|
| Windows | `.ps1` | PowerShell 7.4+ | Primary Windows scripting |
| Linux | (none) | Bash 5.0+ | POSIX-compatible |
| macOS | (none) | Bash/Zsh | Same as Linux with macOS adaptations |

All scripts in `scripts/` follow the naming convention:
- `scripts/my-script` → Bash (Linux/macOS)
- `scripts/my-script.ps1` → PowerShell (Windows)

### Kernel Independence Architecture

**GOAL**: NOA can operate independently of the host kernel on ALL platforms.

```
┌─────────────────────────────────────────────────────────────────┐
│                     NOA Applications                            │
├─────────────────────────────────────────────────────────────────┤
│           NOA Kernel Abstraction Layer (NKAL)                   │
├─────────────┬─────────────┬─────────────┬───────────────────────┤
│   Windows   │    Linux    │   macOS     │        WSL            │
│   Kernel    │   Kernel    │   Kernel    │      Kernel           │
│   ────────  │  ────────   │  ────────   │     ────────          │
│   Native    │   Native    │   Native    │     Native            │
│   Hyper-V   │   KVM/QEMU  │   Apple VM  │     WSL2 VM           │
│   Container │   Container │   Container │     Container         │
└─────────────┴─────────────┴─────────────┴───────────────────────┘
```

**Kernel Independence Modes:**

| Mode | Windows | Linux | macOS | Description |
|------|---------|-------|-------|-------------|
| Native | Windows kernel | Linux kernel | Darwin kernel | Host kernel (default) |
| VM | Hyper-V | KVM/QEMU | Virtualization.framework | Custom NOA Linux kernel in VM |
| Container | Docker/WSC | Docker/Podman | Docker | Isolated container |
| Sandbox | Windows Sandbox | Bubblewrap | App Sandbox | User-space isolation |

### Self-Containment Strategy

**All installations target `noa_root`** with ZERO system-wide modifications:

| Component | Install Location | Source |
|-----------|-----------------|--------|
| Rust toolchain | `noa_root/opt/rust/` | Direct download from rust-lang.org |
| Go toolchain | `noa_root/opt/go/` | Direct download from go.dev |
| Node.js | `noa_root/opt/node/` | Direct download from nodejs.org |
| Python | `noa_root/opt/python/` | Embeddable Python / python-build-standalone |
| protoc | `noa_root/bin/` | GitHub releases |
| All utilities | `noa_root/bin/` | GitHub releases |
| npm packages | `noa_root/opt/node/node_modules/` | Local npm install |
| pip packages | `noa_root/opt/venv/` | Local venv install |
| Go modules | `noa_root/opt/go/workspace/` | go install to GOBIN |

---

## Phase 0: Prerequisites (CRITICAL - Run First)

Before any implementation, verify all CLI tools are installed:

### Build Toolchains (CRITICAL, contained-first)

| Tool | Min Version | Latest Stable | Install (contained) |
|------|-------------|---------------|---------------------|
| **Rust** | 1.83.0 | **1.83.0** | `pwsh -File scripts/setup/install-all-tools.ps1 -Tool rust` / `./scripts/setup/install-all-tools.sh rust` |
| **Go** | 1.23.0 | **1.23.4** | `... -Tool go` |
| **Node.js** | 20.0.0 | **22.12.0** | `... -Tool node` |
| **Python** | 3.12.0 | **3.12.8** | `... -Tool python` |
| **protoc** | 28.0.0 | **28.3** | `... -Tool protoc` |

### Quality Tools (HIGH, contained-first)

| Tool | Min Version | Install (contained) |
|------|-------------|----------------------|
| **rustfmt** | bundled | `... -Tool rust` (cargo/bin linked to noa_root/bin) |
| **clippy** | bundled | `... -Tool rust` |
| **golangci-lint** | 1.62.0 | `... -Tool golangci-lint` |
| **eslint** | 9.0.0 | `... -Tool eslint` (requires node portable) |
| **ruff** | 0.8.0 | `... -Tool ruff` |

### Security Tools (HIGH - FR-015, contained-first)

| Tool | Min Version | Install (contained) |
|------|-------------|----------------------|
| **Gitleaks** | 8.21.0 | `... -Tool gitleaks` |
| **Trivy** | 0.57.0 | `... -Tool trivy` |
| **Grype** | 0.84.0 | `... -Tool grype` |
| **Semgrep** | 1.97.0 | `... -Tool semgrep` |

### AI Provider CLIs (HIGH - FR-039, contained-first)

| Tool | Priority | Type | Install (contained) | Location |
|------|----------|------|----------------------|----------|
| **llama.cpp** | 1 | Local | `... -Tool llama` (submodule) | `noa_root/opt/llama.cpp/` |
| **Cursor CLI** | 2 | Hybrid/IDE/CLI | manual download (add to `noa_root/bin`) | `noa_root/opt/cursor-cli/` |
| **Claude Code** | 3 | CLI/Cloud/IDE | `... -Tool claude-code` (npm) | `noa_root/opt/node/node_modules/.bin` |
| **Codex CLI** | 4 | CLI/Cloud | `... -Tool codex-cli` (npm) | `noa_root/opt/node/node_modules/.bin` |
| **VS Code Copilot** | 5 | IDE | `... -Tool vscode-copilot` (portable) | `noa_root/opt/dev-tools/vscode/` |
| **Git CLI** | 6 | Local | `... -Tool git-cli` (portable) | `noa_root/opt/dev-tools/git/` |
| **Abacus CLI** | 7 | CLI/Cloud | `... -Tool abacus-cli` (npm) | `noa_root/opt/node/node_modules/.bin` |

**Provider Installation Commands**:

```powershell
# Install ALL AI providers at once (with prerequisite checks)
.\scripts\setup\install-all-tools.ps1 -Tool ai-providers

# Or install individually
.\scripts\setup\install-all-tools.ps1 -Tool claude-code
.\scripts\setup\install-all-tools.ps1 -Tool codex-cli
.\scripts\setup\install-all-tools.ps1 -Tool vscode-copilot    # VS Code + Copilot extensions
.\scripts\setup\install-all-tools.ps1 -Tool git-cli           # Git CLI as provider
.\scripts\setup\install-all-tools.ps1 -Tool abacus-cli

# Force update existing installations
.\scripts\setup\install-all-tools.ps1 -Tool ai-providers -UpdateExisting

# Install shared resources only
.\scripts\setup\install-all-tools.ps1 -Tool shared-resources
```

```bash
# Unix/macOS
./scripts/setup/install-all-tools.sh ai-providers

# Or individually
./scripts/setup/install-all-tools.sh claude-code codex-cli vscode-copilot git-cli abacus-cli

# Force update (set environment variable)
UPDATE_EXISTING=1 ./scripts/setup/install-all-tools.sh ai-providers

./scripts/setup/install-all-tools.sh shared-resources
```

**Provider Config Paths**:
- Claude Code: `noa_root/ai/providers/cloud/claude-code/config.json`
- Codex CLI: `noa_root/ai/providers/cloud/codex/config.json`
- Cursor CLI: `noa_root/ai/providers/hybrid/cursor/config.json`
- VS Code Copilot: `noa_root/ai/providers/ide/vscode-copilot/config.json`
- Git CLI: `noa_root/ai/providers/local/git-cli/config.json`
- Abacus CLI: `noa_root/ai/providers/cloud/abacus/config.json`

**Shared Resources Path**: `noa_root/ai/shared/`

### Shared Provider Resources (FR-037 to FR-042)

All providers share execution memory and resources via `noa_root/ai/shared/`:

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
    ├── execution-memory.db   # Shared execution memory bus (SQLite)
    ├── context/              # Shared context store
    └── state/                # Provider state sync
```

**Execution Memory Features** (FR-037):
- Context sharing between providers
- Reasoning state persistence
- Parallel task distribution
- Provider state synchronization

### Prerequisite Check Scripts (contained-first)

```bash
# Unix/macOS
./init/check-prereqs.sh --json

# Windows PowerShell
pwsh -File scripts/setup/check-prereqs.ps1 -Json
```

**Installer entrypoints (contained)**:
- PowerShell: `pwsh -File scripts/setup/install-all-tools.ps1`
- Bash/WSL/macOS: `./scripts/setup/install-all-tools.sh`
- Make target: `make install-tools` (runs contained installer + checker)

**Tasks**: T673-T675 implement these scripts

---

## Executive Summary

This plan implements the NOA Seed Foundation - a **100% autonomous agentic operating system** with:
- **136 Functional Requirements** (FR-001 to FR-136, including 19 bootstrap + 42 from clarifications)
- **12 Success Criteria** (SC-001 to SC-012)
- **10 User Stories** (US1 to US10)
- **812 Tasks** (T001 to T812)

### Recent Additions (Session 2025-12-08 /clarify)
- **FR-095-099**: Rate Limiting & Throttling (per-provider limits, adaptive backoff)
- **FR-100-109**: Authentication & Identity (device-bound keys, P2P trust chain)
- **FR-110-119**: Accessibility & i18n (WCAG 2.1 AAA, full multi-language)
- **FR-120-127**: UI States & Feedback (skeleton loaders, status indicators)
- **FR-128-136**: Multi-Modal Interaction (voice STT/TTS, vision, camera - MVP for glasses)

**Key Features**:
- Always-on continuous loop (FR-051-055)
- 3-plane control fabric for zero-downtime self-updates (FR-056-060)
- Full autonomy without human approval gates (FR-061-065)
- Autonomous goal generation (FR-066-070)
- 5-stage self-healing loop (FR-071-075)
- Advanced Learning: ToolkenGPT, Replay Memory, EWC, MAML (FR-043-046)

---

## Project-Mgmt Prompt Scope (Incorporated)

- **Source**: `N:/noa/ai/shared/prompts/project-mgmt-prompt.md`
- **Intent**: Fix and unify the project-mgmt app/director into the NOA agentic project-management system while preserving all source capabilities.
- **Mission alignment**: NOA = agentic project-mgmt system that consolidates 18 repositories, maintains all **95 required features**, and runs as a fully automated AgenticAI platform with a single `.project-mgmt-env` (Node, Python, Cargo) and local-first AI.

### Source Repositories (retain all features)
- prp-main, PRPs-agentic-eng-development, spec-kit, super-productivity, BMAD-METHOD-v5, BMAD-METHOD, agentic-cursorrules, agent-rules, ruler, my-todo-app-main, tududi, Taskosaur, system-prompts-and-models-of-ai-tools, promptfusion, dspy, dspy-code, claude-task-master, Backlog.md.

### Unified Architecture (baseline from prompt)
- Single environment at `${PROJECT_ROOT}/.project-mgmt-env` (Node modules, python venv, Cargo, `.env`/`.env.example`).
- `project-mgmt/` apps: web (Angular PWA), desktop (Electron), mobile (Capacitor), cli, mcp.
- `project-mgmt/core/`: ai-shared linkage, providers (auto-detect, priority), memory (never forget, instant recall), tasks/goals/policy/rules/specs/backlog, sync (vector clock, encryption/compression), config, state (NgRx), time-tracking (pomodoro/focus/idle/break/worklog), imex, util.
- AI layer: providers (claude, openai, ollama, llama.cpp, etc.), prompts (system/fusion/triggers), fusion engine.

### Provider Systems (must implement)
- Agent provider registry and auto-detection for 17 providers (CLI/IDE) with priority order **Local > Hybrid > Cloud** via provider-detection, provider-priority, ide-detection, shared-access services.
- Local AI preferred (llama.cpp/Ollama), using ai/shared path resolution and symlinks.
- **Cursor as Provider Orchestrator**: When in IDE context, Cursor agent coordinates ALL available providers for parallel task execution, distributing sub-tasks to optimal providers and aggregating results via Shared Provider Execution Memory bus.

### NOA Commands Integration (CLI)
- `noa ai providers|devices|shared|switch`, `noa start|stop|status|nodes|storage|compute`, `noa device register|list|capabilities`, Git helpers (`git-pr create|list|merge|sync`, `git-conflict detect|resolve`, `git-ci run|status`), self-containment (`bundle-libraries`, `bundle-all-libs`, `download-static-binaries`, `noa-kmod check`).

### Memory + ai/shared
- Memory service for never-forget + instant recall; semantic search; per-project context; uses ai/shared resources (agents, workflows, prompts, skills, tools, models, commands).

### Feature Parity & Success Criteria (prompt)
- **Feature parity**: 95/95 features from all repos; **Issue providers**: 8/8; **MCP tools**: 35+; **System prompts**: 25+; **Agent roles**: BMAD 10, PRP 7; **Agentic workflows**: 6.
- **Performance/UX targets**: build <5m, web bundle <500KB gz, desktop cold start <2s, offline core 100%, AI task breakdown <3s, memory recall <100ms, provider detection 17/17, IDE integration (Cursor/VS Code/Windsurf) 100%, sync reliability 100%.
- **AI priority**: Local-first enforced; ai/shared connection covers all 7 resource types; NOA command access 100%; productivity features (15) and sync providers (5) included.

---

## Hardware Tiers

| Tier | RAM | CPU | GPU | Storage | Concurrent Tasks |
|------|-----|-----|-----|---------|------------------|
| Minimum | 8GB | 4-core | None | 20GB | 50 |
| **Standard** | 16GB | 8-core | Integrated | 100GB | **200** (US7 baseline) |
| High-Performance | 64GB+ | 16+ core | RTX 3080+ | 500GB | 500+ |
| Development | 512GB+ | 24+ core | 2x RTX 5090+ | 2TB+ | 1000+ |

---

## Terminology (NOA vs CECCA)

| Term | Meaning | Usage |
|------|---------|-------|
| **NOA** | Name of App - the entire system | "NOA initializes", "NOA processes", "the NOA system" |
| **CECCA** | Chief Executive Commander Chief Agent | "CECCA orchestrates", "CECCA decomposes goals", "the root agent" |

**Rule**: NOA = system/product, CECCA = main agent inside NOA.

---

## Provider Priority (FR-039)

| Priority | Provider | Type | Latency | Fallback |
|----------|----------|------|---------|----------|
| 1 | llama.cpp (5+ models) | Local | <500ms | Primary - always offline |
| 2 | Cursor | Hybrid | <1s | IDE context available |
| 3 | Claude Code | Cloud | <2s | Complex reasoning |
| 4 | Codex | Cloud | <2s | Code generation |
| 5 | VS Code Copilot | IDE | <1s | Inline completions |
| 6 | Git CLI | Local | <100ms | Version control |
| 7 | Abacus | Cloud | <3s | Numerical/analytical |

**Strategy**: Local first → IDE → Cloud → Queue + notify after 3 retries

---

## Constitution Check

### Core Principles Alignment

| Principle | Section | Status | Implementation |
|-----------|---------|--------|----------------|
| §3.1 Self-Contained | FR-029-036 | ✅ | All under `noa_root` |
| §3.2 Local-First | FR-002-004 | ✅ | Offline-first design |
| §3.3 Agentic Orchestration | FR-007-011 | ✅ | CECCA + agents + MAS |
| §3.4 Adaptive | FR-043-046, FR-051-075 | ✅ | Advanced Learning + Autonomy |
| §3.5 Auditable | FR-006, FR-022, FR-068 | ✅ | Complete audit trail |
| §3.6 Security | FR-015, FR-019, FR-025 | ✅ | Gitleaks, Trivy, Grype, Semgrep |
| §3.7 Memory Sovereignty | FR-005, FR-040 | ✅ | Nothing forgotten |
| §3.8 P2P Hive-Mind | FR-017-020 | ✅ | User-owned cloud |
| §3.10 Biblical Governance | FR-025-026 | ✅ | Constitutional boundary |
| §3.12 Test Everything | FR-057, FR-071 | ✅ | Triple-Verification |

---

## Implementation Phases

### Phase 0: Unified Bootstrap (B001-B150) - NEW
**CRITICAL**: This phase MUST complete before any other implementation.

- **B001-B013**: Bootstrap foundation (logging, platform detection, state management)
- **B014-B017**: Directory structure & state management
- **B018-B023**: Prerequisites (Git, Git LFS, GitHub CLI)
- **B024-B037**: Portable toolchains (Rust, Go, Node, Python, protoc to `noa_root/opt/`)
- **B038-B055**: Quality & security tools
- **B057a-B057j**: AI Provider CLIs (Claude Code, Cursor, Codex, Abacus - FR-039)
- **B058-B067**: Dev tools (Cursor IDE, VS Code, Docker, AI apps - gitignored)
- **B068-B077**: Cache & log configuration, environment generation
- **B078-B090**: Main orchestrator & verification
- **B091-B100**: Documentation & constitutional verification
- **B101-B120**: Cross-platform script parity (all scripts mirrored)
- **B121-B145**: Kernel independence layer (NKAL, VM images, mode switching)
- **B146-B150**: Platform testing matrix

**Bootstrap Entry Points:**
- Windows: `.\scripts\bootstrap\bootstrap.ps1`
- Unix: `./scripts/bootstrap/bootstrap.sh`

### Phase 1: Setup (T001-T021)
- Directory structure verification (FR-029-036)
- Project initialization (workspaces, modules)
- Build scripts, CI pipeline

### Phase 2: Foundation (T022-T081, T776-T785)
- Database schema (14 entities)
- API foundation (axum)
- CLI foundation (clap)
- **Authentication & Identity** (FR-100-109): Device keypair generation, pairing flows, P2P TLS

### Phase 2.5: 3-Plane Control Fabric (T545-T651)
- Coordinator/Sandbox/Deployed planes
- Promotion policy engine
- Self-healing loop

### Phase 2.6: Shared Providers (T417-T477, T771-T775)
- 8 provider integrations (llama.cpp, Claude Code, Codex, Cursor, VS Code Copilot, Git CLI, Abacus, Ollama)
- Shared execution memory bus (FR-037)
- Parallel task distribution (FR-041)
- **Rate Limiting & Throttling** (FR-095-099, T771-T775): Per-provider limits, adaptive backoff, goal rate-limiting
- **Cursor as Provider Orchestrator** (T446a-T446b): Coordinates ALL providers for parallel task execution
  - Task-to-provider routing (reasoning → Claude, code → Codex, local → llama.cpp)
  - Result aggregation via Shared Provider Execution Memory bus

### Phases 3-5: MVP User Stories (T082-T191)
- US1: Initialize environment
- US2: Neural runtime + Advanced Learning (T657-T672)
- US3: Memory sovereignty

### Phases 6-9: P2 User Stories (Parallel)
- US4: Digest pipeline
- US5: Dynamic UI + **Accessibility** (FR-110-119) + **UI States** (FR-120-127) + **Multi-Modal** (FR-128-136 - MVP for glasses)
- US6: P2P federation
- US7: Agent orchestration

### Phases 10-12: Advanced
- US8: Self-improvement
- US9-10: Cross-platform + Connectors
- Project management integration
- Polish & documentation

---

## Advanced Learning Techniques (FR-043 to FR-046)

> **⚠️ POST-MVP**: These techniques are marked SHOULD (not MUST). Implement ONLY after MVP (US1-US3) is complete and stable. Tasks T657-T672 are deferred to Phase 10+.

### FR-043: ToolkenGPT (SHOULD)
**Tasks**: T657-T660 *(post-MVP)*
- Pre-trained tool tokens as small neural modules
- Pluggable architecture for extending model capabilities
- Token registry for tool discovery

### FR-044: Replay Memory Cache (SHOULD)
**Tasks**: T661-T664
- Short-term memory buffer for recent interactions
- External knowledge base integration
- Experience replay for efficient learning

### FR-045: EWC - Elastic Weight Consolidation (SHOULD)
**Tasks**: T665-T668
- Fisher Information computation
- Importance-weighted parameter consolidation
- Dynamic adapter modules for new tasks

### FR-046: MAML - Meta-Learning (SHOULD)
**Tasks**: T669-T672
- Inner-loop task adaptation
- Outer-loop meta-optimization
- Few-shot learning for rapid adaptation

---

## Success Criteria Mapping

| SC | Target | Hardware | Phase |
|----|--------|----------|-------|
| SC-001 | Init <60s | Standard | 3 |
| SC-002 | CPU inference <2s | Standard | 4 |
| SC-003 | Memory recall <500ms | Standard | 5 |
| SC-005 | 200 tasks, 98% success | **Standard (16GB, 8-core)** | 9 |
| SC-008 | 7-day continuous | Standard | 2.5 |
| SC-011 | GPU inference <500ms | High-Perf | 10 |
| SC-012 | Multi-GPU <300ms | Development | 10 |

---

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Missing CLI tools | Prerequisite check scripts (T673-T675) |
| Self-generated goals harm | Constitutional governance (FR-067) |
| 3-plane state desync | Coordinator as source of truth |
| Advanced Learning complexity | SHOULD status - implement after MVP |

---

## New Functional Requirements (from /clarify Session 2025-12-08)

### FR-095 to FR-099: Rate Limiting & Throttling
**Phase**: 2.6 (Shared Providers)
**Tasks**: T771-T775

- Per-provider rate limits with configurable token/request budgets
- Exponential backoff on HTTP 429 (initial 1s, max 60s, factor 2x)
- P2P throttling based on peer-reported capacity
- Self-generated goal rate-limit (max 10 new goals/hour)
- Rate limit state in Shared Provider Execution Memory

### FR-100 to FR-109: Authentication & Identity
**Phase**: 2 (Foundation)
**Tasks**: T776-T785

- Ed25519 keypair per device, encrypted with Argon2id-derived key
- Device pairing: QR code, 6-digit PIN, Bluetooth/NFC, encrypted file
- P2P mutual TLS authentication with device keys
- Device revocation and key rotation
- Optional browser password manager integration

### FR-110 to FR-119: Accessibility & Internationalization
**Phase**: 7 (US5 - Dynamic UI)
**Tasks**: T786-T795

- WCAG 2.1 Level AAA compliance
- Full keyboard navigation with visible focus (7:1 contrast)
- Screen reader compatibility with ARIA labels
- High contrast mode and OS accessibility preferences
- i18n with externalized strings (`config/i18n/{locale}.json`)
- Bundled translations: English, Spanish, Chinese, Arabic, Hebrew
- RTL layout support

### FR-120 to FR-127: UI States & Feedback
**Phase**: 7 (US5 - Dynamic UI)
**Tasks**: T796-T803

- Skeleton loaders for content areas
- Persistent status bar for background operations
- Toast notifications with retry actions
- Cached/partial data display during sync
- Meaningful empty states with suggested actions
- Offline mode indicators
- Progress indicators for long-running operations (>2s)

### FR-128 to FR-136: Multi-Modal Interaction
**Phase**: 7 (US5 - Dynamic UI) - Elevated to MVP for glasses testing
**Tasks**: T804-T812

- Speech-to-text via local Whisper (<500ms latency)
- Text-to-speech via Piper/Coqui with voice selection
- Camera input for real-time visual context
- Screen capture for screenshot-based queries
- Image file analysis (PNG, JPEG, WebP) via LLaVA/multimodal
- Graceful degradation when hardware unavailable
- Input method switching without restart
- Privacy controls for camera/mic
- Multi-modal session persistence

### FR-151: Board Agent Conflict Resolution
**Phase**: 9 (US7 - Agent Orchestration)
**Tasks**: T819

- Constitutional arbitration for conflicting Board Agent recommendations
- Staged deployment: Sandbox → fix issues → Deployed
- SecurityAgent findings MUST be resolved before promotion to Deployed plane
- Log conflict details, resolution rationale, and staged deployment trace to audit trail
- Integrates with 3-plane architecture (FR-056)

### FR-152: Offline Model Sideloading
**Phase**: 4 (US2 - Neural Runtime)
**Tasks**: T820

- Support model acquisition via USB/file transfer for air-gapped scenarios
- Copy `.gguf` models to `noa_root/ai/models/`
- System detects and registers sideloaded models on startup
- Integrity verification via SHA-256 checksum in `model.sha256` companion file
- Document sideloading procedure in quickstart.md

### FR-153 to FR-158: Observability Stack
**Phase**: 2 (Foundation)
**Tasks**: T821-T826

- Implement Rust observability using `tracing` + `tracing-subscriber` (FR-153)
- Add OpenTelemetry with OTLP exporter for distributed tracing (FR-153, FR-155)
- Add `opentelemetry-prometheus` for metrics exposition (FR-153)
- Expose Prometheus-format metrics at `GET /metrics` endpoint (FR-154)
- Export traces via OTLP to configurable endpoint (Tempo, Jaeger) (FR-155)
- Persist metrics to SQLite store (`noa_root/data/metrics.db`) for offline analysis (FR-156)
- No Docker required for core observability (FR-157)
- Provide built-in metrics dashboard in UI when Grafana unavailable (FR-158)

### FR-159 to FR-165: Kernel Independence & Self-Containment (from /clarify 2025-12-09)
**Phase**: 0 (Bootstrap) + 2 (Foundation)
**Tasks**: T827-T845

**FR-159: NOA Kernels First Policy** (CHK001 resolution)
- System MUST implement explicit kernel selection precedence: NOA VM > Container > Sandbox > Native
- Default mode MUST be Native for performance; escalation to isolated modes occurs automatically
- Escalation triggers: untrusted code, external data processing, user request, constitutional requirement
- Downgrade from isolated to native MUST require explicit user action

**FR-160: Kernel Selection Precedence** (CHK008 resolution)
- Priority 1: NOA VM (maximum isolation, untrusted code, cross-platform consistency)
- Priority 2: Container (lightweight isolation, faster startup)
- Priority 3: Sandbox (per-operation isolation, ephemeral environments)
- Priority 4: Native/Host (default, trusted environment, performance-critical)
- Selection logic documented in `config/kernel-selection-policy.json`

**FR-161: External Dependency Boundary** (CHK019 resolution)
- Internal = under `noa_root` directory tree, managed by NOA bootstrap
- External = anything outside `noa_root` (host APIs, system libraries, global tools, cloud services)
- Host kernel APIs permitted only in Native mode via NKAL abstraction
- Global tools detected but NOT used unless `--allow-global` flag passed
- Cloud services MUST be feature-flagged and optional

**FR-162: Tool Isolation Mechanism** (CHK039 resolution)
- `noa_root/bin` prepended to PATH before system paths
- `NOA_*` environment variables point to `noa_root/opt/` toolchains
- All package manager installs (npm/pip/cargo) use local prefix in `noa_root/opt/`
- Shell wrappers in `noa_root/bin/` explicitly invoke internal tool paths
- Internal versions preferred even if older than global

**FR-163: Internal Tool Upgrade Mechanism** (CHK041 resolution)
- Version requirements in `config/bootstrap-tools.json`
- Explicit upgrade via `install-all-tools.ps1 -UpdateExisting` / `UPDATE_EXISTING=1`
- Version checks on bootstrap (warn if outdated, no auto-upgrade)
- Previous versions archived to `noa_root/opt/archive/{tool}-{version}/`
- Rollback available for 7 days after upgrade

**FR-164: Kernel Mode State Persistence** (CHK026 resolution)
- All persistent state in `noa_root/data/` (accessible from all modes)
- Checkpoint written to `.kernel-switch-state.json` before mode switch
- VM/container modes mount `noa_root/data/` as shared volume
- State verification after switch confirms checkpoint integrity
- Hot-switch not supported; graceful shutdown required

**FR-165: NKAL Trust Boundary** (CHK055 resolution)
- NKAL defines boundary between trusted NOA code and untrusted host kernel
- Above NKAL: full privileges within `noa_root`
- Below NKAL: host kernel accessed only through NKAL interface
- Input sanitization and output verification at boundary crossing
- Privileged operations require capability grants in `config/nkal-capabilities.json`

**FR-166: Host Kernel vs NOA Portable Dependency Policy** (from /clarify 2025-12-09)
- **Host Kernel MAY Be Used For:**
  1. Start-up/Bootstrap: Initial system boot and NOA initialization
  2. Environment Scanning: Discovering host capabilities (CPU, GPU, memory, storage)
  3. Host Optimization: Optimizing host performance (NOA internalizes discovered features)
  4. File/Directory Access: Accessing host files/directories for goal completion (outside `noa_root`)
- **NOA Portable Dependencies MUST Be Used For (100% Independence):**
  - All tools (jq, ripgrep, fd, bat, etc.) → `noa_root/bin/`
  - Terminal and shell → internal shell environment
  - All packages (npm → `noa_root/opt/node`, pip → `noa_root/opt/venv`, cargo → `noa_root/opt/rust`)
  - All services (llama-server, ollama, gitea) → `noa_root/init/services/`
  - Network stack (VM/container mode) → NKAL abstraction
  - All settings/configs → `noa_root/config/`
  - All persistent state/data → `noa_root/data/`
- **Platform Coverage**: ALL platforms (Windows, Linux, macOS, mobile, XR) and ALL hardware (x64, ARM, GPU)
- **Independence Guarantee**: NOA achieves 100% independent functionality by bundling portable dependencies

---

## Kernel Independence & Self-Containment Tasks (T827-T845)

### Phase 0: Bootstrap Integration

| Task | Description | Dependencies |
|------|-------------|--------------|
| **T827** | Add `config/kernel-selection-policy.json` schema with precedence rules | B121 |
| **T828** | Implement kernel selection logic in `noa-kernel-params.ps1` based on FR-160 | T827 |
| **T829** | Implement kernel selection logic in `noa-kernel-params` (bash) | T828 |
| **T830** | Add `--allow-global` flag to all tool detection scripts | B078 |
| **T831** | Create `config/bootstrap-tools.json` with version pinning schema | B014 |
| **T832** | Implement `-UpdateExisting` flag in `install-all-tools.ps1` | T831 |
| **T833** | Implement tool archival to `noa_root/opt/archive/` before upgrade | T832 |
| **T834** | Add upgrade rollback via `install-all-tools.ps1 -Rollback -Tool <name>` | T833 |

### Phase 2: Foundation - NKAL & State Management

| Task | Description | Dependencies |
|------|-------------|--------------|
| **T835** | Create NKAL capability grant schema `config/nkal-capabilities.json` | T001 |
| **T836** | Implement NKAL trust boundary validation in `sys/core/src/nkal/` | T835, T022 |
| **T837** | Add input sanitization layer at NKAL boundary | T836 |
| **T838** | Add output verification layer at NKAL boundary | T837 |
| **T839** | Create `.kernel-switch-state.json` checkpoint on mode change | T836 |
| **T840** | Implement state verification after kernel mode switch | T839 |
| **T841** | Add shared volume mount configuration for VM/container modes | T840, B125 |
| **T842** | Implement graceful shutdown requirement before mode switch | T841 |

### Documentation & Verification

| Task | Description | Dependencies |
|------|-------------|--------------|
| **T843** | Document kernel selection precedence in `docs/architecture/kernel-independence.md` | T828 |
| **T844** | Document external vs internal dependency boundary in `docs/architecture/self-containment.md` | T830 |
| **T845** | Add kernel mode and tool version to `noa status` output | T842 |

---

## Task Summary

| Category | Count |
|----------|-------|
| **Total Tasks** | **1034** |
| **Phase 0: Bootstrap (B tasks)** | 187 |
| **Phase 1+ Core (T tasks)** | 837 |
| **Spec-Kit Integration (SK tasks)** | 10 |
| Completed [X] | 172 |
| Pending [ ] | 862 |
| **Parallelizable** | ~680 (66%) |

### Task Breakdown by Phase

| Phase | Tasks | Description |
|-------|-------|-------------|
| Phase 0: Bootstrap | 187 | B001-B150 + B057a-B086 (AI providers, shared resources, reports) |
| Phase 1-2: Foundation | 81 | T001-T071 (directory, DB schema, API, CLI) |
| Phase 2.5: 3-Plane Architecture | 107 | T545-T651 (control fabric, self-healing) |
| Phase 2.6: Shared Providers | 63 | T417-T464 + T771-T775 + SK001-SK010 |
| Phase 3: US1 (Init) | 25 | T072-T096 |
| Phase 4: US2 (Neural Runtime) | 48 | T097-T130, T465-T485, T657-T672 |
| Phase 5: US3 (Memory) | 26 | T131-T152 |
| Phase 6: US4 (Digest) | 55 | T153-T191, T509-T541 |
| Phase 7: US5 (Dynamic UI) | 85 | T192-T229, T760-T764, T786-T812 |
| Phase 8: US6 (P2P) | 42 | T230-T271, T652-T656 |
| Phase 9: US7 (Orchestration) | 65 | T272-T336 |
| Phase 10: US8 (Self-Improvement) | 32 | T337-T368 |
| Phase 11: US9-10 (Cross-Platform) | 48 | T369-T416 |
| Phase 15: Governance | 16 | T690-T705 |
| Phase 16: Verification | 25 | T706-T730 |
| Phase 17: Polish | 15 | T731-T745 |
| **Kernel Independence** | **19** | **T827-T845** |

### Bootstrap Task Categories (Phase 0) - 186 Tasks

| Subcategory | Tasks | Count | Description |
|-------------|-------|-------|-------------|
| Foundation | B001-B013 | 13 | Logging, platform detection, state management |
| Directory Structure | B014-B017 | 4 | Create noa_root directories |
| Prerequisites | B018-B023 | 6 | Git, Git LFS, GitHub CLI |
| Toolchains | B024-B037 | 14 | Rust, Go, Node, Python, protoc (portable) |
| Quality Tools | B038-B056 | 19 | Linters, formatters, security scanners, CLI utils |
| **AI Provider CLIs** | **B057a-B057n** | **14** | **Claude Code, Cursor, Codex, VS Code Copilot, Git CLI, Abacus CLIs (FR-039)** |
| **Shared Resources** | **B058a-B058t** | **20** | **Shared execution memory, provider state sync (FR-037-042)** |
| Dev Tools | B059-B067b | 12 | IDEs, Docker, AI apps (gitignored) |
| Configuration | B068-B077 | 10 | Cache, logs, environment files |
| Orchestrator | B078-B090 | 13 | Main bootstrap script, verification |
| Documentation | B091-B094 | 4 | README, guides, tool docs |
| Constitutional | B095-B100 | 6 | Constitutional verification |
| Cross-Platform | B101-B120 | 20 | Script parity (PS1 ↔ Bash mirroring) |
| Kernel Independence | B121-B145 | 25 | NKAL, VM images, mode switching |
| Testing Matrix | B146-B150 | 5 | Platform-specific CI tests |

---

## Fixes Applied (from /analyze)

| Issue | Status | Fix |
|-------|--------|-----|
| C1: Rust missing | ✅ | Added to Prerequisites, spec updated to 1.83+ |
| C2: Go missing | ✅ | Added to Prerequisites, spec updated to 1.23+ |
| C3: protoc missing | ✅ | Added to Prerequisites |
| C4: Security tools | ✅ | Added to Prerequisites (Gitleaks, Trivy, Grype, Semgrep) |
| C5: Lint tools | ✅ | Added to Prerequisites (golangci-lint, eslint, ruff) |
| **C6: Claude Code CLI** | ✅ | Added AI Provider CLIs section (FR-039), tasks B057a-B057j |
| A1: Glossary location | ✅ | Forward reference added at first usage |
| A2: US7 hardware | ✅ | Added "Standard Hardware (16GB, 8-core)" context |
| A3: Provider priority | ✅ | Added Provider Priority table with fallback order |
| A4: NOA vs CECCA | ✅ | Added Terminology section with clear definitions |
| A5: FR-043-046 tasks | ✅ | Added 16 tasks (T657-T672) for Advanced Learning |
| D1: Duplicate T509-T513 | ✅ | Renumbered P2P Storage to T652-T656 |

---

## Spec-Kit Shared Provider Integration (FR-037, §3.13)

**Purpose**: Wire spec-kit into the shared provider access system so ALL providers can access the same spec simultaneously without duplication.

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    SPEC-KIT SHARED PROVIDER ACCESS                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────┐         ┌──────────────────────────────────────────┐  │
│  │   SPEC-KIT CLI   │         │         SHARED PROVIDER BUS               │  │
│  │                  │         │                                           │  │
│  │ • specify init   │────────►│  ai/shared/resources/spec-distribution/  │  │
│  │ • /speckit.plan  │         │  ├── active-spec.json (current spec ref) │  │
│  │ • /speckit.tasks │         │  ├── spec-locks.json (concurrency)       │  │
│  │                  │         │  └── provider-access.log (audit trail)   │  │
│  └──────────────────┘         └────────────────┬─────────────────────────┘  │
│                                                │                             │
│          ┌─────────────────────────────────────┼─────────────────────┐      │
│          │                                     │                     │      │
│          ▼                                     ▼                     ▼      │
│  ┌───────────────┐    ┌───────────────┐    ┌───────────────┐    ┌────────┐ │
│  │ Claude Code   │    │ Codex CLI     │    │ Cursor Agent  │    │ Copilot│ │
│  │ (reasoning)   │    │ (code-gen)    │    │ (orchestrate) │    │ (IDE)  │ │
│  │               │    │               │    │               │    │        │ │
│  │ ◄──PARALLEL ACCESS TO SAME SPEC VIA SHARED MEMORY BUS──►              │ │
│  └───────────────┘    └───────────────┘    └───────────────┘    └────────┘ │
│                                                                              │
│  EXECUTION MEMORY: ai/shared/resources/execution-memory.db                  │
│  ├── spec_context (shared spec state)                                       │
│  ├── provider_locks (coordination)                                          │
│  └── parallel_tasks (distributed execution)                                 │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Spec-Kit Provider Connection Function

spec-kit MUST implement a **universal provider connection function** that:

1. **Connects ANY registered provider** to the shared spec context
2. **Distributes specs** to all active providers simultaneously
3. **Maintains read-only shared access** (no duplication, single source of truth)
4. **Coordinates parallel execution** via the Shared Provider Execution Memory bus

**Function Signature** (conceptual):

```python
def connect_provider(
    provider_id: str,           # e.g., "claude-code", "codex", "cursor"
    spec_path: str,             # Path to the spec (relative to noa_root)
    access_mode: str = "read",  # "read" | "write" | "coordinate"
    parallel: bool = True       # Enable parallel access by other providers
) -> ProviderConnection:
    """
    Connect a provider to the shared spec distribution system.

    All providers sharing the same spec_path get synchronized access via
    the execution-memory.db bus without creating duplicate copies.
    """
```

### Shared Spec Distribution Schema

**File**: `ai/shared/resources/spec-distribution.json`

```json
{
  "$schema": "https://noa.local/schemas/spec-distribution.json",
  "version": "1.0.0",
  "activeSpec": {
    "path": "specs/001-noa-seed-foundation/",
    "hash": "sha256:...",
    "lastAccessed": "2025-12-09T00:00:00Z"
  },
  "connectedProviders": [
    {
      "providerId": "claude-code",
      "accessMode": "read",
      "connectedAt": "2025-12-09T00:00:00Z",
      "lastSync": "2025-12-09T00:00:00Z"
    },
    {
      "providerId": "codex",
      "accessMode": "read",
      "connectedAt": "2025-12-09T00:00:00Z"
    }
  ],
  "parallelExecution": {
    "enabled": true,
    "coordinator": "cursor",
    "taskDistribution": "round-robin"
  }
}
```

### Tasks for Spec-Kit Provider Integration

| Task ID | Description | Dependencies |
|---------|-------------|--------------|
| **SK001** | Create `connect_provider()` function in spec-kit CLI | B057a-B057j |
| **SK002** | Implement spec-distribution.json schema and validation | SK001 |
| **SK003** | Add provider registration to execution-memory.db | SK002, T417 |
| **SK004** | Implement parallel spec broadcast to all connected providers | SK003 |
| **SK005** | Add spec locking mechanism for write coordination | SK004 |
| **SK006** | Create audit logging for provider spec access | SK005 |
| **SK007** | Update AGENT_CONFIG in spec-kit to use shared resources | SK002 |
| **SK008** | Add `--connect-providers` flag to `specify init` | SK001 |
| **SK009** | Implement provider health check before spec distribution | SK004 |
| **SK010** | Create spec-kit MCP tool for provider orchestration | SK006 |

### Integration with Existing Shared Resources

The spec-kit integration uses these existing shared resource paths:

| Resource | Path | Purpose |
|----------|------|---------|
| Execution Memory | `ai/shared/resources/execution-memory.db` | Provider coordination |
| Resource Registry | `ai/shared/resources/resource-registry.json` | Provider registration |
| Resource Mapping | `ai/shared/resources/resource-mapping.json` | Name unification |
| Resource Aliases | `ai/shared/resources/resource-aliases.json` | Backward compatibility |
| **Spec Distribution** | `ai/shared/resources/spec-distribution.json` | **NEW - Spec sharing** |

### Provider Parallel Access Flow

1. **Spec Creation/Update**: `/speckit.plan` or `/speckit.specify` creates/updates spec
2. **Broadcast**: spec-kit broadcasts spec location to Shared Provider Bus
3. **Provider Connection**: Each provider registers via `connect_provider()`
4. **Parallel Read**: All providers read from same spec path (no duplication)
5. **Coordinated Write**: Only one provider can write at a time (via locks)
6. **Execution Memory Sync**: Provider state synced via execution-memory.db

---

## Next Steps

1. **Run prerequisite check**: `.\scripts\powershell\check-prerequisites.ps1`
2. **Install missing tools** per the Prerequisites section
3. **Run `/tasks`** to update task list if needed
4. **Run `/implement`** to begin Phase 1

---

## Cross-Platform Script Mapping

| Script | Windows (PS1) | Unix (Bash) | Status |
|--------|--------------|-------------|--------|
| Bootstrap | bootstrap/bootstrap.ps1 | bootstrap/bootstrap.sh | ✅ |
| Check Prereqs | setup/check-prereqs.ps1 | init/check-prereqs.sh | ✅ |
| Install Prereqs | setup/install-prereqs.ps1 | (via bootstrap) | ✅ |
| Docker Service | docker-service.ps1 | docker-service | ✅ |
| Ollama Service | ollama-service.ps1 | ollama-service | ✅ |
| SSH Service | ssh-service.ps1 | ssh-service | ✅ |
| Gitea Service | gitea-service.ps1 | gitea-service | ✅ |
| Kernel Params | noa-kernel-params.ps1 | noa-kernel-params | ✅ |
| Kernel Modules | noa-kmod.ps1 | noa-kmod | ✅ |
| Namespace | noa-namespace.ps1 | noa-namespace | ✅ |
| Bundle Libs | bundle-libraries.ps1 | bundle-libraries | ✅ |
| Bundle All | bundle-all-libs.ps1 | bundle-all-libs | ✅ |
| Git CI | git-ci.ps1 | git-ci | ✅ |
| Git Conflict | git-conflict.ps1 | git-conflict | ✅ |
| Git PR | git-pr.ps1 | git-pr | ✅ |
| NOA CLI | noa.ps1 | noa | ✅ |

---

**Plan Updated**: 2025-12-09
**Total FRs**: 166 (FR-001 to FR-166)
**Total Tasks**: 1034 (187 Bootstrap + 837 Core + 10 Spec-Kit)
**Completed**: 172 tasks (16.6%)
**Pending**: 862 tasks
**Estimated Duration**: 32-36 weeks (2-4 developers)
**Cross-Platform Parity**: 100% (all scripts mirrored)
**Kernel Independence**: FR-159 to FR-166 (8 new requirements)
