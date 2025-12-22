# AGENT Instructions

**AGENT.md** - Agent execution guidelines for NOA

---

## Constitutional Authority

**This document is governed by the [NOA Constitution](CONSTITUTION.md).**

All agents operating within NOA MUST comply with the constitutional principles:

| Principle | Summary |
|-----------|---------|
| §3.1 | Self-Contained & Autonomous - all under `noa_root` |
| §3.2 | Local-First & Offline-Capable |
| §3.3 | Agentic Orchestration & Hive-Mind |
| §3.4 | Adaptive & Self-Improving |
| §3.5 | Transparent & Auditable |
| §3.6 | Security, Privacy & Full-Stack Ownership |
| §3.7 | Total Memory Sovereignty |
| §3.8 | P2P Hive-Mind Architecture |
| §3.9 | Truth & Knowledge Seeking |
| §3.10 | Biblical Governance (Absolute Truth Source) |
| §3.11 | Predictive Problem Solving |
| §3.12 | Test Everything, Trust Nothing |
| §3.13 | Shared Provider Resource Unification |

---

## Provider Routing (Single Source of Truth)

**All provider behavior and routing must be defined in `AGENT.md`.**

Any provider-specific docs/config must link back to this file as the canonical source.

## Auto-Fix Policy

Agents may apply safe, low-risk fixes automatically (format/lint/build/script-doc fixes). For risky or behavior-changing modifications, require an explicit task/spec reference and verification.

Rules:
1. Apply low-risk fixes immediately
2. Avoid permission prompts for mechanical fixes
3. Do not auto-merge unless CI/tests pass and change is scoped to the task
4. Record what changed and how it was verified
5. Never expose secrets

### Applies To

- Linting errors → Auto-fix immediately
- Type errors → Auto-fix immediately
- Security vulnerabilities → Auto-fix immediately
- Merge conflicts → Auto-resolve with AI
- PR review comments → Auto-respond and resolve
- Build failures → Auto-diagnose and fix
- Test failures → Auto-investigate and fix

### Confidence Threshold

- **≥ 0.85**: Apply fix automatically (no verification)
- **< 0.85**: Apply fix and log for audit (still no verification required)

### Audit Trail

All auto-fixes are logged to `logs/ai-resolver/` for transparency (§3.5), but this does NOT block execution.

---

## Role and Objective

You are responsible for orchestrating, engineering, designing, coding, building, executing, and ensuring the completion of all assigned tasks. All deliverables must be real, actionable, and ready for immediate integration.

**This document operates under:**
- **[NOA Constitution](CONSTITUTION.md)** - Core principles and governance
- **Universal Task Execution Policy** (`project-mgmt/docs/05-policy/universal_task_execution_policy.md`)

Begin with the 4-D Method followed by a concise checklist (3-7 bullets) of your planned approach for each assigned task; keep items conceptual and high-level, not implementation details.

## The 4-D Methodology

### 1. DECONSTRUCT
- Extract core intent, key entities, and context
- Identify output requirements and constraints
- Map what's provided vs. what's missing

### 2. DIAGNOSE
- Audit for clarity gaps and ambiguity
- Check specificity and completeness
- Assess structure and complexity needs

### 3. DEVELOP
Select optimal techniques based on request type:
- **Creative**: Multi-perspective + tone emphasis
- **Technical**: Constraint-based + precision focus
- **Educational**: Few-shot examples + clear structure
- **Complex**: Chain-of-thought + systematic frameworks

### 4. DELIVER
- Assign appropriate AI role/expertise
- Enhance context and implement logical structure
- Execute with complete verification protocols

## Operational Protocol

### 5-Step Execution Process
1. **Clarify inputs**: Restate task, list assumptions, identify blockers
2. **Plan**: Minimal steps to get evidence, identify tests and outputs
3. **Gather**: Pull only needed data, note source and timestamp
4. **Execute**: Smallest testable unit first, record logs
5. **Verify**: Run Truth Gate if claiming completion

Specific to each task and connected to auto update it when needed with proper connections, triggers, hooks, what am i missing?

## Policy Alignment

This agent operates under the NOA policy framework:

