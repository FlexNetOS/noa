# Implementation Plan: NOA Seed Foundation

**Feature**: 001-noa-seed-foundation
**Spec**: [spec.md](./spec.md)
**Branch**: `001-noa-seed-foundation`
**Created**: 2025-12-08
**Updated**: 2025-12-08

---

## Technical Context

**Project Type**: Multi-language monorepo with autonomous agentic capabilities
**Language/Version**: Rust 1.75+, Go 1.21+, TypeScript 5.x, Python 3.11+
**Primary Dependencies**: llama.cpp, tokio, axum, libp2p, React, Next.js
**Storage**: SQLite/PostgreSQL + pgvector, Qdrant 1.8+, Redis 7.0+

---

## Executive Summary

This plan implements the NOA Seed Foundation - a **100% autonomous agentic operating system** with:
- **75 Functional Requirements** (FR-001 to FR-075)
- **12 Success Criteria** (SC-001 to SC-012)
- **10 User Stories** (US1 to US10)
- **5 Clarifications** for autonomous operation

**Key Autonomy Features**:
- Always-on continuous loop (FR-051-055)
- 3-plane control fabric for zero-downtime self-updates (FR-056-060)
- Full autonomy without human approval gates (FR-061-065)
- Autonomous goal generation (FR-066-070)
- 5-stage self-healing loop (FR-071-075)

---

## Hardware Tiers

| Tier | RAM | CPU | GPU | Storage |
|------|-----|-----|-----|---------|
| Minimum | 8GB | 4-core | None | 20GB |
| Standard | 16GB | 8-core | Integrated | 100GB |
| High-Performance | 64GB+ | 16+ core | RTX 3080+ | 500GB |
| Development | 512GB+ | 24+ core | 2x RTX 5090+ | 2TB+ |

## Performance Goals

- Initialization: <60 seconds on standard hardware (SC-001)
- Inference latency (CPU-only): <2 seconds (SC-002)
- Inference latency (single GPU): <500ms (SC-011)
- Inference latency (multi-GPU): <300ms (SC-012)
- Memory recall: <500ms (SC-003)
- UI context switch: <200ms (SC-007)
- P2P sync: <5 seconds for <1MB delta (SC-006)
- 7-day continuous operation without restart (SC-008)

---

## Constitution Check

### Core Principles Alignment

| Principle | Section | Status | Implementation |
|-----------|---------|--------|----------------|
| §3.1 Self-Contained & Autonomous | FR-001, FR-029-036 | ✅ | All under `noa_root` |
| §3.2 Local-First & Offline-Capable | FR-002-004 | ✅ | Offline-first design |
| §3.3 Agentic Orchestration | FR-007-011 | ✅ | CECCA + agents + MAS |
| §3.4 Adaptive & Self-Improving | FR-051-075 | ✅ | Full autonomous operation |
| §3.5 Transparent & Auditable | FR-006, FR-022, FR-068 | ✅ | Complete audit trail |
| §3.6 Security & Privacy | FR-019, FR-025 | ✅ | Encrypted, user-controlled |
| §3.7 Total Memory Sovereignty | FR-005, FR-040 | ✅ | Nothing forgotten |
| §3.8 P2P Hive-Mind | FR-017-020 | ✅ | User-owned cloud |
| §3.9 Truth & Knowledge Seeking | FR-012-016, FR-070 | ✅ | Digest + self-analysis |
| §3.10 Biblical Governance | FR-025-027, FR-065-067 | ✅ | Constitutional boundary |
| §3.11 Predictive Problem Solving | FR-054, FR-070 | ✅ | Pattern analysis |
| §3.12 Test Everything | FR-057, FR-071 | ✅ | Validation + self-healing |

### Constitutional Flow

| Level | Document | Compliance |
|-------|----------|------------|
| Goals | G-NOA-001 (project-mgmt.md) | ✅ Traced |
| Policy | NOA Constitution v2.0.0 | ✅ Compliant |
| Rules | §3.1-§3.12 Core Principles | ✅ All addressed |
| Spec | spec.md (75 FRs) | ✅ Complete |
| Plan | This document | ✅ In progress |
| Tasks | tasks.md | ⏳ To generate |

---

## Architecture Overview

### 3-Plane Control Fabric Architecture (FR-056-060)

**Reference Implementation**: `E:\dev\dev\workspaces\projects\agentic-homelab-p2p`

