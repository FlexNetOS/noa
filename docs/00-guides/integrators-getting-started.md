---
layout: default
title: Integrator Getting Started
---

# Integrator Getting Started

This guide is for people integrating with NOA: adding providers, authoring agents/tools, and consuming contracts.

## Quick orientation

- **Provider registry (runtime)**: [`config/ai-providers.json`](../../config/ai-providers.json)
- **Provider metadata**: [`ai/providers/**/config.json`](../../ai/providers/)
- **Agents**: [`ai/agents/`](../../ai/agents/)
- **Shared resources**: [`ai/shared/`](../../ai/shared/)
- **Contracts**: [`specs/001-noa-seed-foundation/contracts/`](../../specs/001-noa-seed-foundation/contracts/)

## Run locally (core)

From repo root:

1. **Build NOA core**
   - `cd sys/core`
   - `cargo build`

2. **Initialize configs (optional but recommended)**
   - `cargo run --bin noa -- init --root ..`

3. **Start the core API**
   - `cargo run --bin noa -- start --host 127.0.0.1 --port 8080`

The OpenAPI contract for the core API is authored against `http://localhost:8080/api/v1`:
- [`specs/001-noa-seed-foundation/contracts/noa-core.openapi.yaml`](../../specs/001-noa-seed-foundation/contracts/noa-core.openapi.yaml)

## Run locally (UI + “noa-api” sample server)

Some workflows use the `noa-api` binary which binds to `127.0.0.1:3001`:

- API:
  - `cd sys/core`
  - `cargo run --bin noa-api`
- UI:
  - `cd sys/ui`
  - `npm run dev`

See [`QUICKSTART.md`](../../QUICKSTART.md) for the current UI flow.

## Next steps

- Add/enable a provider: [`docs/00-guides/provider-integration.md`](provider-integration.md)
- Browse providers: [`docs/00-guides/provider-catalog.md`](provider-catalog.md)
- Author agents/tools: [`docs/00-guides/agent-tool-authoring.md`](agent-tool-authoring.md)
- Validate schemas/contracts: [`docs/00-guides/schemas-and-contracts.md`](schemas-and-contracts.md)