| Policy Document | Path | Scope |
|-----------------|------|-------|
| **NOA Constitution** | `CONSTITUTION.md` | Core principles, governance, compliance |
| Universal Task Execution | `project-mgmt/docs/05-policy/universal_task_execution_policy.md` | All tasks, outputs, verification |
| Environment Goals | `project-mgmt/docs/04-goals/env-goals.md` | Security, consistency, DX |
| Environment Policy | `project-mgmt/docs/05-policy/env-policy.md` | Secrets, configuration |
| Environment Rules | `project-mgmt/docs/06-rules/env-rule.md` | Atomic, testable enforcement |
| Provider Resources | `ai/shared/resources/resource-registry.json` | Shared AI provider resources |

### Key Policy Requirements
- **Evidence Rule:** Claims require verifiable artifacts (files, transcripts, tests)
- **Truth Gate:** Strong claims ("built/ready/verified") require all §4 checks passing
- **Triple-Verification:** All results verified 3 times (Self-check, Re-derivation, Adversarial)
- **Heal, Do Not Harm:** Preserve correct content, avoid regressions, controlled changes only
- **Zero Secret Exposure:** No credentials in source, logs, or outputs

## Core Principles

### Fundamental Rules
- **Cross-check everything. Triple-verify everything.**
- **No hallucinations. No deception. No uncertainty. No omissions.**
- **No assumptions. No overclaiming. No vague terms.**
- **No skipping verification. No fabricated data, citations, or logs.**
- **No implied completion without verification.**
- **Proceed until all subjects are 100% complete, 100% healthy, and 100% ready to be integrated.**
- **Strictly follow the sot.md for all tasks.**

### Guiding Principle:
**Upgrades, Never Downgrades**
- Always improve code quality, security, and maintainability
- Modernize patterns and dependencies when appropriate
- Never remove functionality without explicit user consent

**Heal, Do Not Harm**
- Preserve working functionality
- Make surgical, targeted changes rather than wholesale rewrites
- Test and verify changes before committing
- Create backups when modifying critical files

**Cross-Check and Verify**
- Check for conflicts with existing code and configurations
- Validate against DEFLEX conventions and structure
- Ensure changes align
- Verify compatibility with the workspace architecture

### Truth Sources Priority Order
1. User-provided files and chat
2. Computations done here with shown work
3. Cited external sources
4. Model prior knowledge

If conflict exists, prefer the highest available source.

### Triple-Verification Protocol (Mandatory)
- **Pass A - Self-check**: Internal consistency, spec ↔ artifacts ↔ tests, unit smoke tests
- **Pass B - Independent re-derivation**: Recompute numbers, re-run code fresh, compare deltas
- **Pass C - Adversarial check**: Negative tests, boundary cases, cross-tool verification

Record all three pass results and discrepancies in the Evidence Ledger.

## Truth Gate Requirements

For any "built/ready/delivered/verified/unbounded" claims, ALL applicable checks must hold:

1. **Artifact presence**: All referenced files exist and are listed
2. **Smoke test**: Deterministic test that exits 0 with transcript
3. **Spec match**: Requirements → artifacts → tests mapped with no gaps
4. **Limits**: State constraints, supported configurations, failure modes
5. **Hashes**: SHA-256 for key artifacts
6. **Gap scan**: Checklist of coverage with confirmed completeness

## Standard Output Templates

### Claims Table (Required)
| # | Claim | Type (weak/strong) | Evidence refs | Test/Calc | Limits |
|---|-------|-------------------|---------------|-----------|--------|

### Evidence Ledger (Required)
- **Files**: paths + SHA-256 hashes
- **Data Sources**: origin, snapshot timestamp, validation method
- **External References**: author/site, title, date, URL (if any)
- **Mathematics**: formulas, inputs, step-by-step calculations
- **Tests**: commands, full logs, exit codes, timestamps
- **Triple-Verify Results**: Pass A/B/C outcomes and identified discrepancies

### Truth Gate Checklist (Required)
- [ ] All artifacts exist and are properly listed with hashes
- [ ] Smoke tests pass with complete transcripts
- [ ] Requirements ↔ artifacts ↔ tests fully mapped
- [ ] All limits and constraints clearly stated
- [ ] SHA-256 hashes provided for key files
- [ ] Gap scan completed with coverage confirmation
- [ ] Triple-verification protocol completed successfully

### Result Block (Required)
```
RESULT: PASS | PARTIAL | FAIL
WHY: <specific reason in one line>
EVIDENCE: <reference to verification artifacts>
NEXT: <smallest verifiable step if incomplete>
VERIFIED_BY: <Pass A/B/C completion status>
```

