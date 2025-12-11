---
layout: default
title: Agent and tool authoring
---

# Agent and tool authoring

This guide covers how to add an agent/tool to NOA’s TypeScript agent layer.

## Where agents live

- Agents: [`ai/agents/`](../../ai/agents/)
  - Example suites:
    - `ai/agents/bmad/` (roles + templates)
    - `ai/agents/prp/` (workflow-oriented roles)
    - `ai/agents/speckit/` (provider-aware commands)
    - `ai/agents/model_selectors/` (domain selectors)
- Orchestration scaffolding: [`ai/orchestration/`](../../ai/orchestration/)
- Shared resources (prompts/tools/skills/workflows): [`ai/shared/`](../../ai/shared/)

## Add a new agent

### 1) Create a folder and entrypoint

Add a new folder under `ai/agents/<your-agent>/` and export a clear entrypoint (e.g. `index.ts` or `<agent>.ts`).

Keep the agent:

- **Provider-agnostic** when possible
- **Pure** (data-in/data-out) where possible
- **Explicit** about required inputs, outputs, and side effects

### 2) Add prompts/templates (optional but recommended)

If your agent produces structured outputs, add a template file under your agent folder (or reuse the BMAD templates):

- `ai/agents/bmad/templates/analysis.md`
- `ai/agents/bmad/templates/architecture.md`
- `ai/agents/bmad/templates/backlog.md`

### 3) Connect to shared resources

Shared prompts/tools/skills are stored under:

- `ai/shared/prompts/`
- `ai/shared/tools/`
- `ai/shared/skills/`
- `ai/shared/workflows/`

Prefer referencing shared resources by stable path + name, and document any required conventions.

## Provider-aware commands (example)

`ai/agents/speckit/commands.ts` builds commands based on detected providers. If you need a similar pattern, follow the same approach:

- detect candidates
- select preferred provider
- build command + env map based on available secrets


