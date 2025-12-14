-- NOA Shared Execution Memory Schema
-- Enables context sharing across AI providers (§4.10)

CREATE TABLE IF NOT EXISTS execution_context (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    provider TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    context_type TEXT NOT NULL,
    content TEXT NOT NULL,
    metadata TEXT,
    UNIQUE(session_id, provider, context_type)
);

CREATE TABLE IF NOT EXISTS reasoning_state (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    provider TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    state_key TEXT NOT NULL,
    state_value TEXT NOT NULL,
    UNIQUE(session_id, state_key)
);

CREATE TABLE IF NOT EXISTS task_distribution (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id TEXT NOT NULL UNIQUE,
    assigned_provider TEXT,
    status TEXT DEFAULT 'pending',
    priority INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    started_at DATETIME,
    completed_at DATETIME,
    result TEXT
);

CREATE TABLE IF NOT EXISTS provider_state (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider TEXT NOT NULL UNIQUE,
    status TEXT DEFAULT 'unknown',
    last_heartbeat DATETIME,
    capabilities TEXT,
    current_load REAL DEFAULT 0.0
);

CREATE TABLE IF NOT EXISTS provider_rate_limits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider TEXT NOT NULL UNIQUE,
    max_rps INTEGER DEFAULT 0,
    burst INTEGER DEFAULT 0,
    backoff_ms INTEGER DEFAULT 1000,
    last_updated DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_context_session ON execution_context(session_id);
CREATE INDEX IF NOT EXISTS idx_reasoning_session ON reasoning_state(session_id);
CREATE INDEX IF NOT EXISTS idx_task_status ON task_distribution(status);
