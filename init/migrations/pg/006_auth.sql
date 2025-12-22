-- NOA PostgreSQL Minimal Auth Schema
-- Migration: pg/006_auth
-- Date: 2025-12-22
-- Purpose: Minimal auth tables for server deployments.

CREATE TABLE IF NOT EXISTS auth_users (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    email TEXT UNIQUE,
    display_name TEXT,
    metadata JSONB
);

CREATE TABLE IF NOT EXISTS auth_accounts (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id UUID NOT NULL REFERENCES auth_users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    provider_account_id TEXT NOT NULL,
    metadata JSONB,
    UNIQUE(provider, provider_account_id)
);

CREATE TABLE IF NOT EXISTS auth_sessions (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id UUID NOT NULL REFERENCES auth_users(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    token_hash TEXT NOT NULL UNIQUE,
    metadata JSONB
);

CREATE INDEX IF NOT EXISTS idx_auth_sessions_user_id ON auth_sessions (user_id);
CREATE INDEX IF NOT EXISTS idx_auth_sessions_expires_at ON auth_sessions (expires_at);
