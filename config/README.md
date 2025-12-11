# NOA Configuration Files

**Purpose**: Documentation for all NOA configuration files  
**Last Updated**: 2025-01-27  
**Phase**: Phase 1 - Setup (Shared Infrastructure)

---

## Overview

This directory contains all configuration files for the NOA system. All configs follow JSON Schema validation, use `${NOA_ROOT}` for path resolution, and prefer camelCase keys. Each file should declare `version`, set `$schema` when available, and keep a short change log in VCS history or adjacent metadata.

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

**Purpose**: AI provider categories, priorities, shared resources, and model defaults  
**Type**: JSON  
**Schema**: `config/schemas/providers.yaml` (grouped categories)  
**Version**: 1.0.0

**Key Fields**:
- `$schema` (string, recommended): Schema pointer
- `version` (string, required): Config version
- `providerPriority` (array, required): Category order (`local`, `hybrid`, `ide`, `cloud`)
- `providers` (object, required): Category blocks keyed by name  
  - Each category: `{ enabled, priority, types[], configPath }`
- `sharedResources` (object): Paths for agents, workflows, prompts, skills, tools, models, commands, resources
- `executionMemory` (object): `{ enabled, path, features[] }`
- `models` (object): Defaults (`defaultContextLength`, `defaultTemperature`, `defaultTopP`, `defaultTopK`, `defaultRepeatPenalty`, `supportedFormats[]`, `modelPaths{}`, `downloadSources{}`)`
- `providerSwitching` (object): `{ enabled, preserveConfigs, migrateState }`

**Default**: Created during bootstrap  
**Validation**: Validated against `providers.yaml` on load (additional properties are rejected)

---

### `features.json`

**Purpose**: Feature flags and toggles  
**Type**: JSON  
**Schema**: `config/schemas/config_schema.json`  
**Version**: 1.0.0

**Key Fields**:
- `version` (string, required): Config version
- `features` (object): Feature name -> enabled (boolean)

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
- `tools` (object): Tool name -> installation state
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
- `paths` (object): Resource type -> path mappings
- `executionMemory` (object): Execution memory bus settings

**Default**: Created during bootstrap  
**Paths**: All use `${NOA_ROOT}/ai/shared/` prefix

---

## Provider-Specific Configuration

### `ai/providers/{category}/{provider}/config.json`

**Purpose**: Individual provider configuration (optional, per provider)  
**Type**: JSON  
**Schema**: Forthcoming; keep consistent with grouped provider policy  
**Version**: 1.0.0

**Recommended Fields (camelCase)**:
- `version` (string, required)
- `id` (string, required): Provider identifier (e.g., `claude-code`)
- `category` (string, required): `local | hybrid | ide | cloud`
- `priority` (integer, required)
- `enabled` (boolean, required)
- `description` (string, recommended)
- `modes` (array): Supported modes (`cli`, `cloud`, `ide`)
- `capabilities` (object): Feature flags per capability
- `cli` (object): `{ command, package, version, binaryPath }`
- `sharedResources` (object): Resource paths (prefer `${NOA_ROOT}`)
- `latency` (object): `{ target, timeout }`
- `timeout` (integer)

**Notes**:
- Use camelCase keys and include `$schema` when a per-provider schema is added.  
- Keep provider files aligned with category entries in `ai-providers.json` (types + priority).  
- Validation: until the schema is published, run lightweight JSON lint and keep fields in this shape.

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