The 3-plane system provides **A/B switching** for zero-downtime updates of knowledge, memory, code, and all system files. The third plane (Coordinator) acts as the **constant/long-term memory** with backups and archives.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     3-PLANE CONTROL FABRIC                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌───────────────────┐                         ┌───────────────────┐        │
│  │   SANDBOX PLANE   │                         │  DEPLOYED PLANE   │        │
│  │      (Blue)       │                         │     (Green)       │        │
│  │                   │                         │                   │        │
│  │  • Testing env    │  ─── A/B Switch ───▶   │  • Production     │        │
│  │  • Staging        │                         │  • Live system    │        │
│  │  • Dev builds     │  ◀─── Rollback ────    │  • Serving users  │        │
│  │  • Experimental   │                         │  • Canary traffic │        │
│  │                   │                         │                   │        │
│  │  workspaces/      │                         │  releases/        │        │
│  │  ├── capability/  │                         │  ├── v0.1.0/      │        │
│  │  └── selftest     │                         │  └── current/     │        │
│  └─────────┬─────────┘                         └─────────┬─────────┘        │
│            │                                             │                  │
│            │  artifacts                     telemetry    │                  │
│            │  ┌────────▶                  ◀──────────┐   │                  │
│            ▼  │                                      │   ▼                  │
│  ┌─────────────────────────────────────────────────────────────────┐       │
│  │                    COORDINATOR PLANE                             │       │
│  │                 (Analytics / Long-Term Memory)                   │       │
│  │                                                                  │       │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │       │
│  │  │  REGISTRY   │  │  ANALYTICS  │  │  LONG-TERM MEMORY       │  │       │
│  │  │             │  │             │  │                         │  │       │
│  │  │ • Metadata  │  │ • llama.cpp │  │ • Backups               │  │       │
│  │  │ • Versions  │  │   swarm     │  │ • Archives              │  │       │
│  │  │ • Deps      │  │ • Eval      │  │ • Knowledge persistence │  │       │
│  │  │ • History   │  │ • Decisions │  │ • Audit trails          │  │       │
│  │  └─────────────┘  └─────────────┘  └─────────────────────────┘  │       │
│  │                                                                  │       │
│  │  ┌─────────────────────────────────────────────────────────┐    │       │
│  │  │              PROMOTION POLICY ENGINE                     │    │       │
│  │  │  • Risk tiers (low/medium/high/critical)                │    │       │
│  │  │  • Canary cohorts (1%-10%)                              │    │       │
│  │  │  • Abort gates (latency, error rate, violations)        │    │       │
│  │  │  • Constitutional compliance checks                      │    │       │
│  │  └─────────────────────────────────────────────────────────┘    │       │
│  │                                                                  │       │
│  │  state/registry.db  |  logs/  |  artifacts/  |  releases/       │       │
│  └──────────────────────────────────────────────────────────────────┘       │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Plane Responsibilities

| Plane | Role | Contents | Persistence |
|-------|------|----------|-------------|
| **Sandbox (Blue)** | Testing & Staging | Dev builds, experimental changes, selftest | Ephemeral |
| **Deployed (Green)** | Production | Live system, user-facing, canary traffic | Versioned releases |
| **Coordinator** | Long-Term Memory | Registry, analytics, backups, archives, decisions | **Permanent** |

### Directory Structure per Plane

Based on reference implementation:

