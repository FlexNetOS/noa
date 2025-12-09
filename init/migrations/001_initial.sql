-- NOA Initial Database Schema
-- Migration: 001_initial
-- Date: 2025-12-09
-- Entities: 24 (per data-model.md)
--
-- FR-003: Local-first database with concurrent modifications
-- FR-005: Persist all interactions, decisions, learnings
-- FR-006: Log all agent actions with audit trail

PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;

-- ============================================================================
-- 1. Memory - Total Memory Sovereignty (FR-005)
-- ============================================================================
CREATE TABLE IF NOT EXISTS memory (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    type TEXT NOT NULL CHECK (type IN ('interaction', 'decision', 'learning', 'artifact')),
    content TEXT NOT NULL,
    metadata TEXT,  -- JSON
    source_agent TEXT REFERENCES agent(id),
    parent_id TEXT REFERENCES memory(id),
    tags TEXT,  -- JSON array
    embedding_id TEXT REFERENCES embedding(id),
    checksum TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_memory_created ON memory(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_memory_type ON memory(type);
CREATE INDEX IF NOT EXISTS idx_memory_source ON memory(source_agent);

-- ============================================================================
-- 2. Embedding - Vector embeddings for semantic search
-- ============================================================================
CREATE TABLE IF NOT EXISTS embedding (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    vector BLOB NOT NULL,  -- 384-dim float array
    model TEXT NOT NULL,
    source_type TEXT NOT NULL CHECK (source_type IN ('memory', 'node', 'document')),
    source_id TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_embedding_source ON embedding(source_type, source_id);

-- ============================================================================
-- 3. Agent - Registered agents (FR-007, FR-008)
-- ============================================================================
CREATE TABLE IF NOT EXISTS agent (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    type TEXT NOT NULL CHECK (type IN ('permanent', 'board', 'stack', 'dynamic')),
    status TEXT NOT NULL CHECK (status IN ('active', 'paused', 'retired')) DEFAULT 'active',
    version TEXT NOT NULL,
    config TEXT NOT NULL,  -- JSON
    capabilities TEXT NOT NULL,  -- JSON array
    parent_id TEXT REFERENCES agent(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    last_active TEXT,
    metrics TEXT  -- JSON
);

CREATE INDEX IF NOT EXISTS idx_agent_name ON agent(name);
CREATE INDEX IF NOT EXISTS idx_agent_type_status ON agent(type, status);

-- ============================================================================
-- 4. AgentLog - Audit trail (FR-006)
-- ============================================================================
CREATE TABLE IF NOT EXISTS agent_log (
    id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL REFERENCES agent(id),
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    action TEXT NOT NULL,
    trigger TEXT,
    inputs TEXT,  -- JSON
    outputs TEXT,  -- JSON
    duration_ms INTEGER,
    status TEXT NOT NULL CHECK (status IN ('success', 'failure', 'timeout')),
    error TEXT,
    memory_ids TEXT,  -- JSON array of UUIDs
    parent_log_id TEXT REFERENCES agent_log(id)
);

CREATE INDEX IF NOT EXISTS idx_agentlog_agent_time ON agent_log(agent_id, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_agentlog_status ON agent_log(status);

-- ============================================================================
-- 5. Task - Work units
-- ============================================================================
CREATE TABLE IF NOT EXISTS task (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL CHECK (status IN ('pending', 'in_progress', 'blocked', 'completed', 'failed', 'cancelled')) DEFAULT 'pending',
    priority INTEGER NOT NULL DEFAULT 0,
    assigned_agent TEXT REFERENCES agent(id),
    stack_id TEXT REFERENCES micro_agent_stack(id),
    parent_task_id TEXT REFERENCES task(id),
    inputs TEXT,  -- JSON
    outputs TEXT,  -- JSON
    deadline TEXT,
    tags TEXT,  -- JSON array
    retry_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 3
);

CREATE INDEX IF NOT EXISTS idx_task_status ON task(status);
CREATE INDEX IF NOT EXISTS idx_task_priority ON task(priority DESC);
CREATE INDEX IF NOT EXISTS idx_task_assigned ON task(assigned_agent);

-- ============================================================================
-- 6. TaskEvent - Task lifecycle events
-- ============================================================================
CREATE TABLE IF NOT EXISTS task_event (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL REFERENCES task(id),
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    event_type TEXT NOT NULL CHECK (event_type IN ('created', 'assigned', 'started', 'progress', 'completed', 'failed', 'retried', 'cancelled')),
    agent_id TEXT REFERENCES agent(id),
    old_status TEXT,
    new_status TEXT,
    details TEXT  -- JSON
);

CREATE INDEX IF NOT EXISTS idx_taskevent_task ON task_event(task_id, timestamp DESC);

-- ============================================================================
-- 7. MicroAgentStack - Deployable clusters (FR-009)
-- ============================================================================
CREATE TABLE IF NOT EXISTS micro_agent_stack (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    status TEXT NOT NULL CHECK (status IN ('bootstrap', 'execute', 'validate', 'package', 'archive', 'terminated')) DEFAULT 'bootstrap',
    objective TEXT NOT NULL,
    commander_agent TEXT REFERENCES agent(id),
    member_agents TEXT,  -- JSON array of UUIDs
    config TEXT,  -- JSON
    workspace_path TEXT NOT NULL,
    artifacts TEXT,  -- JSON
    metrics TEXT,  -- JSON
    terminated_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_stack_status ON micro_agent_stack(status);

-- ============================================================================
-- 8. Capsule - Self-contained environments
-- ============================================================================
CREATE TABLE IF NOT EXISTS capsule (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    version TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('draft', 'active', 'deprecated')) DEFAULT 'draft',
    manifest TEXT NOT NULL,  -- JSON
    image_ref TEXT,
    checksum TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    promoted_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_capsule_status ON capsule(status);

-- ============================================================================
-- 9. KnowledgeNode - Knowledge graph nodes (FR-012-016)
-- ============================================================================
CREATE TABLE IF NOT EXISTS knowledge_node (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL CHECK (type IN ('function', 'class', 'module', 'file', 'repo', 'concept')),
    name TEXT NOT NULL,
    qualified_name TEXT,
    description TEXT,
    source_digest TEXT REFERENCES digest_source(id),
    location TEXT,  -- JSON (file path, lines)
    properties TEXT,  -- JSON
    embedding_id TEXT REFERENCES embedding(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_node_type ON knowledge_node(type);
CREATE INDEX IF NOT EXISTS idx_node_qualified ON knowledge_node(qualified_name);

-- ============================================================================
-- 10. KnowledgeEdge - Knowledge graph edges
-- ============================================================================
CREATE TABLE IF NOT EXISTS knowledge_edge (
    id TEXT PRIMARY KEY,
    source_node TEXT NOT NULL REFERENCES knowledge_node(id),
    target_node TEXT NOT NULL REFERENCES knowledge_node(id),
    relationship TEXT NOT NULL CHECK (relationship IN ('calls', 'imports', 'extends', 'implements', 'contains', 'references')),
    weight REAL DEFAULT 1.0,
    properties TEXT  -- JSON
);

CREATE INDEX IF NOT EXISTS idx_edge_source ON knowledge_edge(source_node);
CREATE INDEX IF NOT EXISTS idx_edge_target ON knowledge_edge(target_node);
CREATE INDEX IF NOT EXISTS idx_edge_relationship ON knowledge_edge(relationship);

-- ============================================================================
-- 11. DigestSource - Digested sources (FR-012-016)
-- ============================================================================
CREATE TABLE IF NOT EXISTS digest_source (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL CHECK (type IN ('repository', 'file', 'api', 'document')),
    uri TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'fetching', 'parsing', 'analyzing', 'complete', 'failed')) DEFAULT 'pending',
    last_digest TEXT,
    version TEXT,
    profile TEXT,  -- JSON
    sbom TEXT,  -- JSON
    security_report TEXT,  -- JSON
    stats TEXT  -- JSON
);

CREATE INDEX IF NOT EXISTS idx_digest_status ON digest_source(status);

-- ============================================================================
-- 12. Model - Registered AI models (FR-004)
-- ============================================================================
CREATE TABLE IF NOT EXISTS model (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    type TEXT NOT NULL CHECK (type IN ('llm', 'embedding', 'vision', 'audio')),
    provider TEXT NOT NULL,
    path TEXT,
    uri TEXT,
    size_bytes INTEGER,
    parameters TEXT,
    context_length INTEGER,
    license TEXT,
    config TEXT,  -- JSON
    status TEXT NOT NULL CHECK (status IN ('available', 'downloading', 'loading', 'loaded', 'error')) DEFAULT 'available',
    metrics TEXT  -- JSON
);

CREATE INDEX IF NOT EXISTS idx_model_type ON model(type);
CREATE INDEX IF NOT EXISTS idx_model_status ON model(status);

-- ============================================================================
-- 13. Device - P2P network devices (FR-017-020)
-- ============================================================================
CREATE TABLE IF NOT EXISTS device (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('desktop', 'laptop', 'mobile', 'server', 'xr')),
    platform TEXT NOT NULL CHECK (platform IN ('windows', 'macos', 'linux', 'ios', 'android')),
    peer_id TEXT UNIQUE,
    status TEXT NOT NULL CHECK (status IN ('online', 'offline', 'syncing')) DEFAULT 'offline',
    last_seen TEXT,
    capabilities TEXT,  -- JSON
    resources TEXT,  -- JSON
    is_local INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_device_status ON device(status);

-- ============================================================================
-- 14. SyncState - P2P synchronization
-- ============================================================================
CREATE TABLE IF NOT EXISTS sync_state (
    id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL REFERENCES device(id),
    entity_type TEXT NOT NULL,
    last_sync TEXT,
    local_version INTEGER NOT NULL DEFAULT 0,
    remote_version INTEGER,
    pending_ops TEXT,  -- JSON
    conflicts TEXT  -- JSON
);

CREATE INDEX IF NOT EXISTS idx_sync_device ON sync_state(device_id);

-- ============================================================================
-- 15. Provider - AI model providers (FR-037-042)
-- ============================================================================
CREATE TABLE IF NOT EXISTS provider (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    type TEXT NOT NULL CHECK (type IN ('local', 'cloud', 'hybrid', 'ide')),
    interface TEXT NOT NULL CHECK (interface IN ('cli', 'cloud', 'ide', 'api')),
    status TEXT NOT NULL CHECK (status IN ('active', 'inactive', 'error', 'rate_limited')) DEFAULT 'active',
    config TEXT NOT NULL,  -- JSON
    capabilities TEXT NOT NULL,  -- JSON array
    priority INTEGER DEFAULT 0,
    rate_limits TEXT,  -- JSON
    metrics TEXT,  -- JSON
    last_used TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_provider_name ON provider(name);
CREATE INDEX IF NOT EXISTS idx_provider_type_status ON provider(type, status);
CREATE INDEX IF NOT EXISTS idx_provider_priority ON provider(priority DESC);

-- ============================================================================
-- 16. SharedExecutionContext - Shared memory across providers (FR-037)
-- ============================================================================
CREATE TABLE IF NOT EXISTS shared_execution_context (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    context_type TEXT NOT NULL CHECK (context_type IN ('reasoning', 'task', 'memory', 'workflow')),
    content TEXT NOT NULL,  -- JSON
    providers TEXT,  -- JSON array of UUIDs
    parent_context TEXT REFERENCES shared_execution_context(id),
    ttl INTEGER,
    checksum TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_shared_context_session ON shared_execution_context(session_id);
CREATE INDEX IF NOT EXISTS idx_shared_context_type ON shared_execution_context(context_type);

-- ============================================================================
-- 17. ProviderTask - Tasks distributed across providers (FR-041)
-- ============================================================================
CREATE TABLE IF NOT EXISTS provider_task (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL REFERENCES task(id),
    provider_id TEXT NOT NULL REFERENCES provider(id),
    context_id TEXT REFERENCES shared_execution_context(id),
    status TEXT NOT NULL CHECK (status IN ('pending', 'running', 'completed', 'failed')) DEFAULT 'pending',
    input TEXT NOT NULL,  -- JSON
    output TEXT,  -- JSON
    started_at TEXT,
    completed_at TEXT,
    duration_ms INTEGER,
    error TEXT,
    retry_count INTEGER DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_provider_task_provider ON provider_task(provider_id);
CREATE INDEX IF NOT EXISTS idx_provider_task_context ON provider_task(context_id);
CREATE INDEX IF NOT EXISTS idx_provider_task_status ON provider_task(status);

-- ============================================================================
-- 18. Goal - Unified goal queue (FR-051-055, FR-066-070)
-- ============================================================================
CREATE TABLE IF NOT EXISTS goal (
    id TEXT PRIMARY KEY,
    source TEXT NOT NULL CHECK (source IN ('user', 'self_generated', 'constitutional')),
    title TEXT NOT NULL,
    description TEXT,
    priority INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL CHECK (status IN ('pending', 'active', 'decomposing', 'executing', 'completed', 'failed')) DEFAULT 'pending',
    rationale TEXT,
    parent_goal TEXT REFERENCES goal(id),
    metadata TEXT,  -- JSON
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    started_at TEXT,
    completed_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_goal_status ON goal(status);
CREATE INDEX IF NOT EXISTS idx_goal_priority ON goal(priority DESC);
CREATE INDEX IF NOT EXISTS idx_goal_source ON goal(source);

-- ============================================================================
-- 19. Plane - 3-plane control fabric (FR-056-060)
-- ============================================================================
CREATE TABLE IF NOT EXISTS plane (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE CHECK (name IN ('sandbox', 'deployed', 'coordinator')),
    role TEXT NOT NULL CHECK (role IN ('testing', 'production', 'memory')),
    status TEXT NOT NULL CHECK (status IN ('active', 'standby', 'promoting', 'draining')) DEFAULT 'standby',
    version TEXT NOT NULL,
    health_score REAL NOT NULL DEFAULT 1.0,
    health_details TEXT,  -- JSON
    config TEXT,  -- JSON
    components TEXT,  -- JSON
    workspaces_path TEXT,
    releases_path TEXT,
    activated_at TEXT,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_plane_status ON plane(status);
CREATE INDEX IF NOT EXISTS idx_plane_role ON plane(role);

-- ============================================================================
-- 20. PlaneTransition - Audit log for plane transitions (FR-060)
-- ============================================================================
CREATE TABLE IF NOT EXISTS plane_transition (
    id TEXT PRIMARY KEY,
    transition_type TEXT NOT NULL CHECK (transition_type IN ('promotion', 'rollback', 'canary_start', 'canary_complete', 'emergency')),
    source_plane TEXT NOT NULL,
    target_plane TEXT NOT NULL,
    capability_id TEXT NOT NULL,
    from_version TEXT NOT NULL,
    to_version TEXT NOT NULL,
    trigger TEXT NOT NULL CHECK (trigger IN ('policy_pass', 'slo_violation', 'gate_breach', 'safety_event', 'manual')),
    risk_tier TEXT NOT NULL CHECK (risk_tier IN ('low', 'medium', 'high', 'critical')),
    canary_config TEXT,  -- JSON
    validation_result TEXT,  -- JSON
    before_state TEXT NOT NULL,  -- JSON
    after_state TEXT NOT NULL,  -- JSON
    decision_rationale TEXT NOT NULL,
    duration_ms INTEGER,
    status TEXT NOT NULL CHECK (status IN ('pending', 'in_progress', 'success', 'failed', 'rolled_back')) DEFAULT 'pending',
    timestamp TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_transition_timestamp ON plane_transition(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_transition_status ON plane_transition(status);
CREATE INDEX IF NOT EXISTS idx_transition_capability ON plane_transition(capability_id);

-- ============================================================================
-- 21. HealingEvent - Self-healing loop events (FR-071-075)
-- ============================================================================
CREATE TABLE IF NOT EXISTS healing_event (
    id TEXT PRIMARY KEY,
    component TEXT NOT NULL,
    stage TEXT NOT NULL CHECK (stage IN ('detect', 'diagnose', 'fix', 'validate', 'escalate')),
    anomaly_type TEXT NOT NULL,
    diagnosis TEXT,  -- JSON
    fix_action TEXT,
    fix_details TEXT,  -- JSON
    outcome TEXT NOT NULL CHECK (outcome IN ('fixed', 'retry', 'escalated', 'pending')) DEFAULT 'pending',
    attempt_count INTEGER NOT NULL DEFAULT 1,
    health_before TEXT,  -- JSON
    health_after TEXT,  -- JSON
    related_plane TEXT REFERENCES plane(id),
    escalated_at TEXT,
    resolved_at TEXT,
    timestamp TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_healing_component ON healing_event(component);
CREATE INDEX IF NOT EXISTS idx_healing_stage ON healing_event(stage);
CREATE INDEX IF NOT EXISTS idx_healing_outcome ON healing_event(outcome);
CREATE INDEX IF NOT EXISTS idx_healing_timestamp ON healing_event(timestamp DESC);

-- ============================================================================
-- 22. HealthMetric - Continuous health monitoring (FR-072)
-- ============================================================================
CREATE TABLE IF NOT EXISTS health_metric (
    id TEXT PRIMARY KEY,
    component TEXT NOT NULL,
    metric_type TEXT NOT NULL CHECK (metric_type IN ('cpu', 'memory', 'latency', 'error_rate', 'success_rate', 'throughput')),
    value REAL NOT NULL,
    threshold_warning REAL,
    threshold_critical REAL,
    status TEXT NOT NULL CHECK (status IN ('healthy', 'warning', 'critical')) DEFAULT 'healthy',
    timestamp TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_health_component_type ON health_metric(component, metric_type);
CREATE INDEX IF NOT EXISTS idx_health_timestamp ON health_metric(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_health_status ON health_metric(status);

-- ============================================================================
-- 23. CapabilityRegistry - Central registry (FR-056-060)
-- ============================================================================
CREATE TABLE IF NOT EXISTS capability_registry (
    id TEXT PRIMARY KEY,
    capability_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    sandbox_version TEXT,
    risk_tier TEXT NOT NULL CHECK (risk_tier IN ('low', 'medium', 'high', 'critical')),
    components TEXT NOT NULL,  -- JSON
    dependencies TEXT,  -- JSON
    slo_baselines TEXT,  -- JSON
    policy_file TEXT NOT NULL,
    sbom_hash TEXT,
    last_promoted_at TEXT,
    last_rollback_at TEXT,
    promotion_count INTEGER NOT NULL DEFAULT 0,
    rollback_count INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL CHECK (status IN ('active', 'deprecated', 'revoked')) DEFAULT 'active',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_registry_capability ON capability_registry(capability_id);
CREATE INDEX IF NOT EXISTS idx_registry_status ON capability_registry(status);
CREATE INDEX IF NOT EXISTS idx_registry_risk ON capability_registry(risk_tier);

-- ============================================================================
-- 24. SharedArtifact - Artifacts for audit (FR-060)
-- ============================================================================
CREATE TABLE IF NOT EXISTS shared_artifact (
    id TEXT PRIMARY KEY,
    capability_id TEXT NOT NULL,
    version TEXT NOT NULL,
    artifact_type TEXT NOT NULL CHECK (artifact_type IN ('sbom', 'risk_assessment', 'telemetry', 'test_results', 'analytics_output')),
    file_path TEXT NOT NULL,
    hash TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    metadata TEXT,  -- JSON
    source_plane TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_artifact_capability ON shared_artifact(capability_id, version);
CREATE INDEX IF NOT EXISTS idx_artifact_type ON shared_artifact(artifact_type);
CREATE INDEX IF NOT EXISTS idx_artifact_created ON shared_artifact(created_at DESC);

-- ============================================================================
-- Module Registry - Content-Addressable Storage (FR-176-180)
-- ============================================================================
CREATE TABLE IF NOT EXISTS module (
    id TEXT PRIMARY KEY,  -- SHA-256 content hash
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('binary', 'package', 'library', 'tool', 'service', 'agent', 'microkernel')),
    status TEXT NOT NULL CHECK (status IN ('registered', 'verified', 'loaded', 'executing', 'unloading', 'archived')) DEFAULT 'registered',
    capabilities TEXT,  -- JSON array
    dependencies TEXT,  -- JSON array of module IDs
    metadata TEXT,  -- JSON
    cas_path TEXT NOT NULL,  -- Path in CAS storage
    checksum TEXT NOT NULL,
    size_bytes INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(name, version)
);

CREATE INDEX IF NOT EXISTS idx_module_name ON module(name);
CREATE INDEX IF NOT EXISTS idx_module_type ON module(type);
CREATE INDEX IF NOT EXISTS idx_module_status ON module(status);

-- ============================================================================
-- 26. Traces - Execution tracing for observability (FR-155)
-- ============================================================================
CREATE TABLE IF NOT EXISTS traces (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    parent_trace_id TEXT REFERENCES traces(id),
    action TEXT NOT NULL,
    input TEXT,  -- JSON
    output TEXT,  -- JSON
    duration_ms INTEGER,
    status TEXT NOT NULL CHECK (status IN ('success', 'failure', 'timeout', 'cancelled')) DEFAULT 'success',
    error TEXT,
    agent_id TEXT REFERENCES agent(id),
    timestamp TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_traces_run ON traces(run_id);
CREATE INDEX IF NOT EXISTS idx_traces_timestamp ON traces(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_traces_status ON traces(status);

-- ============================================================================
-- 27. Claims - Verified claims for trust (FR-005)
-- ============================================================================
CREATE TABLE IF NOT EXISTS claims (
    id TEXT PRIMARY KEY,
    statement TEXT NOT NULL,
    evidence_ids TEXT,  -- JSON array of evidence IDs
    verified INTEGER NOT NULL DEFAULT 0,  -- Boolean
    confidence REAL,  -- 0.0 to 1.0
    source TEXT NOT NULL,
    category TEXT,  -- claim category
    timestamp TEXT NOT NULL DEFAULT (datetime('now')),
    verified_at TEXT,
    verified_by TEXT REFERENCES agent(id)
);

CREATE INDEX IF NOT EXISTS idx_claims_verified ON claims(verified);
CREATE INDEX IF NOT EXISTS idx_claims_category ON claims(category);
CREATE INDEX IF NOT EXISTS idx_claims_timestamp ON claims(timestamp DESC);

-- ============================================================================
-- 28. Evidence - Supporting evidence for claims (FR-005)
-- ============================================================================
CREATE TABLE IF NOT EXISTS evidence (
    id TEXT PRIMARY KEY,
    claim_id TEXT REFERENCES claims(id),
    source TEXT NOT NULL,
    source_type TEXT NOT NULL CHECK (source_type IN ('url', 'file', 'api', 'agent', 'user')),
    content TEXT NOT NULL,
    hash TEXT NOT NULL,  -- SHA-256 of content
    reliability_score REAL,  -- 0.0 to 1.0
    timestamp TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_evidence_claim ON evidence(claim_id);
CREATE INDEX IF NOT EXISTS idx_evidence_source ON evidence(source_type);

-- ============================================================================
-- 29. Metrics - System metrics for observability (FR-156)
-- ============================================================================
CREATE TABLE IF NOT EXISTS metrics (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    value REAL NOT NULL,
    unit TEXT,
    labels TEXT,  -- JSON key-value pairs
    component TEXT NOT NULL,
    timestamp TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_metrics_name ON metrics(name);
CREATE INDEX IF NOT EXISTS idx_metrics_component ON metrics(component);
CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON metrics(timestamp DESC);

-- ============================================================================
-- Schema Migrations Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS schema_migrations (
    version TEXT PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (datetime('now')),
    description TEXT
);

-- Record this migration
INSERT OR IGNORE INTO schema_migrations (version, description)
VALUES ('001_initial', 'Initial schema with 29 entities: 24 core + module + traces/claims/evidence/metrics');

-- ============================================================================
-- Initial Data: 3-Plane Configuration
-- ============================================================================
INSERT OR IGNORE INTO plane (id, name, role, status, version, health_score)
VALUES
    ('sandbox-plane-001', 'sandbox', 'testing', 'standby', '0.1.0', 1.0),
    ('deployed-plane-001', 'deployed', 'production', 'standby', '0.1.0', 1.0),
    ('coordinator-plane-001', 'coordinator', 'memory', 'active', '0.1.0', 1.0);

-- ============================================================================
-- Initial Data: Core Agents
-- ============================================================================
INSERT OR IGNORE INTO agent (id, name, type, status, version, config, capabilities)
VALUES
    ('cecca-001', 'CECCA', 'permanent', 'active', '0.1.0', '{"role": "root_orchestrator"}', '["goal_decomposition", "task_coordination", "agent_management"]'),
    ('fileio-001', 'FileIOAgent', 'permanent', 'active', '0.1.0', '{"timeout_ms": 100}', '["file_read", "file_write", "directory_management"]'),
    ('terminal-001', 'TerminalAgent', 'permanent', 'active', '0.1.0', '{"timeout_s": 30}', '["shell_execution", "process_management"]'),
    ('rag-001', 'RAGAgent', 'permanent', 'active', '0.1.0', '{"latency_ms": 500}', '["context_retrieval", "semantic_search"]'),
    ('microservice-001', 'MicroserviceManagementAgent', 'permanent', 'active', '0.1.0', '{"deploy_timeout_s": 10}', '["service_deployment", "health_check"]');

-- ============================================================================
-- Initial Data: Board Agents (FR-142)
-- ============================================================================
INSERT OR IGNORE INTO agent (id, name, type, status, version, config, capabilities)
VALUES
    ('legal-001', 'LegalAgent', 'board', 'active', '0.1.0', '{"escalation_threshold": "any_ambiguity"}', '["contract_review", "compliance_checking", "license_analysis"]'),
    ('finance-001', 'FinanceAgent', 'board', 'active', '0.1.0', '{"budget_variance_threshold": 0.10}', '["cost_tracking", "resource_budgeting", "roi_analysis"]'),
    ('security-001', 'SecurityAgent', 'board', 'active', '0.1.0', '{"escalation_threshold": "high_critical"}', '["threat_assessment", "vulnerability_triage", "access_control"]'),
    ('operations-001', 'OperationsAgent', 'board', 'active', '0.1.0', '{"escalation_threshold": "slo_breach"}', '["health_monitoring", "capacity_planning", "incident_coordination"]'),
    ('qa-001', 'QAAgent', 'board', 'active', '0.1.0', '{"coverage_threshold": 0.80}', '["test_coverage_analysis", "quality_gate_enforcement", "regression_detection"]'),
    ('architecture-001', 'ArchitectureAgent', 'board', 'active', '0.1.0', '{"escalation_threshold": "violation"}', '["design_review", "dependency_analysis", "tech_debt_tracking"]');

-- ============================================================================
-- Initial Data: AI Providers (FR-039)
-- ============================================================================
INSERT OR IGNORE INTO provider (id, name, type, interface, status, config, capabilities, priority)
VALUES
    ('llama-cpp-001', 'llama.cpp', 'local', 'api', 'active', '{"endpoint": "http://localhost:8080"}', '["inference", "embeddings"]', 100),
    ('cursor-001', 'cursor', 'hybrid', 'ide', 'active', '{"integration": "ide"}', '["code_completion", "refactoring", "orchestration"]', 90),
    ('claude-code-001', 'claude-code', 'cloud', 'cli', 'inactive', '{"api_key_env": "ANTHROPIC_API_KEY"}', '["reasoning", "code_generation", "long_context"]', 80),
    ('codex-001', 'codex', 'cloud', 'api', 'inactive', '{"api_key_env": "OPENAI_API_KEY"}', '["code_completion", "code_generation"]', 70),
    ('vscode-copilot-001', 'vscode-copilot', 'ide', 'ide', 'inactive', '{"integration": "vscode"}', '["inline_completion"]', 60),
    ('git-cli-001', 'git-cli', 'local', 'cli', 'active', '{"path": "git"}', '["version_control"]', 50),
    ('abacus-001', 'abacus', 'cloud', 'api', 'inactive', '{"api_key_env": "ABACUS_API_KEY"}', '["numerical_analysis", "data_processing"]', 40);

-- End of migration

