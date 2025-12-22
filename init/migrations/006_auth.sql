-- NOA Auth (Provider-agnostic)
-- Migration: 006_auth
-- Purpose: Core authentication tables for OAuth + credentials, stored in NOA core DB.

-- NOTE: SQLite foreign key enforcement is connection-local and must be enabled
-- by the application (PRAGMA foreign_keys = ON) when opening the database.
-- Setting it inside a migration may be ineffective when migrations run inside
-- a transaction.

-- ============================================================================
-- Users
-- ============================================================================

CREATE TABLE IF NOT EXISTS auth_users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE,
    email_verified_at TEXT,
    name TEXT,
    image TEXT,

    -- Credentials auth (optional)
    password_hash TEXT,

    -- Authorization
    role TEXT NOT NULL DEFAULT 'user' CHECK (role IN ('user', 'admin')),

    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    disabled_at TEXT
);

CREATE TRIGGER IF NOT EXISTS trg_auth_users_updated_at
AFTER UPDATE ON auth_users
FOR EACH ROW
BEGIN
    UPDATE auth_users SET updated_at = datetime('now') WHERE id = NEW.id;
END;

CREATE INDEX IF NOT EXISTS idx_auth_users_email ON auth_users(email);
CREATE INDEX IF NOT EXISTS idx_auth_users_role ON auth_users(role);

-- ============================================================================
-- Accounts (OAuth provider links)
-- Mirrors NextAuth Account semantics while remaining provider-agnostic.
-- ============================================================================

CREATE TABLE IF NOT EXISTS auth_accounts (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,

    type TEXT NOT NULL, -- oauth | oidc | email | credentials
    provider TEXT NOT NULL,
    provider_account_id TEXT NOT NULL,

    refresh_token TEXT,
    access_token TEXT,
    expires_at INTEGER,
    token_type TEXT,
    scope TEXT,
    id_token TEXT,
    session_state TEXT,

    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),

    UNIQUE(provider, provider_account_id),
    FOREIGN KEY(user_id) REFERENCES auth_users(id) ON DELETE CASCADE
);

CREATE TRIGGER IF NOT EXISTS trg_auth_accounts_updated_at
AFTER UPDATE ON auth_accounts
FOR EACH ROW
BEGIN
    UPDATE auth_accounts SET updated_at = datetime('now') WHERE id = NEW.id;
END;

CREATE INDEX IF NOT EXISTS idx_auth_accounts_user_id ON auth_accounts(user_id);
CREATE INDEX IF NOT EXISTS idx_auth_accounts_provider ON auth_accounts(provider);

-- ============================================================================
-- Sessions
-- ============================================================================

CREATE TABLE IF NOT EXISTS auth_sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,

    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    last_seen_at TEXT,

    expires_at TEXT NOT NULL,
    revoked_at TEXT,

    user_agent TEXT,
    ip_address TEXT,

    FOREIGN KEY(user_id) REFERENCES auth_users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_auth_sessions_user_id ON auth_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_auth_sessions_expires_at ON auth_sessions(expires_at);

-- ============================================================================
-- Verification tokens (email magic link / reset flows)
-- ============================================================================

CREATE TABLE IF NOT EXISTS auth_verification_tokens (
    identifier TEXT NOT NULL,
    token TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),

    PRIMARY KEY (identifier, token)
);

CREATE INDEX IF NOT EXISTS idx_auth_verification_token_token ON auth_verification_tokens(token);
CREATE INDEX IF NOT EXISTS idx_auth_verification_token_expires ON auth_verification_tokens(expires_at);

-- ============================================================================
-- OAuth transient state (CSRF/PKCE)
-- ============================================================================

CREATE TABLE IF NOT EXISTS auth_oauth_states (
    state TEXT PRIMARY KEY,
    provider TEXT NOT NULL,

    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at TEXT NOT NULL,
    used_at TEXT,

    redirect_to TEXT,
    scope TEXT,
    pkce_verifier TEXT,
    pkce_method TEXT
);

CREATE INDEX IF NOT EXISTS idx_auth_oauth_states_provider ON auth_oauth_states(provider);
CREATE INDEX IF NOT EXISTS idx_auth_oauth_states_expires ON auth_oauth_states(expires_at);
