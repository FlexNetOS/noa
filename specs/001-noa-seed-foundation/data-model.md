# Data Model: NOA Seed Foundation

**Feature**: 001-noa-seed-foundation
**Date**: 2025-12-08
**Database**: SQLite (primary) / PostgreSQL (scale-up)

---

## Entity Relationship Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           NOA DATA MODEL                                     │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────────────┐       │
│  │ Memory  │────>│ Agent   │────>│  Task   │────>│ MicroAgentStack │       │
│  └────┬────┘     └────┬────┘     └────┬────┘     └────────┬────────┘       │
│       │               │               │                    │                │
│       v               v               v                    v                │
│  ┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────────────┐       │
│  │Embedding│     │ AgentLog│     │TaskEvent│     │    Capsule      │       │
│  └─────────┘     └─────────┘     └─────────┘     └─────────────────┘       │
│                                                                              │
│  ┌─────────┐     ┌─────────┐     ┌─────────────┐                            │
│  │  Node   │<───>│  Edge   │     │   Digest    │                            │
│  │  (KG)   │     │  (KG)   │     │   Source    │                            │
│  └─────────┘     └─────────┘     └─────────────┘                            │
│                                                                              │
│  ┌─────────┐     ┌─────────┐     ┌─────────────┐                            │
│  │  Model  │     │ Device  │     │    Sync     │                            │
│  └─────────┘     └─────────┘     │    State    │                            │
│                                  └─────────────┘                            │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Core Entities

### 1. Memory

Stores all interactions, decisions, learnings, and data. Nothing is forgotten.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |
| `updated_at` | TIMESTAMP | NOT NULL | Last modification |
| `type` | ENUM | NOT NULL | 'interaction', 'decision', 'learning', 'artifact' |
| `content` | TEXT | NOT NULL | Main content (may be JSON) |
| `metadata` | JSON | | Additional structured data |
| `source_agent` | UUID | FK -> Agent | Agent that created this memory |
| `parent_id` | UUID | FK -> Memory | For threaded memories |
| `tags` | TEXT[] | | Searchable tags |
| `embedding_id` | UUID | FK -> Embedding | Vector representation |
| `checksum` | TEXT | NOT NULL | Content hash for integrity |

**Indexes**:
- `idx_memory_created` (created_at DESC)
- `idx_memory_type` (type)
- `idx_memory_tags` (tags) - GIN index

---

### 2. Embedding

Vector embeddings for semantic search.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |
| `vector` | FLOAT[384] | NOT NULL | Embedding vector (384-dim for MiniLM) |
| `model` | TEXT | NOT NULL | Model used for embedding |
| `source_type` | TEXT | NOT NULL | 'memory', 'node', 'document' |
| `source_id` | UUID | NOT NULL | Reference to source entity |

**Indexes**:
- `idx_embedding_vector` (vector) - HNSW for approximate NN search

---

### 3. Agent

Registered agents in the system.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `name` | TEXT | NOT NULL UNIQUE | Agent name (e.g., 'FileIOAgent') |
| `type` | ENUM | NOT NULL | 'permanent', 'board', 'stack', 'dynamic' |
| `status` | ENUM | NOT NULL | 'active', 'paused', 'retired' |
| `version` | TEXT | NOT NULL | Semver version |
| `configs` | JSON | NOT NULL | Agent configsuration |
| `capabilities` | TEXT[] | NOT NULL | List of capabilities |
| `parent_id` | UUID | FK -> Agent | Parent agent (for hierarchy) |
| `created_at` | TIMESTAMP | NOT NULL | Registration time |
| `last_active` | TIMESTAMP | | Last activity timestamp |
| `metrics` | JSON | | Performance metrics |

**Indexes**:
- `idx_agent_name` (name)
- `idx_agent_type_status` (type, status)

---

### 4. AgentLog

