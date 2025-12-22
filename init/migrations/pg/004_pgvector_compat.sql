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

