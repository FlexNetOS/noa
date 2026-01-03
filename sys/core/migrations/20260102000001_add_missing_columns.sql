-- Add missing columns to match postgres_stores.rs expectations

-- Add created_at to model table
ALTER TABLE model ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- Add updated_at to task table  
ALTER TABLE task ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
ALTER TABLE task ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- Add fields column to agent_log and rename timestamp to created_at
ALTER TABLE agent_log ADD COLUMN IF NOT EXISTS fields JSONB;
ALTER TABLE agent_log ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- Create embedding table for vector embeddings
CREATE TABLE IF NOT EXISTS embedding (
    id UUID PRIMARY KEY,
    source_type VARCHAR(50) NOT NULL,
    source_id UUID NOT NULL,
    model_id UUID REFERENCES model(id),
    vector BYTEA,
    dimensions INTEGER NOT NULL DEFAULT 0,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_embedding_source ON embedding(source_type, source_id);
CREATE INDEX IF NOT EXISTS idx_embedding_model ON embedding(model_id);