Audit trail for all agent actions.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `agent_id` | UUID | FK -> Agent, NOT NULL | Acting agent |
| `timestamp` | TIMESTAMP | NOT NULL | Action time |
| `action` | TEXT | NOT NULL | Action performed |
| `trigger` | TEXT | | What triggered the action |
| `inputs` | JSON | | Input data (sanitized) |
| `outputs` | JSON | | Output data (sanitized) |
| `duration_ms` | INT | | Execution duration |
| `status` | ENUM | NOT NULL | 'success', 'failure', 'timeout' |
| `error` | TEXT | | Error message if failed |
| `memory_ids` | UUID[] | | Related memories created |
| `parent_log_id` | UUID | FK -> AgentLog | Parent action (for nested) |

**Indexes**:
- `idx_agentlog_agent_time` (agent_id, timestamp DESC)
- `idx_agentlog_status` (status)

**Retention**: Append-only, never deleted

---

### 5. Task

Work units managed by agents.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |
| `updated_at` | TIMESTAMP | NOT NULL | Last update |
| `title` | TEXT | NOT NULL | Task title |
| `description` | TEXT | | Detailed description |
| `status` | ENUM | NOT NULL | 'pending', 'in_progress', 'blocked', 'completed', 'failed', 'cancelled' |
| `priority` | INT | NOT NULL DEFAULT 0 | Priority (higher = more urgent) |
| `assigned_agent` | UUID | FK -> Agent | Agent working on task |
| `stack_id` | UUID | FK -> MicroAgentStack | Parent stack |
| `parent_task_id` | UUID | FK -> Task | Parent task (decomposition) |
| `inputs` | JSON | | Task inputs |
| `outputs` | JSON | | Task outputs |
| `deadline` | TIMESTAMP | | Optional deadline |
| `tags` | TEXT[] | | Constitutional principle tags |
| `retry_count` | INT | DEFAULT 0 | Number of retries |
| `max_retries` | INT | DEFAULT 3 | Max retry attempts |

**Indexes**:
- `idx_task_status` (status)
- `idx_task_priority` (priority DESC)
- `idx_task_assigned` (assigned_agent)

---

### 6. TaskEvent

Task lifecycle events for traceability.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `task_id` | UUID | FK -> Task, NOT NULL | Related task |
| `timestamp` | TIMESTAMP | NOT NULL | Event time |
| `event_type` | ENUM | NOT NULL | 'created', 'assigned', 'started', 'progress', 'completed', 'failed', 'retried', 'cancelled' |
| `agent_id` | UUID | FK -> Agent | Agent that triggered |
| `old_status` | TEXT | | Previous status |
| `new_status` | TEXT | | New status |
| `details` | JSON | | Event details |

---

### 7. MicroAgentStack

Deployable clusters of cooperative agents.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `name` | TEXT | NOT NULL | Stack name |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |
| `status` | ENUM | NOT NULL | 'bootstrap', 'execute', 'validate', 'package', 'archive', 'terminated' |
| `objective` | TEXT | NOT NULL | Stack's bounded objective |
| `commander_agent` | UUID | FK -> Agent | CommanderChief agent |
| `member_agents` | UUID[] | | List of member agents |
| `configs` | JSON | | Stack configsuration |
| `workspace_path` | TEXT | NOT NULL | Workspace directory |
| `artifacts` | JSON | | Output artifacts |
| `metrics` | JSON | | Performance metrics |
| `terminated_at` | TIMESTAMP | | Termination time |

---

### 8. Capsule

Self-contained environment definitions.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `name` | TEXT | NOT NULL UNIQUE | Capsule name |
| `version` | TEXT | NOT NULL | Semver version |
| `status` | ENUM | NOT NULL | 'draft', 'active', 'deprecated' |
| `manifest` | JSON | NOT NULL | Capsule manifest (deps, policies) |
| `image_ref` | TEXT | | Container image reference |
| `checksum` | TEXT | NOT NULL | Manifest checksum |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |
| `promoted_at` | TIMESTAMP | | Promotion to active |

---

### 9. KnowledgeNode

