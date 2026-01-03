-- NOA Core PostgreSQL Schema
-- Initial migration

-- Memory table for storing all memories
CREATE TABLE IF NOT EXISTS memory (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    memory_type VARCHAR(50) NOT NULL,
    content TEXT NOT NULL,
    metadata JSONB,
    source_agent VARCHAR(255),
    parent_id UUID REFERENCES memory(id),
    tags TEXT[] DEFAULT '{}',
    embedding_id UUID,
    checksum VARCHAR(64)
);

CREATE INDEX IF NOT EXISTS idx_memory_type ON memory(memory_type);
CREATE INDEX IF NOT EXISTS idx_memory_created_at ON memory(created_at);
CREATE INDEX IF NOT EXISTS idx_memory_source_agent ON memory(source_agent);
CREATE INDEX IF NOT EXISTS idx_memory_parent_id ON memory(parent_id);
CREATE INDEX IF NOT EXISTS idx_memory_tags ON memory USING GIN(tags);

-- Model table for ML models
CREATE TABLE IF NOT EXISTS model (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    provider VARCHAR(100) NOT NULL,
    kind VARCHAR(100) NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'
);

CREATE INDEX IF NOT EXISTS idx_model_name ON model(name);
CREATE INDEX IF NOT EXISTS idx_model_provider ON model(provider);

-- Task table
CREATE TABLE IF NOT EXISTS task (
    id UUID PRIMARY KEY,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    priority INTEGER NOT NULL DEFAULT 0,
    metadata JSONB NOT NULL DEFAULT '{}'
);

CREATE INDEX IF NOT EXISTS idx_task_status ON task(status);
CREATE INDEX IF NOT EXISTS idx_task_priority ON task(priority);

-- Agent table
CREATE TABLE IF NOT EXISTS agent (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    metadata JSONB NOT NULL DEFAULT '{}'
);

CREATE INDEX IF NOT EXISTS idx_agent_name ON agent(name);

-- Agent logs table
CREATE TABLE IF NOT EXISTS agent_log (
    id UUID PRIMARY KEY,
    agent_id UUID NOT NULL REFERENCES agent(id),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    level VARCHAR(20) NOT NULL,
    message TEXT NOT NULL,
    metadata JSONB
);

CREATE INDEX IF NOT EXISTS idx_agent_log_agent_id ON agent_log(agent_id);
CREATE INDEX IF NOT EXISTS idx_agent_log_timestamp ON agent_log(timestamp);
CREATE INDEX IF NOT EXISTS idx_agent_log_level ON agent_log(level);

-- Schema migrations tracking
CREATE TABLE IF NOT EXISTS schema_migrations (
    version VARCHAR(255) PRIMARY KEY,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Vector embeddings table (for pgvector extension if available)
-- Note: Requires pgvector extension to be installed
-- CREATE EXTENSION IF NOT EXISTS vector;
-- CREATE TABLE IF NOT EXISTS embeddings (
--     id UUID PRIMARY KEY,
--     memory_id UUID REFERENCES memory(id),
--     embedding vector(1536),
--     model VARCHAR(100) NOT NULL,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
-- );
