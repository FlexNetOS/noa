PRAGMA foreign_keys = ON;

-- ============================================================================
-- Core Tables
-- ============================================================================

CREATE TABLE IF NOT EXISTS models (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  model_type TEXT NOT NULL,
  provider TEXT NOT NULL,
  status TEXT NOT NULL,
  path TEXT,
  metadata_json TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS devices (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  device_type TEXT NOT NULL,
  platform TEXT NOT NULL,
  peer_id TEXT NOT NULL,
  status TEXT NOT NULL,
  is_local INTEGER NOT NULL DEFAULT 0,
  last_seen TEXT,
  capabilities_json TEXT,
  resources_json TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_devices_peer_id ON devices(peer_id);

CREATE TABLE IF NOT EXISTS logs (
  id TEXT PRIMARY KEY,
  level TEXT NOT NULL,
  target TEXT,
  message TEXT NOT NULL,
  fields_json TEXT,
  created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ============================================================================
-- Agent & Memory
-- ============================================================================

CREATE TABLE IF NOT EXISTS agent (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    name TEXT NOT NULL,
    type TEXT,
    status TEXT,
    metadata TEXT
);

CREATE TABLE IF NOT EXISTS agent_log (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    agent_id TEXT REFERENCES agent(id) ON DELETE SET NULL,
    level TEXT NOT NULL,
    message TEXT NOT NULL,
    fields TEXT,
    timestamp TEXT DEFAULT (datetime('now')),
    status TEXT
);

CREATE TABLE IF NOT EXISTS digest_source (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    uri TEXT NOT NULL,
    source_type TEXT NOT NULL,
    metadata TEXT,
    checksum TEXT,
    status TEXT,
    type TEXT
);

CREATE TABLE IF NOT EXISTS knowledge_node (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    node_type TEXT NOT NULL,
    name TEXT NOT NULL,
    qualified_name TEXT,
    description TEXT,
    location TEXT,
    properties TEXT,
    source_digest TEXT REFERENCES digest_source(id) ON DELETE SET NULL,
    embedding_id TEXT
);

CREATE TABLE IF NOT EXISTS knowledge_edge (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    edge_type TEXT NOT NULL,
    source_node TEXT NOT NULL REFERENCES knowledge_node(id) ON DELETE CASCADE,
    target_node TEXT NOT NULL REFERENCES knowledge_node(id) ON DELETE CASCADE,
    properties TEXT,
    relationship TEXT
);

CREATE TABLE IF NOT EXISTS memory (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    type TEXT NOT NULL,
    content TEXT NOT NULL,
    metadata TEXT,
    source_agent TEXT REFERENCES agent(id) ON DELETE SET NULL,
    parent_id TEXT REFERENCES memory(id) ON DELETE SET NULL,
    tags TEXT,
    checksum TEXT,
    embedding_id TEXT
);

CREATE TABLE IF NOT EXISTS embedding (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    vector BLOB NOT NULL,
    model_id TEXT REFERENCES models(id) ON DELETE SET NULL,
    source_type TEXT NOT NULL CHECK (source_type IN ('memory', 'knowledge_node', 'digest_source')),
    source_id TEXT NOT NULL,
    model TEXT
);

-- ============================================================================
-- Tasks
-- ============================================================================

CREATE TABLE IF NOT EXISTS task (
  id TEXT PRIMARY KEY,
  description TEXT,
  title TEXT,
  priority TEXT NOT NULL DEFAULT 'medium',
  status TEXT NOT NULL DEFAULT 'pending',
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT NOT NULL DEFAULT (datetime('now')),
  assigned_agent TEXT,
  deadline TEXT,
  retry_count INTEGER DEFAULT 0,
  max_retries INTEGER DEFAULT 3,
  metadata TEXT
);

CREATE TABLE IF NOT EXISTS task_event (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    task_id TEXT NOT NULL REFERENCES task(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    payload TEXT
);

CREATE TABLE IF NOT EXISTS sync_state (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    peer_id TEXT,
    last_sync_at TEXT,
    state TEXT
);

CREATE TABLE IF NOT EXISTS claims (
    id TEXT PRIMARY KEY,
    content TEXT,
    verified INTEGER DEFAULT 0,
    timestamp TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS shared_execution_context (
    id TEXT PRIMARY KEY,
    session_id TEXT,
    context_type TEXT,
    data TEXT
);

-- Triggers
CREATE TRIGGER IF NOT EXISTS trg_task_updated_at
AFTER UPDATE ON task
FOR EACH ROW
BEGIN
  UPDATE task SET updated_at = datetime('now') WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS trg_devices_updated_at
AFTER UPDATE ON devices
FOR EACH ROW
BEGIN
  UPDATE devices SET updated_at = datetime('now') WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS trg_models_updated_at
AFTER UPDATE ON models
FOR EACH ROW
BEGIN
  UPDATE models SET updated_at = datetime('now') WHERE id = NEW.id;
END;
