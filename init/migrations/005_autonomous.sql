-- NOA Autonomous Operation Tables
-- Migration: 005_autonomous
-- §3.4: Autonomous operations and 3-plane architecture
-- FR-009-015: Autonomous agent operations

-- ============================================================================
-- Goal Table
-- Unified goal queue for user requests, self-generated goals, and constitutional goals
-- ============================================================================
CREATE TABLE IF NOT EXISTS goal (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),

    -- Goal classification
    type TEXT NOT NULL CHECK (type IN ('user', 'self_generated', 'constitutional', 'maintenance')),
    priority INTEGER NOT NULL DEFAULT 50 CHECK (priority >= 0 AND priority <= 100),

    -- Goal content
    title TEXT NOT NULL,
    description TEXT,
    acceptance_criteria TEXT,  -- JSON array of criteria

    -- State tracking
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN (
        'pending', 'queued', 'in_progress', 'blocked',
        'completed', 'failed', 'cancelled', 'deferred'
    )),

    -- Assignment
    assigned_agent TEXT,
    assigned_plane TEXT CHECK (assigned_plane IN ('sandbox', 'deployed', 'coordinator')),

    -- Relationships
    parent_goal_id TEXT REFERENCES goal(id),
    dependencies TEXT,  -- JSON array of goal IDs

    -- Timing
    deadline TEXT,
    started_at TEXT,
    completed_at TEXT,
    estimated_duration_minutes INTEGER,

    -- Results
    outcome TEXT,
    artifacts TEXT,  -- JSON array of artifact references

    -- Metadata
    metadata TEXT,  -- JSON for extension
    tags TEXT,  -- JSON array

    -- Audit
    created_by TEXT NOT NULL DEFAULT 'system',
    checksum TEXT NOT NULL
);

-- ============================================================================
-- Plane Table
-- 3-Plane architecture: sandbox, deployed, coordinator
-- ============================================================================
CREATE TABLE IF NOT EXISTS plane (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),

    -- Plane identification
    name TEXT NOT NULL UNIQUE CHECK (name IN ('sandbox', 'deployed', 'coordinator')),
    role TEXT NOT NULL CHECK (role IN ('testing', 'production', 'orchestration')),

    -- State
    status TEXT NOT NULL DEFAULT 'inactive' CHECK (status IN (
        'inactive', 'initializing', 'active', 'degraded',
        'maintenance', 'failed', 'shutting_down'
    )),

    -- Version tracking
    version TEXT NOT NULL,
    previous_version TEXT,

    -- Resources
    resource_allocation TEXT,  -- JSON: CPU, memory, storage limits
    current_usage TEXT,        -- JSON: current resource usage

    -- Health
    health_status TEXT NOT NULL DEFAULT 'unknown' CHECK (health_status IN (
        'unknown', 'healthy', 'degraded', 'unhealthy', 'critical'
    )),
    last_health_check TEXT,
    health_details TEXT,  -- JSON with health metrics

    -- Configuration
    config_path TEXT,
    config_hash TEXT,

    -- Metadata
    metadata TEXT,
    checksum TEXT NOT NULL
);

-- ============================================================================
-- Plane Transition Table
-- Audit trail for promotions and rollbacks between planes
-- ============================================================================
CREATE TABLE IF NOT EXISTS plane_transition (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),

    -- Transition details
    type TEXT NOT NULL CHECK (type IN ('promotion', 'rollback', 'migration', 'failover')),
    source_plane TEXT NOT NULL REFERENCES plane(id),
    target_plane TEXT NOT NULL REFERENCES plane(id),

    -- Version tracking
    source_version TEXT NOT NULL,
    target_version TEXT NOT NULL,

    -- State
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN (
        'pending', 'preparing', 'in_progress', 'validating',
        'completed', 'failed', 'rolled_back'
    )),

    -- Timing
    started_at TEXT,
    completed_at TEXT,
    duration_seconds INTEGER,

    -- Validation
    pre_checks TEXT,   -- JSON array of check results
    post_checks TEXT,  -- JSON array of check results
    validation_status TEXT CHECK (validation_status IN ('passed', 'failed', 'skipped')),

    -- Artifacts
    artifacts_transferred TEXT,  -- JSON array of transferred artifacts

    -- Results
    outcome TEXT,
    error_message TEXT,
    rollback_reason TEXT,

    -- Audit
    initiated_by TEXT NOT NULL,
    approved_by TEXT,
    metadata TEXT,
    checksum TEXT NOT NULL
);

-- ============================================================================
-- Healing Event Table
-- 5-stage self-healing audit trail
-- Stages: Detect → Diagnose → Plan → Execute → Verify
-- ============================================================================
CREATE TABLE IF NOT EXISTS healing_event (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),

    -- Event identification
    incident_id TEXT NOT NULL,  -- Groups related healing events
    sequence INTEGER NOT NULL,   -- Order within incident

    -- Current stage
    stage TEXT NOT NULL CHECK (stage IN ('detect', 'diagnose', 'plan', 'execute', 'verify')),

    -- State
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN (
        'pending', 'in_progress', 'completed', 'failed', 'skipped'
    )),

    -- Detection stage
    detection_type TEXT,  -- What triggered: health_check, alert, user_report, agent_observation
    detection_source TEXT,
    anomaly_score REAL,

    -- Diagnosis stage
    root_cause TEXT,
    affected_components TEXT,  -- JSON array
    impact_assessment TEXT,

    -- Plan stage
    proposed_actions TEXT,  -- JSON array of planned actions
    risk_assessment TEXT,
    estimated_recovery_time_minutes INTEGER,
    requires_approval INTEGER DEFAULT 0,
    approved_by TEXT,

    -- Execute stage
    executed_actions TEXT,  -- JSON array of executed actions
    execution_log TEXT,

    -- Verify stage
    verification_checks TEXT,  -- JSON array of check results
    recovery_confirmed INTEGER DEFAULT 0,

    -- Timing
    started_at TEXT,
    completed_at TEXT,
    duration_seconds INTEGER,

    -- Results
    outcome TEXT CHECK (outcome IN ('success', 'partial', 'failure', 'escalated')),
    follow_up_required INTEGER DEFAULT 0,
    follow_up_actions TEXT,

    -- Metadata
    metadata TEXT,
    checksum TEXT NOT NULL
);