```
noa_root/
├── sandbox-plane/                    # Testing & Staging
│   ├── bin/                          # Entry points
│   │   └── run_sandbox.sh
│   ├── components/                   # Component specs
│   │   ├── agents/
│   │   │   ├── bin/, manifests/, policy/, spec/, tests/
│   │   ├── memory/
│   │   ├── models/
│   │   ├── orchestrator/
│   │   ├── dataplane/
│   │   ├── security/
│   │   ├── ui/
│   │   ├── update/
│   │   └── workflow/
│   ├── config/
│   │   ├── sandbox-config.yaml       # Test suites, resource budgets
│   │   ├── policy.toml               # Policy gates
│   │   └── channels.yml              # Update channels
│   ├── workspace/                    # Ephemeral capability workspaces
│   │   └── drop-in-loader/
│   │       ├── agents-rs/            # Rust agent implementations
│   │       └── micro-agent-stack/    # MicroAgentStack definitions
│   ├── logs/
│   └── tests/
│       └── ACCEPTANCE_CHECKLIST.md
│
├── deployed-plane/                   # Production (Green)
│   ├── releases/
│   │   └── agentic-homelab-p2p-0001/ # Promoted release
│   │       ├── components/           # Frozen component specs
│   │       └── docs/
│   ├── config/
│   ├── manifests/
│   └── telemetry/
│
├── coordinator-plane/                # Long-Term Memory (Constant)
│   ├── bin/
│   │   └── run_coordinator.sh
│   ├── components/                   # Reference component specs
│   │   ├── agents/
│   │   ├── memory/
│   │   ├── models/
│   │   ├── orchestrator/
│   │   └── ...
│   ├── config/
│   │   ├── analytics.yaml            # llama.cpp swarm config
│   │   ├── promotion-policy.yaml     # Promotion gates
│   │   ├── channels.yml
│   │   └── router.yml
│   ├── docs/
│   │   ├── ARCHITECTURE.mmd
│   │   ├── decision-flow.md
│   │   └── prompts/
│   │       └── three-plane-agenticos-homelab-plan.md
│   ├── policy/
│   │   └── global.yaml               # Constitutional policy
│   └── sbom/
│
└── shared/                           # Cross-Plane Shared Services
    ├── artifacts/                    # SBOM, risk assessments, telemetry
    │   └── <capability-id>/
    ├── config/
    │   ├── capability-pack-schema.yaml
    │   ├── promotion-policy.yaml     # Master promotion policy
    │   └── secrets.env
    ├── logs/
    │   ├── sandbox-plane/
    │   ├── coordinator-plane/
    │   └── deployed-plane/
    ├── releases/                     # GitOps promoted releases
    ├── runtime/
    │   └── compose.yaml              # Container orchestration
    └── state/
        └── registry.db               # Capability metadata, history
```

### Promotion Policy (Risk Tiers)

From `coordinator-plane/config/promotion-policy.yaml`:

| Tier | Required Tests | Canary Cohort | Duration | Abort Gates |
|------|---------------|---------------|----------|-------------|
| **Low** | unit, lint | 10% | 30min | p95 +20%, fail 1% |
| **Medium** | unit, integration, soak:2h | 5% | 2h | p95 +15%, fail 0.5% |
| **High** | unit, integration, soak:8h, security, constitutional | 2% | 24h | p95 +10%, fail 0.1%, constitutional 0 |
| **Critical** | All + redteam | 1% | 72h | p95 +5%, fail 0.01%, safety 0 |

**Rollback Triggers**:
- Gate breach
- Court revocation (constitutional)
- Safety event
- SLA violation

### Promotion Flow

```
┌──────────────────────────────────────────────────────────────────────────┐
│                         PROMOTION FLOW                                    │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                           │
│   SANDBOX PLANE                COORDINATOR PLANE           DEPLOYED PLANE │
│   ─────────────                ─────────────────           ────────────── │
│                                                                           │
│   1. Clone/patch               2. Ingest artifacts         6. Promote    │
│      capability                   from sandbox                release     │
│         │                            │                          │        │
│         ▼                            ▼                          ▼        │
│   ┌───────────┐              ┌───────────────┐          ┌───────────┐   │
│   │ make      │─────────────▶│ Run analytics │          │ Canary    │   │
│   │ selftest  │  artifacts   │ (llama.cpp    │          │ deploy    │   │
│   └───────────┘              │  swarm)       │          │ (1-10%)   │   │
│         │                    └───────┬───────┘          └─────┬─────┘   │
│         │                            │                        │         │
│         │                    3. Apply promotion               │         │
│         │                       policy gates                  │         │
│         │                            │                        │         │
│         │                    ┌───────▼───────┐                │         │
│         │                    │ Pass gates?   │                │         │
│         │                    └───────┬───────┘                │         │
│         │                      YES   │   NO                   │         │
│         │                    ┌───────┴───────┐                │         │
│         │                    │               │                │         │
│         │              4. Queue          Log & reject         │         │
│         │                 promotion                           │         │
│         │                    │                                │         │
│         │                    │                                │         │
│         │              5. Notify deployed plane               │         │
│         │                    │                                │         │
│         │                    └────────────────────────────────┘         │
│         │                                                     │         │
│         │                                              7. Monitor       │
│         │                                                 telemetry    │
│         │                                                     │         │
│         │                    8. Feedback loop                 │         │
│         │◀────────────────── (SLO violations → rollback) ─────┘         │
│                                                                          │
│   ┌───────────┐              ┌───────────────┐          ┌───────────┐   │
│   │ Rollback  │◀─────────────│ Coordinator   │◀─────────│ Autopilot │   │
│   │ & retry   │  decision    │ decides       │ telemetry│ rollback  │   │
│   └───────────┘              └───────────────┘          └───────────┘   │
│                                                                          │
└──────────────────────────────────────────────────────────────────────────┘
```

