-- NOA PostgreSQL pgvector Compatibility
-- Migration: pg/004_pgvector_compat
-- Date: 2025-12-22
-- Purpose: Optional compatibility migration.
--
-- NOTE:
-- - Server deployments should use RuVector via `CREATE EXTENSION ruvector;`.
-- - This migration intentionally does NOT create tables, indexes, or functions.
-- - We also make extension installation best-effort so environments without
--   pgvector installed do not fail the entire migration chain.

DO $$
BEGIN
    CREATE EXTENSION IF NOT EXISTS vector;
EXCEPTION
    WHEN undefined_file OR undefined_object OR insufficient_privilege THEN
        RAISE NOTICE 'pgvector extension not available or not permitted; skipping';
END $$;
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




