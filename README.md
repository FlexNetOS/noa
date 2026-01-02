# NOA Seed Foundation - Full Stack Application

## 🚀 System Status

**OPERATIONAL** - Full stack application running successfully!

### Supported Platforms

- **Windows**: Windows 10/11 (build 19041+) or Windows Server 2019+
- **Linux**: Ubuntu 20.04+, Debian 11+, or RHEL 8+
- **macOS**: macOS 11.0+ (Big Sur or later)
- **WSL**: WSL2 on Windows (Ubuntu 20.04+ recommended)

### Running Services

- ✅ **API Server**: <http://localhost:3001>
- ✅ **UI Dashboard**: <http://localhost:3000>
- ✅ **Rust Core**: 6 crates compiled and operational
- ✅ **Database Schema**: Defined (SQLite ready)
- ⏳ **Neural Runtime**: Pending llama.cpp integration
- ⏳ **Agent Orchestrator**: Framework ready, needs activation
- ⏳ **P2P Network**: Pending libp2p integration

## 📦 Architecture

### Backend (Rust)

Located in `sys/core/`, the backend consists of 6 crates:

1. **noa-api** - REST API server with Axum
   - Health checks: `/health`
   - System status: `/api/v1/status`
   - Task management: `/api/v1/tasks` (GET, POST)

2. **noa-common** - Shared types and utilities
   - Entity IDs, timestamps
   - Agent types and states
   - Knowledge graph types

3. **noa-embedder** - Vector embedding service
   - FastEmbed integration (pending)
   - Text-to-vector conversion

4. **noa-trainer** - Model training pipeline
   - Fine-tuning capabilities

5. **noa-indexer** - Repository indexing
   - Code analysis and digestion

6. **noa-agent** - Autonomous agent system
   - CECCA orchestrator
   - MicroAgentStack framework
   - Permanent and disposable agents

### Frontend (Next.js + React)

Located in `sys/ui/`, the UI provides:

- **Real-time System Monitoring**
  - Component status dashboard
  - Health indicators
  - Version tracking

- **Chat Interface**
  - Natural language task creation
  - Real-time API integration
  - Message history

- **Modern Design**
  - Gradient backgrounds
  - Glassmorphism effects
  - Responsive layout

## 🛠️ Development

### Starting the Servers

**API Server (Terminal 1):**

```powershell
cd sys/core
cargo run --bin noa-api
```

**UI Server (Terminal 2):**

```powershell
cd sys/ui
npm run dev
```

### Building

**Rust Backend:**

```powershell
cd sys/core
cargo build --release
```

**Next.js UI:**

```powershell
cd sys/ui
npm run build
```

## 🧪 Testing

### Unit Tests

```bash
# All unit tests
cargo test

# Specific crate tests
cargo test -p noa-api-client    # API client
cargo test -p noa-ui-shell      # UI shell components
cargo test -p noa-ui-hived      # Daemon
```

### Integration Tests

```bash
# Sandbox capsule validation
cargo test --test sandbox_integration_test

# E2E tests (requires running services)
cargo test --test ui_e2e_test -- --ignored
```

### Test Files

| Location | Description |
|----------|-------------|
| `gateway/api/client/rust/src/tests.rs` | API client unit tests |
| `ui/app/crates/noa-ui-shell/src/tests.rs` | UI component tests |
| `ui/app/bins/noa-ui-hived/src/tests.rs` | Daemon tests |
| `tests/sandbox_integration_test.rs` | Agent sandbox tests |
| `tests/ui_e2e_test.rs` | End-to-end UI tests |

See [tests/README.md](tests/README.md) for full documentation.

### API Health Check

```powershell
Invoke-WebRequest -Uri http://localhost:3001/health -UseBasicParsing
```

**System Status:**

```powershell
Invoke-WebRequest -Uri http://localhost:3001/api/v1/status -UseBasicParsing
```

**Create Task:**