Nodes in the knowledge graph.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `type` | TEXT | NOT NULL | 'function', 'class', 'module', 'file', 'repo', 'concept' |
| `name` | TEXT | NOT NULL | Node name |
| `qualified_name` | TEXT | | Fully qualified name |
| `description` | TEXT | | Summary/description |
| `source_digest` | UUID | FK -> DigestSource | Source digest |
| `location` | JSON | | File path, line numbers |
| `properties` | JSON | | Type-specific properties |
| `embedding_id` | UUID | FK -> Embedding | Vector representation |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |

**Indexes**:
- `idx_node_type` (type)
- `idx_node_qualified` (qualified_name)

---

### 10. KnowledgeEdge

Edges in the knowledge graph.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `source_node` | UUID | FK -> KnowledgeNode, NOT NULL | Source node |
| `target_node` | UUID | FK -> KnowledgeNode, NOT NULL | Target node |
| `relationship` | TEXT | NOT NULL | 'calls', 'imports', 'extends', 'implements', 'contains', 'references' |
| `weight` | FLOAT | DEFAULT 1.0 | Edge weight |
| `properties` | JSON | | Additional properties |

**Indexes**:
- `idx_edge_source` (source_node)
- `idx_edge_target` (target_node)
- `idx_edge_relationship` (relationship)

---

### 11. DigestSource

Sources that have been digested.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `type` | ENUM | NOT NULL | 'repository', 'file', 'api', 'document' |
| `uri` | TEXT | NOT NULL UNIQUE | Source URI |
| `name` | TEXT | NOT NULL | Human-readable name |
| `status` | ENUM | NOT NULL | 'pending', 'fetching', 'parsing', 'analyzing', 'complete', 'failed' |
| `last_digest` | TIMESTAMP | | Last successful digest |
| `version` | TEXT | | Version/commit SHA |
| `profile` | JSON | | Digest profile (profile.json) |
| `sbom` | JSON | | Software Bill of Materials |
| `security_report` | JSON | | Security findings |
| `stats` | JSON | | File counts, language breakdown |

---

### 12. Model

Registered AI models.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `name` | TEXT | NOT NULL UNIQUE | Model name |
| `type` | ENUM | NOT NULL | 'llm', 'embedding', 'vision', 'audio' |
| `provider` | TEXT | NOT NULL | 'llama.cpp', 'ollama', 'cloud' |
| `path` | TEXT | | Local file path |
| `uri` | TEXT | | Download URI |
| `size_bytes` | BIGINT | | File size |
| `parameters` | TEXT | | Parameter count (e.g., '1.5B') |
| `context_length` | INT | | Max context tokens |
| `license` | TEXT | | License type |
| `configs` | JSON | | Model-specific configs |
| `status` | ENUM | NOT NULL | 'available', 'downloading', 'loading', 'loaded', 'error' |
| `metrics` | JSON | | Performance benchmarks |

---

### 13. Device

P2P network devices.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Device unique identifier |
| `name` | TEXT | NOT NULL | User-assigned name |
| `type` | TEXT | NOT NULL | 'desktop', 'laptop', 'mobile', 'server' |
| `platform` | TEXT | NOT NULL | 'windows', 'macos', 'linux', 'ios', 'android' |
| `peer_id` | TEXT | UNIQUE | libp2p peer ID |
| `status` | ENUM | NOT NULL | 'online', 'offline', 'syncing' |
| `last_seen` | TIMESTAMP | | Last online timestamp |
| `capabilities` | JSON | | Hardware capabilities |
| `resources` | JSON | | Available resources (CPU, RAM, GPU) |
| `is_local` | BOOLEAN | NOT NULL | True for current device |

---

### 14. SyncState

P2P synchronization state.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `device_id` | UUID | FK -> Device, NOT NULL | Remote device |
| `entity_type` | TEXT | NOT NULL | Entity type being synced |
| `last_sync` | TIMESTAMP | | Last successful sync |
| `local_version` | BIGINT | NOT NULL | Local version vector |
| `remote_version` | BIGINT | | Remote version vector |
| `pending_ops` | JSON | | Pending operations |
| `conflicts` | JSON | | Unresolved conflicts |

---

### 15. Provider

