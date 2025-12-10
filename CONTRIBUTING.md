# Contributing Guide

Thanks for helping improve NOA. This guide covers the minimal workflow for contributions.

## Prerequisites
- Run `scripts/setup/setup-noa.ps1` (Windows) or `scripts/setup/setup-noa.sh` (macOS/Linux) to install portable toolchains and shared resources.
- Use the generated `noa-profile` script to load environment variables.
- Ensure `cargo`, `node`, and `python` from `opt/` are on your `PATH`.

## Workflow
1. Create a feature branch from `main`.
2. Keep changes small and atomic; prefer one concern per PR.
3. Add or update tests for new behavior.
4. Run formatting and linters:
   - `cargo fmt && cargo clippy` (Rust)
   - `npm run lint` (UI)
   - `ruff` / `semgrep` where applicable
5. Run relevant test suites (`cargo test`, `npm test`, targeted scripts under `tests/`).
6. Update docs when behavior, flags, or CLI surfaces change.

## Commit and PR Guidelines
- Use clear commit messages (`feat:`, `fix:`, `docs:`, `chore:`).
- Include context in PR descriptions: scope, rationale, testing, and risks.
- Link related tasks from `specs/001-noa-seed-foundation/tasks.md`.

## Security and Privacy
- Do not commit secrets or personal data. Use `.env` and secrets managers.
- Prefer local/offline operation; flag any external calls in your PR.
- Follow constitutional principles: self-contained (3.1), auditable (3.5), secure (3.6), verify-first (3.12).

## Support
- Open issues with clear reproduction steps and expected vs. observed behavior.
- For bootstrap/setup problems, include OS, shell, and the exact command output.