```powershell
Invoke-WebRequest -Uri http://localhost:3001/api/v1/tasks `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"description":"Test task","priority":"normal"}' `
  -UseBasicParsing
```

## 📊 Database Schema

The system uses SQLite with the following tables:

- **knowledge_nodes** - Code entities and concepts
- **knowledge_edges** - Relationships between nodes
- **embeddings** - Vector representations
- **tasks** - User tasks and objectives
- **agents** - Active agent instances

## 🎯 Next Steps

1. **Integrate llama.cpp** for local LLM inference
2. **Activate database** with proper file permissions
3. **Implement embedder** with FastEmbed
4. **Build agent orchestrator** with CECCA
5. **Add P2P networking** with libp2p
6. **Create desktop app** with NDCL

## 📝 API Endpoints

### Health & Status

- `GET /health` - Server health check
- `GET /api/v1/status` - Component status

### Tasks

- `GET /api/v1/tasks` - List all tasks
- `POST /api/v1/tasks` - Create new task

  ```json
  {
    "description": "Task description",
    "priority": "normal|high|low"
  }
  ```

## 🔧 Configuration

### Environment Variables

- `RUST_LOG` - Logging level (default: `noa_api=debug`)
- `PORT` - API server port (default: 3001)

### Toolchain Versions

- Rust: 1.91.1
- Node: 20.18.1
- Go: 1.23.4
- Python: 3.12.8

### Config and Schema Policy

NOA uses a shared and centralized configuration system located in `/config/`. All configuration files follow a unified JSON schema defined in `/config/schemas/`, ensuring consistency and validation.

- **Metadata Structure**: Each config includes a `metadata` object with `version`, `description`, `updated_at`, and other provenance fields for auditability and change tracking.
- **Schema Validation**: Configurations are validated against JSON schemas to prevent errors and ensure compliance with the universal task execution policy.
- **Centralized Management**: Shared resources, providers, and features are configured centrally, with environment-specific overrides supported via `${NOA_ROOT}` variables.
- **Version Control**: Configs are versioned and changes are tracked, with automated validation on commit.

## 📚 Documentation

See `specs/001-noa-seed-foundation/` for:

- `spec.md` - Full system specification
- `plan.md` - Implementation plan
- `tasks.md` - Task breakdown
- `data-model.md` - Data structures

## 🎨 UI Features

- **System Dashboard** - Real-time component monitoring
- **Chat Interface** - Natural language task creation
- **Status Indicators** - Visual health checks
- **Responsive Design** - Works on all screen sizes

## 🚦 Current Status

### ✅ Completed

- Core Rust workspace with 6 crates
- REST API with Axum
- Next.js UI with TypeScript
- Real-time status monitoring
- Task management endpoints
- Chat interface
- Full stack integration

### ⏳ In Progress

- Database integration
- Neural runtime
- Agent orchestration
- P2P networking

### 📋 Pending

- llama.cpp integration
- FastEmbed embeddings
- CECCA activation
- libp2p federation
- Desktop app (NDCL)

---

````markdown
# NOA Unified Architecture (Directory Tree + System Graph)

This consolidates the last few turns into **one coherent, commit-stable layout**:
- **sys/core stays small and trusted**
- **MCP lives in the gateway**
- **Connectors are MCP tool-servers**
- **Execution happens in sandboxes**
- **Personal hive mind (libp2p) extends compute/storage outward to regional/org**
- **Multiple task apps are supported via adapters into a single Task Kernel**
- **Docs (Wiki/Pages/Runbooks) are first-class and enforceable by schema**

---

## Definitions

### Core concepts
- **sys/core**: The trusted “microkernel.” Holds identity, policy, scheduling, registry pointers, audits, and enforcement. It should *not* embed tool logic.
- **Gateway (MCP)**: The tool bus. Routes tool calls to per-tool MCP servers, applies authz, locality routing, budgets, and audit hooks.
- **MCP server (per-tool)**: A single capability boundary. Example: `spec-kit`, `db-ro`, `build-test`, `task-app-A connector`.
- **Connector**: A tool server that interfaces an external app/system. In this architecture, connectors belong at **`gateway/mcp/connectors/`**.
- **Sandbox**: The containment boundary for execution (anti content-rot + token/log/cache bloat). Per-task ephemeral workspace, strict limits, reproducible artifacts.
- **Task Kernel**: NOA’s canonical internal model for tasks. All task apps map into/out of this schema via adapters.
- **CAS**: Content-addressed storage (Merkle DAG). Immutable artifacts, refs/tags on top, GC, and provenance anchoring.
- **Hive mind (libp2p)**:
  - **Personal**: Your devices (trust anchor).
  - **Regional/Org**: Extra compute/storage pools, gated by admission/governance/policy.

---

## Unified Directory Tree (Markdown)

```text
noa/
├─ README.md
├─ AGENT.md
├─ LICENSE
├─ bin/                                # CLI entrypoints (thin)
│  ├─ noa
│  ├─ noa-admin
│  └─ noa-sandbox
│
├─ lib/                                # shared libraries (Rust crates + polyglot)
│  ├─ noa-core/
│  ├─ noa-policy/
│  ├─ noa-mcp/
│  ├─ noa-cas/
│  ├─ noa-p2p/
│  ├─ noa-schema/
│  └─ noa-ui-proto/
│
├─ sys/                                # Trusted microkernel surface
│  ├─ core/
│  │  ├─ identity/                     # users/devices/org roles
│  │  ├─ policy/                       # capability tokens, allow/deny, budgets
│  │  ├─ secrets/                      # secret mediation API (no raw secrets to tools)
│  │  ├─ audit/                        # append-only audit + provenance hooks
│  │  ├─ scheduler/                    # task graph runtime, priorities, quotas
│  │  ├─ world_model/                  # machine-readable SSoT
│  │  ├─ registry/                     # tool/model/server registry pointers + trust pins
│  │  └─ enforcement/                  # Layer 3 hooks: validators/guardrails/diff monitors
│  ├─ api-server/                      # sys control-plane API
│  ├─ init/                            # boot/migrations/health
│  ├─ shell/                           # controlled bash/pwsh adapters
│  └─ etc/                             # read-only defaults mirroring baseline
│
├─ gateway/                            # Tool bus + routing
│  ├─ mcp/                             # MCP is *here*
│  │  ├─ proxy/                        # single ingress for tool calls
│  │  ├─ registry/                     # discovery, pinning, signatures, trust metadata
│  │  ├─ routing/                      # locality routing: local/personal/regional/org
│  │  ├─ authz/                        # capability -> tool permissions
│  │  └─ connectors/                   # connectors are MCP tool servers
│  │     ├─ task-app-A/
│  │     ├─ task-app-B/
│  │     ├─ task-app-C/
│  │     └─ router/                    # authority rules + conflict arbitration
│  ├─ api/                             # non-tool internal APIs used by UI/orchestrator
│  └─ ui-bridge/                       # push events to UI (progress, logs, widgets)
│
├─ orchestrator/                       # Brains: plan/route/execute
│  ├─ router/                          # provider+tool selection + budgets + locality
│  ├─ planner/                         # request -> task packages (DAG)
│  ├─ executor/                        # runs packages via gateway/mcp
│  ├─ workflows/                       # higher-order DAGs (build, migrate, train, etc.)
│  ├─ commands/                        # canonical verbs mapped to packages/workflows
│  └─ task-kernel/                     # canonical task system
│     ├─ schema/
│     ├─ normalization/                # dedupe, conflict rules, canonicalization
│     ├─ mapping/                      # per-task-app mapping configs
│     └─ sync/                         # ingest + emit pipelines
│
├─ task/                               # work management (machine-first)
│  ├─ todo/
│  ├─ project-management/
│  ├─ run-logs/                        # bounded run metadata (CAS-linked)
│  └─ artifacts/                       # outputs promoted from sandbox -> CAS
│
├─ sandbox/                            # execution containment (anti-rot)
│  ├─ runtime/
│  │  ├─ runners/
│  │  ├─ workspaces/
│  │  ├─ mounts/
│  │  ├─ network/
│  │  └─ limits/
│  ├─ snapshots/                       # rollback points + diff monitors
│  └─ policies/                        # sandbox profiles (build, scan, train, db-migrate)
│
├─ tools/                              # per-tool MCP servers (capability boundaries)
│  ├─ spec-kit/
│  ├─ code-scan/
│  ├─ build-test/
│  ├─ db/                              # split ro/rw logically (or separate servers)
│  ├─ vector/
│  ├─ cas/
│  ├─ object-store/
│  ├─ notebook-kernel/
│  └─ package-manager/                 # pnpm isolated behind MCP + sandbox
│
├─ providers/                          # model providers (compute plane)
│  ├─ local/
│  │  ├─ llama_cpp/
│  │  └─ candle/
│  ├─ remote/
│  │  ├─ codex_cli/
│  │  ├─ claude_code_cli/
│  │  └─ copilot_bridge/
│  ├─ shared/                          # shared provider resources (kv cache, emb cache)
│  └─ pool/                            # scheduling, routing, budgets
│
├─ p2p/                                # hive mind mesh (libp2p)
│  ├─ personal/
│  │  ├─ node/
│  │  ├─ discovery/
│  │  ├─ routing/
│  │  ├─ compute/                      # remote execution requests (sandboxed)
│  │  └─ storage/                      # CAS replication/pinning/retrieval
│  ├─ regional/
│  │  ├─ admission/
│  │  ├─ compute/
│  │  └─ storage/
│  └─ org/
│     ├─ governance/
│     ├─ compute/
│     └─ storage/
│
├─ data/                               # durable state plane
│  ├─ cas/
│  │  ├─ blobs/
│  │  ├─ refs/                         # mutable pointers/tags on top of CAS
│  │  ├─ index/
│  │  └─ gc/
│  ├─ db/
│  │  ├─ postgres/
│  │  └─ sqlite/
│  ├─ vectors/
│  ├─ object-store/
│  ├─ logs/                            # bounded + rotated
│  └─ cache/                           # bounded
│
├─ configs/                            # AI-native centralized config (3-layer)
│  ├─ base/                            # Layer 1 immutable baseline (Nix-style)
│  │  ├─ microkernel-layout/
│  │  ├─ toolchain-versions/
│  │  ├─ schemas/
│  │  ├─ safety-rails/
│  │  ├─ sandbox-definitions/
│  │  └─ rollback-points/
│  ├─ semantic/                        # Layer 2 mutable semantic layer
│  │  ├─ preferences/
│  │  ├─ capabilities/
│  │  ├─ device-profiles/
│  │  ├─ world-model-metadata/
│  │  ├─ intent/
│  │  ├─ agent-rules/
│  │  ├─ learned-optimizations/
│  │  └─ hive-state/
│  └─ enforcement/                     # Layer 3 enforcement + self-correcting loop
│     ├─ validator/
│     ├─ schema-checker/
│     ├─ compiler/
│     ├─ guardrails/
│     ├─ snapshot-diff-monitor/
│     └─ policy-engine/
│
├─ settings/                           # generated runtime settings (compiled output)
│  ├─ resolved/
│  ├─ profiles/                        # isolate IDE/app/provider bloat
│  └─ overrides/                       # time-limited, audited
│
├─ secret-store/                       # sealed secrets + access policies
│  ├─ envelopes/
│  ├─ policies/
│  └─ brokers/
│
├─ apps/                               # external apps (task apps live here)
│  └─ task-manager/
│     ├─ upstream/                     # immutable pinned binaries (web/desktop/mobile)
│     ├─ wrappers/                     # embed + deep links + auth + adapter glue
│     ├─ profiles/                     # isolated cache/logs
│     └─ manifests/                    # hashes, versions, SBOM-ish metadata
│
├─ ui/                                 # human-in-loop UI only
│  ├─ app/                             # shell (desktop/mobile/web/XR)
│  ├─ pages/
│  │  ├─ convo/                        # home (chat + widgets)
│  │  ├─ tasks/                        # Tasks Hub + embedded task apps subpages
│  │  ├─ runs/                         # execution runs viewer
│  │  └─ hive/                         # devices/compute/storage mesh view
│  └─ widgets/                         # graphs, DAGs, inspectors
│
├─ ide/                                # IDE separation to prevent cache/log bloat
│  ├─ vscode_bridge/
│  ├─ cursor_bridge/
│  └─ profiles/
│
├─ docs/                               # knowledge system
│  ├─ wiki/                            # navigation + architecture SSoT hub
│  ├─ pages/                           # granular documentation
│  ├─ runbooks/                        # verified actions (triggers/escalation)
│  ├─ api/
│  ├─ schemas/
│  └─ adr/
│
├─ test/
│  ├─ unit/
│  ├─ integration/
│  ├─ e2e/
│  ├─ qa/                              # QA matrices/checklists
│  └─ notebook/
│     ├─ kernels/
│     ├─ notebooks/
│     └─ fixtures/
│
├─ staging/                            # pre-release workspace
│  ├─ builds/
│  ├─ releases/
│  └─ canary/
│
├─ deploy/                             # release strategies
│  ├─ blue-green/
│  ├─ canary/
│  ├─ rollback/
│  └─ hot-swap/
│
├─ training/
│  ├─ datasets/                        # manifests (often CAS-backed)
│  ├─ pipelines/                       # training flows calling tools via MCP
│  ├─ evals/
│  └─ finetune/
│
└─ workflows/                          # cross-domain workflows (automation)
   ├─ build-release/
   ├─ migrate/
   ├─ onboard-device/
   ├─ runbook-automation/
   └─ self-heal/                       # drift -> rollback/repair