Registered AI model providers (local and cloud).

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `name` | TEXT | NOT NULL UNIQUE | Provider name (e.g., 'llama.cpp', 'claude-code') |
| `type` | ENUM | NOT NULL | 'local', 'cloud', 'hybrid' |
| `interface` | ENUM | NOT NULL | 'cli', 'cloud', 'ide', 'api' |
| `status` | ENUM | NOT NULL | 'active', 'inactive', 'error', 'rate_limited' |
| `configs` | JSON | NOT NULL | Provider configsuration (auth, endpoints) |
| `capabilities` | TEXT[] | NOT NULL | Supported capabilities (inference, embeddings, code) |
| `priority` | INT | DEFAULT 0 | Selection priority (higher = preferred) |
| `rate_limits` | JSON | | Rate limiting configsuration |
| `metrics` | JSON | | Usage and performance metrics |
| `last_used` | TIMESTAMP | | Last successful invocation |
| `created_at` | TIMESTAMP | NOT NULL | Registration time |

**Indexes**:
- `idx_provider_name` (name)
- `idx_provider_type_status` (type, status)
- `idx_provider_priority` (priority DESC)

---

### 16. SharedExecutionContext

Shared memory and reasoning state across providers.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `session_id` | UUID | NOT NULL | Execution session ID |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |
| `updated_at` | TIMESTAMP | NOT NULL | Last modification |
| `context_type` | ENUM | NOT NULL | 'reasoning', 'task', 'memory', 'workflow' |
| `content` | JSON | NOT NULL | Shared context data |
| `providers` | UUID[] | | Providers participating in this context |
| `parent_context` | UUID | FK -> SharedExecutionContext | Parent context for threading |
| `ttl` | INT | | Time-to-live in seconds |
| `checksum` | TEXT | NOT NULL | Content hash for sync verification |

**Indexes**:
- `idx_shared_context_session` (session_id)
- `idx_shared_context_type` (context_type)

---

### 17. ProviderTask

Tasks distributed across providers for collaborative execution.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `task_id` | UUID | FK -> Task, NOT NULL | Parent task |
| `provider_id` | UUID | FK -> Provider, NOT NULL | Assigned provider |
| `context_id` | UUID | FK -> SharedExecutionContext | Shared context |
| `status` | ENUM | NOT NULL | 'pending', 'running', 'completed', 'failed' |
| `input` | JSON | NOT NULL | Task input |
| `output` | JSON | | Task output |
| `started_at` | TIMESTAMP | | Execution start |
| `completed_at` | TIMESTAMP | | Execution end |
| `duration_ms` | INT | | Execution duration |
| `error` | TEXT | | Error message if failed |
| `retry_count` | INT | DEFAULT 0 | Number of retries |

**Indexes**:
- `idx_provider_task_provider` (provider_id)
- `idx_provider_task_context` (context_id)
- `idx_provider_task_status` (status)

---

## Validation Rules

### Memory
- `content` must not be empty
- `checksum` must match SHA-256 of content
- `embedding_id` required for searchable memories

### Agent
- `name` must match pattern `[A-Z][a-zA-Z]+Agent`
- `version` must be valid semver
- `capabilities` must not be empty

### Task
- `priority` must be between -1000 and 1000
- `retry_count` must not exceed `max_retries`
- `deadline` if set, must be in future at creation

### KnowledgeEdge
- `source_node` and `target_node` must be different
- `relationship` must be from allowed list

---

## State Transitions

### Task Lifecycle

```
                    ┌─────────────┐
                    │   pending   │
                    └──────┬──────┘
                           │ assign
                    ┌──────▼──────┐
            ┌──────>│ in_progress │<──────┐
            │       └──────┬──────┘       │
            │              │              │ retry
            │    ┌─────────┼─────────┐    │
            │    │         │         │    │
            │    ▼         ▼         ▼    │
         blocked    completed    failed───┘
            │
            │ unblock
            └──────────────────────────────>
```

### MicroAgentStack Lifecycle

```
bootstrap → execute → validate → package → archive
    │          │          │
    └──────────┴──────────┴────> terminated (on failure)
```

---

## Autonomous Operation Entities (FR-051 to FR-075)