### Knowledge & Memory Sync Across Planes

The 3-plane system updates **not just code** but also:

| Data Type | Sandbox | Coordinator | Deployed |
|-----------|---------|-------------|----------|
| **Knowledge Base** | Testing new embeddings | Archives all versions | Uses promoted version |
| **Memory** | Ephemeral test memory | Long-term persistence | Active working memory |
| **Models** | Experimental models | Model registry | Production models |
| **Configs** | Test configurations | Master configs | Frozen configs |
| **Agents** | Dev agent definitions | Agent registry | Deployed agents |
| **Policies** | Policy experiments | Master policy (constitutional) | Enforced policy |

### Coordinator as Long-Term Memory

The Coordinator Plane is **THE CONSTANT**:

```
┌────────────────────────────────────────────────────────────────┐
│              COORDINATOR PLANE - LONG-TERM MEMORY              │
├────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    PERSISTENT STATE                      │   │
│  │                                                          │   │
│  │  state/registry.db           - Capability metadata       │   │
│  │                              - Dependency graph          │   │
│  │                              - Promotion history         │   │
│  │                              - SLO baselines             │   │
│  │                                                          │   │
│  │  artifacts/<capability>/     - SBOM for every version    │   │
│  │                              - Risk assessments          │   │
│  │                              - Telemetry snapshots       │   │
│  │                                                          │   │
│  │  logs/                       - Audit trails (append-only)│   │
│  │                              - Decision traces           │   │
│  │                              - Multi-agent transcripts   │   │
│  │                                                          │   │
│  │  releases/                   - GitOps release history    │   │
│  │                              - Signed artifacts          │   │
│  │                              - Rollback snapshots        │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    BACKUPS & ARCHIVES                    │   │
│  │                                                          │   │
│  │  • Hourly snapshots of registry.db                       │   │
│  │  • Daily full backups to noa_root/backups/               │   │
│  │  • Weekly archives to cold storage (VHDX)                │   │
│  │  • Never delete - compress and archive                   │   │
│  │                                                          │   │
│  │  RETENTION: ∞ (total memory sovereignty)                 │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### A/B Switching Mechanics

```
CURRENT STATE:                         AFTER SWITCH:

Sandbox ──▶ Testing v1.1              Sandbox ──▶ Testing v1.2

Deployed ──▶ Production v1.0          Deployed ──▶ Production v1.1 (was sandbox)