````

---

## Mermaid System Graph (Unified)

```mermaid
flowchart TB
  %% UI / Human loop
  UI[UI Shell\n(conversational + widgets + XR)] --> ORCH[Orchestrator\n(planner/router/executor)]
  IDE[IDE Bridges\n(VS Code/Cursor isolated)] --> ORCH
  TMUI[Task Apps UI\n(embedded panes/subpages)] --> UI

  %% Canonical task layer
  TMUI -->|events/changes| CONN[Gateway MCP Connectors\n(gateway/mcp/connectors/*)]
  CONN --> TK[Task Kernel\n(normalize/dedupe/authority)]
  ORCH <--> TK

  %% MCP gateway + tool execution
  ORCH --> GATE[Gateway MCP Proxy\n(authz + routing + registry)]
  GATE --> TOOL[Per-tool MCP Servers\n(tools/* + connectors/*)]
  TOOL --> SBX[Sandbox Runtime\n(workspaces + limits + snapshots)]
  SBX --> ART[Artifacts\n(CAS refs + provenance)]
  ORCH -->|status/progress| UI

  %% Data plane
  subgraph DATA[Data Plane]
    CAS[(CAS/Merkle DAG\nblobs+refs+index+gc)]
    PG[(Postgres)]
    SQLITE[(SQLite)]
    VDB[(Vector DB)]
    OBJ[(Object Store)]
  end
  ART --> CAS
  TOOL --> PG
  TOOL --> SQLITE
  TOOL --> VDB
  TOOL --> OBJ

  %% sys/core enforcement + config layers
  subgraph CORE[sys/core]
    ID[Identity]
    POL[Policy + Capabilities]
    AUD[Audit/Provenance]
    SCH[Scheduler]
    REG[Registry Pointers + Trust Pins]
    ENF[Enforcement Hooks]
  end
  GATE -->|capability checks| POL
  ORCH --> SCH
  TOOL -->|audit events| AUD
  REG --> GATE

  subgraph CFG[AI-native Config (3 layers)]
    C1[Layer 1: Immutable Base\n(pinned toolchains, baseline schemas,\nrollback, sandbox defs)]
    C2[Layer 2: Mutable Semantic\n(prefs, device profiles,\nworld model metadata, hive state)]
    C3[Layer 3: Enforcement\n(validator/compiler/guardrails\ndrift monitor/self-correct)]
    C1 --> C3
    C2 --> C3
  end
  C3 --> ENF
  C3 --> REG

  %% Hive mind mesh
  subgraph HIVE[libp2p Hive Mind]
    P[p2p/personal\n(trust anchor)]
    R[p2p/regional\n(extra compute/storage)]
    O[p2p/org\n(governed pool)]
  end
  GATE -->|locality routing| HIVE
  HIVE --> TOOL
  HIVE --> CAS
```