### 18. Goal

Unified goal queue for user-provided and self-generated goals.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `source` | ENUM | NOT NULL | 'user', 'self_generated', 'constitutional' |
| `title` | TEXT | NOT NULL | Goal description |
| `description` | TEXT | | Detailed objective |
| `priority` | INT | NOT NULL DEFAULT 0 | Priority in unified queue (higher = more urgent) |
| `status` | ENUM | NOT NULL | 'pending', 'active', 'decomposing', 'executing', 'completed', 'failed' |
| `rationale` | TEXT | | Why this goal (required for self_generated) |
| `parent_goal` | UUID | FK -> Goal | Parent goal for hierarchical decomposition |
| `metadata` | JSON | | Additional structured data |
| `created_at` | TIMESTAMP | NOT NULL | Creation time |
| `started_at` | TIMESTAMP | | Execution start time |
| `completed_at` | TIMESTAMP | | Completion time |

**Indexes**:
- `idx_goal_status` (status)
- `idx_goal_priority` (priority DESC)
- `idx_goal_source` (source)

---

### 19. Plane

3-plane control fabric for zero-downtime self-updates. See `project-mgmt/docs/07-plans/autonomous-system.md` for architecture details.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `name` | ENUM | NOT NULL UNIQUE | 'sandbox', 'deployed', 'coordinator' |
| `role` | ENUM | NOT NULL | 'testing', 'production', 'memory' |
| `status` | ENUM | NOT NULL | 'active', 'standby', 'promoting', 'draining' |
| `version` | TEXT | NOT NULL | Code/deployment version |
| `health_score` | FLOAT | NOT NULL DEFAULT 1.0 | 0.0 (unhealthy) to 1.0 (healthy) |
| `health_details` | JSON | | Detailed health metrics |
| `configs` | JSON | | Plane-specific configsuration |
| `components` | JSON | | Component manifest (agents, memory, models, etc.) |
| `workspaces_path` | TEXT | | Path to capability workspaces (sandbox only) |
| `releases_path` | TEXT | | Path to promoted releases (deployed/coordinator) |
| `activated_at` | TIMESTAMP | | When became active |
| `updated_at` | TIMESTAMP | NOT NULL | Last update time |

**Plane Roles**:
- **sandbox**: Testing/staging - ephemeral workspaces, runs selftest
- **deployed**: Production - serves live traffic, canary deployments
- **coordinator**: Long-term memory (CONSTANT) - registry, analytics, archives, never switches

**Indexes**:
- `idx_plane_status` (status)
- `idx_plane_name` (name)
- `idx_plane_role` (role)

---

### 20. PlaneTransition

Audit log for all plane transitions (promotions, rollbacks). Stored in Coordinator Plane.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `transition_type` | ENUM | NOT NULL | 'promotion', 'rollback', 'canary_start', 'canary_complete', 'emergency' |
| `source_plane` | TEXT | NOT NULL | 'sandbox' or 'deployed' |
| `target_plane` | TEXT | NOT NULL | 'deployed' (for promotion) or 'sandbox' (for rollback) |
| `capability_id` | TEXT | NOT NULL | Capability being transitioned |
| `from_version` | TEXT | NOT NULL | Version before transition |
| `to_version` | TEXT | NOT NULL | Version after transition |
| `trigger` | ENUM | NOT NULL | 'policy_pass', 'slo_violation', 'gate_breach', 'safety_event', 'manual' |
| `risk_tier` | ENUM | NOT NULL | 'low', 'medium', 'high', 'critical' |
| `canary_configs` | JSON | | Canary cohort %, duration, abort gates |
| `validation_result` | JSON | | Test results, analytics output |
| `before_state` | JSON | NOT NULL | Full state snapshot before transition |
| `after_state` | JSON | NOT NULL | Full state snapshot after transition |
| `decision_rationale` | TEXT | NOT NULL | Why Coordinator approved/rejected |
| `duration_ms` | INT | | Transition duration |
| `status` | ENUM | NOT NULL | 'pending', 'in_progress', 'success', 'failed', 'rolled_back' |
| `timestamp` | TIMESTAMP | NOT NULL | Transition time |

