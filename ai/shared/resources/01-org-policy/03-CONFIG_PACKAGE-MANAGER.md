# Package Manager Policy

| Status | Version | FR |
|--------|---------|-----|
| **Active** | 1.0.0 | FR-001, FR-002 |

## Decision

**pnpm is the canonical package manager for NOA.**

All Node.js package operations MUST use pnpm. npm and yarn are prohibited for dependency management (npm may be used only for publishing).

## Rationale

1. **Content-Addressable Storage**: pnpm uses a global content-addressable store, eliminating duplicate packages
2. **Strict Mode**: Prevents phantom dependencies (packages not listed in package.json)
3. **Monorepo Native**: Built-in workspace support for our monorepo structure
4. **Performance**: Faster installs via hard links instead of copying
5. **Disk Efficiency**: Single copy of each package version, shared across projects

---

## Rules

### R1: pnpm Only

```bash
# ✓ CORRECT
pnpm install
pnpm add lodash
pnpm run build

# ✗ FORBIDDEN
npm install     # Never use npm for installs
yarn add        # Never use yarn
```

### R2: Shared Packages Until Microservices

Until the codebase is split into independent microservices:

- **All packages are shared** at the workspace root
- No per-package node_modules duplication
- Use workspace protocol for internal dependencies: `"@noa/core": "workspace:*"`

```yaml
# pnpm-workspace.yaml
packages:
  - 'cmd/*'
  - 'pkg/*'
  - '.'
```

### R3: Upgrade, Never Downgrade

- Package versions MUST only increase
- Downgrades require explicit approval and documented rationale
- Use `pnpm update --latest` for upgrades
- Lock file (`pnpm-lock.yaml`) MUST be committed

### R4: Containment

All pnpm data stays within NOA_ROOT:

| Path | Purpose |
|------|---------|
| `${NOA_ROOT}/opt/pnpm` | pnpm installation (PNPM_HOME) |
| `${NOA_ROOT}/cache/pnpm` | Content-addressable store (PNPM_STORE_DIR) |
| `${NOA_ROOT}/node_modules` | Workspace dependencies |

### R5: Zero Duplication

pnpm's hoisting strategy is configured for optimal sharing:

```yaml
# .npmrc
shamefully-hoist=true
strict-peer-dependencies=false
auto-install-peers=true
```

---

## Environment Variables

Defined in [02-ENV_CANONICAL-VARS.md](02-ENV_CANONICAL-VARS.md):

```yaml
- name: PNPM_HOME
  default: "${NOA_ROOT}/opt/pnpm"
  path_add: true

- name: PNPM_STORE_DIR
  default: "${NOA_ROOT}/cache/pnpm"
```

---

## Installation

pnpm is installed as part of bootstrap:

```bash
# Via corepack (Node.js 16.13+)
corepack enable
corepack prepare pnpm@latest --activate

# Or standalone
curl -fsSL https://get.pnpm.io/install.sh | PNPM_HOME="${NOA_ROOT}/opt/pnpm" sh -
```

---

## Verification

```bash
# Check pnpm is in use
pnpm --version

# Verify store location
pnpm store path
# Should output: ${NOA_ROOT}/cache/pnpm

# Check for duplicate packages
pnpm dedupe --check
```

---

## Migration from npm/yarn

If migrating an existing project:

```bash
# Remove old lock files
rm -rf node_modules package-lock.json yarn.lock

# Import (creates pnpm-lock.yaml)
pnpm import

# Or fresh install
pnpm install
```

---

## Exceptions

| Exception | Approval Required | Documentation |
|-----------|-------------------|---------------|
| npm publish | No | Publishing packages to npm registry |
| npx one-offs | No | Running CLI tools without install |
| Submodule with yarn | Yes | External submodules may use different managers |

---

## Related Policies

- [02-ENV_CANONICAL-VARS.md](02-ENV_CANONICAL-VARS.md) - Environment variables
- [03-CONFIG_WORKSPACE.md](03-CONFIG_WORKSPACE.md) - Monorepo structure
- [03-CONFIG_TOOLS-LIFECYCLE.md](03-CONFIG_TOOLS-LIFECYCLE.md) - Upgrade policy

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2025-12-19 | Initial policy; pnpm canonical; containment rules |