---

## Canonical Schemas (Machine-first)

### 1) Task Kernel Task (canonical internal task)

**Purpose:** unify multiple task apps and agent execution into one model.

```yaml
TaskKernelTask:
  id: string                        # internal stable ID (UUID/ULID)
  source:
    app: string                     # "task-app-A" | "task-app-B" | "noa"
    native_id: string               # the app’s ID
  title: string
  description: string
  project: string?
  tags: [string]
  priority: enum {P0,P1,P2,P3}
  state: enum {TODO,IN_PROGRESS,BLOCKED,DONE,CANCELED}
  assignees: [string]               # user/device/org identities
  deps: [string]                    # other TaskKernelTask ids (DAG)
  authority:
    mode: enum {READ_ONLY,BIDIR,KERNEL_AUTH}  # per task/project rules
    fields: [string]?               # optional per-field authority control
  execution:
    package_ref: string?            # points to a TaskPackage
    sandbox_profile: string         # profile name
    required_capabilities: [string]
  runs:
    - run_id: string
      status: enum {PLANNED,RUNNING,SUCCEEDED,FAILED}
      started_at: timestamp?
      ended_at: timestamp?
      artifacts: [CasRef]
  provenance:
    created_at: timestamp
    updated_at: timestamp
    merged_from: [string]           # native IDs or internal IDs
```

