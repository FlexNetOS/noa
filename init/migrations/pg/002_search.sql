-- NOA PostgreSQL Search (FTS)
-- Migration: pg/002_search
-- Date: 2025-12-22

-- Memory full-text search index
CREATE INDEX IF NOT EXISTS idx_memory_fts
ON memory
USING GIN (to_tsvector('english', coalesce(content, '')));

-- Knowledge node full-text search index
CREATE INDEX IF NOT EXISTS idx_knowledge_node_fts
ON knowledge_node
USING GIN (
    to_tsvector('english',
        coalesce(name, '') || ' ' || coalesce(qualified_name, '') || ' ' || coalesce(description, '')
    )
);

-- Trigram indexes for fast fuzzy matching (requires pg_trgm; created in 001_core)
CREATE INDEX IF NOT EXISTS idx_memory_content_trgm
ON memory
USING GIN (content gin_trgm_ops);

CREATE INDEX IF NOT EXISTS idx_knowledge_node_name_trgm
ON knowledge_node
USING GIN (name gin_trgm_ops);

-- Search helpers

CREATE OR REPLACE FUNCTION search_memory(query TEXT, limit_count INTEGER DEFAULT 20)
RETURNS TABLE (
    id UUID,
    created_at TIMESTAMPTZ,
    memory_type TEXT,
    content TEXT,
    rank REAL
)
LANGUAGE sql
STABLE
AS $$
    SELECT
        m.id,
        m.created_at,
        m.memory_type,
        m.content,
        ts_rank(to_tsvector('english', coalesce(m.content, '')), plainto_tsquery('english', query)) AS rank
    FROM memory m
    WHERE to_tsvector('english', coalesce(m.content, '')) @@ plainto_tsquery('english', query)
    ORDER BY rank DESC, m.created_at DESC
    LIMIT limit_count;
$$;

CREATE OR REPLACE FUNCTION search_knowledge(query TEXT, limit_count INTEGER DEFAULT 20)
RETURNS TABLE (
    id UUID,
    node_type TEXT,
    name TEXT,
    qualified_name TEXT,
    rank REAL
)
LANGUAGE sql
STABLE
AS $$
    SELECT
        n.id,
        n.node_type,
        n.name,
        n.qualified_name,
        ts_rank(
            to_tsvector('english', coalesce(n.name, '') || ' ' || coalesce(n.qualified_name, '') || ' ' || coalesce(n.description, '')),
            plainto_tsquery('english', query)
        ) AS rank
    FROM knowledge_node n
    WHERE to_tsvector('english', coalesce(n.name, '') || ' ' || coalesce(n.qualified_name, '') || ' ' || coalesce(n.description, ''))
        @@ plainto_tsquery('english', query)
    ORDER BY rank DESC
    LIMIT limit_count;
$$;
