# NOA Configuration Files

**Purpose**: Documentation for all NOA configuration files
**Last Updated**: 2025-01-27
**Phase**: Phase 1 - Setup (Shared Infrastructure)

---

## Overview

This directory contains all configuration files for the NOA system. All configs follow JSON Schema validation and use `${NOA_ROOT}` for path resolution.

---

## Core Configuration Files

### `noa-server.json`

**Purpose**: Main server configuration
**Type**: JSON
**Schema**: `config/schemas/config_schema.json`
**Version**: 1.0.0

**Key Fields**:
- `version` (string, required): Config version for migration tracking
- `server` (object): Server host, port, TLS settings
- `database` (object): Database connection settings
- `logging` (object): Log level, format, rotation
- `features` (object): Feature flags

**Example**:
```json
{
  "version": "1.0.0",
  "server": {
    "host": "localhost",
    "port": 8080
  }
}
```

**Default**: Created during `noa init`
**Validation**: Validated against schema on load

---

### `ai-providers.json`

**Purpose**: AI provider configuration and priority
**Type**: JSON
**Schema**: `config/schemas/providers.yaml`
**Version**: 1.0.0

**Key Fields**:
- `version` (string, required): Config version
- `providerPriority` (array): Priority order (local, hybrid, ide, cloud)
- `providers` (object): Provider categories with types and config paths
- `sharedResources` (object): Shared resource paths
- `executionMemory` (object): Execution memory bus configuration
- `models` (object): Model defaults and paths
- `providerSwitching` (object): Provider switching behavior

**Provider Categories**:
- `local` (priority 1): llama.cpp, ollama, local-llm, git-cli
- `hybrid` (priority 2): cursor, local-first-with-cloud-fallback
- `ide` (priority 4): vscode-copilot
- `cloud` (priority 3): openai, anthropic, google, mistral, claude-code, codex, abacus

**Default**: Created during bootstrap
**Validation**: Validated against schema on load

---

### `features.json`

**Purpose**: Feature flags and toggles
**Type**: JSON
**Schema**: `config/schemas/config_schema.json`
**Version**: 1.0.0

**Key Fields**:
- `version` (string, required): Config version
- `features` (object): Feature name → enabled (boolean)

**Example**:
```json
{
  "version": "1.0.0",
  "features": {
    "p2p": true,
    "digest": true,
    "ui": false
  }
}
```

**Default**: Created during `noa init`
**Validation**: Validated against schema on load

---

## Storage Configuration Files

### `database.yaml`

**Purpose**: Database connection configuration
**Type**: YAML
**Version**: 1.0.0

**Key Fields**:
- `type` (string): `sqlite` or `postgres`
- `path` (string): Database file path (SQLite) or connection string (Postgres)
- `pool_size` (integer): Connection pool size

**Default**: SQLite at `${NOA_ROOT}/data/memory/noa.db`

---

### `minio.yaml`

**Purpose**: MinIO S3-compatible storage configuration
**Type**: YAML
**Version**: 1.0.0

**Key Fields**:
- `endpoint` (string): MinIO endpoint
- `access_key` (string): Access key (use env var)
- `secret_key` (string): Secret key (use env var)
- `bucket` (string): Default bucket name

---

### `qdrant.yaml`

**Purpose**: Qdrant vector store configuration
**Type**: YAML
**Version**: 1.0.0

**Key Fields**:
- `url` (string): Qdrant server URL
- `collection` (string): Default collection name
- `dimension` (integer): Vector dimension

---

## Bootstrap Configuration Files

### `bootstrap-state.json`

**Purpose**: Bootstrap tool installation state
**Type**: JSON
**Schema**: `config/schemas/config_schema.json`
**Version**: 1.0.0

**Key Fields**:
- `version` (string, required): State version
- `tools` (object): Tool name → installation state
- `updated_at` (string): ISO 8601 timestamp

**Auto-Generated**: Updated by bootstrap scripts
**Validation**: Validated on read/write

---

### `bootstrap-tools.json`

**Purpose**: Bootstrap tool definitions
**Type**: JSON
**Version**: 1.0.0

**Key Fields**:
- `tools` (array): Tool definitions with name, version, installer paths

**Default**: Created during bootstrap setup

---

## Shared Resources Configuration

### `shared-resources.json`

**Purpose**: Shared AI provider resources configuration
**Type**: JSON
**Version**: 1.0.0

**Key Fields**:
- `version` (string, required): Config version
- `paths` (object): Resource type → path mappings
- `executionMemory` (object): Execution memory bus settings

**Default**: Created during bootstrap
**Paths**: All use `${NOA_ROOT}/ai/shared/` prefix

---

## Provider-Specific Configuration

### `ai/providers/{category}/{provider}/config.json`

**Purpose**: Individual provider configuration
**Type**: JSON
**Schema**: `config/schemas/providers.yaml`
**Version**: 1.0.0