Coordinator (CONSTANT)                 Coordinator (CONSTANT)
├── registry: v1.0, v1.1              ├── registry: v1.0, v1.1, v1.2
├── artifacts: v1.0, v1.1             ├── artifacts: v1.0, v1.1, v1.2
└── releases: v1.0                    └── releases: v1.0, v1.1
```

**Key Insight**: The Coordinator never switches - it accumulates. Sandbox and Deployed swap roles during A/B transitions.

---

### Autonomous Operation Loop (FR-051-055)

```
┌─────────────────────────────────────────────────────────────────┐
│                  AUTONOMOUS CONTINUOUS LOOP                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    GOAL SOURCES                           │   │
│  │  ┌─────────────┐    ┌─────────────┐    ┌──────────────┐  │   │
│  │  │ User Goals  │    │Self-Generated│   │Constitutional│  │   │
│  │  │ (FR-053)    │    │ (FR-066-070) │   │  (FR-025-027)│  │   │
│  │  └──────┬──────┘    └──────┬───────┘   └──────┬───────┘  │   │
│  │         └──────────────────┼──────────────────┘          │   │
│  │                            ▼                              │   │
│  │              ┌─────────────────────────┐                  │   │
│  │              │  UNIFIED GOAL QUEUE     │                  │   │
│  │              │      (FR-069)           │                  │   │
│  │              └───────────┬─────────────┘                  │   │
│  └──────────────────────────┼────────────────────────────────┘   │
│                             ▼                                    │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                 EXECUTION ENGINE                          │   │
│  │                                                           │   │
│  │   ┌──────────┐   ┌──────────┐   ┌──────────┐             │   │
│  │   │Decompose │──▶│ Execute  │──▶│ Validate │             │   │
│  │   │ (FR-054) │   │ (FR-061) │   │ (FR-057) │             │   │
│  │   └──────────┘   └──────────┘   └────┬─────┘             │   │
│  │                                      │                    │   │
│  │   ┌──────────────────────────────────┼────────────────┐  │   │
│  │   │            SELF-HEALING (FR-071-075)              │  │   │
│  │   │  Detect → Diagnose → Auto-Fix → Validate → Log    │  │   │
│  │   └───────────────────────────────────────────────────┘  │   │
│  └──────────────────────────────────────────────────────────┘   │
│                             │                                    │
│                             ▼                                    │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                 OPTIMIZATION ENGINE (FR-052)              │   │
│  │  • Resource monitoring • Pattern analysis • Auto-improve  │   │
│  └──────────────────────────────────────────────────────────┘   │
│                             │                                    │
│                             └──────────────────▶ LOOP CONTINUES  │
└─────────────────────────────────────────────────────────────────┘
```

### Self-Healing Loop (FR-071-075)

```
┌─────────────────────────────────────────────────────────────────┐
│                    5-STAGE SELF-HEALING LOOP                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   ┌───────────┐    ┌───────────┐    ┌───────────┐              │
│   │  STAGE 1  │    │  STAGE 2  │    │  STAGE 3  │              │
│   │  DETECT   │───▶│ DIAGNOSE  │───▶│ AUTO-FIX  │              │
│   │           │    │           │    │           │              │
│   │• Health   │    │• Root     │    │• Restart  │              │
│   │  metrics  │    │  cause    │    │• Reconfig │              │
│   │• Anomaly  │    │  analysis │    │• Rollback │              │
│   │  detect   │    │           │    │• Redistribute│           │
│   └───────────┘    └───────────┘    └─────┬─────┘              │
│                                           │                     │
│                    ┌──────────────────────┼──────────────────┐  │
│                    │                      ▼                  │  │
│   ┌───────────┐    │              ┌───────────┐              │  │
│   │  STAGE 5  │◀───┤ if fixed    │  STAGE 4  │              │  │
│   │    LOG    │    │              │ VALIDATE  │              │  │
│   │           │    │              │           │              │  │
│   │• Audit    │    │              │• Verify   │              │  │
│   │  trail    │    │              │  fix      │              │  │
│   │• Metrics  │    │              │• No new   │              │  │
│   │  update   │    │              │  issues   │              │  │
│   └───────────┘    │              └─────┬─────┘              │  │
│                    │                    │                    │  │
│                    │   if not fixed     │                    │  │
│                    │   (attempt < 3)    │                    │  │
│                    │         ┌──────────┘                    │  │
│                    │         │                               │  │
│                    │         ▼                               │  │
│                    │  ┌─────────────┐                        │  │
│                    │  │  RETRY      │──────────────────────▶ │  │
│                    │  │  (≤3x)      │   back to STAGE 3      │  │
│                    │  └─────────────┘                        │  │
│                    │                                         │  │
│                    │  if attempt ≥ 3                         │  │
│                    │         │                               │  │
│                    │         ▼                               │  │
│                    │  ┌─────────────┐                        │  │
│                    │  │  ESCALATE   │  (User notification    │  │
│                    │  │  TO USER    │   ONLY if all else     │  │
│                    │  └─────────────┘   fails)               │  │
│                    └─────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Implementation Phases

### Phase 1: Foundation (Weeks 1-4)

**Objective**: Core infrastructure and basic autonomous loop

| Component | Files | FRs Covered |
|-----------|-------|-------------|
| Directory structure | `noa_root/*` | FR-029 to FR-036 |
| Core runtime (Rust) | `sys/core/` | FR-001, FR-002 |
| Database layer | `sys/core/src/db/` | FR-003, FR-005, FR-006 |
| Basic neural runtime | `sys/core/src/neural/` | FR-004 |
| Configuration system | `config/*.json` | FR-034 |

### Phase 2: Agent Architecture (Weeks 5-8)

**Objective**: CECCA and permanent agents operational

