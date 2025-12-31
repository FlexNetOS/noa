---
title: "AGENT Instructions"
description: "Agent execution guidelines for NOA - Constitutional authority, provider routing, and operational protocols"
version: "2.0.0"
last_updated: "2025-12-22"
status: "canonical"
tags:
  - agents
  - constitution
  - providers
  - automation
  - workflows
embeddings:
  indexed: true
  priority: high
  context_window: full
metadata:
  authority: "constitutional"
  binding: true
  scope: "system-wide"
  enforcement: "mandatory"
---

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

## Implemented Agents (Phase 9 Complete)

### Core Agents (Operational)
| Agent | Type | Status | Description |
|-------|------|--------|-------------|
| **CommanderChief** | Executive | ✅ Active | Task decomposition, planning, and delegation |
| **FileIO** | Worker | ✅ Active | File operations (read, write, delete, copy) |
| **Terminal** | Worker | ✅ Active | Command execution and shell operations |
| **RAG** | Knowledge | ✅ Active | Retrieval-augmented generation with vector search |
| **ModelSelector** | Intelligence | ✅ Active | Optimal model selection based on task requirements |
| **MicroserviceManagement** | Operations | ✅ Active | Microservice lifecycle and orchestration |

### Executive Layer (Phase 9)
| Component | Status | Purpose |
|-----------|--------|---------|
| **ExecutiveAgent** | ✅ Active | Strategic decision-making |
| **BoardAgent** | ✅ Active | Multi-agent coordination |
| **MultiAgentExecutor** | ✅ Active | Parallel task execution |
| **WorkflowOrchestrator** | ✅ Active | Complex workflow management |

### Automation Services (Advanced Features)
| Service | Status | Capabilities |
|---------|--------|--------------|
| **AutomatedCodeReview** | ✅ Active | Code quality analysis, security scanning |
| **DeploymentAutomation** | ✅ Active | CI/CD pipeline management |
| **KnowledgeBaseInterrogation** | ✅ Active | RAG-powered knowledge queries |

### Agent CLI Commands
```bash
# List all available agents
noa agents list

# Get agent information
noa agents info <agent_name>

# Run an agent with a task
noa agents run <agent_name> <task>

# View agent execution logs
noa agents logs [agent_name]
```

### Workflow Commands
```bash
# Execute multi-agent workflows
noa workflow run <type>  # code-review | deployment | knowledge-query
noa workflow status <workflow_id>
noa workflow cancel <workflow_id>
```

**Implementation Directory:** `sys/core/src/agents/`
**Automation Directory:** `sys/core/src/automation/`

---

## Target Directory Structure (Goal)

This represents the **intended** architecture, not necessarily the current state:

```
noa_root/
├── ai/
│   ├── providers/
│   │   ├── llama.cpp/
│   │   ├── cursor/
│   │   ├── claude/
│   │   ├── codex/
│   │   └── copilot/
│   └── shared/
│       ├── resources/
│       │   ├── resource-registry.json
│       │   ├── resource-mapping.json
│       │   └── spec/
│       └── prompts/
├── apps/                        # First-class UI modules (versioned, pinned, sandbox-runnable)
│   ├── task-app-a/              # Example: Linear, Jira, etc.
│   │   ├── upstream/            # Vendor/original distribution (immutable, pinned)
│   │   │   ├── web/             # Static assets or bundle
│   │   │   ├── desktop/
│   │   │   │   ├── win/
│   │   │   │   ├── mac/
│   │   │   │   └── linux/
│   │   │   └── mobile/
│   │   │       ├── ios/
│   │   │       └── android/
│   │   ├── wrappers/            # NOA-specific integration glue (thin)
│   │   │   ├── connector/       # Adapter to/from Task Kernel (bidirectional sync)
│   │   │   ├── mcp/             # Optional: MCP server around the app's API
│   │   │   ├── deep-links/      # URL schemes, intents, navigation contracts
│   │   │   ├── auth/            # SSO/OAuth mapping to sys/identity
│   │   │   └── ui-embed/        # Embed adapters (webview, iframe, tauri window)
│   │   ├── profiles/            # Settings separation: logs/cache isolated
│   │   ├── config/              # App-specific config (generated; not secrets)
│   │   └── manifests/           # Pinned versions + hashes + SBOM-ish metadata
│   ├── task-app-b/              # Example: ClickUp, Notion, etc.
│   │   └── wrappers/
│   │       └── connector/       # Each app has its own adapter
│   └── task-app-c/              # Example: Asana, Monday.com, etc.
│       └── wrappers/
│           └── connector/
├── sys/
│   ├── core/                    # Rust core system
│   │   ├── src/
│   │   │   ├── agents/          # ✅ Phase 9 Complete
│   │   │   ├── automation/      # ✅ Advanced features
│   │   │   ├── cli/             # CLI commands
│   │   │   ├── healing/         # Self-healing logic
│   │   │   ├── providers/       # Provider management
│   │   │   ├── services/        # Core services
│   │   │   └── vector/          # Vector DB (Qdrant)
│   └── ui/                      # Next.js frontend (conversational command center)
│       ├── app/
│       │   ├── shell/           # Main nav + layout
│       │   ├── pages/
│       │   │   ├── convo/       # Default home (chat + widgets)
│       │   │   ├── tasks/       # Tasks Hub - unified canonical view
│       │   │   │   ├── hub/     # Main aggregated view (all apps normalized)
│       │   │   │   ├── app-a/   # Embedded App A (optional deep-link view)
│       │   │   │   ├── app-b/   # Embedded App B (optional)
│       │   │   │   └── app-c/   # Embedded App C (optional)
│       │   │   ├── runs/        # Task execution runs (logs/artifacts)
│       │   │   └── hive/        # Devices, compute, storage mesh view
│       │   └── widgets/
│       │       ├── task-summary/    # "My top tasks", "Blocked", "Agent running"
│       │       ├── kanban-mini/
│       │       └── dag-viewer/
│       └── src/
│           ├── components/
│           ├── contexts/
│           └── services/
├── gateway/
│   └── mcp/
│       └── connectors/
│           └── tasks/           # Task integration layer
│               ├── app-a/       # Connector for App A (endpoints + auth + mapping)
│               ├── app-b/       # Connector for App B
│               ├── app-c/       # Connector for App C
│               └── router/      # Authority router (which app owns what scopes)
├── orchestrator/
│   ├── task-kernel/             # Canonical task model + unification rules
│   │   ├── schema/              # TaskKernelTask schema (JSON/Proto)
│   │   │   └── task_kernel.json # Canonical internal task representation
│   │   ├── normalization/       # Canonicalization + dedupe + conflict resolution
│   │   ├── mapping/             # Per-app mapping configs (app schema → kernel schema)
│   │   └── sync/                # Ingestion + emission pipeline (bidirectional sync)
│   └── packages/
│       ├── schema/
│       │   └── task_package.json    # Task Package schema (execution-oriented)
│       └── templates/
│           └── task-sync/       # Task synchronization workflows
├── data/
│   ├── vectors/                 # Qdrant data
│   ├── models/                  # Local models
│   ├── cache/
│   │   └── apps/
│   │       ├── task-app-a/      # App-specific cache isolation
│   │       │   └── <profile-id>/
│   │       ├── task-app-b/
│   │       │   └── <profile-id>/
│   │       └── task-app-c/
│   │           └── <profile-id>/
│   └── logs/
│       └── apps/
│           ├── task-app-a/      # App-specific log isolation
│           │   └── <profile-id>/
│           ├── task-app-b/
│           │   └── <profile-id>/
│           └── task-app-c/
│               └── <profile-id>/
├── bin/                         # Executables
├── config/                      # Configuration files
└── logs/                        # System logs
```

### Task Management Architecture (Multi-App Support)