-- ============================================================================
-- Health Metric Table
-- Continuous health monitoring metrics
-- ============================================================================
CREATE TABLE IF NOT EXISTS health_metric (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),

    -- Source
    component TEXT NOT NULL,
    plane TEXT REFERENCES plane(id),

    -- Metric identification
    metric_name TEXT NOT NULL,
    metric_type TEXT NOT NULL CHECK (metric_type IN (
        'gauge', 'counter', 'histogram', 'summary'
    )),

    -- Values
    value REAL NOT NULL,
    unit TEXT,

    -- Thresholds
    warning_threshold REAL,
    critical_threshold REAL,

    -- Status
    status TEXT NOT NULL DEFAULT 'ok' CHECK (status IN (
        'ok', 'warning', 'critical', 'unknown'
    )),

    -- Labels
    labels TEXT,  -- JSON key-value pairs for filtering

    -- Metadata
    metadata TEXT
);

-- ============================================================================
-- Indexes for Autonomous Operations
-- ============================================================================

-- Goal indexes
CREATE INDEX IF NOT EXISTS idx_goal_type ON goal(type);
CREATE INDEX IF NOT EXISTS idx_goal_status ON goal(status);
CREATE INDEX IF NOT EXISTS idx_goal_priority ON goal(priority DESC);
CREATE INDEX IF NOT EXISTS idx_goal_assigned_agent ON goal(assigned_agent);
CREATE INDEX IF NOT EXISTS idx_goal_assigned_plane ON goal(assigned_plane);
CREATE INDEX IF NOT EXISTS idx_goal_parent ON goal(parent_goal_id);
CREATE INDEX IF NOT EXISTS idx_goal_created ON goal(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_goal_deadline ON goal(deadline);

-- Plane indexes
CREATE INDEX IF NOT EXISTS idx_plane_status ON plane(status);
CREATE INDEX IF NOT EXISTS idx_plane_health ON plane(health_status);

-- Plane transition indexes
CREATE INDEX IF NOT EXISTS idx_transition_type ON plane_transition(type);
CREATE INDEX IF NOT EXISTS idx_transition_status ON plane_transition(status);
CREATE INDEX IF NOT EXISTS idx_transition_source ON plane_transition(source_plane);
CREATE INDEX IF NOT EXISTS idx_transition_target ON plane_transition(target_plane);
CREATE INDEX IF NOT EXISTS idx_transition_created ON plane_transition(created_at DESC);

-- Healing event indexes
CREATE INDEX IF NOT EXISTS idx_healing_incident ON healing_event(incident_id);
CREATE INDEX IF NOT EXISTS idx_healing_stage ON healing_event(stage);
CREATE INDEX IF NOT EXISTS idx_healing_status ON healing_event(status);
CREATE INDEX IF NOT EXISTS idx_healing_created ON healing_event(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_healing_sequence ON healing_event(incident_id, sequence);

-- Health metric indexes
CREATE INDEX IF NOT EXISTS idx_health_component ON health_metric(component);
CREATE INDEX IF NOT EXISTS idx_health_plane ON health_metric(plane);
CREATE INDEX IF NOT EXISTS idx_health_metric ON health_metric(metric_name);
CREATE INDEX IF NOT EXISTS idx_health_status ON health_metric(status);
CREATE INDEX IF NOT EXISTS idx_health_created ON health_metric(created_at DESC);

-- Composite indexes for common queries
CREATE INDEX IF NOT EXISTS idx_goal_queue ON goal(status, priority DESC, created_at);
CREATE INDEX IF NOT EXISTS idx_goal_active ON goal(assigned_agent, status) WHERE status = 'in_progress';
CREATE INDEX IF NOT EXISTS idx_healing_active ON healing_event(incident_id, status) WHERE status IN ('pending', 'in_progress');
CREATE INDEX IF NOT EXISTS idx_health_alerts ON health_metric(status, created_at DESC) WHERE status IN ('warning', 'critical');

-- ============================================================================
-- Initial Data: Plane Configuration
-- ============================================================================

-- Insert the three planes
INSERT OR IGNORE INTO plane (id, name, role, status, version, health_status, checksum)
VALUES
    ('plane-sandbox', 'sandbox', 'testing', 'inactive', '0.1.0', 'unknown', 'initial'),
    ('plane-deployed', 'deployed', 'production', 'inactive', '0.1.0', 'unknown', 'initial'),
    ('plane-coordinator', 'coordinator', 'orchestration', 'inactive', '0.1.0', 'unknown', 'initial');

-- ============================================================================
-- Triggers for Updated Timestamps
-- ============================================================================

CREATE TRIGGER IF NOT EXISTS goal_updated
AFTER UPDATE ON goal
BEGIN
    UPDATE goal SET updated_at = datetime('now') WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS plane_updated
AFTER UPDATE ON plane
BEGIN
    UPDATE plane SET updated_at = datetime('now') WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS healing_updated
AFTER UPDATE ON healing_event
BEGIN
    UPDATE healing_event SET updated_at = datetime('now') WHERE id = NEW.id;
END;

