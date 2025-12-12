# NOA Config Policy (Source of Truth)

This document defines **what “properly synced configs” means** for NOA.

## Scope

Applies to committed configuration under:
- `config/`
- `ai/providers/**/config.json`
- `containers/**` (where config-like files are present)

## Core rules

- **CFG-001 (Versioned)**: All committed config files MUST include a `version` field (SemVer).
- **CFG-002 (Schema pointer)**: If a schema exists for a config, the config MUST declare `$schema`.
- **CFG-003 (Schema validation)**: Configs MUST be parseable and SHOULD validate against their schema in CI.
- **CFG-004 (Env substitution)**: Paths MUST use `${NOA_ROOT}` (and other `${ENV_VAR}`) rather than hard-coded machine paths.
- **CFG-005 (No secrets)**: Secrets MUST NOT be committed. Use env vars or local-only override files.

## Defaults vs overrides

- **Committed defaults** live in `config/` and `ai/providers/**/config.json`.
- **Local overrides** (machine/user specific) MUST be gitignored and documented as such.

Recommended pattern:
- Commit: `config/noa.json` (safe defaults)
- Local only: `config/noa.local.json` (gitignored; contains machine overrides)

## Schema mapping

- Schemas live in `config/schemas/`.
- `$schema` may use a stable URL form like `https://noa.local/schemas/<name>.json`.
- CI validation resolves these URLs to local schema files when present.

## Validation

Run locally:
- PowerShell: `pwsh -File scripts/validate/validate-configs.ps1`
- Bash: `bash scripts/validate/validate-configs.sh`

CI runs validation on pull requests and `main` pushes.


