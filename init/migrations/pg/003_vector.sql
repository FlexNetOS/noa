-- NOA PostgreSQL Vector Search (RuVector)
-- Migration: pg/003_vector
-- Date: 2025-12-22

-- ANN index for embeddings
-- Uses RuVector index access method `ruhnsw` and opclass `ruvector_l2_ops`.
CREATE INDEX IF NOT EXISTS idx_embedding_vector_hnsw
ON embedding
USING ruhnsw (vector ruvector_l2_ops);

-- Similarity search helpers

CREATE OR REPLACE FUNCTION find_similar_memories(
    query_vector ruvector(384),
    limit_count INTEGER DEFAULT 10
)
RETURNS TABLE (
    memory_id UUID,
    distance DOUBLE PRECISION
)
LANGUAGE sql
STABLE
AS $$
    SELECT
        m.id AS memory_id,
        (e.vector <-> query_vector) AS distance
    FROM memory m
    JOIN embedding e ON e.id = m.embedding_id
    WHERE m.embedding_id IS NOT NULL
    ORDER BY e.vector <-> query_vector
    LIMIT limit_count;
$$;

CREATE OR REPLACE FUNCTION find_similar_knowledge(
    query_vector ruvector(384),
    limit_count INTEGER DEFAULT 10
)
RETURNS TABLE (
    node_id UUID,
    distance DOUBLE PRECISION
)
LANGUAGE sql
STABLE
AS $$
    SELECT
        n.id AS node_id,
        (e.vector <-> query_vector) AS distance
    FROM knowledge_node n
    JOIN embedding e ON e.id = n.embedding_id
    WHERE n.embedding_id IS NOT NULL
    ORDER BY e.vector <-> query_vector
    LIMIT limit_count;
$$;
