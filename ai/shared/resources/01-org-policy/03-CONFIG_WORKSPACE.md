# Workspace Structure Policy

| Status | Version | FR |
|--------|---------|-----|
| **Active** | 1.0.0 | FR-001, FR-002 |

## Overview

NOA uses a **pnpm monorepo** structure with shared packages at the workspace root. This policy defines the workspace organization, package sharing strategy, and path to microservices.

---

## Current Structure

```yaml
# pnpm-workspace.yaml
packages:
  - 'cmd/*'      # Command-line applications
  - 'pkg/*'      # Shared packages/libraries
  - '.'          # Root package
```

### Directory Layout

```
${NOA_ROOT}/
├── cmd/                    # Executable applications
│   ├── api-server/         # REST/GraphQL API server
│   ├── mcp-server/         # Model Context Protocol server
│   ├── tasks-cli/          # Task management CLI
│   ├── tasks-mcp/          # Tasks MCP integration
│   └── tasks-extension/    # VS Code extension
├── pkg/                    # Shared libraries
│   ├── tm-core/            # Task management core
│   ├── common/             # Shared utilities
│   └── ...
├── node_modules/           # Hoisted dependencies (pnpm)
├── package.json            # Root package
├── pnpm-workspace.yaml     # Workspace definition
└── pnpm-lock.yaml          # Lock file (MUST commit)
```

---

## Rules

### R1: Shared Packages Until Microservices

**Current phase: Monorepo with shared dependencies**

All packages share dependencies at the workspace root:
- Zero duplication
- Single version of each dependency
- Consistent tooling across all packages

```json
// Any package's package.json
{
  "dependencies": {
    "@noa/core": "workspace:*",
    "lodash": "catalog:"  // Uses workspace catalog
  }
}
```

### R2: Workspace Protocol

Internal dependencies use the `workspace:` protocol:

```json
{
  "dependencies": {
    "@noa/common": "workspace:*",     // Any version in workspace
    "@noa/tm-core": "workspace:^1.0.0" // Semver within workspace
  }
}
```

### R3: Catalogs for External Dependencies

Use pnpm catalogs to ensure version consistency:

```yaml
# pnpm-workspace.yaml
catalog:
  lodash: "^4.17.21"
  typescript: "^5.3.0"
  vitest: "^1.0.0"
```

```json
// package.json
{
  "devDependencies": {
    "typescript": "catalog:"  // Uses catalog version
  }
}
```

### R4: Hoisting Strategy

Configure hoisting for optimal sharing:

```ini
# .npmrc
shamefully-hoist=true       # Hoist all to root (legacy compat)
strict-peer-dependencies=false
auto-install-peers=true
public-hoist-pattern[]=*types*
public-hoist-pattern[]=*eslint*
```

### R5: No Per-Package node_modules

Packages should NOT have their own node_modules:

```bash
# ✓ CORRECT
${NOA_ROOT}/node_modules/  # All dependencies here

# ✗ INCORRECT  
${NOA_ROOT}/cmd/api-server/node_modules/  # No!
${NOA_ROOT}/pkg/tm-core/node_modules/      # No!
```

---

## Package Naming

| Pattern | Usage | Example |
|---------|-------|---------|
| `@noa/*` | Published packages | `@noa/core`, `@noa/cli` |
| `cmd/*` | Internal applications | `cmd/api-server` |
| `pkg/*` | Internal libraries | `pkg/tm-core` |

---

## Build Order

pnpm respects dependency order automatically:

```bash
# Build all packages in dependency order
pnpm -r build

# Build specific package with dependencies
pnpm --filter @noa/api-server... build
```

---

## Path to Microservices

When splitting to microservices:

### Phase 1: Current (Monorepo)
- All packages in single repo
- Shared dependencies
- Single pnpm-lock.yaml

### Phase 2: Hybrid
- Core packages remain in monorepo
- Services extracted to separate repos
- Shared packages published to npm

### Phase 3: Full Microservices
- Each service in own repo
- Dependencies via npm registry
- Independent deployment

**Trigger for Phase 2**:
- Team size > 10 developers
- Deploy frequency differs significantly between services
- Service boundaries clearly defined

---

## Scripts

Workspace-level scripts in root package.json:

```json
{
  "scripts": {
    "build": "pnpm -r build",
    "test": "pnpm -r test",
    "lint": "pnpm -r lint",
    "clean": "pnpm -r clean",
    "dev": "pnpm --parallel -r dev"
  }
}
```

---

## Dependencies Management

### Adding Dependencies

```bash
# Add to root (shared)
pnpm add lodash -w

# Add to specific package
pnpm --filter @noa/api-server add express

# Add dev dependency
pnpm add -D typescript -w
```

### Updating Dependencies

```bash
# Update all packages
pnpm update -r

# Update specific dependency everywhere
pnpm update lodash -r

# Interactive update
pnpm update -i -r
```

### Checking for Duplicates

```bash
# Check for duplicates
pnpm dedupe --check

# Fix duplicates
pnpm dedupe
```

---

## Related Policies

- [03-CONFIG_PACKAGE-MANAGER.md](03-CONFIG_PACKAGE-MANAGER.md) - pnpm usage
- [03-CONFIG_TOOLS-LIFECYCLE.md](03-CONFIG_TOOLS-LIFECYCLE.md) - Upgrade policy
- [02-ENV_CANONICAL-VARS.md](02-ENV_CANONICAL-VARS.md) - Environment setup

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-12-19 | Initial policy; workspace structure; microservices path |
