---
layout: default
title: Schemas and contracts
---

# Schemas and contracts

This page maps **config files → schemas** and **runtime contracts → source specs**.

## Config schemas

Schemas live under:

- [`config/schemas/`](../../config/schemas/)

Key mappings:

| Config | Purpose | Schema |
|---|---|---|
| `config/ai-providers.json` | Grouped provider categories (enabled/priority/types/configPath) | `config/schemas/providers.yaml` |
| `config/providers/default.yaml` | Grouped provider category defaults | `config/schemas/providers.yaml` |
| `config/desktop-apps.json` | Desktop apps registry | `config/schemas/desktop-apps.json` |
| `config/governance.json` | Governance events | `config/schemas/governance.json` |
| `config/shared-resources.json` | Shared AI resources | `config/schemas/resource-registry.json` (and related resources under `ai/shared/resources/`) |

## API and protocol contracts

Contract sources live under:

- [`specs/001-noa-seed-foundation/contracts/`](../../specs/001-noa-seed-foundation/contracts/)

| Contract | Source | Docs |
|---|---|---|
| Core HTTP API | `noa-core.openapi.yaml` | `docs/api/README.md` |
| Digest pipeline HTTP API | `digest-pipeline.openapi.yaml` | `docs/api/README.md` |
| P2P protocol | `p2p-protocol.proto` | `docs/api/README.md` |

## How to keep things in sync (recommended)

- Make contract/schema changes **first** (OpenAPI / JSON Schema / proto).
- Then update implementation.
- Then update the relevant docs page(s) and rerun docs checks.


