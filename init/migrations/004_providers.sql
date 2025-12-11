-- Providers and Shared Execution Memory (Phase 2.6, FR-037 to FR-042)
-- Note: WAL mode is set during database initialization

-- Provider registry
CREATE TABLE IF NOT EXISTS providers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    kind TEXT NOT NULL, -- local | cloud | ide | cli
    priority INTEGER DEFAULT 0,
    status TEXT DEFAULT 'unknown',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Shared execution contexts for providers
CREATE TABLE IF NOT EXISTS shared_execution_contexts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider_id INTEGER NOT NULL,
    context_key TEXT NOT NULL,
    context_value TEXT NOT NULL,
    metadata TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(provider_id, context_key),
    FOREIGN KEY(provider_id) REFERENCES providers(id)
);

-- Provider task queue
CREATE TABLE IF NOT EXISTS provider_tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider_id INTEGER NOT NULL,
    task_type TEXT NOT NULL,
    status TEXT DEFAULT 'pending',
    payload TEXT,
    result TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    started_at DATETIME,
    completed_at DATETIME,
    FOREIGN KEY(provider_id) REFERENCES providers(id)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_providers_name ON providers(name);
CREATE INDEX IF NOT EXISTS idx_providers_status ON providers(status);
CREATE INDEX IF NOT EXISTS idx_provider_tasks_status ON provider_tasks(status);
CREATE INDEX IF NOT EXISTS idx_provider_tasks_provider ON provider_tasks(provider_id);

-- Seed known providers with priorities (llama first)
INSERT OR IGNORE INTO providers (name, kind, priority, status) VALUES
('llama.cpp', 'local', 1, 'ready'),
('cursor', 'ide', 2, 'ready'),
('claude', 'cloud', 3, 'ready'),
('codex', 'cloud', 4, 'ready'),
('copilot', 'ide', 5, 'ready'),
('git', 'local', 6, 'ready'),
('abacus', 'cloud', 7, 'ready');
