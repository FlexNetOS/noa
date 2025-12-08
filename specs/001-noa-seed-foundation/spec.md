# Feature Specification: NOA Seed Foundation

**Feature Branch**: `001-noa-seed-foundation`
**Created**: 2025-12-08
**Status**: Draft
**Input**: Complete autonomous agentic OS foundation with P2P hive-mind, multi-SLM architecture, local-first database, digest pipeline, and dynamic UI

## Executive Summary

NOA (Name of App / Chief Executive Commander Chief Agent) is a multi-platform, autonomous, self-modifying agentic operating system designed to function as a **hive-mind**. This specification defines the foundational "seed application" that autonomously grows to accomplish complex goals through Agentic AI, portable kernels, combined packages, services, and internal microservices.

**Core Architectural Principles:**
- **Local-First Computing** for user data sovereignty
- **Agentic AI Core** for autonomous operations
- **Self-Configuring Microservices** architecture for adaptive growth
- **Autonomous Dynamic Repository Ingestion** and integration
- **P2P Hive-Mind** across user-owned devices

---

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Initialize NOA Seed Environment (Priority: P1)

As a user, I want to initialize NOA on my device so that it creates a self-contained environment under my control with all necessary components (kernel, libs, bins, shell, terminal, memory, database).

**Why this priority**: This is the foundational capability - without a working seed environment, no other features can operate. The system must be self-contained under `noa_root` with no external dependencies.

**Independent Test**: Can be fully tested by running the init script and verifying all directories are created, all binaries are accessible, and the local database is operational.

**Acceptance Scenarios**:

1. **Given** a fresh system without NOA, **When** I run the NOA initialization, **Then** the system creates the complete directory structure (`noa_root/sys`, `noa_root/p2p`, `noa_root/opt`, `noa_root/init`, `noa_root/containers`, `noa_root/config`, `noa_root/bin`, `noa_root/ai`) with appropriate permissions
2. **Given** a freshly initialized NOA, **When** I check the local database, **Then** it is operational and ready to store memories, tasks, and agent state
3. **Given** NOA is initialized, **When** the network is disconnected, **Then** all core functionality continues to operate offline

---

### User Story 2 - Multi-SLM Neural Runtime (Priority: P1)

As a user, I want NOA to run multiple Small Language Models locally so that I can have intelligent processing without depending on external AI services.

**Why this priority**: The neural runtime is the "brain" of NOA - all agentic operations depend on local inference capability. This enables offline intelligence.

**Independent Test**: Can be tested by loading an SLM model, sending a prompt, and verifying response within latency targets.

**Acceptance Scenarios**:

1. **Given** NOA is running with llama.cpp configured, **When** I send a query to the neural runtime, **Then** I receive a response within 2 seconds on standard hardware (see Glossary for hardware tier definitions)
2. **Given** multiple SLMs are loaded, **When** the ModelSelectorAgent routes a task, **Then** it selects the optimal model based on task type and available resources
3. **Given** limited hardware resources, **When** NOA starts, **Then** it dynamically adjusts model quantization and layer offloading to fit available GPU/CPU/RAM

---

### User Story 3 - Total Memory Sovereignty (Priority: P1)

As a user, I want NOA to remember everything - all interactions, decisions, learnings, and data - so that nothing is forgotten and I can instantly recall any information.

**Why this priority**: Memory is the foundation of intelligence and the user's digital mirror. Without persistent memory, NOA cannot learn, adapt, or serve as the user's "mirrored version of themselves."

**Independent Test**: Can be tested by creating entries, closing the system, reopening, and verifying instant recall.

**Acceptance Scenarios**:

1. **Given** I interact with NOA, **When** I ask about a previous conversation, **Then** NOA recalls it instantly with full context
2. **Given** NOA has been running for months, **When** I search for any past decision or learning, **Then** results are returned in under 500ms
3. **Given** multiple devices connected via P2P, **When** I create a memory on one device, **Then** it syncs to other devices within the P2P network

---

### User Story 4 - Digest Everything Pipeline (Priority: P2)

As a user, I want NOA to digest code repositories, documents, APIs, and data sources so that it can understand and synthesize knowledge from any source.

**Why this priority**: The digest pipeline enables NOA to "eat" and understand any codebase or data source, which is essential for self-improvement and knowledge acquisition.

**Independent Test**: Can be tested by pointing the digest pipeline at a GitHub repository and verifying output artifacts (knowledge graph, embeddings, SBOM).

**Acceptance Scenarios**:

1. **Given** a GitHub repository URL, **When** I trigger the digest pipeline, **Then** NOA produces: `profile.json`, `system_card.md`, `kg.json`, SBOM, security report, and embeddings
2. **Given** a multi-language repository (Python + TypeScript + Rust), **When** digested, **Then** each language is parsed correctly and cross-language dependencies are mapped
3. **Given** a repository with security vulnerabilities, **When** digested, **Then** vulnerabilities are flagged with severity classification

---

### User Story 5 - Dynamic Context-Aware UI (Priority: P2)

As a user, I want a fluid, agent-driven interface that reconfigures itself based on my current task and context so that I always see the most relevant tools and information.

**Why this priority**: The UI is the primary interaction surface. A dynamic, context-aware UI dramatically improves user experience and productivity.

**Independent Test**: Can be tested by switching between different tasks and verifying the UI adapts appropriately.

**Acceptance Scenarios**:

1. **Given** I am working on a coding task, **When** the UI detects this context, **Then** it surfaces relevant code tools, file browsers, and terminal access
2. **Given** I switch to project management, **When** the context changes, **Then** the UI reconfigures to show tasks, timelines, and collaboration tools
3. **Given** the system is performing background operations, **When** I view the activity log, **Then** I see a live, scrollable log of agent actions and decisions