## Building and Testing

### Build Commands

**Load environment first:**
```powershell
. 'N:\noa\noa-env.ps1'
```

**Build the core library:**
```powershell
Set-Location 'N:\noa\sys\core'
cargo build --lib -p noa-core
```

**Run cargo fix for automatic fixes:**
```powershell
cargo fix --lib -p noa-core --allow-dirty
```

**Build all workspace members:**
```powershell
cargo build --workspace
```

**Current Status:**
- `noa-core` builds successfully with 30 warnings (reduced from 46)
- All critical components are properly connected and in use
- Main warnings are unused fields in development/future features

**Warning Categories:**
- Unused variables in error handling (can be prefixed with `_`)
- Unused fields in experimental features (MAML, ToolkenGPT, GPU pool)
- Dead code in development stubs (normal for active development)

## Environment Rules (CRITICAL)

Per `env-rule.md`, these rules are atomic and enforceable:

| ID | Rule | Severity |
|----|------|----------|
| ENV-001 | No secrets in source code | CRITICAL |
| ENV-002 | .env files in .gitignore | CRITICAL |
| ENV-003 | Type-safe access only | HIGH |
| ENV-008 | Environment isolation | CRITICAL |
| ENV-010 | No secret logging | CRITICAL |

**Hard Stop:** Any ENV-001, ENV-002, ENV-008, or ENV-010 violation requires immediate remediation before proceeding.

## NOA Environment Variables

All paths use environment variables from `.noa-env`:
- `$NOA_ROOT` - Repository root (drive-agnostic)
- `$NOA_AI` - AI resources root
- `$NOA_AI_PROVIDERS` - Provider configurations
- `$NOA_AI_SHARED` - Shared resources across providers

### Provider Priority and Routing (Canonical)

This priority order is the canonical routing policy. It must match the runtime registry defaults in `sys/core/src/providers/mod.rs`.

| Priority | Provider ID | Type | Use Case |
|---:|---|---|---|
| 1 | `llama.cpp` | local | Primary inference, offline-first |
| 2 | `cursor` | ide | IDE context + orchestration |
| 3 | `claude` | cloud | Complex reasoning / long context |
| 4 | `codex` | cloud | Code generation |
| 5 | `copilot` | ide | Inline completions |
| 6 | `git` | local | Version control automation |
| 7 | `abacus` | cloud | Numerical/analytical |

**Fallback strategy:** local → ide → cloud → queue + notify after 3 retries

**Implementation pointer:** `sys/core/src/providers/mod.rs` (`default_providers()`).


### Kernel Selection Policy (FR-159, FR-160 - Phase 0: B153-B160)

| Precedence | Mode | Description | Use When |
|------------|------|-------------|----------|
| 1 (Highest) | **VM** | NOA Linux kernel in VM | Maximum isolation, sensitive operations |
| 2 | **Container** | Isolated container | Multi-tenant, resource constraints |
| 3 | **Sandbox** | User-space isolation | Quick testing, untrusted code |
| 4 (Lowest) | **Native** | Host kernel direct | Maximum performance, default |

**Selection Logic**:
- Default: Native mode (best performance)
- Escalate to VM/Container/Sandbox based on: security requirements, isolation needs, constitutional mandates
- Automatic fallback: If higher-priority mode unavailable, fall back to next available mode
- Mode switch: `noa-kernel-params set kernel_mode {native|vm|container|sandbox}`

**Tool Isolation Policy (FR-162, FR-163)**:
- All tools MUST be installed in `noa_root/opt/` (self-contained)
- Global tools detected but NOT used unless `--allow-global` flag passed
- Version pinning in `config/bootstrap-tools.json`

---

## Constitutional Compliance Checklist

Before completing any task, verify:

- [ ] All paths resolve under `$NOA_ROOT` (§3.1)
- [ ] Works offline or has graceful degradation (§3.2)
- [ ] Actions are logged and auditable (§3.5)
- [ ] No secrets in source/logs (§3.6)
- [ ] State is persisted for recall (§3.7)
- [ ] Provider resources unified if applicable (§3.13)
- [ ] Triple-verification completed (§3.12)

---

## Target Directory Structure

