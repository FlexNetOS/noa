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

**Prerequisite Commands (platform-aware):**
- Bash/WSL/macOS: `scripts/bash/check-prerequisites.sh --json`
- PowerShell/Windows: `scripts/powershell/check-prerequisites.ps1 -Json`

**Architecture References:**
- [NOA AER Spec v2](../noa_aer_spec_v2.md) - High-level Agentic Environment Runtime architecture
- [AGENT.md](../AGENT.md) - Agent execution guidelines and 4-D methodology
- [NOA Constitution](../../CONSTITUTION.md) - Core principles and governance

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
4. **Given** platform prerequisites need verification, **When** I run the prereq check, **Then** I invoke the platform-appropriate shim (`scripts/bash/check-prerequisites.sh --json` on Bash/WSL/macOS or `scripts/powershell/check-prerequisites.ps1 -Json` on PowerShell/Windows) and receive ✅/❌/⚠️ results for all required tools

---

### User Story 2 - Multi-SLM Neural Runtime (Priority: P1)

As a user, I want NOA to run multiple Small Language Models locally so that I can have intelligent processing without depending on external AI services.

**Why this priority**: The neural runtime is the "brain" of NOA - all agentic operations depend on local inference capability. This enables offline intelligence.

**Independent Test**: Can be tested by loading an SLM model, sending a prompt, and verifying response within latency targets.

**Model Selection Criteria** (used by ModelSelectorAgent):
1. **Task Type Matching**: code generation → code-specialized model, reasoning → reasoning model, summarization → general model
2. **Context Length**: task requires >4K tokens → large-context model, otherwise → efficient model
3. **Latency Requirements**: real-time interaction → fastest model, background task → most capable model
4. **Resource Availability**: GPU available → GPU-optimized model, CPU-only → quantized model
5. **Historical Performance**: model with best accuracy on similar past tasks (tracked in execution memory)
6. **Cost/Efficiency**: for equivalent capability, prefer smaller/faster model

**Acceptance Scenarios**:

1. **Given** NOA is running with llama.cpp configured, **When** I send a query to the neural runtime, **Then** I receive a response within 2 seconds on standard hardware (see Glossary for hardware tier definitions)
2. **Given** multiple SLMs are loaded, **When** the ModelSelectorAgent routes a task, **Then** it selects the optimal model using the criteria above and logs the selection rationale
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

**Beneficial Modification Triggers** (when NOA proposes self-improvement):
1. **Performance Degradation**: latency >20% above baseline for 24+ hours
2. **Error Rate Increase**: error rate >5% above baseline for 100+ operations
3. **Resource Inefficiency**: CPU/memory usage >30% above baseline for equivalent workload
4. **Repeated Failures**: same task type fails >3 times with same error pattern
5. **User Feedback**: explicit user correction of NOA behavior (logged as improvement signal)
6. **Pattern Recognition**: identifies code duplication, unused branches, or optimization opportunities via static analysis
7. **Dependency Update**: new version of dependency available with security/performance improvements

**Acceptance Scenarios**:

1. **Given** NOA detects inefficiency via any trigger above, **When** it proposes an improvement, **Then** the proposal includes before/after comparison, expected benefit, and rollback path
2. **Given** a self-modification is applied, **When** tests fail, **Then** the system automatically rolls back to the previous state
3. **Given** continuous operation, **When** NOA self-improves, **Then** all changes are logged with trigger, rationale, and audit trail

---

### User Story 9 - Cross-Platform Deployment (Priority: P3)

As a user, I want NOA to run on Windows, macOS, Linux, mobile devices, laptops, ipads, tablets, and XR devices so that I can use it everywhere.

**Why this priority**: Cross-platform reach is essential for the P2P hive-mind vision, but requires the core system to be stable first.

**Independent Test**: Can be tested by running the same NOA seed on different platforms and verifying consistent behavior.

**Acceptance Scenarios**:

1. **Given** the NOA seed binary, **When** run on Windows 11, macOS, or Ubuntu, **Then** core functionality works identically
2. **Given** a mobile companion app (stub implementation: P2P connectivity only, no full mobile features), **When** connected to the desktop NOA, **Then** it participates in the P2P hive-mind
3. **Given** different hardware capabilities, **When** NOA initializes, **Then** it adapts its resource usage to available hardware

---

### User Story 10 - Connectors & External Integration (Priority: P3)

As a user, I want NOA to connect to my existing accounts and services (Gmail, GitHub, cloud storage) so that it can integrate with my digital life.

**Why this priority**: External integrations extend NOA's reach but are secondary to core autonomous operation. These must be optional and feature-flagged.

**Independent Test**: Can be tested by configuring a connector and verifying data sync.

**OAuth Permission Scopes** (enumerated per connector):
- **GitHub**: `repo` (read/write repositories), `read:user` (read profile), `read:org` (read org membership) - minimum required
- **Gmail**: `gmail.readonly` (read emails), `gmail.send` (send emails) - minimum required
- **Google Drive**: `drive.file` (access files created by NOA only), `drive.metadata.readonly` (list files) - minimum required
- **Dropbox**: `files.content.read`, `files.content.write`, `files.metadata.read` - minimum required
- Additional scopes MAY be requested with explicit user consent via UI prompt

**Acceptance Scenarios**:

1. **Given** a configured OAuth connector for GitHub, **When** enabled, **Then** NOA can access repositories with the enumerated permissions (repo, read:user, read:org)
2. **Given** an OAuth callback is received, **When** the token exchange completes, **Then** the connector is authenticated within 5 seconds
3. **Given** connectors are configured, **When** the network is unavailable, **Then** NOA continues operating with cached data
4. **Given** a connector is disabled via feature flag, **When** that feature is accessed, **Then** it degrades gracefully with clear user feedback

---

### User Story 11 - Desktop Application Hosting (Priority: P2)

As a user, I want NOA to host my desktop applications (ChatGPT Desktop, Claude Desktop, GitHub Desktop) within its containment layer, so that all application data is stored in `noa_root` and I have unified control over these apps.

**Why this priority**: Desktop app hosting completes NOA's role as the central hub for all AI-powered tools, providing unified data management and control while maintaining the self-containment principle.

**Constitutional Reference**: §3.1 Self-Contained & Autonomous

**Independent Test**: Can be tested by launching a hosted app and verifying its data writes to `$NOA_DATA/apps/` instead of system paths.

**Hosted Applications**:

| Application | Install Path | Data Path | Notes |
|-------------|--------------|-----------|-------|
| ChatGPT Desktop | `$NOA_OPT/apps/chatgpt/` | `$NOA_DATA/apps/chatgpt/` | Electron app |
| Claude Desktop | `$NOA_OPT/apps/claude/` | `$NOA_DATA/apps/claude/` | Electron app |
| GitHub Desktop | `$NOA_OPT/apps/github-desktop/` | `$NOA_DATA/apps/github-desktop/` | Electron app |

**Architecture Components**:
- **NDCL (NOA Desktop Containment Layer)**: Provides environment redirection, network isolation, and process sandboxing
- **Network Proxy**: Routes desktop app traffic through NOA for inspection and P2P routing
- **OAuth Proxy**: Captures and manages authentication tokens for all hosted apps
- **Display Forwarding**: Enables GUI rendering in VM/container modes

**Acceptance Scenarios**:

1. **Given** ChatGPT Desktop is installed via NOA, **When** launched via `noa run chatgpt`, **Then** all data is written to `$NOA_DATA/apps/chatgpt/` not system paths
2. **Given** Claude Desktop is running in NOA containment, **When** it authenticates via OAuth, **Then** tokens are stored in NOA's credential vault
3. **Given** GitHub Desktop is hosted by NOA, **When** network proxy is enabled, **Then** all traffic passes through NOA's proxy service
4. **Given** a desktop app is launched, **When** NOA is running in VM mode, **Then** the app GUI is forwarded to the host display
5. **Given** a desktop app attempts to write to system paths, **When** NDCL is active, **Then** writes are redirected to `noa_root`

---

### Edge Cases