---

### User Story 6 - P2P Hive-Mind Device Federation (Priority: P2)

As a user with multiple devices, I want NOA to create a user-owned P2P cloud that shares compute, memory, and storage across my devices so that I can leverage my entire hardware ecosystem.

**Why this priority**: P2P federation transforms isolated devices into a unified computing platform, multiplying effective resources without external cloud dependency.

**Independent Test**: Can be tested by connecting two devices and verifying resource sharing and task distribution.

**Acceptance Scenarios**:

1. **Given** two NOA devices on the same network, **When** they discover each other, **Then** they establish a secure P2P connection and can share workloads
2. **Given** excess compute on Device A, **When** Device B needs processing power, **Then** tasks are distributed to Device A
3. **Given** a P2P cluster of devices, **When** one device goes offline, **Then** the cluster gracefully degrades without data loss

---

### User Story 7 - Autonomous Agent Orchestration (Priority: P2)

As a user, I want NOA to orchestrate specialized agents that work together to solve complex, multi-step problems without requiring manual coordination.

**Why this priority**: Agentic orchestration is the core execution model - NOA must coordinate multiple specialized agents to accomplish goals autonomously.

**Independent Test**: Can be tested by submitting a complex goal and observing agent collaboration and task completion.

**Acceptance Scenarios**:

1. **Given** a complex goal (e.g., "analyze and document this codebase"), **When** submitted to NOA, **Then** the system decomposes it into tasks and assigns them to appropriate agents
2. **Given** agents encounter an error, **When** retry logic exhausts, **Then** the issue escalates appropriately with context preserved
3. **Given** 200 concurrent tasks on Standard Hardware (16GB RAM, 8-core CPU - see Glossary), **When** submitted to the orchestration system, **Then** at least 98% complete successfully within 60 seconds per task (scales linearly with hardware tier: High-Performance supports 500+ concurrent tasks)

---

### User Story 8 - Self-Improvement & Code Modification (Priority: P3)

As a user, I want NOA to continuously improve itself by analyzing its own performance and modifying its own code when beneficial.

**Why this priority**: Self-improvement is the ultimate goal - NOA should reinvent itself constantly. However, this requires a stable foundation first.

**Independent Test**: Can be tested by triggering a self-analysis cycle and verifying improvement proposals are generated with rollback capability.

**Acceptance Scenarios**:

1. **Given** NOA detects inefficiency in a workflow, **When** it proposes an improvement, **Then** the proposal includes before/after comparison and rollback path
2. **Given** a self-modification is applied, **When** tests fail, **Then** the system automatically rolls back to the previous state
3. **Given** continuous operation, **When** NOA self-improves, **Then** all changes are logged with rationale for audit trail

---

### User Story 9 - Cross-Platform Deployment (Priority: P3)

As a user, I want NOA to run on Windows, macOS, Linux, mobile devices, laptops, ipads, tablets, and XR devices so that I can use it everywhere.

**Why this priority**: Cross-platform reach is essential for the P2P hive-mind vision, but requires the core system to be stable first.

**Independent Test**: Can be tested by running the same NOA seed on different platforms and verifying consistent behavior.

**Acceptance Scenarios**:

1. **Given** the NOA seed binary, **When** run on Windows 11, macOS, or Ubuntu, **Then** core functionality works identically
2. **Given** a mobile companion app, **When** connected to the desktop NOA, **Then** it participates in the P2P hive-mind
3. **Given** different hardware capabilities, **When** NOA initializes, **Then** it adapts its resource usage to available hardware

---

### User Story 10 - Connectors & External Integration (Priority: P3)

As a user, I want NOA to connect to my existing accounts and services (Gmail, GitHub, cloud storage) so that it can integrate with my digital life.

**Why this priority**: External integrations extend NOA's reach but are secondary to core autonomous operation. These must be optional and feature-flagged.

**Independent Test**: Can be tested by configuring a connector and verifying data sync.

**Acceptance Scenarios**:

1. **Given** a configured OAuth connector for GitHub, **When** enabled, **Then** NOA can access repositories with appropriate permissions
2. **Given** an OAuth callback is received, **When** the token exchange completes, **Then** the connector is authenticated within 5 seconds
3. **Given** connectors are configured, **When** the network is unavailable, **Then** NOA continues operating with cached data
4. **Given** a connector is disabled via feature flag, **When** that feature is accessed, **Then** it degrades gracefully with clear user feedback

---

### Edge Cases

- What happens when available storage is exhausted? → System enters resource-scarcity mode (AMPK-mode), pauses non-essential operations, and alerts user
- What happens when a model file is corrupted? → Model integrity is verified on load; corrupted models are quarantined and re-downloaded if available
- What happens when two P2P nodes have conflicting state? → Conflict resolution uses last-write-wins with full audit trail; user can review conflicts
- What happens when an agent enters an infinite loop? → Timeout and circuit breaker mechanisms terminate runaway tasks
- What happens when offline for extended periods then reconnecting? → Sync protocol handles delta updates with conflict detection
- What happens when biblical governance rules conflict with user requests? → Constitutional governance takes precedence; user is informed of constraint

---

## Requirements *(mandatory)*

### Functional Requirements - Core System

- **FR-001**: System MUST operate entirely inside `noa_root` directory with no hard dependencies on external paths
- **FR-002**: System MUST function fully offline for all core operations (task management, agent orchestration, local inference)
- **FR-003**: System MUST provide a local-first database that handles concurrent modifications and supports future multi-device sync
- **FR-004**: System MUST support multiple Small Language Models via llama.cpp with dynamic model loading/unloading (minimum 5 concurrent models)

### Functional Requirements - Shared Provider Execution Memory