### 2) Task Package (microservice/package format)

**Purpose:** request → reproducible, sandboxed execution plan.

```yaml
TaskPackage:
  id: string
  intent: string                    # human/agent intent summary
  dag:
    nodes:
      - id: string
        tool: string                # MCP tool name
        action: string              # method/operation
        inputs: object
        outputs: object?
        sandbox_profile: string
        budgets:
          time_ms: int
          cpu: string?
          gpu: string?
          net: enum {OFF,ALLOWLIST,ON}
    edges:
      - from: string
        to: string
  artifacts:
    outputs:
      - name: string
        cas_ref: CasRef?
        type: string
  policies:
    required_capabilities: [string]
    audit_level: enum {MIN,NORMAL,STRICT}
```

### 3) CAS Ref (immutable artifact pointer)

```yaml
CasRef:
  algo: string                      # e.g., blake3/sha256 (choose one)
  hash: string
  size: int
  mime: string?
  refs:                             # optional mutable pointers on top
    tag: string?
    ref: string?
```

### 4) Runbook Header (verified action pages)

**Purpose:** enforce “Five A’s” style runbooks with triggers/escalation.

```yaml
RunbookHeader:
  id: string
  title: string
  triggers:
    - signal: string                # alert name / condition
      severity: enum {S1,S2,S3,S4}
  escalation:
    - level: int
      contact: string               # role/team (not personal secrets)
      condition: string
  prerequisites:
    - capability: string
    - tool: string
  steps:
    - kind: enum {COMMAND,CHECK,LINK,NOTE}
      value: string
  verification:
    last_dry_run: timestamp
    owner: string
```

