-- NOA Seed Foundation - Complete Database Schema
-- SQLite Primary Schema with all 24 entities
-- Date: 2025-12-10

-- Enable foreign keys
PRAGMA foreign_keys = ON;

-- ============================================================================
-- CORE ENTITIES
-- ============================================================================

-- 1. Memory - Total Memory Sovereignty
CREATE TABLE IF NOT EXISTS memory (
    id TEXT PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    type TEXT NOT NULL CHECK(type IN ('interaction', 'decision', 'learning', 'artifact')),
    content TEXT NOT NULL,
    metadata TEXT, -- JSON
    source_agent TEXT,
    parent_id TEXT,
    tags TEXT, -- JSON array
    embedding_id TEXT,
    checksum TEXT NOT NULL,
    FOREIGN KEY (source_agent) REFERENCES agent(id),
    FOREIGN KEY (parent_id) REFERENCES memory(id),
    FOREIGN KEY (embedding_id) REFERENCES embedding(id)
);

CREATE INDEX IF NOT EXISTS idx_memory_created ON memory(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_memory_type ON memory(type);
CREATE INDEX IF NOT EXISTS idx_memory_checksum ON memory(checksum);

-- 2. Embedding - Vector representations
CREATE TABLE IF NOT EXISTS embedding (
    id TEXT PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    vector BLOB NOT NULL, -- 384-dim float array serialized
    model TEXT NOT NULL,
    source_type TEXT NOT NULL,
    source_id TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_embedding_source ON embedding(source_type, source_id);

-- 3. Agent - Registered agents
CREATE TABLE IF NOT EXISTS agent (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    type TEXT NOT NULL CHECK(type IN ('permanent', 'board', 'stack', 'dynamic')),
    status TEXT NOT NULL CHECK(status IN ('active', 'paused', 'retired')),
    version TEXT NOT NULL,
    config TEXT NOT NULL, -- JSON
    capabilities TEXT NOT NULL, -- JSON array
    parent_id TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_active TIMESTAMP,
    metrics TEXT, -- JSON
    FOREIGN KEY (parent_id) REFERENCES agent(id)
);

CREATE INDEX IF NOT EXISTS idx_agent_name ON agent(name);
CREATE INDEX IF NOT EXISTS idx_agent_type_status ON agent(type, status);

-- 4. AgentLog - Audit trail
CREATE TABLE IF NOT EXISTS agent_log (
    id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    action TEXT NOT NULL,
    trigger TEXT,
    inputs TEXT, -- JSON
    outputs TEXT, -- JSON
    duration_ms INTEGER,
    status TEXT NOT NULL CHECK(status IN ('success', 'failure', 'timeout')),
    error TEXT,
    memory_ids TEXT, -- JSON array
    parent_log_id TEXT,
    FOREIGN KEY (agent_id) REFERENCES agent(id),
    FOREIGN KEY (parent_log_id) REFERENCES agent_log(id)
);

CREATE INDEX IF NOT EXISTS idx_agentlog_agent_time ON agent_log(agent_id, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_agentlog_status ON agent_log(status);

-- 5. Task - Work units
CREATE TABLE IF NOT EXISTS task (
    id TEXT PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL CHECK(status IN ('pending', 'in_progress', 'blocked', 'completed', 'failed', 'cancelled')),
    priority INTEGER NOT NULL DEFAULT 0,
    assigned_agent TEXT,
    stack_id TEXT,
    parent_task_id TEXT,
    inputs TEXT, -- JSON
    outputs TEXT, -- JSON
    deadline TIMESTAMP,
    tags TEXT, -- JSON array
    retry_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 3,
    FOREIGN KEY (assigned_agent) REFERENCES agent(id),
    FOREIGN KEY (stack_id) REFERENCES micro_agent_stack(id),
    FOREIGN KEY (parent_task_id) REFERENCES task(id)
);

CREATE INDEX IF NOT EXISTS idx_task_status ON task(status);
CREATE INDEX IF NOT EXISTS idx_task_priority ON task(priority DESC);
CREATE INDEX IF NOT EXISTS idx_task_assigned ON task(assigned_agent);

-- 6. TaskEvent - Task lifecycle events
CREATE TABLE IF NOT EXISTS task_event (
    id TEXT PRIMARY KEY,
    task_id TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    event_type TEXT NOT NULL CHECK(event_type IN ('created', 'assigned', 'started', 'progress', 'completed', 'failed', 'retried', 'cancelled')),
    agent_id TEXT,
    old_status TEXT,
    new_status TEXT,
    details TEXT, -- JSON
    FOREIGN KEY (task_id) REFERENCES task(id),
    FOREIGN KEY (agent_id) REFERENCES agent(id)
);

CREATE INDEX IF NOT EXISTS idx_taskevent_task ON task_event(task_id, timestamp DESC);

-- 7. MicroAgentStack - Agent clusters
CREATE TABLE IF NOT EXISTS micro_agent_stack (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status TEXT NOT NULL CHECK(status IN ('bootstrap', 'execute', 'validate', 'package', 'archive', 'terminated')),
    objective TEXT NOT NULL,
    commander_agent TEXT,
    member_agents TEXT, -- JSON array
    config TEXT, -- JSON
    workspace_path TEXT NOT NULL,
    artifacts TEXT, -- JSON
    metrics TEXT, -- JSON
    terminated_at TIMESTAMP,
    FOREIGN KEY (commander_agent) REFERENCES agent(id)
);

CREATE INDEX IF NOT EXISTS idx_stack_status ON micro_agent_stack(status);

-- 8. Capsule - Self-contained environments
CREATE TABLE IF NOT EXISTS capsule (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    version TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('draft', 'active', 'deprecated')),
    manifest TEXT NOT NULL, -- JSON
    image_ref TEXT,
    checksum TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    promoted_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_capsule_name_version ON capsule(name, version);

-- ============================================================================
-- KNOWLEDGE GRAPH
-- ============================================================================

-- 9. KnowledgeNode - Nodes in knowledge graph
CREATE TABLE IF NOT EXISTS knowledge_node (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL,
    name TEXT NOT NULL,
    qualified_name TEXT,
    description TEXT,
    source_digest TEXT,
    location TEXT, -- JSON
    properties TEXT, -- JSON
    embedding_id TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (embedding_id) REFERENCES embedding(id)
);

CREATE INDEX IF NOT EXISTS idx_node_type ON knowledge_node(type);
CREATE INDEX IF NOT EXISTS idx_node_qualified ON knowledge_node(qualified_name);

-- 10. KnowledgeEdge - Edges in knowledge graph
CREATE TABLE IF NOT EXISTS knowledge_edge (
    id TEXT PRIMARY KEY,
    source_node TEXT NOT NULL,
    target_node TEXT NOT NULL,
    edge_type TEXT NOT NULL,
    weight REAL DEFAULT 1.0,
    properties TEXT, -- JSON
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (source_node) REFERENCES knowledge_node(id),
    FOREIGN KEY (target_node) REFERENCES knowledge_node(id)
);

CREATE INDEX IF NOT EXISTS idx_edge_source ON knowledge_edge(source_node);
CREATE INDEX IF NOT EXISTS idx_edge_target ON knowledge_edge(target_node);
CREATE INDEX IF NOT EXISTS idx_edge_type ON knowledge_edge(edge_type);

-- ============================================================================
-- DIGEST PIPELINE
-- ============================================================================

-- 11. DigestSource - Source repositories/files
CREATE TABLE IF NOT EXISTS digest_source (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL CHECK(type IN ('git_repo', 'local_dir', 'file', 'url')),
    uri TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('pending', 'processing', 'completed', 'failed')),
    config TEXT, -- JSON
    last_digest_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_digest_status ON digest_source(status);

-- 12. DigestJob - Digest processing jobs
CREATE TABLE IF NOT EXISTS digest_job (
    id TEXT PRIMARY KEY,
    source_id TEXT NOT NULL,
    started_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    status TEXT NOT NULL CHECK(status IN ('running', 'completed', 'failed')),
    files_processed INTEGER DEFAULT 0,
    nodes_created INTEGER DEFAULT 0,
    edges_created INTEGER DEFAULT 0,
    error TEXT,
    FOREIGN KEY (source_id) REFERENCES digest_source(id)
);

CREATE INDEX IF NOT EXISTS idx_digestjob_source ON digest_job(source_id);

-- ============================================================================
-- MODEL MANAGEMENT
-- ============================================================================

-- 13. Model - AI models
CREATE TABLE IF NOT EXISTS model (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    type TEXT NOT NULL CHECK(type IN ('llm', 'embedding', 'vision', 'audio')),
    provider TEXT NOT NULL,
    version TEXT NOT NULL,
    config TEXT NOT NULL, -- JSON
    capabilities TEXT, -- JSON array
    status TEXT NOT NULL CHECK(status IN ('available', 'downloading', 'loading', 'loaded', 'failed')),
    file_path TEXT,
    file_size INTEGER,
    checksum TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_model_type ON model(type);
CREATE INDEX IF NOT EXISTS idx_model_status ON model(status);

-- 14. ModelMetrics - Model performance metrics
CREATE TABLE IF NOT EXISTS model_metrics (
    id TEXT PRIMARY KEY,
    model_id TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metric_type TEXT NOT NULL,
    value REAL NOT NULL,
    metadata TEXT, -- JSON
    FOREIGN KEY (model_id) REFERENCES model(id)
);

CREATE INDEX IF NOT EXISTS idx_modelmetrics_model ON model_metrics(model_id, timestamp DESC);

-- ============================================================================
-- P2P NETWORKING
-- ============================================================================

-- 15. Device - P2P devices
CREATE TABLE IF NOT EXISTS device (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    device_type TEXT NOT NULL,
    peer_id TEXT NOT NULL UNIQUE,
    public_key TEXT NOT NULL,
    capabilities TEXT, -- JSON array
    status TEXT NOT NULL CHECK(status IN ('online', 'offline', 'syncing')),
    last_seen TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_device_status ON device(status);
CREATE INDEX IF NOT EXISTS idx_device_peer ON device(peer_id);

-- 16. SyncState - P2P sync state
CREATE TABLE IF NOT EXISTS sync_state (
    id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    version INTEGER NOT NULL,
    checksum TEXT NOT NULL,
    synced_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (device_id) REFERENCES device(id)
);

CREATE INDEX IF NOT EXISTS idx_sync_device ON sync_state(device_id);
CREATE INDEX IF NOT EXISTS idx_sync_entity ON sync_state(entity_type, entity_id);

-- 17. P2PMessage - P2P messages
CREATE TABLE IF NOT EXISTS p2p_message (
    id TEXT PRIMARY KEY,
    from_device TEXT NOT NULL,
    to_device TEXT,
    message_type TEXT NOT NULL,
    payload TEXT NOT NULL, -- JSON
    signature TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    delivered_at TIMESTAMP,
    FOREIGN KEY (from_device) REFERENCES device(id),
    FOREIGN KEY (to_device) REFERENCES device(id)
);

CREATE INDEX IF NOT EXISTS idx_p2pmsg_from ON p2p_message(from_device, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_p2pmsg_to ON p2p_message(to_device, delivered_at);

-- ============================================================================
-- SELF-IMPROVEMENT
-- ============================================================================

-- 18. CodeModification - Code changes
CREATE TABLE IF NOT EXISTS code_modification (
    id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    file_path TEXT NOT NULL,
    change_type TEXT NOT NULL CHECK(change_type IN ('create', 'modify', 'delete', 'refactor')),
    diff TEXT NOT NULL,
    reason TEXT NOT NULL,
    approved BOOLEAN DEFAULT 0,
    applied BOOLEAN DEFAULT 0,
    rollback_id TEXT,
    FOREIGN KEY (agent_id) REFERENCES agent(id),
    FOREIGN KEY (rollback_id) REFERENCES code_modification(id)
);

CREATE INDEX IF NOT EXISTS idx_codemod_agent ON code_modification(agent_id, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_codemod_file ON code_modification(file_path);

-- 19. TestResult - Test execution results
CREATE TABLE IF NOT EXISTS test_result (
    id TEXT PRIMARY KEY,
    code_modification_id TEXT,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    test_suite TEXT NOT NULL,
    passed INTEGER NOT NULL,
    failed INTEGER NOT NULL,
    skipped INTEGER DEFAULT 0,
    duration_ms INTEGER,
    details TEXT, -- JSON
    FOREIGN KEY (code_modification_id) REFERENCES code_modification(id)
);

CREATE INDEX IF NOT EXISTS idx_testresult_mod ON test_result(code_modification_id);

-- ============================================================================
-- CONFIGURATION & POLICIES
-- ============================================================================

-- 20. Policy - Constitutional policies
CREATE TABLE IF NOT EXISTS policy (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL,
    principle_ref TEXT NOT NULL,
    rule TEXT NOT NULL,
    enforcement TEXT NOT NULL CHECK(enforcement IN ('strict', 'warn', 'audit')),
    active BOOLEAN DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_policy_category ON policy(category);
CREATE INDEX IF NOT EXISTS idx_policy_active ON policy(active);

-- 21. PolicyViolation - Policy violations
CREATE TABLE IF NOT EXISTS policy_violation (
    id TEXT PRIMARY KEY,
    policy_id TEXT NOT NULL,
    agent_id TEXT,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    severity TEXT NOT NULL CHECK(severity IN ('critical', 'high', 'medium', 'low')),
    description TEXT NOT NULL,
    context TEXT, -- JSON
    resolved BOOLEAN DEFAULT 0,
    resolution TEXT,
    FOREIGN KEY (policy_id) REFERENCES policy(id),
    FOREIGN KEY (agent_id) REFERENCES agent(id)
);

CREATE INDEX IF NOT EXISTS idx_violation_policy ON policy_violation(policy_id);
CREATE INDEX IF NOT EXISTS idx_violation_resolved ON policy_violation(resolved);

-- ============================================================================
-- ARTIFACTS & STORAGE
-- ============================================================================

-- 22. Artifact - Generated artifacts
CREATE TABLE IF NOT EXISTS artifact (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    agent_id TEXT,
    task_id TEXT,
    file_path TEXT NOT NULL,
    file_size INTEGER,
    mime_type TEXT,
    checksum TEXT NOT NULL,
    metadata TEXT, -- JSON
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (agent_id) REFERENCES agent(id),
    FOREIGN KEY (task_id) REFERENCES task(id)
);

CREATE INDEX IF NOT EXISTS idx_artifact_agent ON artifact(agent_id);
CREATE INDEX IF NOT EXISTS idx_artifact_task ON artifact(task_id);
CREATE INDEX IF NOT EXISTS idx_artifact_checksum ON artifact(checksum);

-- 23. StorageQuota - Storage quotas
CREATE TABLE IF NOT EXISTS storage_quota (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    quota_bytes INTEGER NOT NULL,
    used_bytes INTEGER DEFAULT 0,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_quota_entity ON storage_quota(entity_type, entity_id);

-- 24. AuditLog - System-wide audit log
CREATE TABLE IF NOT EXISTS audit_log (
    id TEXT PRIMARY KEY,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    actor_type TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    action TEXT NOT NULL,
    resource_type TEXT,
    resource_id TEXT,
    details TEXT, -- JSON
    ip_address TEXT,
    user_agent TEXT
);

CREATE INDEX IF NOT EXISTS idx_audit_timestamp ON audit_log(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_audit_actor ON audit_log(actor_type, actor_id);
CREATE INDEX IF NOT EXISTS idx_audit_resource ON audit_log(resource_type, resource_id);

-- ============================================================================
-- TRIGGERS FOR UPDATED_AT
-- ============================================================================

CREATE TRIGGER IF NOT EXISTS update_memory_timestamp 
AFTER UPDATE ON memory
BEGIN
    UPDATE memory SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_task_timestamp 
AFTER UPDATE ON task
BEGIN
    UPDATE task SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_policy_timestamp 
AFTER UPDATE ON policy
BEGIN
    UPDATE policy SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

-- ============================================================================
-- INITIAL DATA
-- ============================================================================

-- Insert CECCA (Chief Executive Command & Control Agent)
INSERT OR IGNORE INTO agent (id, name, type, status, version, config, capabilities)
VALUES (
    'cecca-root-001',
    'CECCA',
    'permanent',
    'active',
    '0.1.0',
    '{"role":"orchestrator","max_stacks":100}',
    '["goal_decomposition","task_routing","stack_management","resource_allocation"]'
);

-- Insert Permanent Board Agents
INSERT OR IGNORE INTO agent (id, name, type, status, version, config, capabilities)
VALUES 
    ('board-fileio-001', 'FileIOAgent', 'board', 'active', '0.1.0', '{}', '["file_read","file_write","directory_ops"]'),
    ('board-network-001', 'NetworkAgent', 'board', 'active', '0.1.0', '{}', '["http_request","websocket","api_call"]'),
    ('board-compute-001', 'ComputeAgent', 'board', 'active', '0.1.0', '{}', '["code_execution","process_management"]'),
    ('board-memory-001', 'MemoryAgent', 'board', 'active', '0.1.0', '{}', '["memory_store","memory_retrieve","memory_search"]');

-- Insert Constitutional Policies
INSERT OR IGNORE INTO policy (id, name, category, principle_ref, rule, enforcement)
VALUES
    ('pol-001', 'Self-Contained Paths', 'infrastructure', '§3.1', 'All file paths must resolve under noa_root', 'strict'),
    ('pol-002', 'Offline Capability', 'infrastructure', '§3.2', 'System must function without internet', 'warn'),
    ('pol-003', 'Agent Transparency', 'governance', '§3.5', 'All agent actions must be logged', 'strict'),
    ('pol-004', 'Data Sovereignty', 'security', '§3.7', 'User data never leaves device without consent', 'strict'),
    ('pol-005', 'Test Everything', 'quality', '§3.12', 'All code changes must have tests', 'warn');

-- ============================================================================
-- VIEWS FOR COMMON QUERIES
-- ============================================================================

CREATE VIEW IF NOT EXISTS v_active_tasks AS
SELECT 
    t.*,
    a.name as agent_name,
    s.name as stack_name
FROM task t
LEFT JOIN agent a ON t.assigned_agent = a.id
LEFT JOIN micro_agent_stack s ON t.stack_id = s.id
WHERE t.status IN ('pending', 'in_progress', 'blocked');

CREATE VIEW IF NOT EXISTS v_agent_performance AS
SELECT 
    a.id,
    a.name,
    a.type,
    COUNT(DISTINCT al.id) as total_actions,
    SUM(CASE WHEN al.status = 'success' THEN 1 ELSE 0 END) as successful_actions,
    AVG(al.duration_ms) as avg_duration_ms,
    MAX(al.timestamp) as last_action
FROM agent a
LEFT JOIN agent_log al ON a.id = al.agent_id
GROUP BY a.id, a.name, a.type;

CREATE VIEW IF NOT EXISTS v_knowledge_graph_stats AS
SELECT 
    (SELECT COUNT(*) FROM knowledge_node) as total_nodes,
    (SELECT COUNT(*) FROM knowledge_edge) as total_edges,
    (SELECT COUNT(DISTINCT type) FROM knowledge_node) as node_types,
    (SELECT COUNT(DISTINCT edge_type) FROM knowledge_edge) as edge_types;

-- ============================================================================
-- SCHEMA VERSION
-- ============================================================================

CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description TEXT
);

INSERT INTO schema_version (version, description)
VALUES (1, 'Initial schema with all 24 entities');