- **FR-037**: System MUST implement a Shared Provider Execution Memory bus where multiple model providers share context and reasoning state
- **FR-038**: System MUST support collaborative reasoning where models reason together and execute separate tasks simultaneously
- **FR-039**: System MUST integrate minimum 8 provider types: llama.cpp (5+ local models), Claude Code (CLI/Cloud/IDE), Codex (CLI/Cloud/IDE), VS Code Copilot (IDE), Git CLI, Cursor (IDE/CLI/Cloud), Abacus (CLI/Cloud)
- **FR-040**: System MUST persist shared execution memory across sessions for continuity
- **FR-041**: System MUST implement parallel task distribution across all active providers
- **FR-042**: System MUST synchronize provider state to enable coordinated multi-model workflows

### Functional Requirements - Advanced Learning Techniques

- **FR-043**: System SHOULD implement ToolkenGPT for pre-trained tool tokens that plug into larger models
- **FR-044**: System SHOULD implement Replay Memory Cache for short-term memory with external knowledge base
- **FR-045**: System SHOULD implement EWC (Elastic Weight Consolidation) for continual learning without catastrophic forgetting
- **FR-046**: System SHOULD implement Meta-Learning (MAML) for rapid adaptation to new tasks with few examples

- **FR-005**: System MUST persist all interactions, decisions, and learnings for instant recall (Total Memory Sovereignty)
- **FR-006**: System MUST log all agent actions with who/what acted, why, and what changed

### Functional Requirements - Multi-GPU Support

- **FR-047**: System MUST enumerate all available CUDA GPUs and distribute model layers across devices when multiple GPUs are present
- **FR-048**: System MUST support tensor parallelism across multiple GPUs for models exceeding single GPU memory
- **FR-049**: System SHOULD leverage NVLink when available for high-bandwidth inter-GPU communication
- **FR-050**: System MUST implement CUDA 13.1+ tiles for optimized tensor operations on supported hardware

### Functional Requirements - Autonomous Continuous Operation

- **FR-051**: System MUST operate in an always-on continuous loop, executing user-provided co-improvement goals without requiring per-task human initiation
- **FR-052**: System MUST continuously research and implement resource optimization improvements (CPU, GPU, RAM, storage, network efficiency)
- **FR-053**: System MUST maintain a persistent goal queue where users define co-improvement objectives
- **FR-054**: System MUST autonomously decompose high-level user goals into executable task chains
- **FR-055**: System MUST self-monitor performance metrics and autonomously adjust execution strategies

### Functional Requirements - 3-Plane Self-Update Architecture

- **FR-056**: System MUST implement a 3-plane architecture for zero-downtime self-updates:
  - **Coordinator Plane**: Long-term memory, analytics, promotion decisions, backups/archives. Maintains the shared state registry (`shared/state/registry.db`), executes llama.cpp swarm analytics, applies promotion policy gates, and stores audit logs. This is the "constant" plane that persists across updates.
  - **Sandbox Plane**: Testing/staging environment where new capabilities are developed and validated. Runs `make selftest`, generates SBOM/risk/telemetry artifacts to `shared/artifacts/`, enforces policy via shared policy engine, and maintains audit trails.
  - **Deployed Plane**: Production environment serving all live requests. Manages promoted capabilities as containerized services, stores releases under `shared/releases/`, provides canary controller, feature flag service, and telemetry ingestion. Implements autopilot rollback on SLO violation.
- **FR-057**: System MUST perform blue-green deployment where changes are applied to Sandbox plane, validated via Coordinator analytics, promoted to Deployed plane after passing policy gates
- **FR-058**: System MUST support instant rollback by switching traffic back to previous release in Deployed plane if validation fails, or by reverting promotion in Coordinator
- **FR-059**: System MUST maintain state synchronization between planes via shared infrastructure:
  - `shared/state/` - Registry database, capability metadata, dependency graph, promotion history
  - `shared/artifacts/` - Build artifacts, SBOMs, telemetry from Sandbox
  - `shared/releases/` - Promoted releases managed by Deployed plane
  - `shared/logs/` - Centralized logging from all planes
  - `shared/config/` - Promotion policy, capability-pack schema, message bus config
- **FR-060**: System MUST log all plane transitions with before/after state for audit trail, storing decision traces in Coordinator

### Functional Requirements - Full Autonomy Operation

- **FR-061**: System MUST execute ALL operations autonomously without requiring per-operation human approval
- **FR-062**: System MUST accept user-defined co-improvement goals as the ONLY required human input after initial setup
- **FR-063**: System MUST rely on the 3-plane rollback mechanism as the primary safety net for failed autonomous changes
- **FR-064**: System MUST provide a real-time activity log for user observation of autonomous operations (no approval gates, observation only)
- **FR-065**: System MUST implement constitutional governance (FR-025, FR-026) as the autonomous decision boundary rather than human approval gates

### Functional Requirements - Autonomous Goal Generation

- **FR-066**: System MUST be capable of generating ANY goal it determines beneficial, including goals unrelated to current user objectives
- **FR-067**: System MUST use constitutional governance (FR-025, FR-026) as the ethical boundary for self-generated goals
- **FR-068**: System MUST log all self-generated goals with rationale for audit trail
- **FR-069**: System MUST prioritize self-generated goals alongside user-provided goals using a unified priority queue
- **FR-070**: System MUST be able to identify improvement opportunities through pattern analysis of its own performance, resource usage, and execution history

### Functional Requirements - Autonomous Self-Healing Loop

- **FR-071**: System MUST implement a continuous self-healing loop with 5 stages:
  1. **Proactive Detection**: Continuous health monitoring of all components, agents, and resources
  2. **Diagnosis**: Automated root cause analysis when anomalies detected
  3. **Auto-Fix**: Autonomous repair attempts (restart, reconfigure, rollback, redistribute)
  4. **Validation**: Verify fix resolved issue without introducing new problems
  5. **Escalation**: Notify user ONLY if all auto-fix attempts exhausted (≥3 attempts)