NOA supports **multiple task management apps** running concurrently, unified through a canonical **Task Kernel**. This approach keeps each app's native UX intact while providing a single source of truth for agent execution.

#### Design Philosophy

- **Conversational UI** = Command center (chat, intent, narration, approvals)
- **Task Apps** = Structured work cockpits (boards, Gantt, docs, workflows)
- **Task Kernel** = Canonical internal representation + sync layer
- **Agents/Models** = Wired through Task Kernel, not directly to apps

#### What Stays Separate

Each task app maintains:
- Its own binaries + UX (boards, Gantt, docs, etc.)
- Native data model (for now)
- App-specific workflows/features
- Isolated profiles, logs, and cache

#### What Becomes Unified (Immediately)

NOA provides:
- **TaskKernelTask** schema (canonical internal representation)
- **Sync layer** with per-app adapters (app schema ↔ kernel schema)
- **Single execution truth** (runs, artifacts, provenance in CAS)
- **Tasks Hub UI** (normalized view across all apps)

#### Integration Boundary

Task apps ↔ NOA integration happens via **Task Kernel + Connectors**:

```
Task App A/B/C → Gateway Connector → Task Kernel → Orchestrator → Agents
                      ↓                    ↓
                  Mapping Rules      Normalization
                                          ↓
                                  Task Package (execution)
                                          ↓
                              Sandbox → CAS (artifacts)
                                          ↓
                          Status/Progress → Apps + UI Widgets
```

#### Data Flow

1. **Task created/changed** in any task app
2. Gateway connector **ingests** event → maps to **TaskKernelTask**
3. Task Kernel **normalizes** + deduplicates + resolves conflicts
4. Orchestrator converts to **TaskPackage** and routes to agents/providers
5. Execution occurs in sandbox → **artifacts stored in CAS**
6. Status/progress **emitted back** to source app(s) + UI widgets

#### Authority Modes

Each app can operate in one of three modes (per project or per field):

| Mode | Description | Use Case |
|------|-------------|----------|
| **Read-only source** | Ingest tasks, never write back | Legacy apps, external integrations |
| **Bidirectional** | Two-way sync (rare at first) | Primary working app per project |
| **Kernel-authoritative** | Tasks created in NOA, app is view | Advanced: NOA-native task creation |

**Practical rollout:**
1. Start with **read-only ingestion** from all apps
2. Pick *one* app for **bidirectional** sync per project
3. Gradually migrate to **kernel-authoritative** as adoption grows

#### TaskKernelTask Schema

The canonical internal representation includes:

- `id` (stable, internal UUID)
- `source` (app identifier + native task ID)
- `title`, `description`
- `state` (canonical enum: `planned` | `running` | `succeeded` | `failed`)
- `priority` (normalized scale)
- `tags[]`
- `project`, `milestone`
- `assignee(s)`
- `deps[]` (task DAG for dependencies)
- `artifacts[]` (CAS refs to execution outputs)
- `runs[]` (execution run history)
- `policy` (required capabilities, sandbox profile)
- `metadata` (app-specific fields, conflict resolution hints)

#### Adapter Contract

Each app connector implements three core functions:

```rust
trait TaskAppAdapter {
    // Ingest app event → canonical kernel task
    fn ingest(&self, app_event: AppEvent) -> Result<TaskKernelTask>;
    
    // Emit kernel task → app mutation (optional, for write-back)
    fn emit(&self, task: &TaskKernelTask) -> Result<AppMutation>;
    
    // Resolve conflicts when multiple apps have divergent state
    fn reconcile(&self, conflicts: Vec<TaskConflict>) -> Result<TaskKernelTask>;
}
```

#### Normalization & Deduplication

The Task Kernel normalization layer handles:

- **Status mapping**: Deterministic conversion (e.g., `todo` → `planned`, `done` → `succeeded`)
- **Deduplication**: Via `(normalized_title + project + assignee + time_window)` + optional fuzzy match
- **Conflict resolution**: Keep all originals as provenance, mark "merged into" with references
- **Never discard**: All data retained; CAS provides immutable anchors

