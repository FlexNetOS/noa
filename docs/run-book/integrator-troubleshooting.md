---
layout: default
title: Integrator troubleshooting
---

# Integrator troubleshooting

## “My provider isn’t being selected”

Check:

1. `config/ai-providers.json` has the provider under `providers.<id>` and `enabled: true`.
2. `type` is one of `local|hybrid|ide|cloud`.
3. `priority` is a positive integer (lower is preferred).
4. `configPath` exists on disk (you should have `ai/providers/...` checked out).

## “The API port doesn’t match the OpenAPI contract”

There are two common local API surfaces:

- **Core API** (`noa start`) defaults to port `8080` (matches `noa-core.openapi.yaml` server URL).
- **Sample server** (`noa-api`) binds to `127.0.0.1:3001` (used by some UI flows).

Decide which one you’re targeting, then use the matching base URL.

## “Docs links are broken”

Run docs checks:

- `scripts/docs/check-docs.ps1` (Windows / PowerShell)
- `scripts/docs/check-docs.sh` (macOS/Linux)

If a link points to a file that moved, update the markdown to a working relative link.