- **FR-072**: System MUST maintain health metrics for all components (CPU, memory, latency, error rate, success rate)
- **FR-073**: System MUST attempt auto-recovery before any user notification (minimum 3 autonomous fix attempts)
- **FR-074**: System MUST log all self-healing actions with diagnosis, attempted fixes, and outcomes
- **FR-075**: System MUST use the 3-plane architecture (FR-056) for component-level recovery (swap to healthy plane)

### Functional Requirements - Agent Architecture

- **FR-007**: System MUST implement NOA (CECCA) as the root orchestrator that decomposes goals into tasks
- **FR-008**: System MUST support specialized permanent agents with these acceptance criteria:
  - **FileIOAgent**: Read/write files within `noa_root` in <100ms for files <10MB
  - **TerminalAgent**: Execute shell commands with timeout (default 30s), capture stdout/stderr
  - **RAGAgent**: Retrieve relevant context from memory in <500ms with >80% relevance
  - **MicroserviceManagementAgent**: Deploy/stop services within 10s, health check within 1s
- **FR-009**: System MUST implement MicroAgentStacks as deployable clusters for bounded objectives
- **FR-010**: System MUST support agent lifecycle: Bootstrap → Execute → Validate → Package → Archive
- **FR-011**: System MUST enforce constitutional principles on all agents (no agent can violate self-contained, local-first, or security constraints)

### Functional Requirements - Digest Pipeline

- **FR-012**: System MUST implement 7-step digest pipeline: Discover → Fetch → Parse → Analyze → Summarize → Surface → Secure
- **FR-013**: System MUST support multi-language parsing: Python (AST), TypeScript (ts-morph), Go (go/ast), Rust (syn), Java (JavaParser)
- **FR-014**: System MUST generate SBOM (Software Bill of Materials) for all digested sources
- **FR-015**: System MUST scan for secrets (Gitleaks), vulnerabilities (Trivy/Grype), and perform static analysis (Semgrep)
- **FR-016**: System MUST produce knowledge graph (kg.json), embeddings, system_card.md, and profile.json for each digest

### Functional Requirements - P2P & Resources

- **FR-017**: System MUST support P2P connections for shared compute/storage across user devices
- **FR-018**: System MUST leverage excess hardware resources (CPU, GPU, RAM, Storage) across the P2P network
- **FR-019**: System MUST implement secure, encrypted communication between P2P nodes
- **FR-020**: System MUST gracefully degrade when P2P nodes disconnect

### Functional Requirements - UI & Interaction

- **FR-021**: System MUST provide a dynamic, context-aware UI that reconfigures based on current task
- **FR-022**: System MUST display a live, scrollable activity log of agent actions and decisions
- **FR-023**: System MUST support multi-modal interaction (text, voice, vision) where hardware permits
- **FR-024**: System MUST function with full UI capability offline

### Functional Requirements - Governance & Safety

- **FR-025**: System MUST implement constitutional governance with audit trail for all decisions
- **FR-026**: System MUST use biblical texts (original Greek/Hebrew) as source of absolute truth for ethical governance, transformed via lexical analysis → semantic embedding → knowledge graph integration pipeline
- **FR-027**: System MUST implement reward/correction mechanisms for agent compliance (rewards for obedience, testing loops for drift)
- **FR-028**: System MUST provide rollback capability for all self-modifications

### Functional Requirements - Directory Structure

- **FR-029**: System MUST create and populate `noa_root/sys/` with system-level components
- **FR-030**: System MUST create and populate `noa_root/p2p/` with peer-to-peer networking components
- **FR-031**: System MUST create and populate `noa_root/opt/` with optional packages (llama.cpp, llama-cpp-rs)
- **FR-032**: System MUST create and populate `noa_root/init/` with initialization scripts
- **FR-033**: System MUST create and populate `noa_root/containers/` with container definitions
- **FR-034**: System MUST create and populate `noa_root/config/` with configuration files
- **FR-035**: System MUST create and populate `noa_root/bin/` with executable binaries and wrappers
- **FR-036**: System MUST create and populate `noa_root/ai/` with AI providers, models, and prompts

---

### Glossary

| Term | Definition |
|------|------------|
| **NOA** | **N**ame **o**f **A**pp - the brand name for the entire autonomous agentic operating system. NOA encompasses all planes, agents, and infrastructure. When referring to "the system," use "NOA." |
| **CECCA** | **C**hief **E**xecutive **C**ommander **C**hief **A**gent - the root orchestrator agent that lives *inside* NOA. CECCA decomposes goals into tasks and coordinates all other agents. When referring to "the main agent," use "CECCA." |
| **MicroAgentStack (MAS)** | A deployable cluster of cooperative agents; code prefix: `mas_*` (reusable) or `gen_mas` (disposable) |
| **Minimum Hardware** | 8GB RAM, 4-core CPU, no dedicated GPU, 20GB storage - suitable for basic operation with single small model |
| **Standard Hardware** | 16GB RAM, 8-core CPU (x64/arm64), integrated GPU, 100GB+ storage - recommended for multi-SLM operation, used as baseline for latency targets (e.g., US2 "2 seconds", US7 "200 concurrent tasks") |
| **High-Performance Hardware** | 64GB+ RAM, 16+ core CPU, dedicated GPU (RTX 3080+), 500GB+ NVMe - optimal for local inference |
| **Development Hardware** | 512GB+ RAM, 24+ core CPU (Threadripper/EPYC), multi-GPU (2x RTX 5090+), 2TB+ NVMe, CUDA 13.1+ - full tensor parallelism |
| **Provider** | An AI model interface (local or cloud) that can execute inference tasks. See **Provider Priority** below. |

