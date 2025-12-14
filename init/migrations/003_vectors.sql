-- NOA Vector Search Setup
-- Migration: 003_vectors
-- Date: 2025-12-09
-- Purpose: Configure sqlite-vss for vector similarity search
--
-- NOTE: Requires sqlite-vss extension to be loaded:
--   .load ./sqlite-vss
-- 
-- For SQLite < 3.41, use the loadable extension.
-- For production, consider using Qdrant (see config/qdrant.yaml)

-- ============================================================================
-- Vector Search Virtual Tables
-- ============================================================================

-- HNSW Index for embeddings (384-dim for MiniLM-L6-v2)
-- This creates a virtual table that indexes vectors for fast approximate nearest neighbor search
CREATE VIRTUAL TABLE IF NOT EXISTS vss_embeddings USING vss0(
    vector(384)
);

-- Memory embeddings index
CREATE VIRTUAL TABLE IF NOT EXISTS vss_memory USING vss0(
    vector(384)
);

-- Knowledge node embeddings index  
CREATE VIRTUAL TABLE IF NOT EXISTS vss_knowledge USING vss0(
    vector(384)
);

-- ============================================================================
-- Helper Tables for Vector ID Mapping
-- ============================================================================

-- Map VSS rowids to embedding IDs
CREATE TABLE IF NOT EXISTS vss_embedding_map (
    vss_rowid INTEGER PRIMARY KEY,
    embedding_id TEXT NOT NULL UNIQUE REFERENCES embedding(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_vss_embed_id ON vss_embedding_map(embedding_id);

-- Map VSS rowids to memory IDs
CREATE TABLE IF NOT EXISTS vss_memory_map (
    vss_rowid INTEGER PRIMARY KEY,
    memory_id TEXT NOT NULL UNIQUE REFERENCES memory(id),
    embedding_id TEXT NOT NULL REFERENCES embedding(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_vss_memory_id ON vss_memory_map(memory_id);

-- Map VSS rowids to knowledge node IDs
CREATE TABLE IF NOT EXISTS vss_knowledge_map (
    vss_rowid INTEGER PRIMARY KEY,
    node_id TEXT NOT NULL UNIQUE REFERENCES knowledge_node(id),
    embedding_id TEXT NOT NULL REFERENCES embedding(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_vss_node_id ON vss_knowledge_map(node_id);

-- ============================================================================
-- Vector Search Configuration
-- ============================================================================

-- Store vector search configuration
CREATE TABLE IF NOT EXISTS vss_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Default configuration
INSERT OR IGNORE INTO vss_config (key, value) VALUES
    ('model', 'all-MiniLM-L6-v2'),
    ('dimensions', '384'),
    ('distance_metric', 'cosine'),
    ('ef_construction', '200'),
    ('ef_search', '100'),
    ('m', '16');

-- ============================================================================
-- Helper Views for Vector Search
-- ============================================================================

-- View to simplify memory vector search results
CREATE VIEW IF NOT EXISTS v_memory_with_embedding AS
SELECT 
    m.id,
    m.type,
    m.content,
    m.metadata,
    m.created_at,
    m.tags,
    e.vector,
    e.model
FROM memory m
JOIN embedding e ON m.embedding_id = e.id;

-- View to simplify knowledge node vector search results
CREATE VIEW IF NOT EXISTS v_knowledge_with_embedding AS
SELECT 
    n.id,
    n.type,
    n.name,
    n.qualified_name,
    n.description,
    n.properties,
    e.vector,
    e.model
FROM knowledge_node n
JOIN embedding e ON n.embedding_id = e.id;

-- Record this migration
INSERT OR IGNORE INTO schema_migrations (version, description)
VALUES ('003_vectors', 'sqlite-vss vector search setup with HNSW indexes');