- What happens when available storage is exhausted? → System enters resource-scarcity mode (AMPK-mode), pauses non-essential operations, and alerts user
- What happens when a model file is corrupted? → Model integrity is verified on load; corrupted models are quarantined and re-downloaded if available
- What happens when two P2P nodes have conflicting state? → Conflict resolution uses last-write-wins with full audit trail; user can review conflicts via UI or admin CLI command `noa conflicts list`, `noa conflicts resolve <id>`
- What happens when an agent enters an infinite loop? → Timeout (default 60s, configurable per-agent) and circuit breaker (3 failures in 5 minutes) mechanisms terminate runaway tasks
- What happens when offline for extended periods then reconnecting? → Sync protocol handles delta updates with conflict detection
- What happens when biblical governance rules conflict with user requests? → Constitutional governance takes precedence; user is informed of constraint
- What happens when maximum memory capacity is reached? → Oldest entries are archived (not deleted) to `noa_root/data/archives/memory/`; upper bound configurable (default 10GB)
- What happens when all agents are simultaneously busy? → Tasks are queued with priority ordering (P1 > P2 > P3); queue size limit 1000 tasks; overflow logged and user notified

### Scenario Coverage - Lifecycle Operations

**Upgrade/Migration Requirements (FR-145)**:
- System MUST detect existing NOA installation version via `noa_root/config/version.json`
- System MUST support in-place upgrade: backup current state → apply migrations → verify integrity → resume
- Migration path MUST be defined for each major version (e.g., v1.x → v2.x)
- Migrations MUST be idempotent (safe to run multiple times)
- Rollback MUST be available for 72 hours after upgrade via `noa rollback-upgrade`

**Uninstall/Cleanup Requirements (FR-146)**:
- System MUST provide clean uninstall via `scripts/uninstall.ps1` (Windows) or `scripts/uninstall.sh` (Unix)
- Uninstall MUST remove: binaries, services, scheduled tasks, environment variables, PATH modifications
- Uninstall MUST preserve (by default, --purge flag to remove): `noa_root/data/`, `noa_root/memory/`, `noa_root/config/`
- Uninstall MUST log all removed items to `noa_root/logs/uninstall.log` (preserved unless --purge)

### Scenario Coverage - Recovery Operations

**Model Download Interruption & Resume (FR-147)**:
- System MUST track download progress in `noa_root/tmp/downloads/<model-id>.progress.json`
- System MUST resume interrupted downloads from last completed chunk (chunk size: 10MB)
- System MUST verify partial downloads via checksum before resume
- System MUST timeout stale downloads after 24 hours and restart from scratch

**Database Corruption Recovery (FR-148)**:
- System MUST detect database corruption via integrity checks on startup (`PRAGMA integrity_check`)
- System MUST maintain automatic backups: hourly (last 24), daily (last 7), weekly (last 4)
- Recovery procedure: (1) attempt WAL recovery, (2) restore from latest backup, (3) alert user if all backups corrupt
- Backup location: `noa_root/data/backups/`

**Out-of-Memory Handling During Inference (FR-149)**:
- System MUST monitor memory usage during inference (threshold: 90% of available RAM)
- On OOM risk: (1) pause new inference requests, (2) unload least-recently-used model, (3) retry original request
- If OOM occurs despite mitigation: gracefully terminate inference, return error to caller, log incident
- System MUST implement model layer offloading to disk when GPU VRAM is exhausted

**Authentication Token Expiry Handling (FR-150)**:
- System MUST track token expiry times for all OAuth connectors
- System MUST proactively refresh tokens when within 10% of expiry (e.g., 1-hour token refreshes at 54 minutes)
- If refresh fails: (1) retry 3 times with exponential backoff, (2) notify user, (3) continue with cached data
- Expired tokens MUST NOT be used for API calls; system MUST return clear error to user

---

## Requirements *(mandatory)*

### Functional Requirements - Core System

- **FR-001**: System MUST operate entirely inside `noa_root` directory with no hard dependencies on external paths
- **FR-002**: System MUST function offline for core local operations (task management, agent orchestration, local inference) when user chooses offline mode; internet is required for P2P and cloud provider features
- **FR-152**: System MUST support model sideloading via USB/file transfer for offline scenarios:
  - Copy `.gguf` model files to `noa_root/ai/models/`
  - System detects and registers sideloaded models on next startup
  - Sideloaded models MUST pass integrity verification (SHA-256 checksum in `model.sha256` companion file)

### Functional Requirements - Observability Stack

- **FR-153**: System MUST implement observability using Rust ecosystem:
  - `tracing` + `tracing-subscriber` for structured logging and spans
  - `opentelemetry` with OTLP exporter for distributed tracing
  - `opentelemetry-prometheus` for metrics exposition
- **FR-154**: System MUST expose Prometheus-format metrics at `GET /metrics` endpoint
- **FR-155**: System MUST export traces via OTLP to configurable endpoint (Tempo, Jaeger, or similar)
- **FR-156**: System MUST persist metrics to local SQLite store (`noa_root/data/metrics.db`) for offline analysis
- **FR-157**: System MUST NOT require Docker for core observability functionality; Grafana/Prometheus/Tempo are optional external services
- **FR-158**: System MUST provide built-in metrics dashboard via UI when external Grafana unavailable
- **FR-003**: System MUST provide a local-first database that handles concurrent modifications and supports future multi-device sync
- **FR-004**: System MUST support multiple Small Language Models via llama.cpp with dynamic model loading/unloading (minimum 5 concurrent models)

### Functional Requirements - Shared Provider Execution Memory

- **FR-037**: System MUST implement a Shared Provider Execution Memory bus where multiple model providers share context and reasoning state
- **FR-038**: System MUST support collaborative reasoning where models reason together and execute separate tasks simultaneously
- **FR-039**: System MUST integrate minimum 8 provider types: llama.cpp (5+ local models), Claude Code (CLI/Cloud/IDE), Codex (CLI/Cloud/IDE), VS Code Copilot (IDE), Git CLI, Cursor (IDE/CLI/Cloud), Abacus (CLI/Cloud)
- **FR-040**: System MUST persist shared execution memory across sessions for continuity; SharedExecutionContext entries MUST be retained for minimum 90 days; entries older than 90 days MAY be archived to `noa_root/data/archives/execution-memory/`
- **FR-041**: System MUST implement parallel task distribution across all active providers; distribution MUST complete within 5 seconds or tasks are queued for retry with exponential backoff
- **FR-042**: System MUST synchronize provider state to enable coordinated multi-model workflows

### Functional Requirements - Rate Limiting & Throttling

- **FR-095**: System MUST implement per-provider rate limits with configurable token/request budgets stored in `config/rate-limits.json`
- **FR-096**: System MUST use exponential backoff (initial 1s, max 60s, factor 2x) when receiving HTTP 429 responses from cloud providers
- **FR-097**: System MUST throttle P2P requests based on peer-reported capacity and network conditions
- **FR-098**: System MUST rate-limit self-generated goals to maximum 10 new goals per hour to prevent runaway task creation
- **FR-099**: System MUST track rate limit state in Shared Provider Execution Memory for coordinated multi-provider operations

### Functional Requirements - Authentication & Identity

- **FR-100**: System MUST generate an Ed25519 keypair per device on initialization, stored encrypted in `noa_root/config/device-identity.enc`
- **FR-101**: System MUST encrypt device keys with a user-provided master passphrase using Argon2id key derivation
- **FR-102**: System MUST support device pairing via QR code (time-limited encrypted token, expires in 5 minutes)
- **FR-103**: System MUST support device pairing via 6-digit PIN displayed on new device, entered on existing device
- **FR-104**: System MUST support device pairing via Bluetooth/NFC proximity verification when hardware available
- **FR-105**: System MUST support device pairing via encrypted trust bundle file transfer (USB/shared folder)
- **FR-106**: System MUST maintain a device trust registry in `noa_root/config/trusted-devices.json` with device public keys and approval timestamps
- **FR-107**: System MUST require all agents to sign actions with the device key for audit trail verification
- **FR-108**: System MUST use mutual TLS with device keys for all P2P connections between user's devices
- **FR-109**: System SHOULD support optional browser password manager integration for master passphrase (web UI convenience only)