### Provider Priority & Fallback Order (FR-039)

When multiple providers are available, NOA uses this priority order:

| Priority | Provider | Type | Latency | Use Case |
|----------|----------|------|---------|----------|
| 1 | **llama.cpp** (5+ local models) | Local | <500ms | Primary inference - always available offline |
| 2 | **Cursor** (IDE/CLI/Cloud) | Hybrid | <1s | Code-aware tasks when IDE context available |
| 3 | **Claude Code** (CLI/Cloud/IDE) | Cloud | <2s | Complex reasoning, long context |
| 4 | **Codex** (CLI/Cloud/IDE) | Cloud | <2s | Code generation, completion |
| 5 | **VS Code Copilot** (IDE) | IDE | <1s | Inline completions when VS Code active |
| 6 | **Git CLI** | Local | <100ms | Version control operations |
| 7 | **Abacus** (CLI/Cloud) | Cloud | <3s | Specialized numerical/analytical tasks |

**Fallback Strategy**:
1. Always try local providers (llama.cpp) first for offline capability
2. If local fails/unavailable, try IDE providers if IDE context exists
3. If IDE unavailable, try cloud providers in priority order
4. If all fail, queue task and notify user after 3 retry attempts

### 3-Plane Self-Update Architecture

The 3-plane system enables zero-downtime autonomous self-updating with long-term memory persistence:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          3-PLANE ARCHITECTURE                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────┐    ┌──────────────────┐    ┌──────────────────┐      │
│  │  COORDINATOR     │    │    SANDBOX       │    │    DEPLOYED      │      │
│  │  (Analytics)     │    │   (Staging)      │    │  (Production)    │      │
│  │                  │    │                  │    │                  │      │
│  │ • Long-term      │    │ • Test new       │    │ • Serve live     │      │
│  │   memory         │    │   capabilities   │    │   requests       │      │
│  │ • Backups &      │    │ • Run selftest   │    │ • Canary deploy  │      │
│  │   archives       │    │ • Generate       │    │ • Feature flags  │      │
│  │ • llama.cpp      │    │   artifacts      │    │ • SLO monitoring │      │
│  │   analytics      │    │ • Policy enforce │    │ • Auto-rollback  │      │
│  │ • Promotion      │    │                  │    │                  │      │
│  │   decisions      │    │                  │    │                  │      │
│  └────────┬─────────┘    └────────┬─────────┘    └────────┬─────────┘      │
│           │                       │                       │                 │
│           └───────────────────────┼───────────────────────┘                 │
│                                   │                                         │
│                    ┌──────────────▼──────────────┐                          │
│                    │      SHARED INFRASTRUCTURE   │                          │
│                    │  • shared/state/   - Registry DB, capability metadata  │
│                    │  • shared/artifacts/ - Build outputs, SBOM, telemetry  │
│                    │  • shared/releases/ - Promoted releases (immutable)    │
│                    │  • shared/logs/     - Centralized audit logs           │
│                    │  • shared/config/   - Promotion policy, schemas        │
│                    │  • shared/runtime/  - Container specs, message bus     │
│                    └─────────────────────────────────────────────────────┘  │
│                                                                              │
│  PROMOTION FLOW:                                                             │
│  Sandbox(test) → Coordinator(analyze/decide) → Deployed(canary→full)        │
│                                                                              │
│  RISK TIERS (promotion-policy.yaml):                                         │
│  • Low:    unit + short_integration → 10% canary for 30min                  │
│  • Medium: unit + integration + soak:2h → 5% canary for 4h                  │
│  • High:   unit + integration + soak:8h + security + redteam → 1% for 24h   │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Plane Components** (each plane contains these 14 components):
- `agents/` - Agent orchestration and registry
- `dataplane/` - Data routing and transformation
- `memory/` - Memory management for that plane
- `models/` - Model loading and inference
- `networking/` - Inter-plane communication
- `observability/` - Metrics, logs, traces
- `orchestrator/` - Task scheduling
- `packaging/` - Artifact bundling
- `perception/` - Input processing
- `security/` - Auth, encryption, policy enforcement
- `system/` - System utilities
- `ui/` - UI runtime and state
- `update/` - Self-update mechanisms
- `workflow/` - Workflow execution engine

**Promotion Sequence:**
1. **Sandbox** develops and tests new capability, runs `make selftest`
2. Artifacts (SBOM, risk report, telemetry) pushed to `shared/artifacts/<capability>/`
3. **Coordinator** ingests artifacts, runs llama.cpp analytics for evaluation
4. Coordinator applies promotion policy gates (risk tier determines required tests)
5. If passed, Coordinator approves promotion and notifies **Deployed** plane
6. **Deployed** receives promoted capability, deploys as canary (cohort % based on risk tier)
7. Deployed monitors SLOs; if violated, triggers autopilot rollback
8. If canary succeeds for duration, promotes to full deployment
9. Coordinator logs decision trace and updates capability registry

**Abort Gates** (automatic rollback if exceeded):
- Latency: p95 +15%, p99 +25%
- Reliability: error_rate 0.2%, crash_rate 0.01%
- Safety: security_events = 0, constitutional_violations = 0

### Key Entities

