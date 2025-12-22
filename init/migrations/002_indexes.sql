-- NOA Database Indexes
-- Migration: 002_indexes
-- Date: 2025-12-09
-- Purpose: Additional indexes for performance optimization

PRAGMA foreign_keys = ON;

-- ============================================================================
-- Composite Indexes for Common Query Patterns
-- ============================================================================

-- Memory: Common filter by type + time range
CREATE INDEX IF NOT EXISTS idx_memory_type_created
ON memory(type, created_at DESC);

-- Agent: Active agents by type
CREATE INDEX IF NOT EXISTS idx_agent_status_type
ON agent(status, type);

-- Task: Queue ordering (status + priority + deadline)
CREATE INDEX IF NOT EXISTS idx_task_queue
ON task(status, priority DESC, deadline ASC);

-- Task: Agent workload
CREATE INDEX IF NOT EXISTS idx_task_agent_status
ON task(assigned_agent, status);

-- AgentLog: Audit queries by time window
CREATE INDEX IF NOT EXISTS idx_agentlog_time_status
ON agent_log(timestamp DESC, status);

-- Knowledge Graph: Path traversal optimization
CREATE INDEX IF NOT EXISTS idx_edge_source_rel
ON knowledge_edge(source_node, relationship);

CREATE INDEX IF NOT EXISTS idx_edge_target_rel
ON knowledge_edge(target_node, relationship);

-- DigestSource: Processing queue
CREATE INDEX IF NOT EXISTS idx_digest_status_type
ON digest_source(status, type);

-- Provider: Selection priority
-- CREATE INDEX IF NOT EXISTS idx_provider_status_priority
-- ON provider(status, priority DESC);

-- Provider Task: Queue management
-- CREATE INDEX IF NOT EXISTS idx_providertask_status_provider
-- ON provider_task(status, provider_id);

-- Goals: Queue ordering
-- CREATE INDEX IF NOT EXISTS idx_goal_queue
-- ON goal(status, priority DESC, created_at ASC);

-- Plane transitions: Capability history
-- CREATE INDEX IF NOT EXISTS idx_transition_cap_time
-- ON plane_transition(capability_id, timestamp DESC);

-- Healing: Component health history
-- CREATE INDEX IF NOT EXISTS idx_healing_comp_time
-- ON healing_event(component, timestamp DESC);

-- Health metrics: Time series queries
-- CREATE INDEX IF NOT EXISTS idx_health_time_component
-- ON health_metric(timestamp DESC, component);

-- Shared context: Session lookups
-- CREATE INDEX IF NOT EXISTS idx_context_session_type
-- ON shared_execution_context(session_id, context_type);

-- ============================================================================
-- Partial Indexes for Filtered Queries
-- ============================================================================

-- Active tasks only (most common query)
CREATE INDEX IF NOT EXISTS idx_task_active
ON task(priority DESC, deadline ASC)
WHERE status IN ('pending', 'in_progress', 'blocked');

-- Active agents only
CREATE INDEX IF NOT EXISTS idx_agent_active
ON agent(name, type)
WHERE status = 'active';

-- Unverified claims (need attention)
-- CREATE INDEX IF NOT EXISTS idx_claims_unverified
-- ON claims(timestamp DESC)
-- WHERE verified = 0;

-- Failed tasks (for retry/investigation)
CREATE INDEX IF NOT EXISTS idx_task_failed
ON task(updated_at DESC)
WHERE status = 'failed' AND retry_count < max_retries;

-- Critical health alerts
-- CREATE INDEX IF NOT EXISTS idx_health_critical
-- ON health_metric(timestamp DESC, component)
-- WHERE status = 'critical';

-- ============================================================================
-- Full-Text Search Indexes (FTS5)
-- ============================================================================

-- Memory content full-text search
CREATE VIRTUAL TABLE IF NOT EXISTS memory_fts USING fts5(
    id UNINDEXED,
    content,
    tags,
    content='memory',
    content_rowid='rowid'
);

-- Create triggers to keep FTS in sync
CREATE TRIGGER IF NOT EXISTS memory_ai AFTER INSERT ON memory BEGIN
    INSERT INTO memory_fts(rowid, id, content, tags)
    VALUES (NEW.rowid, NEW.id, NEW.content, NEW.tags);
END;

CREATE TRIGGER IF NOT EXISTS memory_ad AFTER DELETE ON memory BEGIN
    INSERT INTO memory_fts(memory_fts, rowid, id, content, tags)
    VALUES('delete', OLD.rowid, OLD.id, OLD.content, OLD.tags);
END;

CREATE TRIGGER IF NOT EXISTS memory_au AFTER UPDATE ON memory BEGIN
    INSERT INTO memory_fts(memory_fts, rowid, id, content, tags)
    VALUES('delete', OLD.rowid, OLD.id, OLD.content, OLD.tags);
    INSERT INTO memory_fts(rowid, id, content, tags)
    VALUES (NEW.rowid, NEW.id, NEW.content, NEW.tags);
END;

-- Knowledge node full-text search
CREATE VIRTUAL TABLE IF NOT EXISTS knowledge_node_fts USING fts5(
    id UNINDEXED,
    name,
    qualified_name,
    description,
    content='knowledge_node',
    content_rowid='rowid'
);

CREATE TRIGGER IF NOT EXISTS knode_ai AFTER INSERT ON knowledge_node BEGIN
    INSERT INTO knowledge_node_fts(rowid, id, name, qualified_name, description)
    VALUES (NEW.rowid, NEW.id, NEW.name, NEW.qualified_name, NEW.description);
END;

CREATE TRIGGER IF NOT EXISTS knode_ad AFTER DELETE ON knowledge_node BEGIN
    INSERT INTO knowledge_node_fts(knowledge_node_fts, rowid, id, name, qualified_name, description)
    VALUES('delete', OLD.rowid, OLD.id, OLD.name, OLD.qualified_name, OLD.description);
END;

CREATE TRIGGER IF NOT EXISTS knode_au AFTER UPDATE ON knowledge_node BEGIN
    INSERT INTO knowledge_node_fts(knowledge_node_fts, rowid, id, name, qualified_name, description)
    VALUES('delete', OLD.rowid, OLD.id, OLD.name, OLD.qualified_name, OLD.description);
    INSERT INTO knowledge_node_fts(rowid, id, name, qualified_name, description)
    VALUES (NEW.rowid, NEW.id, NEW.name, NEW.qualified_name, NEW.description);
END;

-- Record this migration
INSERT OR IGNORE INTO schema_migrations (version, description)
VALUES ('002_indexes', 'Performance indexes, partial indexes, and FTS5 tables');