### Functional Requirements - Accessibility & Internationalization

- **FR-110**: System MUST comply with WCAG 2.1 Level AAA for all UI components
- **FR-111**: System MUST support full keyboard navigation with visible focus indicators (contrast ratio ≥7:1)
- **FR-112**: System MUST provide screen reader compatibility with ARIA labels for all interactive elements
- **FR-113**: System MUST support high contrast mode and respect OS accessibility preferences
- **FR-114**: System MUST externalize all UI strings to `noa_root/config/i18n/{locale}.json` files
- **FR-115**: System MUST bundle translations locally (no cloud translation service dependency)
- **FR-116**: System MUST support RTL (right-to-left) layout for Arabic, Hebrew, and other RTL languages
- **FR-117**: System MUST provide locale detection from OS settings with manual override option
- **FR-118**: System MUST support dynamic locale switching without restart
- **FR-119**: System SHOULD include English, Spanish, Chinese (Simplified), Arabic, and Hebrew translations at launch

### Functional Requirements - UI States & Feedback

- **FR-120**: System MUST display skeleton loaders (animated placeholders) for content areas during data fetch operations
- **FR-121**: System MUST provide a persistent status bar showing background operation status (P2P sync, model loading, AI processing)
- **FR-122**: System MUST use toast notifications for transient errors with "retry" action when applicable
- **FR-123**: System MUST always display cached/partial data when available, with visual indicator that sync is in progress
- **FR-124**: System MUST show meaningful empty states with suggested actions (e.g., "No memories yet. Start a conversation to create your first memory.")
- **FR-125**: System MUST indicate offline status clearly in the status bar with automatic reconnection attempts
- **FR-126**: System MUST queue failed operations for retry when connection is restored
- **FR-127**: System MUST provide progress indicators for long-running operations (>2s) showing estimated time remaining when calculable

### Functional Requirements - Feature Flags & Configuration

- **FR-137**: System MUST implement feature flags via `noa_root/config/feature-flags.json` with runtime toggle capability
- **FR-138**: System MUST support feature flag scopes: `global`, `per-user`, `per-device`, `per-provider`
- **FR-139**: System MUST log all feature flag changes to audit trail with before/after state, timestamp, and trigger source
- **FR-140**: System MUST provide graceful degradation when a feature-flagged capability is disabled (clear user feedback, no crashes)
- **FR-141**: System MUST allow feature flags to be toggled without restart (runtime reload from config file)

### Functional Requirements - Multi-Modal Interaction

- **FR-128**: System MUST support speech-to-text via local Whisper model with <500ms latency on standard hardware
- **FR-129**: System MUST support text-to-speech via local TTS (Piper/Coqui) with voice selection and speed control
- **FR-130**: System MUST support camera input for real-time visual context when hardware available
- **FR-131**: System MUST support screen capture for screenshot-based queries and context sharing
- **FR-132**: System MUST support image file analysis (PNG, JPEG, WebP) via local multimodal models (LLaVA or similar)
- **FR-133**: System MUST gracefully degrade when multi-modal hardware is unavailable (fall back to text)
- **FR-134**: System MUST store multi-modal models in `noa_root/ai/models/multimodal/` with lazy loading
- **FR-135**: System MUST support voice activation wake word detection for hands-free operation
- **FR-136**: System SHOULD support XR/AR glasses integration via camera stream and spatial audio output

### Functional Requirements - Memory & Logging

- **FR-005**: System MUST persist all interactions, decisions, and learnings for instant recall (Total Memory Sovereignty). Explicit exclusions: System MUST NOT persist temporary inference cache data, expired session tokens, raw model weights during inference, or intermediate computation buffers. These exclusions are necessary for performance and storage efficiency.
- **FR-006**: System MUST log all agent actions with who/what acted, why, and what changed

### Functional Requirements - Advanced Learning Techniques

- **FR-043**: System SHOULD implement ToolkenGPT for pre-trained tool tokens that plug into larger models. Implementation criteria: (1) tool token vocabulary of minimum 50 tool types, (2) integration with at least 2 local SLMs, (3) tool invocation latency <100ms overhead
- **FR-044**: System SHOULD implement Replay Memory Cache for short-term memory with external knowledge base. Capacity limits: maximum 10,000 cached items per session; eviction policy: LRU (Least Recently Used) with 24-hour TTL for inactive items
- **FR-045**: System SHOULD implement EWC (Elastic Weight Consolidation) for continual learning without catastrophic forgetting. Trigger conditions: consolidation occurs after (1) 1000 new training examples, (2) significant performance degradation (>10% accuracy drop), or (3) manual trigger via admin command
- **FR-046**: System SHOULD implement Meta-Learning (MAML) for rapid adaptation to new tasks with few examples. "Few examples" defined as: 1-shot (minimum), 5-shot (typical), 10-shot (maximum) for task adaptation

### Functional Requirements - Multi-GPU Support

- **FR-047**: System MUST enumerate all available CUDA GPUs and distribute model layers across devices when multiple GPUs are present
- **FR-048**: System MUST support tensor parallelism across multiple GPUs for models exceeding single GPU memory
- **FR-049**: System SHOULD leverage NVLink when available for high-bandwidth inter-GPU communication *(Development Hardware tier only - 2x RTX 5090+ with NVLink bridge)*
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

- **FR-007**: System MUST implement NOA (CECCA) as the root orchestrator that decomposes goals into tasks. CECCA delegation rules: (1) tasks requiring file operations → FileIOAgent, (2) tasks requiring shell execution → TerminalAgent, (3) tasks requiring context retrieval → RAGAgent, (4) tasks requiring service management → MicroserviceManagementAgent, (5) tasks requiring specialized domain execution → appropriate Executive Agent (see FR-142), (6) tasks requiring advisory analysis → consult appropriate Board Agent (see FR-184), (7) complex multi-step tasks → spawn MicroAgentStack
- **FR-008**: System MUST support specialized permanent agents with these acceptance criteria:
  - **FileIOAgent**: Read/write files within `noa_root` in <100ms for files <10MB. Failure recovery: retry 3x with exponential backoff, then escalate to CECCA with error context
  - **TerminalAgent**: Execute shell commands with timeout (default 30s), capture stdout/stderr. Failure recovery: timeout → kill process, capture partial output, report to CECCA
  - **RAGAgent**: Retrieve relevant context from memory in <500ms with >80% relevance (measured via NDCG@10 on benchmark query set stored in `noa_root/test-data/rag-benchmark.json`). Failure recovery: cache miss → expand search scope, still fail → return empty with warning
  - **MicroserviceManagementAgent**: Deploy/stop services within 10s, health check within 1s. Failure recovery: deployment fail → rollback to previous version, health fail → restart service, 3 consecutive fails → alert CECCA
- **FR-009**: System MUST implement MicroAgentStacks as deployable clusters for bounded objectives. Bounded objective examples: "analyze repository X", "generate report Y", "process dataset Z". Termination criteria: (1) objective marked complete by commander agent, (2) timeout exceeded (default 1 hour, configurable), (3) error threshold exceeded (>10 failures), (4) manual termination via admin command. `gen_mas` stacks are disposed immediately after termination; `mas_*` stacks persist for potential reuse
- **FR-010**: System MUST support agent lifecycle: Bootstrap → Execute → Validate → Package → Archive. Transition conditions: Bootstrap→Execute (all dependencies loaded), Execute→Validate (task output produced), Validate→Package (output passes quality checks), Package→Archive (artifact stored in CAS), any state→Terminated (on failure after 3 retries)
- **FR-011**: System MUST enforce constitutional principles on all agents (no agent can violate self-contained, local-first, or security constraints)

### Functional Requirements - Executive Agents (Operational Execution)