**Key Fields** (per CHK122-CHK127):
- `name` (string, required): Provider name
- `type` (string, required): Provider type (local/hybrid/ide/cloud)
- `priority` (integer, required): Unique priority (1-7)
- `enabled` (boolean, required): Whether provider is enabled
- `description` (string, required): Provider description
- `cli` (object, required): CLI configuration
  - `command` (string): CLI command name
  - `package` (string): Package name
  - `version` (string): Required version
  - `binaryPath` (string): Path to binary (uses `${NOA_ROOT}`)
- `modes` (array, required): Supported modes (cli, cloud, ide)
- `capabilities` (object, required): Provider capabilities
- `sharedResources` (object, required): Shared resource paths
- `latency` (object, optional): Latency targets
  - `target` (integer): Target latency in ms
  - `timeout` (integer): Timeout in ms
- `timeout` (integer, optional): Default timeout in ms

**Example**:
```json
{
  "name": "claude-code",
  "type": "cloud",
  "priority": 3,
  "enabled": true,
  "description": "Anthropic Claude Code CLI",
  "cli": {
    "command": "claude",
    "package": "@anthropic-ai/claude-code",
    "version": "latest",
    "binaryPath": "${NOA_ROOT}/opt/node/node_modules/.bin/claude"
  },
  "modes": ["cli", "cloud"],
  "capabilities": {
    "code": true,
    "chat": true
  },
  "sharedResources": {
    "path": "${NOA_ROOT}/ai/shared"
  },
  "latency": {
    "target": 2000,
    "timeout": 30000
  },
  "timeout": 30000
}
```

**Default**: Created during provider installation
**Validation**: Validated against schema on load

---

## Schema Files

### `config/schemas/`

**Purpose**: JSON Schema definitions for validation
**Type**: JSON/YAML
**Schema Version**: JSON Schema draft-07

**Files**:
- `config_schema.json`: Main config schema
- `providers.yaml`: Provider config schema
- `desktop-apps.json`: Desktop app schema

**Validation**: All configs validated against schemas on load (CHK068, CHK082)

---

## Environment Variables

All configs support environment variable substitution using `${ENV_VAR}` syntax (CHK067).

**Common Variables**:
- `${NOA_ROOT}`: NOA root directory (required)
- `${HOME}`: User home directory
- `${USER}`: Current user

**Example**:
```json
{
  "path": "${NOA_ROOT}/data/memory"
}
```

---

## Configuration Validation

### On Load (CHK084)

All configs are validated against their schemas when loaded:
- JSON files: Validated against JSON Schema
- YAML files: Validated after conversion to JSON

### Error Messages (CHK083)

Validation errors include:
- **Path**: JSON pointer to invalid field
- **Expected**: Expected type/value
- **Got**: Actual value received

**Example**:
```
Validation Error: config/noa-server.json#/server/port
Expected: integer (1-65535)
Got: "8080" (string)
```

---

## Configuration Migration

### Version Tracking (CHK059, CHK085)

All configs include `version` field for migration tracking:
- Version format: `MAJOR.MINOR.PATCH`
- Breaking changes: Increment MAJOR
- Backward-compatible changes: Increment MINOR/PATCH

### Migration Procedures (CHK076)

When schema changes:
1. Update schema version
2. Create migration script in `init/migrations/`
3. Document breaking changes
4. Update config version in files

---

## Best Practices

1. **Never commit sensitive values** (CHK069)
   - Use environment variables
   - Store secrets in separate, gitignored files
   - Use `${ENV_VAR}` syntax in configs

2. **Use consistent path patterns** (CHK070)
   - Always use `${NOA_ROOT}/` prefix
   - Never use `noa_root/` or hardcoded paths

3. **Validate before use**
   - Run schema validation on config load
   - Check required fields
   - Verify path existence

4. **Document changes** (CHK063)
   - Log config changes with reason and timestamp
   - Update version numbers
   - Document breaking changes

---

## Quick Reference

| Config File | Purpose | Type | Schema |
|-------------|---------|------|--------|
| `noa-server.json` | Server config | JSON | `schemas/config_schema.json` |
| `ai-providers.json` | Provider config | JSON | `schemas/providers.yaml` |
| `features.json` | Feature flags | JSON | `schemas/config_schema.json` |
| `database.yaml` | Database config | YAML | N/A |
| `minio.yaml` | MinIO config | YAML | N/A |
| `qdrant.yaml` | Qdrant config | YAML | N/A |
| `bootstrap-state.json` | Bootstrap state | JSON | `schemas/config_schema.json` |
| `shared-resources.json` | Shared resources | JSON | N/A |

---

**Documentation Version**: 1.0.0
**Last Updated**: 2025-01-27
**Maintained By**: NOA Development Team