---

## Data Flow (End-to-end)

### A) Human → Tasks → Agents → Artifacts → Back to UI

1. User creates/edits a task in **Task App A/B/C** (or in NOA Tasks Hub).
2. **`gateway/mcp/connectors/task-app-*`** ingests events.
3. **Task Kernel** normalizes:

   * maps fields into canonical schema
   * dedupes and resolves conflicts using **authority rules**
4. Orchestrator turns the task into a **Task Package DAG**:

   * selects tools/providers
   * assigns sandbox profiles and budgets
5. Gateway MCP routes tool calls to **per-tool MCP servers**.
6. Execution runs inside **sandbox/runtime**:

   * bounded logs/cache
   * controlled mounts/network
   * snapshot/diff monitors
7. Outputs are promoted to **CAS** (immutable), linked to the task run.
8. Status + artifacts are surfaced in:

   * **UI widgets** (graphs/DAGs/progress)
   * optionally written back to task apps via connectors (when bidirectional)

### B) Locality-aware routing (personal → regional → org)

1. Orchestrator asks Gateway for tool execution.
2. Gateway’s routing chooses:

   * local device first (fast + trusted)
   * personal hive nodes next
   * regional/org pools if policy allows and extra compute/storage needed
3. Remote execution still runs under **sandbox profiles**, and artifacts return as CAS refs.