- **FR-142**: System MUST implement Executive Agents for domain-specific operational execution:
  - **LegalExecutive**: Contract review, compliance checking, license analysis. Escalation threshold: any legal ambiguity. Authority: delegate legal tasks, enforce compliance.
  - **FinanceExecutive**: Cost tracking, resource budgeting, ROI analysis. Escalation threshold: budget variance >10%. Authority: delegate financial tasks, enforce budgets.
  - **SecurityExecutive**: Threat assessment, vulnerability triage, access control review. Escalation threshold: any HIGH/CRITICAL finding. Authority: delegate security tasks, enforce security policies.
  - **OperationsExecutive**: System health monitoring, capacity planning, incident coordination. Escalation threshold: SLO breach. Authority: delegate ops tasks, manage incidents.
  - **QAExecutive**: Test coverage analysis, quality gate enforcement, regression detection. Escalation threshold: coverage <80%. Authority: delegate QA tasks, enforce quality gates.
  - **ArchitectureExecutive**: Design review, dependency analysis, technical debt tracking. Escalation threshold: architectural violation. Authority: delegate design tasks, enforce architecture.
- **FR-143**: Executive Agents MUST report to CECCA, have execution authority, and MUST delegate task execution to MicroAgentStacks or permanent agents
- **FR-144**: Executive Agents MUST ensure proper task execution and MUST fix issues when tasks fail
- **FR-183**: Executive Agents MUST log all delegations, executions, and fixes with rationale to audit trail
- **FR-151**: When Executive Agents provide conflicting recommendations, CECCA MUST:
  1. Defer to constitutional governance (FR-025, FR-026) for ethical/value conflicts
  2. Consult Board Agents (FR-184) for advisory recommendations
  3. Deploy to Sandbox plane first (testing/staging environment)
  4. Resolve SecurityExecutive findings before promotion
  5. Only promote to Deployed plane after security issues addressed
  6. Log conflict details, resolution rationale, and staged deployment trace to audit trail

### Functional Requirements - Board Agents (Advisory & Higher Functions)

- **FR-184**: System MUST implement Board Agents as advisory-only agents connected to reasoning models via ModelSelector:
  - **KnowledgeBoard**: Knowledge synthesis, information retrieval, fact verification. No execution authority - advisory only.
  - **LearningBoard**: Self-learning strategies, skill acquisition, capability growth. No execution authority - advisory only.
  - **EvolutionBoard**: Self-upgrading recommendations, capability enhancement proposals. No execution authority - advisory only.
  - **EnvironmentBoard**: Environment awareness, context sensing, situational analysis. No execution authority - advisory only.
  - **HealingBoard**: Self-healing strategies, recovery recommendations, fault diagnosis. No execution authority - advisory only.
  - **StrategyBoard**: Problem solving, decision support, optimization recommendations. No execution authority - advisory only.
  - **PredictionBoard**: Predictive analysis, trend forecasting, risk anticipation. No execution authority - advisory only.
- **FR-185**: Board Agents MUST NOT have execution authority - they provide recommendations only
- **FR-186**: Board Agents MUST be connected to reasoning models via ModelSelector for deep analysis
- **FR-187**: Board Agents MUST be responsible for stack layer oversight:
  - **KnowledgeBoard** → Memory & Knowledge Graph layers
  - **LearningBoard** → Training & Fine-tuning layers
  - **EvolutionBoard** → Self-modification & Update layers
  - **EnvironmentBoard** → Perception & Context layers
  - **HealingBoard** → System health & Recovery layers
  - **StrategyBoard** → Planning & Orchestration layers
  - **PredictionBoard** → Analytics & Forecasting layers
- **FR-188**: Board Agents MUST provide advisory input to Executive Agents before execution
- **FR-189**: Executive Agents MAY override Board Agent recommendations with logged rationale
- **FR-190**: Board Agent recommendations MUST be logged to audit trail with model reasoning trace

### Functional Requirements - Digest Pipeline

- **FR-012**: System MUST implement 7-step digest pipeline: Discover → Fetch → Parse → Analyze → Summarize → Surface → Secure
- **FR-013**: System MUST support multi-language parsing: Python (AST), TypeScript (ts-morph), Go (go/ast), Rust (syn), Java (JavaParser)
- **FR-014**: System MUST generate SBOM (Software Bill of Materials) for all digested sources in CycloneDX 1.5+ format (JSON). SPDX 2.3 export SHOULD be available via `--format spdx` flag
- **FR-015**: System MUST scan for secrets (Gitleaks), vulnerabilities (Trivy/Grype), and perform static analysis (Semgrep)
- **FR-016**: System MUST produce knowledge graph (kg.json), embeddings, system_card.md, and profile.json for each digest

### Functional Requirements - P2P & Resources

- **FR-017**: System MUST support P2P connections for shared compute/storage across user devices
- **FR-018**: System MUST leverage excess hardware resources (CPU, GPU, RAM, Storage) across the P2P network. Resource sharing limits: by default, each device shares maximum 50% of excess CPU, 50% of excess GPU, 25% of available RAM, and 10% of free storage. Limits are configurable per-device in `noa_root/config/p2p-resources.json`
- **FR-019**: System MUST implement secure, encrypted communication between P2P nodes using TLS 1.3 with cipher suites: TLS_AES_256_GCM_SHA384 (preferred), TLS_CHACHA20_POLY1305_SHA256 (fallback). Key exchange via X25519 (ECDHE). All data in transit encrypted; no plaintext P2P communication permitted
- **FR-020**: System MUST gracefully degrade when P2P nodes disconnect

### Functional Requirements - UI & Interaction

- **FR-021**: System MUST provide a dynamic, context-aware UI that reconfigures based on current task
- **FR-022**: System MUST display a live, scrollable activity log of agent actions and decisions. Retention: UI displays last 10,000 log entries (approximately 7 days at typical usage); older entries available via search/filter from persistent database. Log entries include timestamp, agent ID, action type, and summary
- **FR-023**: System MUST support multi-modal interaction (text, voice, vision) where hardware permits *(Full MVP scope for glasses testing - see FR-128 to FR-136 for implementation details)*
- **FR-024**: System MUST function with full UI capability offline

### Functional Requirements - Governance & Safety

