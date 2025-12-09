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
- **75 Functional Requirements** (FR-001 to FR-075)
- **12 Success Criteria** (SC-001 to SC-012)
- **10 User Stories** (US1 to US10)
- **747 Tasks** (T001 to T747)

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

### Phase 2: Foundation (T022-T081)
- Database schema (14 entities)
- API foundation (axum)
- CLI foundation (clap)

### Phase 2.5: 3-Plane Control Fabric (T545-T651)
- Coordinator/Sandbox/Deployed planes
- Promotion policy engine
- Self-healing loop

### Phase 2.6: Shared Providers (T417-T477)
- 8 provider integrations
- Shared execution memory
- Parallel task distribution

### Phases 3-5: MVP User Stories (T082-T191)
- US1: Initialize environment
- US2: Neural runtime + Advanced Learning (T657-T672)
- US3: Memory sovereignty

### Phases 6-9: P2 User Stories (Parallel)
- US4: Digest pipeline
- US5: Dynamic UI
- US6: P2P federation
- US7: Agent orchestration

### Phases 10-12: Advanced
- US8: Self-improvement
- US9-10: Cross-platform + Connectors
- Project management integration
- Polish & documentation

---

## Advanced Learning Techniques (FR-043 to FR-046)

### FR-043: ToolkenGPT (SHOULD)
**Tasks**: T657-T660
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

## Task Summary

| Category | Count |
|----------|-------|
| **Total Tasks** | 907 |
| **Phase 0: Bootstrap** | 160 (B001-B150 + B057a-B057j AI Providers) |
| Phase 1-2 | 81 |
| 3-Plane Architecture | 107 |
| Shared Providers | 48 |
| Advanced Learning | 16 (T657-T672) |
| MVP (US1-3) | 99 |
| P2 Stories | 193 |
| P3 Stories | 93 |
| Integration & Polish | 100 |
| **Parallelizable** | 611 (67%) |

### Bootstrap Task Categories (Phase 0)

| Subcategory | Tasks | Description |
|-------------|-------|-------------|
| Foundation | B001-B013 | Logging, platform detection, state management |
| Directory Structure | B014-B017 | Create noa_root directories |
| Prerequisites | B018-B023 | Git, Git LFS, GitHub CLI |
| Toolchains | B024-B037 | Rust, Go, Node, Python, protoc (portable) |
| Quality Tools | B038-B055 | Linters, formatters, security scanners |
| **AI Provider CLIs** | **B057a-B057j** | **Claude Code, Cursor, Codex, Abacus CLIs (FR-039)** |
| Dev Tools | B058-B067 | IDEs, Docker, AI apps (gitignored) |
| Configuration | B068-B077 | Cache, logs, environment files |
| Orchestrator | B078-B090 | Main bootstrap script, verification |
| Documentation | B091-B100 | README, guides, constitutional checks |
| Cross-Platform | B101-B120 | Script parity (PS1 ↔ Bash mirroring) |
| Kernel Independence | B121-B145 | NKAL, VM images, mode switching |
| Testing Matrix | B146-B150 | Platform-specific CI tests |

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

**Plan Updated**: 2025-12-08
**Total FRs**: 94 (75 core + 19 bootstrap)
**Total Tasks**: 897 (150 bootstrap + 747 core)
**Estimated Duration**: 28-32 weeks (2-4 developers)
**Cross-Platform Parity**: 100% (all scripts mirrored)