---

## Why this layout (the non-negotiables)

### 1) Prevent tool sprawl from corrupting sys/core

sys/core stays small because **MCP is the only “door”** for tools. Tools crash? Core survives. Tools evolve? Core stays stable.

### 2) Stop content rot and bloat (logs/caches/tokens)

The sandbox is a deliberate “garbage boundary”:

* per-task workspaces
* bounded caches and logs
* artifacts promoted to CAS (immutable, dedupe-friendly)
  This keeps your system from becoming a pile of stale state.

### 3) Support multiple task apps without chaos

You don’t “unify” by rewriting apps; you unify by:

* **Task Kernel** + **adapters**
* **authority rules** per project/field
* “Tasks Hub” view in UI, plus deep links to source apps

### 4) Make “everyone a creator” real

The UI stays conversational, but can spawn:

* widgets
* DAG graphs
* run viewers
* task boards
  …while the agents do the heavy lifting behind policy and sandbox constraints.

### 5) Make the hive mind safe

libp2p expands compute/storage **without expanding trust**:

* personal nodes are the anchor
* regional/org nodes are capability-scoped and governed
* execution is always sandboxed, artifacts always anchored in CAS

---

## UI Placement for Task Apps (Final)

* **`ui/pages/convo/`** = command center (chat + widgets)
* **`ui/pages/tasks/hub/`** = canonical unified task hub
* **`ui/pages/tasks/app-*`** = embedded task app panes (optional)
* Deep link format example: `noa://tasks/<TaskKernelTask.id>` + “Open in App A/B”

---

## One-line invariants (so refactors don’t drift)

* Agents never touch the world except through **Gateway MCP**.
* Connectors that agents can call live at **`gateway/mcp/connectors/`**.
* Execution happens in **sandbox**; durable outputs go to **CAS**.
* Multiple task apps unify through **Task Kernel** (not by merging UIs first).
* Configs follow **Layer1 base + Layer2 semantic + Layer3 enforcement**.

```

If you want the next step that turns this from “architecture doc” into “machine-enforceable system,” it’s a single **schema compiler** that:
1) validates `configs/base + configs/semantic`  
2) emits `settings/resolved`  
3) registers tools/connectors into `sys/core/registry`  
4) generates the UI navigation (Tasks Hub + embedded panes) from the registry.
::contentReference[oaicite:0]{index=0}
```