- **FR-025**: System MUST implement constitutional governance with audit trail for all decisions. Audit trail format: JSON Lines (JSONL) in `noa_root/logs/audit/`. Retention: NEVER deleted (append-only). Each entry includes: timestamp (ISO 8601), decision_id (UUID), agent_id, decision_type, inputs, outputs, rationale, constitutional_references. Storage: compressed after 30 days, archived to `noa_root/data/archives/audit/` after 1 year
- **FR-026**: System MUST use biblical texts (original Greek NA28/UBS5 New Testament and Hebrew BHS/WLC Old Testament from licensed digital sources) as source of absolute truth for ethical governance, transformed via lexical analysis → semantic embedding → knowledge graph integration pipeline
  - **Licensed Digital Sources (FR-026.1)**:
    - **Greek New Testament**:
      - **NA28** (Nestle-Aland 28th Edition): Stuttgart Scholarly Editions, Deutsche Bibelgesellschaft. License: Academic/Research use via [Logos Bible Software](https://www.logos.com) or [Stuttgart Scholarly Editions Bundle](https://faithlife.com/store/product/55088/stuttgart-scholarly-editions-core-bundle)
      - **UBS5** (United Bible Societies 5th Edition): Available via [Logos Bible Software](https://www.logos.com/product/55181/the-greek-new-testament-5th-ed-ubs5-with-critical-apparatus) with critical apparatus
      - **SBLGNT** (SBL Greek New Testament): [sblgnt.com](https://sblgnt.com/) - Open license for non-commercial use, free download
    - **Hebrew Old Testament**:
      - **BHS** (Biblia Hebraica Stuttgartensia): Stuttgart Scholarly Editions, Deutsche Bibelgesellschaft. License: Academic/Research use via [Stuttgart Scholarly Editions Bundle](https://faithlife.com/store/product/55088/stuttgart-scholarly-editions-core-bundle)
      - **WLC** (Westminster Leningrad Codex): [Bible-Discovery Software](https://bible-discovery.com/bible-license-wlc.php) - Free license for electronic use
      - **OSIS XML Format**: Available via [Crosswire Bible Society](https://www.crosswire.org/) for open-source applications
  - **Implementation Note**: For development/testing, use open-licensed sources (SBLGNT, WLC). For production with full critical apparatus, acquire Stuttgart Scholarly Editions license
- **FR-027**: System MUST implement reward/correction mechanisms for agent compliance. Reward thresholds: (1) task completion within SLA → +1 compliance score, (2) constitutional adherence verified → +2 compliance score, (3) novel problem solving → +3 compliance score. Correction thresholds: (1) SLA violation → enter testing loop, (2) constitutional drift detected (compliance score <0) → mandatory retraining, (3) repeated violations (3+ in 24h) → agent quarantine pending human review
- **FR-028**: System MUST provide rollback capability for all self-modifications. Rollback scopes: (1) `single` - revert specific modification by ID, (2) `batch` - revert all modifications from a single session, (3) `checkpoint` - revert to named checkpoint, (4) `time` - revert all modifications since timestamp. Default scope: `single`. All rollbacks logged to audit trail

### Functional Requirements - Directory Structure

- **FR-029**: System MUST create and populate `noa_root/sys/` with system-level components
- **FR-030**: System MUST create and populate `noa_root/p2p/` with peer-to-peer networking components
- **FR-031**: System MUST create and populate `noa_root/opt/` with optional packages (llama.cpp, llama-cpp-rs)
- **FR-032**: System MUST create and populate `noa_root/init/` with initialization scripts
- **FR-033**: System MUST create and populate `noa_root/containers/` with container definitions
- **FR-034**: System MUST create and populate `noa_root/config/` with configuration files
- **FR-035**: System MUST create and populate `noa_root/bin/` with executable binaries and wrappers
- **FR-036**: System MUST create and populate `noa_root/ai/` with AI providers, models, and prompts

### Functional Requirements - Unified Bootstrap System

- **FR-076**: System MUST provide a single entry point script for each platform:
  - Windows: `scripts/bootstrap/bootstrap.ps1`
  - Unix: `scripts/bootstrap/bootstrap.sh`
- **FR-077**: System MUST install tools in correct dependency order: Git → Toolchains → Quality tools → Security tools → Utilities
- **FR-078**: System MUST detect the current platform (Windows native/WSL1/WSL2, macOS Intel/Apple Silicon, Linux Debian/RHEL/Arch)
- **FR-079**: System MUST validate installed tool versions against minimum requirements
- **FR-080**: System MUST install portable tools to `noa_root/bin/` (jq, ripgrep, fd, bat, fzf, gitleaks, trivy, grype)
- **FR-081**: System MUST install portable toolchains to `noa_root/opt/`:
  - Rust → `noa_root/opt/rust/` (RUSTUP_HOME, CARGO_HOME)
  - Go → `noa_root/opt/go/` (GOROOT, GOPATH, GOBIN)
  - Node.js → `noa_root/opt/node/` (NODE_PATH, npm_config_prefix)
  - Python → `noa_root/opt/python/` + `noa_root/opt/venv/` (VIRTUAL_ENV)
  - protoc → `noa_root/bin/`
- **FR-082**: System MUST generate environment files: `noa-env.ps1`, `.noa-env`, `config/noa.json`
- **FR-083**: System MUST optionally integrate with user shell profiles (PowerShell $PROFILE, .bashrc, .zshrc)
- **FR-084**: System MUST provide comprehensive verification of all installations
- **FR-085**: System MUST log all bootstrap actions to `logs/bootstrap-{timestamp}.log`
- **FR-086**: System MUST be idempotent (safe to run multiple times)
- **FR-087**: System MUST handle errors gracefully with retry guidance

### Functional Requirements - Cross-Platform Script Parity

- **FR-088**: System MUST provide mirrored scripts for ALL platforms (every Bash script has PowerShell equivalent and vice versa)
- **FR-089**: System MUST ensure mirrored scripts accept the same arguments and return the same exit codes
- **FR-090**: System MUST document all scripts in `scripts/README.md` with cross-platform mapping table

### Functional Requirements - Kernel Independence

- **FR-091**: System MUST support operation independent of the host kernel on all platforms:
  - Windows: Hyper-V VM with custom Linux kernel
  - Linux: KVM/QEMU VM or container isolation
  - macOS: Virtualization.framework VM
- **FR-092**: System MUST provide a Kernel Abstraction Layer (NKAL) with unified interface regardless of underlying kernel
- **FR-093**: System MUST default to host kernel (native mode) for performance, with independence mode available for maximum isolation
- **FR-094**: System MUST support kernel mode switching via `noa-kernel-params set kernel_mode {native|vm|container}`

### Functional Requirements - Module Abstraction (AER Spec Integration)

- **FR-176**: System MUST implement a unified Module abstraction where ALL artifacts (binaries, packages, libraries, tools, services, agents, microkernels) are content-addressable, immutable, and versioned entities
- **FR-177**: System MUST maintain a Module Registry at `noa_root/data/modules/registry.db` tracking all modules with: module_id (content hash), name, version, type, dependencies, capabilities, and metadata
- **FR-178**: System MUST store module content in Content-Addressable Storage (CAS) at `noa_root/data/modules/cas/` using SHA-256 content hashing
- **FR-179**: System MUST support module lifecycle: Register → Verify → Load → Execute → Unload → Archive
- **FR-180**: System MUST provide module dependency resolution and version conflict detection before loading

### Functional Requirements - IDE Data Containment (§3.1 Extension)

- **FR-181**: System MUST redirect Cursor IDE data to `noa_root/data/apps/cursor/` including:
  - Extensions: `noa_root/data/apps/cursor/extensions/`
  - User settings: `noa_root/data/apps/cursor/User/`
  - Workspace storage: `noa_root/data/apps/cursor/workspaceStorage/`
  - Cache: `noa_root/data/apps/cursor/Cache/`
- **FR-182**: System MUST redirect VS Code data to `noa_root/data/apps/vscode/` including:
  - Extensions: `noa_root/data/apps/vscode/extensions/`
  - User settings: `noa_root/data/apps/vscode/User/`
  - Workspace storage: `noa_root/data/apps/vscode/workspaceStorage/`
  - Cache: `noa_root/data/apps/vscode/Cache/`

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
| 4 | **VS Code Copilot** (IDE) | IDE | <1s | Inline completions when VS Code active |
| 5 | **Git CLI** | Local | <100ms | Version control operations |
| 6 | **Abacus** (CLI/Cloud) | Cloud | <3s | Specialized numerical/analytical tasks |

**Provider Orchestration Mode** (Production - Full NOA System):
- **CECCA as Orchestrator**: When full NOA UI is operational, CECCA coordinates ALL available providers for parallel task execution
- User submits goals via NOA UI → CECCA decomposes → distributes to optimal providers
- Parallel execution across providers with result aggregation
- Shared context maintained via Shared Provider Execution Memory bus (FR-037)

**IDE-Phase Orchestration** (Development - Before Full NOA UI):
- **Spec-Kit as Orchestrator**: During development, `/speckit.implement` serves as the orchestration entry point
- Entry point: User in Cursor IDE → `/speckit.implement <task-id>` command
- Spec-Kit uses `connect_provider()` (SK001) to connect multiple providers simultaneously
- Task distribution via `spec-distribution.json` and `execution-memory.db`
- Results aggregated back to Cursor IDE for unified output
- Flow: `/speckit.implement` → `connect_provider()` → [Claude, Codex, Copilot, llama.cpp] → `execution-memory.db` → Cursor

**Fallback Strategy**:
1. Always try local providers (llama.cpp) first for offline capability
2. If local fails/unavailable, try IDE providers if IDE context exists
3. If IDE unavailable, try cloud providers in priority order
4. If Cursor available with IDE context, use orchestration mode for complex multi-step tasks
5. If all fail, queue task and notify user after 3 retry attempts

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
- **Executive Agent**: An agent with execution authority that delegates and ensures proper task execution. Responsible for domain-specific operational execution (Legal, Finance, Security, Operations, QA, Architecture). Reports to CECCA.
- **Board Agent**: An advisory-only agent connected to reasoning models via ModelSelector. Provides recommendations for higher cognitive functions (Knowledge, Learning, Evolution, Environment, Healing, Strategy, Prediction). NO execution authority.
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

## Non-Functional Requirements *(mandatory)*

### Performance Requirements

**Cold-Start vs Warm-Start Performance (NFR-001)**:
- **Cold-Start** (first run after boot): System initialization <90 seconds, model loading <30 seconds, first inference <5 seconds
- **Warm-Start** (subsequent runs): System initialization <60 seconds (SC-001), model loading <10 seconds (cached), first inference <2 seconds (SC-002)
- All Success Criteria (SC-001 to SC-012) assume warm-start unless explicitly noted

**Degraded Mode Performance Targets (NFR-002)**:
- **AMPK-Mode** (low storage): inference latency may increase 2x, batch operations paused, real-time queries prioritized
- **Low Memory Mode** (<4GB available): concurrent models reduced to 2, inference latency may increase 3x
- **Network Degraded** (P2P disconnected): local operations unaffected, sync queued, no performance impact for local tasks
- **CPU Throttled** (thermal/power): inference latency may increase proportionally to throttle level

### Security Requirements

**Data-at-Rest Encryption (NFR-003)**:
- All databases (SQLite, Qdrant) MUST be encrypted using AES-256-GCM
- Encryption key derived from user master passphrase using Argon2id (memory: 256MB, iterations: 3, parallelism: 4)
- Key stored in OS-native secure storage (Windows: DPAPI, macOS: Keychain, Linux: libsecret)
- Encryption MUST NOT impact query latency by more than 10%

### Scalability Requirements

**P2P Device Scalability (NFR-004)**:
- System MUST support minimum 10 devices per P2P cluster
- System SHOULD support up to 50 devices with graceful performance degradation
- Beyond 50 devices: sub-clustering with elected coordinators
- Network overhead: <5% of available bandwidth for cluster coordination

**Knowledge Graph Scalability (NFR-005)**:
- System MUST support minimum 1 million nodes and 10 million edges per knowledge graph
- Query latency: <100ms for 2-hop traversals on graphs up to 1M nodes
- Beyond 1M nodes: automatic sharding with cross-shard query support

**Memory Entry Scalability (NFR-006)**:
- System MUST support minimum 10 million memory entries
- Search latency: <500ms (SC-003) for up to 10M entries using vector index
- Beyond 10M entries: automatic archival of oldest entries (configurable threshold)
- Total storage: configurable limit (default 10GB), warning at 80%, archive at 90%

---

## Constitutional Compliance *(mandatory for NOA)*

### Data Locality & Offline Behavior

- **Offline Support**: ☑ Full (user choice) - Core local operations work offline; P2P/cloud require network
- **Data Residency**: All data stored under `noa_root` directory? ☑ Yes
- **Network Requirements**:
  - Internet required for: P2P hive-mind sync, cloud AI providers, model downloads, OAuth connectors
  - Offline mode (user choice): local inference, local memory, local agents continue to work
  - Model sideloading: USB/file transfer supported for offline model installation
- **External Dependencies**:
  - GitHub API (feature-flagged, optional)
  - OAuth providers for connectors (feature-flagged, optional)
  - Model download sources (one-time, cached locally, or sideloaded)

### Agent Orchestration

- **Responsible Agents**:
  - NOA (CECCA) - Root orchestrator
  - ModelSelectorAgent - Model routing
  - DigestAgent - Knowledge acquisition
  - FileIOAgent - File operations
  - TerminalAgent - Shell/command execution
  - RAGAgent - Retrieval-augmented generation
  - MicroserviceManagementAgent - Service deployment
  - All Executive Agents (Legal, Finance, Operations, Security, QA, Architecture)
  - All Board Agents (Knowledge, Learning, Evolution, Environment, Healing, Strategy, Prediction)

- **Multi-SLM Compatibility**: ☑ Yes - Uses llama.cpp with multiple <3B parameter models
- **Orchestration Pattern**: Hierarchical with NOA at root, consulting Board Agents (advisory), delegating to Executive Agents (execution), then to MicroAgentStacks

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

All task execution MUST comply with [Universal Task Execution Policy](../../data/archive/sessions/project-mgmt/docs/05-policy/universal_task_execution_policy.md):

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

1. User has administrative/elevated access for initial installation:
   - **Windows**: Administrator privileges required for: modifying PATH, registering services, installing to Program Files (if chosen). User-mode installation to `%USERPROFILE%\noa` requires no admin access
   - **macOS**: Admin privileges required for: installing to `/Applications`, modifying LaunchDaemons. User-mode installation to `~/noa` requires no admin access
   - **Linux**: Root access required for: systemd service registration, `/usr/local` installation. User-mode installation to `~/.noa` requires no admin access
   - **Post-installation**: No elevated privileges required for normal operation
2. Hardware tiers (see Glossary):
   - **Minimum**: 8GB RAM, 4-core CPU, no dedicated GPU - baseline operation with single small model
   - **Standard**: 16GB RAM, 8-core CPU, integrated GPU - recommended for multi-SLM operation
   - **High-Performance**: 64GB+ RAM, 16+ cores, dedicated GPU - optimal for local inference
   - **Development**: 512GB+ RAM, 24+ cores, multi-GPU - full development with tensor parallelism
3. Minimum 20GB storage for base installation (100GB+ recommended, 2TB+ for development hardware)
4. Network access available for initial setup (downloading models, packages)
5. Standard web browser available for UI interaction:
   - **Supported browsers**: Chrome 120+, Firefox 120+, Safari 17+, Edge 120+
   - **Required features**: ES2022 JavaScript, CSS Grid, WebSocket, IndexedDB
   - **Recommended resolution**: 1920x1080 or higher
   - **Accessibility**: Screen reader compatible (tested with NVDA, VoiceOver, JAWS)
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

### Rust Observability Crates
- **tracing**: Structured logging and span-based instrumentation
- **tracing-subscriber**: Configurable subscriber for tracing events
- **opentelemetry**: OpenTelemetry SDK for distributed tracing
- **opentelemetry-otlp**: OTLP exporter for traces/metrics
- **opentelemetry-prometheus**: Prometheus metrics exporter

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
./init/check-prereqs.sh

# Windows PowerShell
.\scripts\setup\check-prereqs.ps1
```

The script will output:
- ✅ Tool installed with version
- ❌ Tool missing with install command
- ⚠️ Tool installed but version too old

---

## Out of Scope (for this foundation release)

- Full mobile companion apps *(P3+ / Future)* - stub implementation only: P2P connectivity to desktop NOA, no native mobile UI/features
- XR/AR/VR interfaces *(Future)* - architecture support but no implementation
- Enterprise multi-tenant deployment *(Future)*
- Cloud-native distributed deployment *(Future)*
- Full CRM strangler implementation *(P2+ / Shadow mode only)*
- Complete biblical text ML transformation *(P2+ / Reference implementation only)* - lexical analysis → embedding pipeline placeholder tasks in Phase 2.5
- Multi-modal voice/vision interaction *(P3+ / US5)* - text interaction is MVP scope

---

## Clarifications

### Session 2025-12-08

- Q: What triggers autonomous task initiation when no human submits a goal? → A: Always-on continuous loop. User provides co-improvement goals; system continuously loops to complete those goals while always researching and implementing ways to optimize and use resources more efficiently and effectively.
- Q: How does NOA apply code changes to itself while running? → A: 3-plane architecture with Coordinator/Sandbox/Deployed planes. New capabilities are developed in Sandbox, analyzed by Coordinator (llama.cpp analytics), then promoted to Deployed with canary rollout. Coordinator maintains long-term memory and backups. No downtime.
- Q: Which operations require human approval vs. proceed autonomously? → A: Full autonomy. NOA executes ALL operations autonomously without per-operation human approval. User only sets initial co-improvement goals. Rollback via 3-plane system is the safety net for failed changes.
- Q: Can NOA generate its own improvement goals or only execute user-provided goals? → A: Full goal autonomy. NOA can generate ANY goal it determines beneficial, including goals unrelated to current user objectives. Constitutional governance (FR-025, FR-026) provides the ethical boundary.
- Q: What self-healing mechanism should NOA use for autonomous operation? → A: Full self-healing loop. Proactive detection → diagnosis → auto-fix → validation → escalation-to-user ONLY if all auto-fix attempts fail. Maintains always-on operation.
- Q: What is the role of each plane in the 3-plane system? → A: **Coordinator** is the constant plane for long-term memory, backups, archives, analytics, and promotion decisions. **Sandbox** is for testing/staging new capabilities. **Deployed** is production serving live traffic with canary deployments and auto-rollback on SLO violation.
- Q: How should providers coordinate for complex tasks? → A: **Cursor as orchestrator**. When operating in IDE context, Cursor agent MUST coordinate ALL available providers for parallel task execution. Cursor distributes sub-tasks to optimal providers (reasoning → Claude, code → Codex, local → llama.cpp), executes in parallel, and aggregates results via Shared Provider Execution Memory bus.
- Q: How should rate limiting work for AI providers and P2P? → A: **Per-provider rate limits with adaptive backoff**. Each cloud provider has configurable token/request limits; system uses exponential backoff on 429 responses. P2P throttles based on peer capacity. Self-generated goals are rate-limited to prevent runaway task creation.
- Q: How do users and agents authenticate to NOA? → A: **Device-bound keys with P2P trust chain**. Each device generates Ed25519 keypair on init; user sets local master passphrase (optionally saved in browser password managers). New device pairing via: (1) QR code scan, (2) 6-digit PIN approval, (3) Bluetooth/NFC proximity, or (4) encrypted file transfer. Agents sign actions with device key; P2P uses mutual TLS with user's device keys.
- Q: What are the accessibility and localization requirements? → A: **WCAG 2.1 AAA with full i18n from day one**. Maximum accessibility compliance; multi-language support with RTL (Arabic, Hebrew); all UI strings externalized; local-first translations bundled (no cloud dependency).
- Q: How should the UI handle loading, empty, and error states? → A: **Skeleton + status indicators**. Use skeleton loaders for content areas; persistent status bar for background operations (sync, AI processing); toast notifications for errors; always display cached/partial data when available during sync.
- Q: What is the scope of multi-modal interaction (FR-023)? → A: **Full multi-modal in MVP** for glasses testing. Voice: STT (Whisper) + TTS (Piper/Coqui) bidirectional. Vision: camera input, screen capture, image file analysis via local multimodal models. All capabilities in foundation release to enable XR/glasses device testing.

### Session 2025-12-09

- Q: How does orchestration work during IDE-phase development (before full NOA UI)? → A: **Spec-Kit as IDE Orchestrator**. During development phase, `/speckit.implement` serves as the orchestration entry point. It uses `connect_provider()` (task SK001) to connect multiple providers simultaneously. Tasks are distributed via `execution-memory.db` and results aggregated back to the user's Cursor IDE. This bridges the gap until the full NOA UI is built, enabling immediate parallel provider execution from IDE chat.
- Q: When Executive Agents provide conflicting recommendations, how should CECCA resolve? → A: **Constitutional arbitration with Board consultation and staged deployment**. Conflicts are resolved via: (1) constitutional governance (FR-025, FR-026), (2) Board Agent advisory consultation (FR-184), (3) staged deployment to Sandbox first, (4) address SecurityExecutive findings, (5) promote to Deployed plane. This leverages the 3-plane architecture and Board/Executive separation for safe conflict resolution.
- Q: Is internet access required or is air-gapped operation mandatory? → A: **Internet required for normal operation; offline mode is optional user choice**. Internet access is mandatory for P2P hive-mind functionality and initial model downloads. "Fully air-gapped" is an OPTIONAL capability, not a requirement. When user chooses offline mode, constraints apply: no cloud provider access, no new model downloads, no P2P sync. USB/file transfer is supported for sideloading models in offline scenarios. Core local operations (inference, memory, agents) continue to work offline.
- Q: What observability stack should NOA use for metrics and tracing? → A: **OpenTelemetry + Prometheus + local SQLite**. Rust crates: `tracing`, `tracing-subscriber`, `opentelemetry` (OTLP), `opentelemetry-prometheus`. Export to Tempo/Prometheus/Grafana stack. Local SQLite store (`noa_root/data/metrics.db`) for offline analysis. No Docker required for Rust observability components.
- Q: Which provider handles verification and error fixing during /implement? → A: **Claude Code (claude-code)**. Deep error analysis, long context understanding, agentic iteration with file operations. Interprets linter/compiler errors, proposes fixes, validates fix success.
- Q: Which provider handles refactoring and tool creation during /implement? → A: **Cursor (Opus 4.5)**. IDE context awareness for multi-file refactoring, codebase understanding. Keeps architecture clean: Opus reasons/plans → Codex/Abacus implement.
- Q: How should workload be distributed across providers for parallel execution? → A: **Even distribution for maximum parallelism**. Orchestrator assigns tasks to balance provider utilization. Decompose large tasks into parallelizable subtasks. Route by task type: reasoning→Cursor, fast_code→Codex, complex_code→Abacus, verification→Claude, local_inference→llama.cpp. Track provider load in execution-memory.db to prevent bottlenecks.
- Q: How should llama.cpp be utilized for maximum parallelism? → A: **5 SLM coding subagents**. llama.cpp runs 5 concurrent coding SLMs (DeepSeek-Coder, CodeLlama, Qwen2.5-Coder, StarCoder2, Phi-3.5-mini) as a subagent pool. Main providers (Cursor, Codex, Abacus, Claude) delegate implementation subtasks (single functions, boilerplate, tests, configs) to this pool. Each subagent handles tasks ≤2000 tokens, single files, with 60s timeout. This multiplies coding capacity 5x while keeping costs zero (local inference).
- Q: What is the kernel selection precedence ("NOA kernels first")? → A: **NOA VM > Container > Sandbox > Native (host)**. Priority order for kernel mode selection:
  1. **NOA VM** (Priority 1): Custom NOA Linux kernel in VM. Use when: maximum isolation required, untrusted code execution, cross-platform consistency critical.
  2. **Container** (Priority 2): Isolated container with minimal kernel interface. Use when: lightweight isolation sufficient, faster startup than VM needed.
  3. **Sandbox** (Priority 3): User-space isolation (Windows Sandbox, Bubblewrap, App Sandbox). Use when: per-operation isolation, ephemeral environments.
  4. **Native/Host** (Priority 4, DEFAULT): Host kernel for maximum performance. Use when: trusted environment, development mode, performance-critical operations.

  **Selection Logic**: System defaults to Native for performance. Automatic escalation to higher isolation occurs when: (1) running untrusted code from digest pipeline, (2) processing external/unverified data, (3) explicit user request via `noa-kernel-params set kernel_mode`, (4) constitutional governance requires isolation. Downgrade from isolated to native requires explicit user action.
- Q: What defines an "external" vs "internal" dependency? → A: **Internal = under `noa_root`, External = outside `noa_root`**. Internal dependencies are tools, libraries, and resources installed within the `noa_root` directory tree and managed by NOA bootstrap. External dependencies are: (1) host OS kernel APIs (permitted for native mode), (2) system libraries outside `noa_root`, (3) globally-installed tools (deprecated - use internal versions), (4) cloud services (feature-flagged). The boundary is the `noa_root` directory - anything outside is external and must be either avoided, wrapped by NKAL abstraction, or explicitly feature-flagged.
- Q: How does tool isolation work between internal NOA tools and global system tools? → A: **PATH precedence + environment isolation**. Internal tools take precedence via: (1) `noa_root/bin` prepended to PATH before any system paths, (2) explicit `NOA_*` environment variables (`GOROOT`, `RUSTUP_HOME`, `NODE_PATH`) pointing to `noa_root/opt/`, (3) all npm/pip/cargo installs go to `noa_root/opt/` with local prefix settings, (4) shell wrapper scripts in `noa_root/bin/` that explicitly invoke internal tool paths. Global tools are detected but NOT used unless `--allow-global` flag passed. Internal versions are always preferred even if older than global.
- Q: How are internally-hosted tools updated/upgraded? → A: **Controlled upgrade via bootstrap with version pinning**. Tool upgrades use: (1) version requirements in `config/bootstrap-tools.json`, (2) `install-all-tools.ps1 -UpdateExisting` or `UPDATE_EXISTING=1 install-all-tools.sh` for explicit upgrades, (3) version checks on every bootstrap run (warn if outdated, don't auto-upgrade), (4) manual upgrade triggers only (no silent updates). Rollback: previous tool versions archived to `noa_root/opt/archive/{tool}-{version}/` before upgrade.
- Q: How is state persisted when switching kernel modes? → A: **Shared state directory + migration checkpoint**. Kernel mode switching: (1) all persistent state lives in `noa_root/data/` (accessible from all modes), (2) before switch: checkpoint written to `noa_root/data/.kernel-switch-state.json` with current state hash, (3) VM/container modes mount `noa_root/data/` as shared volume, (4) after switch: state verification confirms checkpoint matches, (5) hot-switch not supported - graceful shutdown required before mode change. In-memory state (caches, inference context) is lost on switch.
- Q: What is the trust boundary between NOA and the host kernel? → A: **NKAL is the trust boundary**. The NOA Kernel Abstraction Layer (NKAL) defines the boundary: (1) above NKAL: trusted NOA code with full privileges within `noa_root`, (2) below NKAL: untrusted host kernel accessed only through NKAL interface, (3) NKAL validates all data crossing the boundary (input sanitization, output verification), (4) privileged operations (raw sockets, kernel modules) require explicit NKAL capability grants in `config/nkal-capabilities.json`, (5) VM/container modes provide hardware-enforced trust boundary via hypervisor/container runtime.
- Q: When should host OS kernel be used vs NOA portable dependencies? → A: **Host kernel for bootstrap/access, NOA portable for everything else**.

  **Host Kernel MAY Be Used For:**
  1. **Start-up/Bootstrap**: Initial system boot and NOA initialization sequence
  2. **Environment Scanning**: Discovering host capabilities for adaptive optimization (CPU features, GPU availability, memory, storage)
  3. **Host Optimization**: When NOA needs to optimize host performance (NOA internalizes discovered features for optimization control)
  4. **File/Directory Access**: Accessing host directories or files required for goal completion (outside `noa_root`)

  **NOA Kernel/Portable Dependencies MUST Be Used For (100% Independence):**
  - All tools (jq, ripgrep, fd, bat, etc.)
  - Terminal and shell (internal shell environment)
  - All packages and package managers (npm to `noa_root/opt/node`, pip to `noa_root/opt/venv`, cargo to `noa_root/opt/rust`)
  - All services (llama-server, ollama, gitea, etc.)
  - Network stack (when in VM/container mode)
  - All settings and configurations (under `noa_root/config/`)
  - All persistent state and data (under `noa_root/data/`)

  **Platform Coverage**: This architecture applies to ALL platforms (Windows, Linux, macOS, mobile, XR) and ALL hardware types (x64, ARM, GPU configurations). NOA achieves 100% independent functionality by bundling portable versions of all dependencies within `noa_root`.
- Q: What is the difference between Board Agents and Executive Agents? → A: **Executive Agents execute; Board Agents advise**.
  - **Executive Agents** (FR-142-144, FR-183): Have execution authority and responsibility. They delegate task execution to MicroAgentStacks and permanent agents, ensure proper execution, and fix issues when tasks fail. Examples: LegalExecutive, SecurityExecutive, OperationsExecutive.
  - **Board Agents** (FR-184-190): Advisory only with NO execution authority. Connected to reasoning models via ModelSelector for deep analysis. Responsible for higher cognitive functions: knowledge synthesis, self-learning, self-upgrading, environment awareness, self-healing, problem solving, predictive analysis. Examples: KnowledgeBoard, LearningBoard, StrategyBoard.
  - **Hierarchy**: User → CECCA → consults Board Agents → delegates to Executive Agents → delegates to MicroAgentStacks/permanent agents.
  - **Stack Layer Mapping**: Each Board Agent is responsible for advising on specific stack layers (see FR-187).

### Session 2025-12-09 (continued)

- Q: Should Environment changes (toolchain upgrades, dev tools) go through the 3-plane A/B switching cycle? → A: **Environment outside planes**. Toolchains (`opt/rust/`, `opt/go/`, etc.), dev tools, IDEs, and AI provider CLIs are shared infrastructure that all planes depend on equally. They are NOT subject to A/B switching. The 3-plane system (Coordinator/Sandbox/Deployed) is for NOA's **runtime capabilities and code changes**, not for the development environment itself. Environment changes use the separate bootstrap upgrade mechanism with version pinning and rollback (FR-163).
- Q: How do IDEs (Cursor, VS Code) interact with the 3-plane system? → A: **IDE drives Sandbox only**. IDEs write code that gets deployed to Sandbox plane. After Coordinator validates (llama.cpp analytics, policy gates), changes are promoted to Deployed plane. IDE never directly modifies Deployed code. This maintains 3-plane integrity: Dev work → Sandbox → Coordinator validation → Deployed promotion.
- Q: Where does execution-memory.db fit in the 3-plane model? → A: **Single shared DB, Coordinator-owned**. One `ai/shared/resources/execution-memory.db` is used by all providers across all planes. Coordinator is the "owner" with authority to manage the DB, but all providers have equal read/write access for task distribution and context sharing. **Coordinator applies rewards/restrictions to models based on behavior**: models that consistently succeed get priority access; models with repeated failures, SLA violations, or policy violations get restricted task assignments or rate-limited. This enables runtime optimization of provider selection based on empirical performance.
- Q: What happens during a promotion from Sandbox → Deployed? → A: **Hot swap with canary deployment**. Deployed receives new code while running with zero downtime. Canary deployment: (1) new capability deployed to small cohort (5-10%), (2) SLO monitoring during canary window, (3) gradual traffic shift if SLOs met, (4) automatic rollback if SLOs violated. This aligns with FR-057 (blue-green) and FR-058 (instant rollback). NOA's always-on autonomous operation requires zero downtime.
- Q: How does P2P synchronization work across devices with the 3-plane system? → A: **Coordinator is P2P leader on device with superior compute/storage**. The user's device with the best compute and storage resources runs as the Coordinator P2P leader. Other devices run Sandbox/Deployed and sync to the leader. This provides: (1) clear authority for conflict resolution, (2) resource-appropriate leader selection, (3) efficient data distribution from central point, (4) failover to next-best device if leader goes offline. P2P syncs include: `data/`, `ai/shared/`, model weights, and execution-memory.db state.
- Q: What is the noa Core Microkernel and how does it relate to CECCA? → A: **Environment-based subagent under CECCA control**. The noa Core Microkernel is an environment-based operational unit that functions as a subagent/helper to CECCA. It is not a separate top-level entity but operates under/inside CECCA's domain. CECCA also has environment agents that work alongside the microkernel for environment management tasks.
- Q: How does the EOM/TSM/PSM triad from the AER spec map to the NOA plan? → A: **PSM maps to Executive Agents; EOM/TSM maps to Board Agents**. The Policy & Safety Model (PSM) maps to Executive Agents (LegalExecutive, SecurityExecutive, etc.) which have execution authority to enforce governance. The Environment Orchestration Model (EOM) maps to Board Agents (EnvironmentBoard, EvolutionBoard, HealingBoard) which provide advisory intelligence. The Tool & Code Synthesis Model (TSM) maps to the Advanced Learning techniques (FR-043-046) and StrategyBoard advisory functions.
- Q: How does Module Abstraction from the AER spec fit into the NOA architecture? → A: **Adopt as new requirement (FR-176-180)**. The AER spec defines "Module" as a unified abstraction for all artifacts (binaries, packages, libraries, tools, services, agents, microkernels). NOA MUST implement this as a Module Registry with content-addressable storage (CAS). All artifacts become content-addressable, immutable, and versioned entities stored under `noa_root/data/modules/`.
- Q: Should IDE data (Cursor/VS Code) be contained within `noa_root`? → A: **Yes - extend FR-001 containment (FR-181-182)**. IDE configurations and extensions MUST be redirected to `noa_root/data/apps/cursor/` and `noa_root/data/apps/vscode/`. This extends the NDCL (FR-167) to include development IDEs, ensuring complete data sovereignty.