**Indexes**:
- `idx_transition_timestamp` (timestamp DESC)
- `idx_transition_status` (status)
- `idx_transition_capability` (capability_id)
- `idx_transition_type` (transition_type)

**Retention**: Append-only, NEVER deleted (audit trail per FR-060)

---

### 21. HealingEvent

Self-healing loop event tracking.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `component` | TEXT | NOT NULL | Affected component/agent/service |
| `stage` | ENUM | NOT NULL | 'detect', 'diagnose', 'fix', 'validate', 'escalate' |
| `anomaly_type` | TEXT | NOT NULL | Type of detected anomaly |
| `diagnosis` | JSON | | Root cause analysis results |
| `fix_action` | TEXT | | Fix action taken |
| `fix_details` | JSON | | Detailed fix parameters |
| `outcome` | ENUM | NOT NULL | 'fixed', 'retry', 'escalated', 'pending' |
| `attempt_count` | INT | NOT NULL DEFAULT 1 | Number of fix attempts |
| `health_before` | JSON | | Health metrics before fix |
| `health_after` | JSON | | Health metrics after fix |
| `related_plane` | UUID | FK -> Plane | If plane swap was used |
| `escalated_at` | TIMESTAMP | | When escalated to user (if applicable) |
| `resolved_at` | TIMESTAMP | | When fully resolved |
| `timestamp` | TIMESTAMP | NOT NULL | Event time |

**Indexes**:
- `idx_healing_component` (component)
- `idx_healing_stage` (stage)
- `idx_healing_outcome` (outcome)
- `idx_healing_timestamp` (timestamp DESC)

**Retention**: Append-only, never deleted (audit trail)

---

### 22. HealthMetric

Continuous health monitoring data.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `component` | TEXT | NOT NULL | Component being monitored |
| `metric_type` | ENUM | NOT NULL | 'cpu', 'memory', 'latency', 'error_rate', 'success_rate', 'throughput' |
| `value` | FLOAT | NOT NULL | Metric value |
| `threshold_warning` | FLOAT | | Warning threshold |
| `threshold_critical` | FLOAT | | Critical threshold |
| `status` | ENUM | NOT NULL | 'healthy', 'warning', 'critical' |
| `timestamp` | TIMESTAMP | NOT NULL | Measurement time |

**Indexes**:
- `idx_health_component_type` (component, metric_type)
- `idx_health_timestamp` (timestamp DESC)
- `idx_health_status` (status)

**Retention**: Rolling window (keep last 7 days, aggregate older)

---

### 23. CapabilityRegistry

Central registry in Coordinator Plane for all capabilities across planes. Source of truth for promotions.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `capability_id` | TEXT | NOT NULL UNIQUE | Capability identifier (e.g., 'deflex-ai-os') |
| `name` | TEXT | NOT NULL | Human-readable name |
| `version` | TEXT | NOT NULL | Current version in Deployed plane |
| `sandbox_version` | TEXT | | Version currently in Sandbox testing |
| `risk_tier` | ENUM | NOT NULL | 'low', 'medium', 'high', 'critical' |
| `components` | JSON | NOT NULL | Component manifest (agents, memory, models, etc.) |
| `dependencies` | JSON | | Dependency graph |
| `slo_baselines` | JSON | | SLO baselines for canary validation |
| `policy_file` | TEXT | NOT NULL | Path to capability policy |
| `sbom_hash` | TEXT | | Hash of latest SBOM |
| `last_promoted_at` | TIMESTAMP | | Last successful promotion |
| `last_rollback_at` | TIMESTAMP | | Last rollback (if any) |
| `promotion_count` | INT | NOT NULL DEFAULT 0 | Total promotions |
| `rollback_count` | INT | NOT NULL DEFAULT 0 | Total rollbacks |
| `status` | ENUM | NOT NULL | 'active', 'deprecated', 'revoked' |
| `created_at` | TIMESTAMP | NOT NULL | Registration time |
| `updated_at` | TIMESTAMP | NOT NULL | Last update time |

