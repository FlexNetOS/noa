---
layout: default
title: Provider integration
---

# Provider integration

NOA models “providers” as **runnable capabilities** (local, hybrid, IDE, cloud) configured in a **grouped category registry**.

## Where provider configuration lives

- **Provider registry (runtime selection)**: [`config/ai-providers.json`](../../config/ai-providers.json)
  - Validated by: [`config/schemas/providers.yaml`](../../config/schemas/providers.yaml)
  - Loaded by Rust core: `sys/core/src/config/loader.rs`
- **Provider metadata**: [`ai/providers/<category>/<provider>/config.json`](../../ai/providers/)
  - Human-facing: install/auth hints, env vars, capabilities, CLI metadata

## Add a new provider (step-by-step)

### 1) Create provider metadata

Create:

- `ai/providers/<category>/<provider>/config.json`

Use the existing provider configs as examples, e.g.:

- `ai/providers/cloud/claude-code/config.json`
- `ai/providers/local/ollama/config.json`

At minimum, include:

- `name`, `type` (`local|hybrid|ide|cloud`), `enabled`, `priority`
- `description`
- `modes` and any auth/env var notes under `modes.*`

### 2) Register the provider in `config/ai-providers.json`

Add the provider **ID** to the appropriate category’s `types[]` list, and ensure the category `configPath` points at the right folder:

- `providers.<category>.types[]`: includes the provider ID (e.g. `claude-code`)
- `providers.<category>.enabled`: enables/disables the whole category
- `providers.<category>.priority`: category priority (lower is preferred)
- `providers.<category>.configPath`: points to `ai/providers/<category>`

### 3) Validate locally

- Schema: `config/schemas/providers.yaml`
- Link and markdown checks: see [`CONTRIBUTING.md`](../../CONTRIBUTING.md) and `scripts/docs/check-docs.*`

## Provider selection rules (current)

- `providerPriority` defines the order of categories to try (e.g. `local` before `cloud`).
- Category `priority` can be used to break ties or apply category weighting.
- `providers.<category>.enabled=false` excludes the category.

## Secrets and auth

- Do **not** commit API keys to this repo.
- Provider configs should reference **environment variables** for secrets (e.g. `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `ABACUS_API_KEY`).