- **NOA (CECCA)**: The Chief Executive Commander Chief Agent - root orchestrator that transforms goals into actionable work plans
- **Agent**: An autonomous unit that performs specific functions (specialized in planning, execution, QA, digestion, etc.)
- **MicroAgentStack**: A deployable cluster of cooperative agents assembled for a bounded objective
- **Capsule**: A self-contained environment with dependencies, policies, and versioning; supports blue/green deployment
- **Cell**: An atomic computational unit within a capsule (sensor, parser, router, reasoner, actuator, validator)
- **Memory**: Persistent storage of all interactions, decisions, learnings, and data
- **Knowledge Graph**: Linked representation of entities, relationships, functions, and dependencies
- **Digest**: The process and output of analyzing/extracting knowledge from a source
- **WorkPlan**: Structured representation of goal → tasks → checkpoints → deliverables → review gates

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: System initializes and becomes operational within 60 seconds on standard hardware
- **SC-002**: Local inference responds to queries within 2 seconds using llama.cpp on CPU-only hardware
- **SC-011**: Local inference responds to queries within 500ms on dedicated GPU hardware (single GPU)
- **SC-012**: Local inference responds to queries within 300ms on multi-GPU hardware (2+ GPUs with tensor parallelism)
- **SC-003**: Memory recall returns results within 500ms for any stored interaction
- **SC-004**: Digest pipeline processes a 10,000-file repository within 30 minutes
- **SC-005**: System handles 200 concurrent agent tasks with ≥98% success rate
- **SC-006**: P2P sync completes within 5 seconds for delta updates under 1MB
- **SC-007**: UI reconfigures for context switch within 200ms
- **SC-008**: System operates continuously for 7 days without requiring restart
- **SC-009**: All core functionality works identically across Windows, macOS, and Linux
- **SC-010**: Self-modification proposals include valid rollback path in 100% of cases

---

## Constitutional Compliance *(mandatory for NOA)*

### Data Locality & Offline Behavior

- **Offline Support**: ☑ Full - All core operations work offline
- **Data Residency**: All data stored under `noa_root` directory? ☑ Yes
- **External Dependencies**:
  - GitHub API (feature-flagged, optional)
  - OAuth providers for connectors (feature-flagged, optional)
  - Model download sources (one-time, cached locally)

### Agent Orchestration

- **Responsible Agents**:
  - NOA (CECCA) - Root orchestrator
  - ModelSelectorAgent - Model routing
  - DigestAgent - Knowledge acquisition
  - FileIOAgent - File operations
  - TerminalAgent - Shell/command execution
  - RAGAgent - Retrieval-augmented generation
  - MicroserviceManagementAgent - Service deployment
  - All Board Agents (Legal, Finance, Operations, Security, etc.)

- **Multi-SLM Compatibility**: ☑ Yes - Uses llama.cpp with multiple <3B parameter models
- **Orchestration Pattern**: Hierarchical with NOA at root, delegating to Board Agents, then to MicroAgentStacks

### Memory & P2P Considerations

- **Memory Persistence**:
  - Local-first database (SQLite/PostgreSQL with pgvector)
  - CAS (Content-Addressable Storage) for artifacts
  - Vector embeddings for semantic search

- **P2P Resource Sharing**: ☑ Supported
- **Cross-Device Sync**:
  - P2P discovery on local network
  - Encrypted sync with conflict resolution
  - Delta updates for efficiency

### Constitutional Flow

