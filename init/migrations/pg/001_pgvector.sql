-- NOA PostgreSQL Vector Search Setup
-- Migration: pg/001_pgvector
-- Date: 2025-12-09
-- Purpose: Configure pgvector for PostgreSQL scale-up deployment
--
-- Prerequisites:
--   CREATE EXTENSION vector;
--   (requires PostgreSQL 11+ with pgvector installed)

-- Enable pgvector extension
CREATE EXTENSION IF NOT EXISTS vector;

-- ============================================================================
-- Embeddings Table with Vector Column
-- ============================================================================

-- Add vector column to embeddings table if using PostgreSQL
-- Note: This assumes the base table exists from 001_initial.sql adapted for PG

-- Create embeddings table with native vector type
CREATE TABLE IF NOT EXISTS embedding (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    vector vector(384) NOT NULL,  -- pgvector native type
    model TEXT NOT NULL,
    source_type TEXT NOT NULL CHECK (source_type IN ('memory', 'node', 'document')),
    source_id UUID NOT NULL
);

-- Create HNSW index for fast approximate nearest neighbor search
CREATE INDEX IF NOT EXISTS idx_embedding_hnsw 
ON embedding 
USING hnsw (vector vector_cosine_ops)
WITH (m = 16, ef_construction = 200);

-- Alternative: IVFFlat index (faster build, slightly less accurate)
-- CREATE INDEX IF NOT EXISTS idx_embedding_ivfflat
-- ON embedding
-- USING ivfflat (vector vector_cosine_ops)
-- WITH (lists = 100);

-- ============================================================================
-- Memory Table with Embedding Reference
-- ============================================================================

-- Memory table for PostgreSQL
CREATE TABLE IF NOT EXISTS memory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    type TEXT NOT NULL CHECK (type IN ('interaction', 'decision', 'learning', 'artifact')),
    content TEXT NOT NULL,
    metadata JSONB,
    source_agent UUID,
    parent_id UUID REFERENCES memory(id),
    tags TEXT[],
    embedding_id UUID REFERENCES embedding(id),
    checksum TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_memory_created ON memory(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_memory_type ON memory(type);
CREATE INDEX IF NOT EXISTS idx_memory_tags ON memory USING GIN(tags);

-- ============================================================================
-- Knowledge Node Table with Embedding Reference
-- ============================================================================

CREATE TABLE IF NOT EXISTS knowledge_node (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type TEXT NOT NULL CHECK (type IN ('function', 'class', 'module', 'file', 'repo', 'concept')),
    name TEXT NOT NULL,
    qualified_name TEXT,
    description TEXT,
    source_digest UUID,
    location JSONB,
    properties JSONB,
    embedding_id UUID REFERENCES embedding(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_node_type ON knowledge_node(type);
CREATE INDEX IF NOT EXISTS idx_node_qualified ON knowledge_node(qualified_name);

-- ============================================================================
-- Helper Functions for Vector Search
-- ============================================================================

-- Function to find similar memories by vector
CREATE OR REPLACE FUNCTION find_similar_memories(
    query_vector vector(384),
    limit_count INTEGER DEFAULT 10,
    similarity_threshold FLOAT DEFAULT 0.7
)
RETURNS TABLE (
    id UUID,
    content TEXT,
    type TEXT,
    similarity FLOAT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        m.id,
        m.content,
        m.type,
        1 - (e.vector <=> query_vector) AS similarity
    FROM memory m
    JOIN embedding e ON m.embedding_id = e.id
    WHERE 1 - (e.vector <=> query_vector) >= similarity_threshold
    ORDER BY e.vector <=> query_vector
    LIMIT limit_count;
END;
$$ LANGUAGE plpgsql;

-- Function to find similar knowledge nodes
CREATE OR REPLACE FUNCTION find_similar_knowledge(
    query_vector vector(384),
    limit_count INTEGER DEFAULT 10,
    similarity_threshold FLOAT DEFAULT 0.7
)
RETURNS TABLE (
    id UUID,
    name TEXT,
    qualified_name TEXT,
    type TEXT,
    similarity FLOAT
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        n.id,
        n.name,
        n.qualified_name,
        n.type,
        1 - (e.vector <=> query_vector) AS similarity
    FROM knowledge_node n
    JOIN embedding e ON n.embedding_id = e.id
    WHERE 1 - (e.vector <=> query_vector) >= similarity_threshold
    ORDER BY e.vector <=> query_vector
    LIMIT limit_count;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- Migration Tracking
-- ============================================================================

CREATE TABLE IF NOT EXISTS schema_migrations (
    version TEXT PRIMARY KEY,
    applied_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    description TEXT
);

INSERT INTO schema_migrations (version, description)
VALUES ('pg/001_pgvector', 'PostgreSQL pgvector extension with HNSW indexes')
ON CONFLICT (version) DO NOTHING;