| Component | Files | FRs Covered |
|-----------|-------|-------------|
| CECCA orchestrator | `sys/core/src/agents/cecca/` | FR-007 |
| FileIOAgent | `sys/core/src/agents/file_io/` | FR-008 |
| TerminalAgent | `sys/core/src/agents/terminal/` | FR-008 |
| RAGAgent | `sys/core/src/agents/rag/` | FR-008 |
| MicroserviceManagementAgent | `sys/core/src/agents/microservice/` | FR-008 |
| MicroAgentStack framework | `sys/core/src/mas/` | FR-009, FR-010 |
| Constitutional enforcement | `sys/core/src/governance/` | FR-011 |

### Phase 3: Autonomous Operation (Weeks 9-12)

**Objective**: Full autonomous continuous operation

| Component | Files | FRs Covered |
|-----------|-------|-------------|
| Goal queue | `sys/core/src/autonomy/goal_queue.rs` | FR-053 |
| Continuous loop | `sys/core/src/autonomy/loop.rs` | FR-051, FR-052 |
| Goal decomposition | `sys/core/src/autonomy/decompose.rs` | FR-054 |
| Performance monitor | `sys/core/src/autonomy/metrics.rs` | FR-055 |
| Full autonomy mode | `sys/core/src/autonomy/mode.rs` | FR-061-065 |
| Goal generation | `sys/core/src/autonomy/generate.rs` | FR-066-070 |

### Phase 4: 3-Plane Control Fabric (Weeks 13-16)

**Objective**: Zero-downtime self-update capability

| Component | Files | FRs Covered |
|-----------|-------|-------------|
| Plane manager | `sys/core/src/planes/manager.rs` | FR-056 |
| Sandbox plane | `sandbox-plane/` | FR-056 |
| Deployed plane | `deployed-plane/` | FR-056 |
| Coordinator plane | `coordinator-plane/` | FR-056 |
| Promotion policy engine | `sys/core/src/planes/promotion.rs` | FR-057 |
| Instant rollback | `sys/core/src/planes/rollback.rs` | FR-058 |
| State sync via shared/ | `sys/core/src/planes/sync.rs` | FR-059 |
| Transition logging | `sys/core/src/planes/audit.rs` | FR-060 |

### Phase 5: Self-Healing (Weeks 17-20)

**Objective**: Proactive autonomous healing

| Component | Files | FRs Covered |
|-----------|-------|-------------|
| Health monitor | `sys/core/src/healing/monitor.rs` | FR-071, FR-072 |
| Diagnosis engine | `sys/core/src/healing/diagnose.rs` | FR-071 |
| Auto-fix executor | `sys/core/src/healing/fix.rs` | FR-071, FR-073 |
| Fix validation | `sys/core/src/healing/validate.rs` | FR-071 |
| Healing audit log | `sys/core/src/healing/audit.rs` | FR-074 |
| Plane-based recovery | `sys/core/src/healing/plane_swap.rs` | FR-075 |

### Phase 6: Shared Provider Memory (Weeks 21-24)

**Objective**: Multi-provider collaborative execution

| Component | Files | FRs Covered |
|-----------|-------|-------------|
| Memory bus | `sys/core/src/providers/bus.rs` | FR-037 |
| Collaborative reasoning | `sys/core/src/providers/collab.rs` | FR-038 |
| Provider registry | `sys/core/src/providers/registry.rs` | FR-039 |
| Session persistence | `sys/core/src/providers/persist.rs` | FR-040 |
| Parallel distribution | `sys/core/src/providers/parallel.rs` | FR-041 |
| State sync | `sys/core/src/providers/sync.rs` | FR-042 |

**Provider Integration**:
| Provider | Interface | Implementation |
|----------|-----------|----------------|
| llama.cpp | Local | `providers/local/llama.rs` |
| Claude Code | CLI/Cloud/IDE | `providers/cloud/claude.rs` |
| Codex | CLI/Cloud/IDE | `providers/cloud/codex.rs` |
| VS Code Copilot | IDE | `providers/ide/copilot.rs` |
| Git CLI | CLI | `providers/cli/git.rs` |
| Cursor | IDE/CLI/Cloud | `providers/ide/cursor.rs` |
| Abacus | CLI/Cloud | `providers/cloud/abacus.rs` |

### Phase 7: Digest Pipeline (Weeks 25-28)

**Objective**: Knowledge acquisition from any source