**Indexes**:
- `idx_registry_capability` (capability_id)
- `idx_registry_status` (status)
- `idx_registry_risk` (risk_tier)

**Storage**: Coordinator Plane `shared/state/registry.db`

---

### 24. SharedArtifact

Artifacts generated by Sandbox, consumed by Coordinator for analytics, stored for audit.

| Field | Type | Constraints | Description |
|-------|------|-------------|-------------|
| `id` | UUID | PK | Unique identifier |
| `capability_id` | TEXT | NOT NULL | FK to CapabilityRegistry |
| `version` | TEXT | NOT NULL | Capability version |
| `artifact_type` | ENUM | NOT NULL | 'sbom', 'risk_assessment', 'telemetry', 'test_results', 'analytics_output' |
| `file_path` | TEXT | NOT NULL | Path in shared/artifacts/ |
| `hash` | TEXT | NOT NULL | SHA-256 hash for integrity |
| `size_bytes` | BIGINT | NOT NULL | File size |
| `metadata` | JSON | | Artifact-specific metadata |
| `source_plane` | TEXT | NOT NULL | 'sandbox' or 'deployed' |
| `created_at` | TIMESTAMP | NOT NULL | Generation time |

**Indexes**:
- `idx_artifact_capability` (capability_id, version)
- `idx_artifact_type` (artifact_type)
- `idx_artifact_created` (created_at DESC)

**Storage**: `shared/artifacts/<capability_id>/<version>/`

**Retention**: NEVER deleted (part of permanent audit trail)

---

## State Transitions - Autonomous Operation

### Goal Lifecycle

```
                    ┌─────────────┐
                    │   pending   │
                    └──────┬──────┘
                           │ pick from queue
                    ┌──────▼──────┐
                    │   active    │
                    └──────┬──────┘
                           │ CECCA decomposition
                    ┌──────▼──────┐
                    │ decomposing │
                    └──────┬──────┘
                           │ tasks created
                    ┌──────▼──────┐
            ┌──────>│  executing  │<──────┐
            │       └──────┬──────┘       │
            │              │              │ sub-goal
            │    ┌─────────┼─────────┐    │
            │    │         │         │    │
            │    ▼         ▼         ▼    │
         (retry)    completed    failed───┘
```

### 3-Plane Promotion Flow

```
SANDBOX PLANE                  COORDINATOR PLANE              DEPLOYED PLANE
─────────────                  ─────────────────              ──────────────

┌───────────┐                  ┌────────────────┐
│ testing   │ ──artifacts───▶  │ evaluating     │
│ (selftest)│                  │ (analytics)    │
└───────────┘                  └───────┬────────┘
                                       │
                               pass    │   fail
                        ┌──────────────┼──────────────┐
                        │              │              │
                        ▼              │              ▼
               ┌────────────┐         │     ┌─────────────┐
               │ promoting  │         │     │ rejected    │
               │ (queued)   │         │     │ (log only)  │
               └──────┬─────┘         │     └─────────────┘
                      │               │
                      │ approved      │
                      ▼               │
                                      │         ┌───────────────┐
                                      │         │ production    │◀──┐
                                      │         │ (serving)     │   │
                                      │         └───────┬───────┘   │
                                      │                 │           │
                                      │      canary     │  full     │
                                      │      start      │  switch   │
                                      │                 │           │
                                      │         ┌───────▼───────┐   │
                                      │         │ canary        │   │
                                      │         │ (1-10%)       │───┤ SLO OK
                                      │         └───────┬───────┘   │
                                      │                 │           │
                                      │      SLO fail   │           │
                                      │                 ▼           │
                                      │         ┌───────────────┐   │
                                      └────────▶│ rolling_back  │───┘
                                                │ (autopilot)   │
                                                └───────────────┘

COORDINATOR (constant): always 'active' for memory/registry role
```

### Plane Role Matrix

