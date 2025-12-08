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

---

## Phase 0: Prerequisites (CRITICAL - Run First)

Before any implementation, verify all CLI tools are installed:

### Build Toolchains (CRITICAL)

| Tool | Min Version | Latest Stable | Install (Windows) |
|------|-------------|---------------|-------------------|
| **Rust** | 1.75.0 | **1.83.0** | `winget install Rustlang.Rustup && rustup default stable` |
| **Go** | 1.21.0 | **1.23.4** | `winget install GoLang.Go` |
| **Node.js** | 20.0.0 | **22.12.0** | `winget install OpenJS.NodeJS.LTS` |
| **Python** | 3.11.0 | **3.12.8** | `winget install Python.Python.3.12` |
| **protoc** | 25.0.0 | **28.3** | `winget install Google.Protobuf` |

### Quality Tools (HIGH)

| Tool | Min Version | Install Command |
|------|-------------|-----------------|
| **rustfmt** | (bundled) | `rustup component add rustfmt` |
| **clippy** | (bundled) | `rustup component add clippy` |
| **golangci-lint** | 1.62.0 | `go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest` |
| **eslint** | 9.0.0 | `npm install -g eslint` |
| **ruff** | 0.8.0 | `pip install ruff` |

### Security Tools (HIGH - FR-015)

| Tool | Min Version | Install Command |
|------|-------------|-----------------|
| **Gitleaks** | 8.21.0 | `choco install gitleaks` |
| **Trivy** | 0.57.0 | `choco install trivy` |
| **Grype** | 0.84.0 | `choco install grype` |
| **Semgrep** | 1.97.0 | `pip install semgrep` |

### Prerequisite Check Scripts

```bash
# Unix/macOS
./init/check-prereqs.sh

# Windows PowerShell
.\scripts\setup\check-prereqs.ps1
```

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

### Phase 1: Setup (T001-T021)
- **Prerequisite checks** (T673-T675) - CRITICAL
- Directory structure (FR-029-036)
- Toolchain initialization

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
| **Total Tasks** | 747 |
| Prerequisite Check | 3 (T673-T675) |
| Phase 1-2 | 81 |
| 3-Plane Architecture | 107 |
| Shared Providers | 48 |
| Advanced Learning | 16 (T657-T672) |
| MVP (US1-3) | 99 |
| P2 Stories | 193 |
| P3 Stories | 93 |
| Integration & Polish | 100 |
| **Parallelizable** | 546 (74%) |

---

## Fixes Applied (from /analyze)

| Issue | Status | Fix |
|-------|--------|-----|
| C1: Rust missing | ✅ | Added to Prerequisites, spec updated to 1.83+ |
| C2: Go missing | ✅ | Added to Prerequisites, spec updated to 1.23+ |
| C3: protoc missing | ✅ | Added to Prerequisites |
| C4: Security tools | ✅ | Added to Prerequisites (Gitleaks, Trivy, Grype, Semgrep) |
| C5: Lint tools | ✅ | Added to Prerequisites (golangci-lint, eslint, ruff) |
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

**Plan Updated**: 2025-12-08
**Total FRs**: 75
**Total Tasks**: 747
**Estimated Duration**: 24-28 weeks (2-4 developers)