| Level | Document | Link |
|-------|----------|------|
| Goal | G-NOA-001 | [project-mgmt.md High-Level Goals] |
| Policy | NOA Constitution v2.0.0 | [memory/constitution.md] |
| Rule | Constitutional Principles §3.1-§3.12 | [memory/constitution.md#3-core-principles] |
| Execution | Universal Task Execution Policy | [05-policy/universal_task_execution_policy.md] |

### Universal Task Execution Policy Integration

All task execution MUST comply with [Universal Task Execution Policy](../../project-mgmt/docs/05-policy/universal_task_execution_policy.md):

**Truth Sources (§0 Priority Order)**:
1. User-provided files and chat (highest)
2. Computations done with shown work
3. Cited external sources with dates
4. Model prior (lowest)

**Hard Stop Rule**: If any required check fails, do not proceed. Return FAIL + reasons + remedy.

**Clean Code Artifacts (§9)**: All implementations MUST produce:

| Artifact | Location | Contents |
|----------|----------|----------|
| `FINAL_REPORT.md` | `noa_root/test-results/` | Claims table, evidence ledger, gap scan |
| `HASHES.txt` | `noa_root/test-results/` | SHA-256 for all key artifacts |
| `COVERAGE.md` | `noa_root/test-results/` | Requirements → artifacts → tests mapping |
| `REPRO.md` | `noa_root/test-results/` | Exact environment, versions, commands |
| `EVIDENCE_LEDGER.md` | `noa_root/test-results/` | Files, data, citations, Triple-Verify outcomes |

**Triple-Verification Protocol (§5.6)**:
- **Pass A**: Self-check (internal consistency, spec ↔ artifacts ↔ tests)
- **Pass B**: Independent re-derivation (fresh clone, recompute metrics)
- **Pass C**: Adversarial check (negative tests, boundary cases, cross-tool)

**Rich Metadata Requirements**:
- All data entities MUST include: `id`, `created_at`, `updated_at`, `checksum`
- All embeddings MUST include: `model`, `vector`, `source_type`, `source_id`
- All configurations MUST follow schemas in `config/schemas/`
- All artifacts MUST have SHA-256 hashes in `HASHES.txt`

**Cross-Reference**: See [verification.md](./checklists/verification.md) for complete Truth Gate checklist (248 items)

---

## Technical Architecture Overview

### Language Requirements (22 Languages)

| Language | Version | Purpose | Location |
|----------|---------|---------|----------|
| **Rust** | 1.75+ | Core runtime, llama-cpp-rs bindings, performance-critical | `noa_root/sys/core/`, `noa_root/opt/llama-cpp-rs/` |
| **Go** | 1.21+ | Network services, P2P layer, CLI tools | `noa_root/sys/services/`, `noa_root/p2p/` |
| **TypeScript** | 5.x | Frontend UI, agent definitions, configuration | `noa_root/sys/ui/`, `noa_root/ai/agents/` |
| **JavaScript** | ES2022+ | Runtime scripts, browser extensions, quick prototypes | `noa_root/scripts/`, `noa_root/bin/` |
| **Python** | 3.11+ | ML/AI integration, digest pipeline, analysis tools | `noa_root/ai/providers/`, `noa_root/sys/digest/` |
| **C/C++** | C++17 | llama.cpp core inference engine (submodule) | `noa_root/opt/llama.cpp/` |
| **YAML** | 1.2 | Configuration, workflow definitions, K8s manifests | `noa_root/config/`, `noa_root/containers/` |
| **JSON** | - | Data interchange, manifests, schemas, agent configs | `noa_root/config/`, `noa_root/ai/` |
| **TOML** | 1.0 | Rust configuration, project settings | `noa_root/opt/*/Cargo.toml` |
| **Markdown** | - | Documentation, prompts, reports, system cards | `noa_root/ai/prompts/`, `noa_root/docs/` |
| **Shell/Bash** | 5.x | Linux/macOS scripts, init scripts | `noa_root/scripts/bash/`, `noa_root/init/` |
| **PowerShell** | 7.x | Windows scripts, automation | `noa_root/scripts/powershell/` |
| **SQL** | SQLite/PostgreSQL | Database schema, migrations, queries | `noa_root/init/migrations/` |
| **Protobuf** | 3.x | P2P protocol definitions, gRPC services | `noa_root/contracts/*.proto` |
| **OpenAPI** | 3.1 | REST API specifications | `noa_root/contracts/*.yaml` |
| **GraphQL** | - | API schema (optional alternative to REST) | `noa_root/contracts/` |
| **HTML** | 5 | UI templates, web components | `noa_root/sys/ui/` |
| **CSS/SCSS** | 3/- | Styling, Tailwind, advanced styling | `noa_root/sys/ui/`, `noa_root/docs/assets/css/` |
| **SVG** | 1.1 | Vector graphics, icons, diagrams | `noa_root/docs/assets/` |
| **PNG/WebP** | - | Raster images, screenshots | `noa_root/docs/assets/` |
| **CSV** | - | Task tables, data export, logging, agent manifests | `noa_root/data/` |
| **GBNF** | - | Grammar files for llama.cpp structured output | `noa_root/ai/grammars/` |
| **Dockerfile** | - | Container definitions, multi-stage builds | `noa_root/containers/` |

### Directory Structure Specification

```
noa_root/
├── sys/                    # System-level components
│   ├── core/              # Core runtime (Rust)
│   ├── services/          # Network services (Go)
│   ├── ui/                # Dynamic UI (TypeScript)
│   ├── digest/            # Digest pipeline (Python)
│   └── kernel/            # Portable kernel components
│
├── p2p/                    # Peer-to-peer networking
│   ├── discovery/         # Device discovery
│   ├── sync/              # State synchronization
│   ├── compute/           # Distributed compute
│   └── storage/           # Distributed storage
│
├── opt/                    # Optional packages
│   ├── llama.cpp/         # Core inference engine
│   ├── llama-cpp-rs/      # Rust bindings
│   └── [extensible]/      # Future packages
│
├── init/                   # Initialization
│   ├── noa-init           # Main init script
│   ├── bootstrap/         # Bootstrap sequences
│   └── migrations/        # Database migrations
│
├── containers/             # Container definitions
│   ├── capsules/          # Capsule specs
│   ├── compose/           # Docker Compose files
│   └── k8s/               # Kubernetes manifests
│
├── config/                 # Configuration
│   ├── ai-providers.json  # AI provider configs
│   ├── noa-server.json    # Server settings
│   ├── device-orchestration.json
│   └── [feature].json     # Feature-specific configs
│
├── bin/                    # Executables
│   ├── llama-cli          # LLM CLI
│   ├── llama-server       # LLM server
│   ├── node               # Node.js runtime
│   ├── npm/npx            # Package managers
│   └── ollama             # Ollama wrapper
│
├── ai/                     # AI components
│   ├── providers/
│   │   ├── cloud/         # Cloud providers (optional)
│   │   └── local/         # Local providers (llama.cpp, ollama)
│   ├── shared/
│   │   ├── commands/      # Shared commands
│   │   └── prompts/       # Shared prompts
│   ├── models/            # Model storage
│   ├── agents/            # Agent definitions
│   └── grammars/          # GBNF grammar files
│
├── data/                   # Persistent data
│   ├── memory/            # Memory store
│   ├── knowledge/         # Knowledge graphs
│   ├── embeddings/        # Vector embeddings
│   └── artifacts/         # CAS artifact store
│
├── etc/                    # System configuration
│   ├── docker/            # Docker daemon config
│   ├── ssh/               # SSH keys/config
│   └── [system].conf      # System configs
│
├── docs/                   # Documentation
│   ├── setup/             # Setup guides
│   ├── assets/            # Images, diagrams
│   └── api/               # API documentation
│
└── scripts/                # Utility scripts
    ├── bash/              # Bash scripts
    ├── powershell/        # PowerShell scripts
    └── setup/             # Setup scripts
```

---

## Assumptions

1. User has administrative/root access to install NOA on their device
2. Hardware tiers (see Glossary):
   - **Minimum**: 8GB RAM, 4-core CPU, no dedicated GPU - baseline operation with single small model
   - **Standard**: 16GB RAM, 8-core CPU, integrated GPU - recommended for multi-SLM operation
   - **High-Performance**: 64GB+ RAM, 16+ cores, dedicated GPU - optimal for local inference
   - **Development**: 512GB+ RAM, 24+ cores, multi-GPU - full development with tensor parallelism
3. Minimum 20GB storage for base installation (100GB+ recommended, 2TB+ for development hardware)
4. Network access available for initial setup (downloading models, packages)
5. Standard web browser available for UI interaction
6. For P2P: devices on same local network or VPN for initial discovery
7. For multi-GPU: CUDA 13.1+ toolkit with tiles support required

---

## Dependencies

- **llama.cpp**: Core inference engine for local SLMs
- **SQLite/PostgreSQL**: Local-first database with pgvector 0.5.0+ extension
- **Redis 7.0+**: Event bus for inter-service communication (optional, falls back to in-process bus)
- **Qdrant 1.8+**: Vector store for embeddings (or sqlite-vss for lightweight deployments)
- **Node.js 20+**: JavaScript runtime for UI and scripts
- **Rust toolchain 1.83+**: For core system components (latest stable as of Dec 2024)
- **Go toolchain 1.23+**: For network services (latest stable as of Dec 2024)
- **Python 3.12+**: For AI/ML integration and digest pipeline (latest stable)
- **Docker/containerd**: Optional, for container-based deployment

---

## CLI Tool Prerequisites (CRITICAL)

Before building or running NOA, the following CLI tools MUST be installed:

### Build Toolchains (CRITICAL - Required for Compilation)

| Tool | Minimum Version | Install Command (Windows) | Purpose |
|------|-----------------|---------------------------|---------|
| **Rust** | 1.83.0 | `winget install Rustlang.Rustup && rustup default stable` | Core runtime, llama-cpp-rs bindings |
| **Go** | 1.23.0 | `winget install GoLang.Go` | P2P services, network layer |
| **Node.js** | 20.0.0 | `winget install OpenJS.NodeJS.LTS` | UI, scripts, agent definitions |
| **Python** | 3.12.0 | `winget install Python.Python.3.12` | Digest pipeline, ML integration |
| **protoc** | 28.0.0 | `winget install Google.Protobuf` | P2P protocol buffer compilation |

### Code Quality Tools (HIGH - Required for Quality Gates)

| Tool | Minimum Version | Install Command | Purpose |
|------|-----------------|-----------------|---------|
| **rustfmt** | (bundled) | `rustup component add rustfmt` | Rust code formatting |
| **clippy** | (bundled) | `rustup component add clippy` | Rust linting |
| **golangci-lint** | 1.62.0 | `go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest` | Go linting |
| **eslint** | 9.0.0 | `npm install -g eslint` | TypeScript/JavaScript linting |
| **ruff** | 0.8.0 | `pip install ruff` | Python linting (fast) |

### Security Scanning Tools (HIGH - Required for FR-015)

| Tool | Minimum Version | Install Command | Purpose |
|------|-----------------|-----------------|---------|
| **Gitleaks** | 8.21.0 | `choco install gitleaks` or `brew install gitleaks` | Secrets detection |
| **Trivy** | 0.57.0 | `choco install trivy` or `brew install trivy` | Vulnerability scanning |
| **Grype** | 0.84.0 | `choco install grype` or `brew install grype` | SBOM vulnerability matching |
| **Semgrep** | 1.97.0 | `pip install semgrep` | Static analysis |

### Optional Tools

| Tool | Version | Install Command | Purpose |
|------|---------|-----------------|---------|
| **Docker** | 27.0.0+ | `winget install Docker.DockerDesktop` | Container deployment |
| **kubectl** | 1.31.0+ | (bundled with Docker Desktop) | K8s management |
| **Make** | 4.4.0+ | `choco install make` | Build automation |

### Prerequisite Check Script

Run the following to verify all prerequisites:

```bash
# Unix/macOS
./scripts/bash/check-prerequisites.sh

# Windows PowerShell
.\scripts\powershell\check-prerequisites.ps1
```

The script will output:
- ✅ Tool installed with version
- ❌ Tool missing with install command
- ⚠️ Tool installed but version too old

---

## Out of Scope (for this foundation release)

- Full mobile companion apps (stub implementation only)
- XR/AR/VR interfaces (architecture support but no implementation)
- Enterprise multi-tenant deployment
- Cloud-native distributed deployment
- Full CRM strangler implementation (shadow mode only)
- Complete biblical text ML transformation (reference implementation only)

---

## Clarifications

### Session 2025-12-08

- Q: What triggers autonomous task initiation when no human submits a goal? → A: Always-on continuous loop. User provides co-improvement goals; system continuously loops to complete those goals while always researching and implementing ways to optimize and use resources more efficiently and effectively.
- Q: How does NOA apply code changes to itself while running? → A: 3-plane architecture with Coordinator/Sandbox/Deployed planes. New capabilities are developed in Sandbox, analyzed by Coordinator (llama.cpp analytics), then promoted to Deployed with canary rollout. Coordinator maintains long-term memory and backups. No downtime.
- Q: Which operations require human approval vs. proceed autonomously? → A: Full autonomy. NOA executes ALL operations autonomously without per-operation human approval. User only sets initial co-improvement goals. Rollback via 3-plane system is the safety net for failed changes.
- Q: Can NOA generate its own improvement goals or only execute user-provided goals? → A: Full goal autonomy. NOA can generate ANY goal it determines beneficial, including goals unrelated to current user objectives. Constitutional governance (FR-025, FR-026) provides the ethical boundary.
- Q: What self-healing mechanism should NOA use for autonomous operation? → A: Full self-healing loop. Proactive detection → diagnosis → auto-fix → validation → escalation-to-user ONLY if all auto-fix attempts fail. Maintains always-on operation.
- Q: What is the role of each plane in the 3-plane system? → A: **Coordinator** is the constant plane for long-term memory, backups, archives, analytics, and promotion decisions. **Sandbox** is for testing/staging new capabilities. **Deployed** is production serving live traffic with canary deployments and auto-rollback on SLO violation.
