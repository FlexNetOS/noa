---
layout: default
title: Integration touchpoints map
---

# Integration touchpoints map

This page is the **source-of-truth map** for integrators: where extension points live, and which files define contracts/config.

## Touchpoints

| What you’re integrating | Source-of-truth | Where to change | Notes |
|---|---|---|---|
| Provider selection (category priority + enabled + types list) | [`config/ai-providers.json`](../../config/ai-providers.json) | Edit `config/ai-providers.json` | Loaded by Rust core config loader. Categories contain `types[]` and a `configPath` for provider metadata folders. |
| Provider metadata (CLI/env vars/capabilities) | [`ai/providers/**/config.json`](../../ai/providers/) | Add/edit `ai/providers/<category>/<provider>/config.json` | Human-readable metadata and install/auth hints. |
| Grouped provider schema | [`config/schemas/providers.yaml`](../../config/schemas/providers.yaml) | Update schema and configs together | `config/ai-providers.json` is schema-backed by `providers.yaml`. |
| Agents (TypeScript) | [`ai/agents/`](../../ai/agents/) | Add a new folder/file under `ai/agents/` | Agent code and templates live here. |
| Orchestration scaffolding | [`ai/orchestration/`](../../ai/orchestration/) | Add/extend orchestration modules | Queues/scheduling/tasks scaffolding. |
| Shared resources (prompts/tools/skills) | [`ai/shared/`](../../ai/shared/) | Add prompts/tools/skills/workflows/resources | Shared across providers and agents. |
| HTTP API contracts | [`specs/001-noa-seed-foundation/contracts/*.openapi.yaml`](../../specs/001-noa-seed-foundation/contracts/) | Edit the OpenAPI spec(s) first | Generated docs live under `docs/api/`. |
| P2P protocol contract | [`specs/001-noa-seed-foundation/contracts/p2p-protocol.proto`](../../specs/001-noa-seed-foundation/contracts/p2p-protocol.proto) | Edit `.proto` first | Keep generated stubs/docs in sync. |

## Known seams (and how we handle them)

- **Provider registry format**: `config/ai-providers.json` is **grouped by category** and validated by [`config/schemas/providers.yaml`](../../config/schemas/providers.yaml). Provider metadata is in `ai/providers/**/config.json`.
- **Ports and API surfaces**:
  - The core OpenAPI contract is authored with `http://localhost:8080/api/v1` (see `noa-core.openapi.yaml`).
  - There is also a `noa-api` binary that binds to `127.0.0.1:3001` (used by some quickstart flows).

## What “done” looks like for an integrator

You should be able to answer (from docs alone):

1. Where do I add a provider and how do I configure it?
2. How do I add an agent/tool and connect it to shared resources?
3. Where are the schemas/contracts, and how do I validate changes locally?


