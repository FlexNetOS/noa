-- NOA Vector Search Setup
-- Migration: 003_vectors
-- Date: 2025-12-09
-- Purpose: Configure vector similarity search (placeholder tables)
--
-- NOTE: Full VSS support requires sqlite-vss extension.
-- These placeholder tables allow the API to work without the extension.
-- For production vector search, use Qdrant (see config/qdrant.yaml)

PRAGMA foreign_keys = ON;

-- ============================================================================
-- Vector ID Mapping Tables (work without VSS extension)
-- ============================================================================

-- Map rowids to embedding IDs
CREATE TABLE IF NOT EXISTS vss_embedding_map (
    vss_rowid INTEGER PRIMARY KEY,
    embedding_id TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (embedding_id) REFERENCES embedding(id)
);

CREATE INDEX IF NOT EXISTS idx_vss_embed_id ON vss_embedding_map(embedding_id);

-- Map rowids to memory IDs
CREATE TABLE IF NOT EXISTS vss_memory_map (
    vss_rowid INTEGER PRIMARY KEY,
    memory_id TEXT NOT NULL UNIQUE,
    embedding_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (memory_id) REFERENCES memory(id),
    FOREIGN KEY (embedding_id) REFERENCES embedding(id)
);

CREATE INDEX IF NOT EXISTS idx_vss_mem_memory ON vss_memory_map(memory_id);
CREATE INDEX IF NOT EXISTS idx_vss_mem_embed ON vss_memory_map(embedding_id);

-- Map rowids to knowledge node IDs
CREATE TABLE IF NOT EXISTS vss_knowledge_map (
    vss_rowid INTEGER PRIMARY KEY,
    node_id TEXT NOT NULL UNIQUE,
    embedding_id TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (node_id) REFERENCES knowledge_node(id),
    FOREIGN KEY (embedding_id) REFERENCES embedding(id)
);

CREATE INDEX IF NOT EXISTS idx_vss_know_node ON vss_knowledge_map(node_id);
CREATE INDEX IF NOT EXISTS idx_vss_know_embed ON vss_knowledge_map(embedding_id);

-- ============================================================================
-- Vector Search Configuration
-- ============================================================================

CREATE TABLE IF NOT EXISTS vss_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Default configuration values
INSERT OR IGNORE INTO vss_config (key, value, description) VALUES 
    ('model', 'all-MiniLM-L6-v2', 'Default embedding model'),
    ('dimensions', '384', 'Default vector dimensions'),
    ('distance_metric', 'cosine', 'Distance metric for similarity search'),
    ('index_type', 'flat', 'Vector index type (flat for small datasets)'),
    ('ef_search', '40', 'HNSW search parameter'),
    ('ef_construction', '200', 'HNSW construction parameter');

-- ============================================================================
-- Stored Procedures via Views
-- ============================================================================

-- View for recent embeddings with memory links
CREATE VIEW IF NOT EXISTS v_recent_embeddings AS
SELECT 
    e.id,
    e.model,
    e.dimensions,
    e.vector,
    m.memory_id,
    e.created_at
FROM embedding e
LEFT JOIN vss_memory_map m ON e.id = m.embedding_id
ORDER BY e.created_at DESC
LIMIT 100;
