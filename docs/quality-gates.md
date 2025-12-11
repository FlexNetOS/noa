# Quality Gates (Staged)

This repo is a multi-language monorepo (Rust/Go/TS/Python). To avoid breaking everyone at once, **quality gates are introduced in stages**.

## Stage 0 (now): available locally, not blocking by default

- Config validation: `scripts/validate/validate-configs.*`
- Provider config validation: `scripts/bootstrap/verify/validate-provider-configs.*`
- Aggregate runner: `scripts/validate/quality-gates.*`

## Stage 1 (report-only): CI runs, does not block merges

Planned checks:
- Rust formatting + clippy: `sys/core`, `coordinator-plane`
- Go lint: `p2p`
- TS lint/typecheck: `sys/ui`, `project-mgmt` (once lockfiles are standardized)
- Python lint: `sys/digest` and other python tools

## Stage 2 (blocking): required checks for protected branches

Once repos are cleaned up and deterministic installs exist (lockfiles + pinned toolchain), the Stage 1 checks become **required**.

## Local usage

- PowerShell (report-only default):
  - `pwsh -File scripts/validate/quality-gates.ps1`
- Bash (report-only default):
  - `bash scripts/validate/quality-gates.sh`

To make it strict (fail on any missing tool or failing check), set `STRICT=true` (bash) or use `-Strict` (PowerShell).


