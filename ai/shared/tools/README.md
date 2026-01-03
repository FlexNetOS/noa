# Tools Directory

This directory contains MCP tool definitions for AI agents.

## Purpose

Tools are functions that AI agents can invoke to interact with external systems.
They follow the Model Context Protocol (MCP) specification for interoperability.

---

## Available Tools

All tools are available to any provider (llama.cpp, Ollama, OpenAI, Anthropic, etc.) via the unified resource system.

### Tool Registry Table

| Tool | Category | Version | Description | Permissions | Rate Limit |
|------|----------|---------|-------------|-------------|------------|
| [code-generation-tool](#code-generation-tool) | Development | 1.0.0 | Generate code from natural language descriptions | `file:read` | 60/min |
| [reasoning-tool](#reasoning-tool) | Analysis | 1.0.0 | Complex reasoning and structured problem-solving | None | 30/min |
| [documentation-generator](#documentation-generator) | Documentation | 1.0.0 | AI-driven docs with multi-pass adaptive execution | `file:read`, `file:write` | 20/min |
| [backupctl](#backupctl) | Operations | 1.0.0 | Workspace backup orchestration and verification | `file:read`, `file:write`, `shell:execute` | 10/min |
| [sandboxctl](#sandboxctl) | Operations | 1.0.0 | Sandbox lifecycle management with backup enforcement | `shell:execute` | 10/min |
| [database-query-tool](#database-query-tool) | Database | 1.0.0 | Execute SQL queries with parameterized queries | `database:read`, `database:write` | 120/min |
| [database-migrate-tool](#database-migrate-tool) | Database | 1.0.0 | Schema migrations with version tracking | `database:write`, `database:admin` | 10/min |
| [database-backup-tool](#database-backup-tool) | Database | 1.0.0 | Create/restore backups with integrity verification | `database:read`, `database:write`, `database:admin` | 5/min |
| [vector-search-tool](#vector-search-tool) | Database | 1.0.0 | Semantic vector search for knowledge retrieval | `database:read` | 60/min |

### Tool Capabilities Matrix

| Tool | Languages | Agent Integration | Fallback Chain | Subagents |
|------|-----------|-------------------|----------------|-----------|
| code-generation-tool | Python, TS, JS, Rust, Go, Java, C# | code-generation-agent | — | — |
| reasoning-tool | N/A | reasoning-agent | — | — |
| documentation-generator | Rust, Markdown | RustDoc, Clippy, Fmt | llama.cpp → copilot → anthropic → openai → git | 4 Rust subagents |
| backupctl | Python, Bash | — | — | — |
| sandboxctl | Python | — | — | — |

---

## Tool Details

### code-generation-tool

**Generate code from natural language descriptions or specifications.**

| Property | Value |
|----------|-------|
| **File** | `code-generation-tool.json` |
| **Category** | Development |
| **Implementation** | Agent-based (`code-generation-agent`) |
| **Handler** | `noa.tools.codegen.generate` |

**Input Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `description` | string | ✅ | Natural language description of code to generate |
| `language` | enum | ✅ | Target: `python`, `typescript`, `javascript`, `rust`, `go`, `java`, `csharp` |
| `context` | string | ❌ | Existing code context or file contents |
| `style` | enum | ❌ | Code style: `minimal`, `documented` (default), `verbose` |
| `include_tests` | boolean | ❌ | Include unit tests (default: false) |
| `framework` | string | ❌ | Specific framework (e.g., `axum`, `express`, `fastapi`) |

**Output:**

```json
{
  "code": "string",
  "explanation": "string",
  "dependencies": ["string"],
  "tests": "string",
  "usage_example": "string"
}
```

---

### reasoning-tool

**Complex reasoning and analysis for structured problem-solving.**

| Property | Value |
|----------|-------|
| **File** | `reasoning-tool.json` |
| **Category** | Analysis |
| **Implementation** | Agent-based (`reasoning-agent`) |
| **Handler** | `noa.tools.reasoning.analyze` |

**Input Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `problem` | string | ✅ | The problem or question to analyze |
| `context` | string | ❌ | Additional context or constraints |
| `reasoning_type` | enum | ❌ | Type: `deductive` (default), `inductive`, `abductive`, `analogical` |
| `depth` | enum | ❌ | Analysis depth: `shallow`, `normal` (default), `deep` |
| `output_format` | enum | ❌ | Format: `structured` (default), `narrative`, `bullet_points` |

**Output:**

```json
{
  "analysis": "string",
  "conclusions": ["string"],
  "confidence": 0.85,
  "reasoning_chain": [
    {"step": 1, "premise": "string", "inference": "string"}
  ],
  "alternatives": ["string"]
}
```

---

### documentation-generator

**AI-driven documentation generator using Litho/wiki-rs with adaptive multi-pass execution.**

| Property | Value |
|----------|-------|
| **File** | `documentation-generator.json` |
| **Category** | Documentation |
| **Implementation** | Binary + Adapter (`opt/wiki-rs/target/release/litho`) |
| **Config** | `config/litho.toml` |

**Multi-Pass Pipeline:**

| Pass | Name | Subagent | Description | Parallelizable |
|------|------|----------|-------------|----------------|
| 1 | Structure | RustCrateScannerAgent | Extract dependency tree, SBOM, crate metadata | ❌ |
| 2 | Analysis | RustClippyAgent | Code quality signals for doc annotations | ✅ |
| 3 | Generation | RustDocAgent | Rustdoc + Litho markdown generation | ✅ |
| 4 | Validation | RustFmtAgent | Cross-reference validation, link checking | ✅ |

**Fallback Chain:**

| Priority | Provider | Model | Local |
|----------|----------|-------|-------|
| 1 | llama.cpp | qwen2.5-coder:1.5b | ✅ |
| 2 | copilot | gpt-4 | ❌ |
| 3 | anthropic | claude-3-haiku | ❌ |
| 4 | openai | gpt-4o-mini | ❌ |
| 5 | git | template-based | ✅ |

**Adaptive Execution:**

| Setting | Value | Description |
|---------|-------|-------------|
| Mode | `adaptive` | Switch parallel/sequential based on resources |
| Parallel Threshold | 35% | CPU/memory threshold for parallel execution |
| Check Interval | 500ms | Resource polling interval |
| Drain Timeout | 5s | Graceful drain timeout on resource spikes |

**Commands:**

```bash
noa wiki generate --full          # Full codebase regeneration
noa wiki generate --incremental   # Changed files only
noa wiki status                   # Show generation progress
noa wiki cancel                   # Cancel running generation
```

---

### backupctl

**Workspace backup orchestration with hash-verified manifests and approval tokens.**

| Property | Value |
|----------|-------|
| **Directory** | `backupctl/` |
| **Implementation** | Python script (`backupctl.py`) |
| **Scripts** | `scripts/hourly-incremental-backup.sh`, `scripts/nightly-full-backup.sh` |

**Commands:**

| Command | Description |
|---------|-------------|
| `backupctl run --operation <op> --target <name>` | Execute full backup pipeline |
| `backupctl push <op-id> [--exec] --mark` | Sync artifacts via rclone |
| `backupctl approve --id <op-id> --approver <name>` | Emit signed approval token |
| `backupctl list` | Display recent backups |
| `backupctl verify --id <op-id>` | Recalculate hashes against manifest |

**File Layout:**

| Path | Purpose |
|------|---------|
| `.backups/archives/` | Backup archives |
| `.backups/manifests/` | Backup manifests |
| `logs/backups/` | Tokens, ledger, approval tokens |

---

### sandboxctl

**Sandbox lifecycle management with backup enforcement policy.**

| Property | Value |
|----------|-------|
| **Directory** | `sandboxctl/` |
| **Implementation** | Python script (`sandboxctl.py`) |

**Commands:**

| Command | Description |
|---------|-------------|
| `sandboxctl init <name>` | Initialize new sandbox |
| `sandboxctl list` | List all sandboxes |
| `sandboxctl destroy <name> --backup-token <token>` | Destroy sandbox (requires backup token) |
| `sandboxctl prune --backup-token <token>` | Prune unused sandboxes |

**Key Features:**
- Backup-before-destructive policy enforcement
- SHA-512 manifest + Sigstore signature verification
- Dry-run mode (`--dry-run`) for previewing operations

---

### database-query-tool

**Execute SQL queries against SQLite, PostgreSQL, or in-memory databases with parameterized queries and result formatting.**

| Property | Value |
|----------|-------|
| **File** | `database-query-tool.json` |
| **Category** | Database |
| **Implementation** | Builtin (`noa.tools.database.query`) |
| **Dependencies** | rusqlite, sqlx |

**Input Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `query` | string | ✅ | — | SQL query to execute |
| `database` | string | ❌ | `primary` | Database connection identifier |
| `parameters` | array | ❌ | — | Parameterized query values (prevents SQL injection) |
| `driver` | enum | ❌ | `sqlite` | Driver: `sqlite`, `postgres`, `memory` |
| `format` | enum | ❌ | `json` | Output: `json`, `csv`, `table`, `raw` |
| `limit` | integer | ❌ | 1000 | Maximum rows to return |
| `timeout_ms` | integer | ❌ | 30000 | Query timeout in milliseconds |
| `readonly` | boolean | ❌ | false | Enforce read-only mode |

**Examples:**

```bash
# Select with parameters
noa db query "SELECT * FROM users WHERE status = ? AND created_at > ?" --params '["active", "2025-01-01"]'

# Read-only mode with timeout
noa db query "SELECT COUNT(*) FROM logs" --readonly --timeout 5000

# Output as CSV
noa db query "SELECT id, name FROM tasks" --format csv
```

---

### database-migrate-tool

**Manage database schema migrations with version tracking, rollback support, and integrity verification.**

| Property | Value |
|----------|-------|
| **File** | `database-migrate-tool.json` |
| **Category** | Database |
| **Implementation** | Builtin (`noa.tools.database.migrate`) |
| **Dependencies** | sqlx |

**Input Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `action` | enum | ✅ | — | `run`, `rollback`, `status`, `create`, `verify`, `reset` |
| `database` | string | ❌ | `primary` | Database connection identifier |
| `migrations_path` | string | ❌ | `${NOA_ROOT}/sys/core/migrations` | Path to migrations directory |
| `target_version` | string | ❌ | — | Target migration version |
| `name` | string | ❌ | — | Migration name (for create action) |
| `dry_run` | boolean | ❌ | false | Preview changes without applying |
| `force` | boolean | ❌ | false | Force migration even with warnings |

**Examples:**

```bash
# Run all pending migrations
noa db migrate run

# Check migration status
noa db migrate status

# Create new migration
noa db migrate create --name add_user_preferences_table

# Rollback to specific version
noa db migrate rollback --target 20251231120000 --dry-run
```

---

### database-backup-tool

**Create and restore database backups with integrity verification and compression.**

| Property | Value |
|----------|-------|
| **File** | `database-backup-tool.json` |
| **Category** | Database |
| **Implementation** | Builtin (`noa.tools.database.backup`) |
| **Dependencies** | rusqlite |

**Input Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `action` | enum | ✅ | — | `create`, `restore`, `list`, `verify`, `prune` |
| `database` | string | ❌ | `primary` | Database connection identifier |
| `backup_path` | string | ❌ | `${NOA_ROOT}/data/backups/database` | Backup directory |
| `backup_id` | string | ❌ | — | Specific backup ID |
| `compression` | enum | ❌ | `zstd` | `none`, `gzip`, `zstd`, `lz4` |
| `verify_after` | boolean | ❌ | true | Verify backup integrity after creation |
| `retention_days` | integer | ❌ | 30 | Days to retain backups |
| `include_wal` | boolean | ❌ | true | Include WAL file in backup |

**Examples:**

```bash
# Create compressed backup with verification
noa db backup create --compression zstd

# List available backups
noa db backup list

# Restore from specific backup
noa db backup restore --id 20260101_120000_primary

# Prune old backups (keep 7 days)
noa db backup prune --retention 7
```

---

### vector-search-tool

**Semantic vector search for knowledge retrieval using embedded vectors and similarity matching.**

| Property | Value |
|----------|-------|
| **File** | `vector-search-tool.json` |
| **Category** | Database |
| **Implementation** | Builtin (`noa.tools.database.vector`) |
| **Dependencies** | ruvector, noa-embedder |

**Input Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `query` | string | ✅ | — | Natural language query to search for |
| `collection` | string | ❌ | `default` | Vector collection/index to search |
| `top_k` | integer | ❌ | 10 | Number of top results to return |
| `similarity_threshold` | number | ❌ | 0.7 | Minimum similarity score (0-1) |
| `filter` | object | ❌ | — | Metadata filter for results |
| `embedding_model` | string | ❌ | `nomic-embed-text` | Model for query embedding |
| `include_metadata` | boolean | ❌ | true | Include document metadata |
| `include_vectors` | boolean | ❌ | false | Include raw vectors in results |

**Examples:**

```bash
# Basic semantic search
noa vector search "How to configure database connections" --top-k 5

# Filtered search with threshold
noa vector search "error handling best practices" \
  --collection documentation \
  --filter '{"type": "markdown"}' \
  --threshold 0.8

# Code search
noa vector search "async database connection pool" \
  --collection codebase \
  --filter '{"source": "rust"}' \
  --top-k 20
```

---

## Tool Definition Format

Tools are defined as JSON files following MCP schema:

```json
{
  "$schema": "https://noa.local/schemas/mcp-tool.json",
  "name": "file_read",
  "version": "1.0.0",
  "description": "Read contents of a file",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": {
        "type": "string",
        "description": "Path to the file to read"
      },
      "encoding": {
        "type": "string",
        "default": "utf-8",
        "description": "File encoding"
      }
    },
    "required": ["path"]
  },
  "outputSchema": {
    "type": "object",
    "properties": {
      "content": { "type": "string" },
      "size": { "type": "integer" }
    }
  },
  "implementation": {
    "type": "builtin",
    "handler": "noa.tools.file_read"
  },
  "permissions": ["file:read"],
  "rate_limit": {
    "requests_per_minute": 100
  }
}
```

---

## Tool Categories

| Category | Purpose | Examples |
|----------|---------|----------|
| **Development** | Code generation, refactoring | code-generation-tool |
| **Analysis** | Reasoning, problem-solving | reasoning-tool |
| **Documentation** | Doc generation, wiki management | documentation-generator |
| **Operations** | Backup, sandbox, deployment | backupctl, sandboxctl |
| **File Operations** | Read, write, search files | (builtin) |
| **Git Operations** | Commit, diff, branch management | (builtin) |
| **Shell Commands** | Execute system commands | (builtin) |
| **HTTP Requests** | API calls, web scraping | (builtin) |
| **Database** | Query, insert, update data | (builtin) |

---

## Implementation Types

| Type | Description | Example |
|------|-------------|---------|
| `builtin` | Implemented in NOA core | File operations |
| `agent` | Delegated to AI agent | code-generation-tool |
| `script` | External script (bash, python) | backupctl |
| `http` | HTTP endpoint call | External APIs |
| `mcp` | Delegate to MCP server | External MCP tools |
| `binary` | Native binary execution | documentation-generator |

---

## Security Considerations

Tools require explicit permissions. Available permissions:

| Permission | Description | Risk Level |
|------------|-------------|------------|
| `file:read` | Read file system | Low |
| `file:write` | Write file system | Medium |
| `shell:execute` | Command execution | High |
| `network:http` | HTTP requests | Medium |
| `git:read` | Git read operations | Low |
| `git:write` | Git write operations | Medium |

---

## Creating a New Tool

1. Create a new JSON file: `my-tool.json`
2. Define input/output schemas following MCP spec
3. Specify implementation type and permissions
4. Register in `../resources/resource-registry.json`
5. Update this README with tool documentation

---

## Related Files

| File | Purpose |
|------|---------|
| `../agents/` | Agents that use these tools |
| `../workflows/` | Workflows that invoke tools |
| `../resources/resource-registry.json` | Central registry |
| `../models/README.md` | Available models for tool inference |