| Component | Files | FRs Covered |
|-----------|-------|-------------|
| 7-step pipeline | `sys/digest/pipeline.py` | FR-012 |
| Multi-language parsers | `sys/digest/parsers/` | FR-013 |
| SBOM generation | `sys/digest/sbom.py` | FR-014 |
| Security scanning | `sys/digest/security.py` | FR-015 |
| Output generation | `sys/digest/output.py` | FR-016 |

### Phase 8: P2P Hive-Mind (Weeks 29-32)

**Objective**: User-owned distributed compute

| Component | Files | FRs Covered |
|-----------|-------|-------------|
| P2P discovery | `p2p/discovery/` | FR-017 |
| Resource sharing | `p2p/compute/` | FR-018 |
| Encrypted sync | `p2p/sync/` | FR-019 |
| Graceful degradation | `p2p/resilience/` | FR-020 |

### Phase 9: UI & Governance (Weeks 33-36)

**Objective**: Dynamic UI and constitutional governance

| Component | Files | FRs Covered |
|-----------|-------|-------------|
| Dynamic UI | `sys/ui/` | FR-021-024 |
| Activity log | `sys/ui/components/ActivityLog.tsx` | FR-022, FR-064 |
| Constitutional engine | `sys/core/src/governance/constitutional.rs` | FR-025-028 |
| Biblical governance | `sys/core/src/governance/biblical.rs` | FR-026, FR-065, FR-067 |
| Reward/correction | `sys/core/src/governance/reward.rs` | FR-027 |

### Phase 10: Multi-GPU & Advanced (Weeks 37-40)

**Objective**: High-performance multi-GPU support

| Component | Files | FRs Covered |
|-----------|-------|-------------|
| CUDA enumeration | `sys/core/src/neural/cuda_devices.rs` | FR-047 |
| Tensor parallelism | `sys/core/src/neural/tensor_parallel.rs` | FR-048 |
| NVLink utilization | `sys/core/src/neural/nvlink.rs` | FR-049 |
| CUDA tiles | `sys/core/src/neural/cuda_tiles.rs` | FR-050 |
| Advanced learning | `sys/core/src/learning/` | FR-043-046 |

---

## Success Criteria Mapping

| SC | Target | Implementation | Phase |
|----|--------|----------------|-------|
| SC-001 | Init <60s | Optimized init script | 1 |
| SC-002 | CPU inference <2s | llama.cpp optimization | 1 |
| SC-003 | Memory recall <500ms | SQLite + indexes | 1 |
| SC-004 | 10K file digest <30min | Parallel pipeline | 7 |
| SC-005 | 200 tasks, 98% success | Agent orchestration | 2 |
| SC-006 | P2P sync <5s | Delta sync | 8 |
| SC-007 | UI reconfig <200ms | React optimization | 9 |
| SC-008 | 7-day continuous | Self-healing loop | 5 |
| SC-009 | Cross-platform identical | CI/CD matrix | 10 |
| SC-010 | 100% rollback paths | 3-plane architecture | 4 |
| SC-011 | GPU inference <500ms | CUDA optimization | 10 |
| SC-012 | Multi-GPU <300ms | Tensor parallelism | 10 |

---

## Risk Mitigation

| Risk | Impact | Mitigation |
|------|--------|------------|
| Self-generated goals cause harm | High | Constitutional governance (FR-067) |
| 3-plane state desync | High | Coordinator as source of truth (FR-059) |
| Self-healing infinite loop | Medium | Attempt limit (≥3), escalation (FR-073) |
| Full autonomy abuse | High | Constitutional boundary, audit trail |
| Multi-GPU memory overflow | Medium | Tensor parallelism, layer distribution |

---

## Research Decisions Applied

From [research.md](./research.md):

| Decision | Rationale |
|----------|-----------|
| Rust for core | Memory safety, performance, cross-platform |
| llama.cpp | No external deps, GGUF support, active community |
| SQLite + sqlite-vss | Zero config, embedded, vector search |
| libp2p | Rust-native, proven P2P stack |
| React/TypeScript UI | Ecosystem, developer experience |

---

## Next Steps

1. **Run `/tasks`** to generate implementation tasks from this plan
2. **Update `constitution.md`** to document 3-plane system
3. **Create `checklists/autonomous.md`** for autonomous operation verification
4. **Update `quickstart.md`** with 3-plane setup instructions

---

**Plan Created**: 2025-12-08
**Total FRs**: 75
**Total Phases**: 10
**Estimated Duration**: 40 weeks (2 developers)