```
┌────────────────┬───────────────┬──────────────────┬──────────────────┐
│     Plane      │     Role      │     Status       │   Persistence    │
├────────────────┼───────────────┼──────────────────┼──────────────────┤
│   SANDBOX      │   testing     │ testing/idle     │   Ephemeral      │
│   (Blue)       │               │                  │   (workspaces)   │
├────────────────┼───────────────┼──────────────────┼──────────────────┤
│   DEPLOYED     │   production  │ production/      │   Versioned      │
│   (Green)      │               │ canary/draining  │   (releases/)    │
├────────────────┼───────────────┼──────────────────┼──────────────────┤
│   COORDINATOR  │   memory      │ ALWAYS active    │   PERMANENT      │
│   (Constant)   │               │ (never switches) │   (registry.db)  │
└────────────────┴───────────────┴──────────────────┴──────────────────┘
```
                         └──────────┘
```

### Healing Event Flow

```
detect → diagnose → fix → validate
                      │       │
                      │       │ if failed & attempts < 3
                      │       └────────────────────────┐
                      │                                │
                      │ if failed & attempts >= 3     │
                      ▼                                │
                  escalate                             │
                                                       │
                      ▲                                │
                      │ retry                          │
                      └────────────────────────────────┘
```

---

## Indexes Summary

| Table | Index | Columns | Type |
|-------|-------|---------|------|
| memory | idx_memory_created | created_at DESC | B-tree |
| memory | idx_memory_type | type | B-tree |
| memory | idx_memory_tags | tags | GIN |
| embedding | idx_embedding_vector | vector | HNSW |
| agent | idx_agent_name | name | B-tree |
| agentlog | idx_agentlog_agent_time | agent_id, timestamp DESC | B-tree |
| task | idx_task_status | status | B-tree |
| task | idx_task_priority | priority DESC | B-tree |
| knowledgenode | idx_node_type | type | B-tree |
| knowledgeedge | idx_edge_source | source_node | B-tree |
| knowledgeedge | idx_edge_target | target_node | B-tree |
| provider | idx_provider_name | name | B-tree |
| provider | idx_provider_priority | priority DESC | B-tree |
| sharedexecutioncontext | idx_shared_context_session | session_id | B-tree |
| providertask | idx_provider_task_provider | provider_id | B-tree |
| providertask | idx_provider_task_status | status | B-tree |
| goal | idx_goal_status | status | B-tree |
| goal | idx_goal_priority | priority DESC | B-tree |
| goal | idx_goal_source | source | B-tree |
| plane | idx_plane_status | status | B-tree |
| planetransition | idx_transition_timestamp | timestamp DESC | B-tree |
| healingevent | idx_healing_component | component | B-tree |
| healingevent | idx_healing_timestamp | timestamp DESC | B-tree |
| healthmetric | idx_health_component_type | component, metric_type | B-tree |
| healthmetric | idx_health_status | status | B-tree |

---

## Entity Count Summary

| Category | Entities | FRs Covered |
|----------|----------|-------------|
| Core Data | Memory, Embedding, Agent, AgentLog, Task, TaskEvent | FR-005, FR-006 |
| Agent Architecture | MicroAgentStack, Capsule | FR-009, FR-010 |
| Knowledge | KnowledgeNode, KnowledgeEdge, DigestSource | FR-012-016 |
| Infrastructure | Model, Device, SyncState | FR-004, FR-017-020 |
| Providers | Provider, SharedExecutionContext, ProviderTask | FR-037-042 |
| Autonomous Operation | Goal, Plane, PlaneTransition, HealingEvent, HealthMetric | FR-051-075 |
| 3-Plane Control Fabric | CapabilityRegistry, SharedArtifact | FR-056-060 |

**Total Entities**: 24

### Coordinator Plane Persistent State

The Coordinator Plane maintains the following persistent entities:
- **CapabilityRegistry** - Central source of truth for all capabilities
- **PlaneTransition** - Complete audit trail of all promotions/rollbacks
- **SharedArtifact** - All artifacts from Sandbox and Deployed planes
- **Goal** - Long-term goal history (including self-generated goals)
- **HealingEvent** - Complete healing history

This data is NEVER deleted (Total Memory Sovereignty per §3.7).