#### UI: Tasks Hub

The conversational UI provides a unified Tasks Hub:

- **`/tasks`** = Canonical hub (aggregated view, normalized across all apps)
  - Filters: source app, owner, project, hive scope (personal/regional/org)
  - Agent run status: planned/running/failed/succeeded
  - Deep links: "Open in App X" buttons
- **`/tasks/app-a`** = Embedded App A view (optional)
- **`/tasks/app-b`** = Embedded App B view (optional)
- **`/tasks/app-c`** = Embedded App C view (optional)

#### Truth Distribution

- **Task Apps** = Human-visible truth (boards, priorities, owners, native UX)
- **Task Kernel** = Canonical internal state (normalized, deduplicated)
- **NOA Orchestrator** = Execution truth (runs, artifacts, provenance, logs)

#### Agent Integration

**Agents NEVER talk directly to task apps.** Instead:

- Agents query **Task Kernel** (what to do)
- Agents use **Orchestrator** (how to do it)
- Agents invoke **MCP tools** (do it)
- **Task Kernel emits status** back through connectors to apps

This keeps the agent loop stable even when switching or adding task apps.

#### Isolation Policy

Each app maintains strict isolation:

- **Logs**: `data/logs/apps/task-app-{a,b,c}/<profile-id>/`
- **Cache**: `data/cache/apps/task-app-{a,b,c}/<profile-id>/`
- **Config**: `apps/task-app-{a,b,c}/config/` (generated; not secrets)
- **Binaries**: `apps/task-app-{a,b,c}/upstream/<platform>/`

#### Deep Linking Format

- **Canonical**: `noa://tasks/<kernel-task-id>`
- **App-specific**: `noa://tasks/app-a/<native-task-id>`

#### Status Mapping Examples

| Task App State | NOA Kernel State | Notes |
|----------------|------------------|-------|
| `todo`, `backlog` | `planned` | Not yet started |
| `in-progress`, `active` | `running` | Agent/human working |
| `done`, `closed`, `resolved` | `succeeded` | Completed successfully |
| `cancelled`, `blocked`, `failed` | `failed` | Did not complete |

#### Migration Strategy

**Phase 1: Read-only ingestion** (Current)
- Ingest tasks from all apps into Task Kernel
- No write-back to apps
- Agents execute based on kernel state

**Phase 2: Selective bidirectional** (Next)
- Pick one app per project for bidirectional sync
- Status updates flow back to that app
- Other apps remain read-only

**Phase 3: Kernel-authoritative** (Future)
- Tasks created directly in NOA
- Apps become views/editors
- Full agent-driven task lifecycle

#### When to Unify/Refactor

Unify apps into a single native implementation only when:
- 80% of tasks flow through the canonical hub
- Stable mappings and authority rules proven
- One app is clearly redundant
- ROI is obvious (cost of maintaining adapters > cost of custom app)

Until then, **keep apps separate** and let the Task Kernel handle integration.

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

## Provider Integration Points

All providers must reference this file as the canonical source for:
- **Priority and routing logic** (see Provider Priority table above)
- **Fallback strategies**
- **Resource sharing policies** (§3.13)
- **Constitutional compliance requirements**

### Provider Configuration Files
Each provider should contain a reference link:
```markdown
<!-- See AGENT.md for canonical provider routing and policies -->
[AGENT.md](../../AGENT.md)
```

**Location:** `$NOA_AI_PROVIDERS/<provider>/README.md`

### Shared Resources
All shared resources (prompts, templates, schemas) are catalogued in:
- `$NOA_AI_SHARED/resources/resource-registry.json`
- `$NOA_AI_SHARED/resources/resource-mapping.json`

**Implementation:** `sys/core/src/providers/mod.rs` must match the priority table in this file.

---
