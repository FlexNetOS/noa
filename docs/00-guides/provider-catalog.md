---
layout: default
title: Provider catalog
---

# Provider catalog

This is the “what exists today” list of providers in the repo.

## Provider registry

The runtime selection registry is **grouped by category**:

- [`config/ai-providers.json`](../../config/ai-providers.json)

Provider metadata is stored under:

- [`ai/providers/`](../../ai/providers/)

## Providers in `ai/providers/`

| Provider | Category | Metadata | Auth env var (if any) |
|---|---|---|---|
| `llama.cpp` | local | `ai/providers/local/llama-cpp.json` + `ai/providers/local/llama.cpp/` | N/A |
| `ollama` | local | `ai/providers/local/ollama/config.json` | N/A |
| `git-cli` | local | `ai/providers/local/git-cli/config.json` | N/A |
| `cursor` | hybrid | `ai/providers/hybrid/cursor/config.json` | OAuth (see metadata) |
| `vscode-copilot` | ide | `ai/providers/ide/vscode-copilot/config.json` | GitHub OAuth (Copilot subscription) |
| `claude-code` | cloud | `ai/providers/cloud/claude-code/config.json` | `ANTHROPIC_API_KEY` |
| `codex` | cloud | `ai/providers/cloud/codex/config.json` | `OPENAI_API_KEY` |
| `abacus` | cloud | `ai/providers/cloud/abacus/config.json` | `ABACUS_API_KEY` |

## Notes

- A provider listed in `ai/providers/**/config.json` only becomes selectable when its ID appears in the category `types[]` list inside `config/ai-providers.json`.
- If a category `configPath` points to a missing directory, NOA can warn at startup (see `sys/core/src/config/validator.rs`).