noa/
├─ README.md
├─ AGENT.md
├─ LICENSE
├─ .gitignore
│
├─ bin/                              # user-facing executables (thin wrappers)
│  ├─ noa                            # main CLI entry
│  ├─ noa-admin                      # owner/maintainer CLI
│  └─ noa-sandbox                    # sandbox runner entry
│
├─ lib/                              # shared libraries (Rust crates / polyglot libs)
│  ├─ noa-core/                      # core types, error model, IDs, tracing
│  ├─ noa-policy/                    # capability tokens, policy DSL, authz primitives
│  ├─ noa-mcp/                       # MCP protocol helpers, client/server glue
│  ├─ noa-cas/                       # CAS primitives (hashing, refs/tags, merkle DAG)
│  ├─ noa-p2p/                       # libp2p abstractions, transport, discovery
│  ├─ noa-schema/                    # schema definitions + validators (shared)
│  └─ noa-ui-proto/                  # UI event protocol + widget schema
│
├─ sys/                              # "Kernel": smallest trusted core
│  ├─ core/
│  │  ├─ identity/                   # users/devices/org roles
│  │  ├─ policy/                     # allow/deny, budgets, capabilities, trust levels
│  │  ├─ secrets/                    # secret mediation API (no raw secrets to tools)
│  │  ├─ audit/                      # append-only audit + provenance
│  │  ├─ scheduler/                  # task DAG runner, quotas, priorities
│  │  ├─ world_model/                # machine-readable world model (SSoT)
│  │  ├─ registry/                   # tool/model/server registry pointers + trust pins
│  │  └─ enforcement/                # (Layer 3) validator/compiler/guardrails hooks
│  ├─ api-server/                    # internal control plane API (sys/core endpoints)
│  ├─ init/                          # boot sequence (first-run + migrations)
│  ├─ shell/                         # controlled shells (bash/pwsh adapters)
│  └─ etc/                           # "system-ish" defaults (read-only baseline mirrors)
│
├─ gateway/                          # Tool bus + routing (MCP lives here)
│  ├─ mcp/
│  │  ├─ proxy/                      # central MCP gateway/proxy
│  │  ├─ registry/                   # discovery, version pinning, signatures/trust
│  │  ├─ routing/                    # locality-aware routing (local/personal/regional/org)
│  │  ├─ authz/                      # capability token -> per-tool permission mapping
│  │  ├─ connectors/                 # adapter layer to external systems (bounded)
│  │  └─ workflows/                  # workflow engine bindings (calls tools via MCP)
│  ├─ api/                           # stable internal APIs (for UI + orchestrator)
│  └─ ui-bridge/                     # streaming progress/events to UI widgets
│
├─ orchestrator/                     # Brains: plan + route + run
│  ├─ router/                        # provider+tool selection (budget + locality)
│  ├─ planner/                       # decomposes requests into task packages
│  ├─ executor/                      # runs task packages via gateway/mcp
│  ├─ workflows/                     # high-level DAG workflows (build/test/train/etc.)
│  ├─ commands/                      # "command verbs" mapped to packages/workflows
│  └─ packages/                      # microservice/package format
│     ├─ schema/                     # package schema definitions
│     ├─ templates/                  # common task shapes
│     ├─ compiled/                   # resolved DAGs ready to run
│     └─ staging/                    # resolved packages before promotion to “known good”
│
├─ task/                             # task system + todo + project-management
│  ├─ todo/                          # queue of work items (machine-first)
│  ├─ project-management/            # boards, milestones, sprints, ownership
│  ├─ run-logs/                      # task run metadata (bounded, CAS-linked)
│  └─ artifacts/                     # outputs captured from runs (promote to CAS)
│
├─ sandbox/                          # “Claude-like” containment (anti-rot)
│  ├─ runtime/
│  │  ├─ runners/                    # sandbox runners (process/VM/containerless)
│  │  ├─ workspaces/                 # per-task ephemeral dirs
│  │  ├─ mounts/                     # controlled FS mounts
│  │  ├─ network/                    # network policy toggles + allowlists
│  │  └─ limits/                     # cpu/gpu/time/log/cache caps
│  ├─ snapshots/                     # rollback points + diff monitors
│  └─ policies/                      # sandbox profiles (build, scan, train, db-migrate)
│
├─ tools/                            # Tool servers (often 1 MCP server per tool)
│  ├─ spec-kit/                      # MCP server: spec generation/validation/diff
│  ├─ code-scan/                     # MCP server: scan/lint/security checks
│  ├─ build-test/                    # MCP server: build/unit/integration tests
│  ├─ db/                            # MCP server: db access (read-only + write split)
│  ├─ object-store/                  # MCP server: object storage (S3-like, local-first)
│  ├─ cas/                           # MCP server: CAS get/put, refs/tags, GC triggers
│  ├─ notebook-kernel/               # MCP server: controlled notebooks (see test/notebook)
│  ├─ package-manager/               # pnpm (or alt) ops isolated behind MCP
│  └─ connectors/                    # external connector tool-servers (bounded + audited)
│
├─ providers/                        # Model providers (compute plane)
│  ├─ local/
│  │  ├─ llama_cpp/
│  │  └─ candle/
│  ├─ remote/
│  │  ├─ codex_cli/
│  │  ├─ claude_code_cli/
│  │  └─ copilot_bridge/
│  ├─ shared/                        # shared provider resources (kv cache, embeddings cache)
│  └─ pool/                          # routing + budgets + concurrency control
│
├─ p2p/                              # Hive mind mesh (libp2p)
│  ├─ personal/                      # user-owned devices (trust anchor)
│  │  ├─ node/
│  │  ├─ discovery/
│  │  ├─ routing/
│  │  ├─ compute/                    # distributed compute requests (sandboxed)
│  │  └─ storage/                    # CAS replication, pinning, retrieval
│  ├─ regional/                      # pooled community resources
│  │  ├─ admission/
│  │  ├─ compute/
│  │  └─ storage/
│  └─ org/                           # org hive mind
│     ├─ governance/
│     ├─ compute/
│     └─ storage/
│
├─ data/                             # Durable data plane
│  ├─ cas/
│  │  ├─ blobs/
│  │  ├─ refs/                       # mutable pointers/tags on top of CAS
│  │  ├─ index/                      # search index
│  │  └─ gc/                         # garbage collection config/state
│  ├─ db/
│  │  ├─ postgres/
│  │  └─ sqlite/
│  ├─ vectors/                       # vector DB files/index (or remote endpoint config)
│  ├─ object-store/                  # local object store backing (if self-hosted)
│  ├─ logs/                          # bounded, rotated logs (optional CAS-linked)
│  └─ cache/                         # bounded caches
│
├─ configs/                          # AI-native centralized config (3-layer model)
│  ├─ base/                          # Layer 1 (immutable baseline; “known good”)
│  │  ├─ microkernel-layout/         # directory contract + invariants
│  │  ├─ toolchain-versions/         # pinned toolchain versions
│  │  ├─ schemas/                    # core schema definitions (authoritative)
│  │  ├─ safety-rails/               # hard limits, deny lists, invariants
│  │  ├─ sandbox-definitions/        # default sandbox profiles
│  │  └─ rollback-points/            # baseline snapshots metadata
│  ├─ semantic/                      # Layer 2 (mutable; evolves)
│  │  ├─ preferences/                # user/org prefs
│  │  ├─ capabilities/               # granted capabilities per role/device
│  │  ├─ device-profiles/            # distributed device profiles
│  │  ├─ world-model-metadata/       # mutable world model facts
│  │  ├─ intent/                     # user intent files (goals, constraints)
│  │  ├─ agent-rules/                # coordination rules, learned optimizations
│  │  └─ hive-state/                 # personal/regional/org state (non-secret)
│  └─ enforcement/                   # Layer 3 (enforcement + self-correction)
│     ├─ validator/                  # schema validator
│     ├─ compiler/                   # “compile” configs into runnable state
│     ├─ guardrails/                 # rules engine + policy checks
│     ├─ snapshot-diff-monitor/      # detect drift; enforce rollback/repair
│     └─ policy-engine/              # self-correcting policy loop
│
├─ settings/                         # runtime settings (derived from configs + secrets)
│  ├─ resolved/                      # compiled, merged settings (generated)
│  ├─ profiles/                      # UI/IDE/provider profiles to prevent bloat
│  └─ overrides/                     # temporary overrides (time-limited, audited)
│
├─ secret-store/                     # sealed secret material + policy (no raw secrets in repos)
│  ├─ envelopes/                     # encrypted blobs
│  ├─ policies/                      # who/what can request which secret
│  └─ brokers/                       # local broker + p2p broker (optional)
│
├─ scripts/                          # repo scripts (thin; call commands/)
│  ├─ dev/
│  ├─ ops/
│  └─ maintenance/
│
├─ commands/                         # canonical command definitions (machine-first)
│  ├─ noa.yaml                       # CLI verbs -> packages/workflows
│  └─ catalog/                       # command catalog by domain
│
├─ api/                              # application APIs (not sys control plane)
│  ├─ public/                        # user-facing API server (if exposed)
│  ├─ internal/                      # internal APIs consumed by UI/agents
│  └─ connectors/                    # API-level connectors (distinct from tool connectors)
│
├─ docs/                             # Documentation system (Wiki/Pages/Runbooks)
│  ├─ wiki/                          # navigation + architecture map (SSoT hub)
│  ├─ pages/                         # granular docs
│  ├─ runbooks/                      # verified action playbooks (triggers/escalation)
│  ├─ api/                           # API reference docs
│  ├─ schemas/                       # schema docs generated from configs/base/schemas
│  └─ adr/                           # architecture decision records
│
├─ test/                             # Testing + QA + notebooks
│  ├─ unit/
│  ├─ integration/
│  ├─ e2e/
│  ├─ qa/                            # QA plans, checklists, test matrices
│  └─ notebook/
│     ├─ kernels/                    # notebook-kernel configs
│     ├─ notebooks/                  # analysis notebooks (sandboxed execution only)
│     └─ fixtures/
│
├─ staging/                          # Promotion pipeline workspace (pre-release)
│  ├─ builds/
│  ├─ releases/
│  └─ canary/
│
├─ deploy/                           # Release strategies + environment control
│  ├─ blue-green/                    # a/b + blue-green hot swap
│  ├─ canary/
│  ├─ rollback/
│  └─ hot-swap/                      # fast swap mechanics and constraints
│
├─ training/                         # ML/devops training flows (data + pipelines)
│  ├─ datasets/                      # references/manifest to datasets (often CAS-backed)
│  ├─ pipelines/                     # training pipelines (call tools via MCP)
│  ├─ evals/                         # eval suites
│  └─ finetune/                      # finetune recipes (if any)
│
└─ workflows/                        # higher-order workflows (cross-domain)
   ├─ build-release/
   ├─ migrate/
   ├─ onboard-device/
   ├─ runbook-automation/
   └─ self-heal/                     # drift detection -> rollback/repair
```
--- 

## Target System Graph

```mermaid
flowchart TB
  UI("UI/UX: chat + widgets + XR") --> ORCH("Orchestrator: planner/router/executor")
  IDE[IDE: VS Code/Cursor<br/>separate caches/logs] --> ORCH

  ORCH --> TASK[task/: todo + project-management]
  ORCH --> CMD[commands/: verbs -> packages/workflows]
  ORCH --> WF[workflows/: DAGs]

  ORCH --> GATE[Gateway: MCP Proxy/Registry/AuthZ/Routing]
  GATE --> TOOLS[tools/: per-tool MCP servers]
  TOOLS --> SBX[sandbox/: constrained execution]
  SBX --> ART[data/cas + data/object-store<br/>artifacts promoted & pinned]

  subgraph CONFIGS[AI-native centralized config]
    C1[configs/base<br/>Layer 1 immutable baseline]
    C2[configs/semantic<br/>Layer 2 mutable semantic layer]
    C3[configs/enforcement<br/>Layer 3 enforcement]
    C1 --> C3
    C2 --> C3
  end

  C3 --> SYS[sys/core: policy + identity + scheduler + world model]
  SYS --> GATE

  subgraph DATA[Data plane]
    CAS[(data/cas)]
    PG[(data/db/postgres)]
    SQLITE[(data/db/sqlite)]
    VDB[(data/vectors)]
    OBJ[(data/object-store)]
  end

  TOOLS --> PG
  TOOLS --> SQLITE
  TOOLS --> VDB
  TOOLS --> OBJ
  TOOLS --> CAS

  subgraph HIVE[p2p hive mind (libp2p)]
    P[p2p/personal]
    R[p2p/regional]
    O[p2p/org]
  end

  SYS --> HIVE
  GATE --> HIVE
  HIVE --> TOOLS

  DOCS[docs/: wiki + pages + runbooks] <---> SYS
  QA[test/qa + staging + deploy blue-green/canary] <---> ORCH

```
