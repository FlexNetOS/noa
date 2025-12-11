# Code & Config Policy

Vision: reproducible, portable, clean, and enforceable across devices; configs are schema-validated, agent-healable, and consistently linted/tested.

## Canonical Conventions
- **Key style**: camelCase for all JSON/YAML config keys (aligns with current configs). If snake_case is desired later, plan migration + transformer first.
- **Metadata**: every config carries `version`, `$schema` (when available), `owner`, `last_reviewed`, and change history in VCS or adjacent log.
- **Paths & env**: prefer `${NOA_ROOT}`; avoid hardcoded absolute paths; document required env vars.
- **additionalProperties**: default to `false` unless extension points are explicitly intended.
- **Validation**: required on commit (CI) and on service start; treat failures as blocking.
- **Layout reference** (example):
```
config/
  profiles/
    default.yaml
    dev.yaml
  user/preferences.json
  device/hardware.json
  global/settings.json
  secrets/token.key
  data/knowledge_graph.db
  data/logs/app.log
  data/cache/temp_files/
  state/session.state
  state/runtime.state
  modules/
    agent/runtime.cfg
    microkernel/boot.cfg
    ui/theme.json
```

## Config Governance
- Every config change: bump `version` when shape changes; record `last_reviewed`; update `$schema` pointer.
- Run schema validation pre-commit and at startup; reject unknown fields (`additionalProperties: false`) unless documented.
- Require env-substitution guardrails: allow `${VAR}`; disallow unbounded `~` or platform-specific hardcodes.
- Secrets: never in config; reference env/secret stores only.
- Change control: describe why + rollback steps in commit message or adjacent CHANGELOG block.

## Language Policies (must match linters/formatters)
- **JS/TS**: Use `.config/eslint.config.mjs` (ESLint + typescript-eslint recommended). Blockers: no unused vars (except `_`-prefixed), prefer typed APIs; `any` warns; keep project on TS recommended settings.
- **Python**: `.config/ruff.toml` enforced (line length 100, Py3.12 target, E/F/I/N/W/UP/B/C4/SIM). Sorting via isort profile. Fix violations or justify ignores inline.
- **Go**: `.config/.golangci.yml` enforced (go 1.23). Required linters include gofmt/goimports/staticcheck/errcheck/etc. Run `golangci-lint run` locally; no unchecked errors unless documented.
- **Rust**: `.config/rustfmt.toml` + `.config/clippy.toml` (msrv 1.83, cognitive/args thresholds). Run `cargo fmt` and `cargo clippy -- -D warnings`.
- **CI**: All lint/format commands are mandatory gates; add pre-commit hooks where available.

## Coding Standards
- Naming: meaningful, domain-specific; no magic numbers/strings-use constants/enums.
- Functions: small, single responsibility; avoid deep nesting; prefer composition.
- Error handling: return actionable errors; no silent catch; log with context (who/what/where); avoid panics except in init.
- Logging: default to info/warn/error; avoid PII; include correlation/request ids where available.
- Dependencies: pin versions; remove unused; review licenses; prefer internal bundles under `${NOA_ROOT}/opt`.
- Security: validate inputs; least privilege; never log secrets.

## Kernel & Self-Containment Policy
- Default kernel mode: NOA kernels first (VM > container > sandbox > native). Host kernel MAY be used for bootstrap/scanning/file access but NOT required for normal operation.
- Offline-first: all required tools/packages live under `${NOA_ROOT}/opt` with PATH precedence over system tools.
- Snapshot/rollback: take checkpoint before kernel mode changes; maintain `.kernel-switch-state.json`; document recovery steps.
- Allowed host interactions: read-only scans, targeted file IO, performance tuning that is internalized; anything else requires explicit allowlist.
- Status/enforcement: `noa status` (or equivalent) must report kernel mode, tool origins, and validation results.

## Provider Policy
- Priority: lower `priority` = higher preference; `providerPriority` defines category order (`local`, `hybrid`, `ide`, `cloud`).
- Validation: `config/ai-providers.json` must pass `config/schemas/providers.yaml`; per-provider files should mirror category entries and stay camelCase.
- Drift detection: ensure `types` in category blocks match installed providers; flag mismatches between `config/ai-providers.json` and `config/providers/{...}`.
- Switching: respect `providerSwitching` flags (`enabled`, `preserveConfigs`, `migrateState`); document migration steps when changing priorities.
- Versioning: increment config `version` on breaking shape or priority changes; include migration notes.

## Immutable Base / Mutable Semantic Layer / Enforcement
- **Immutable base**: microkernel layout, toolchain versions, core schemas, sandbox definitions; changes require explicit approval + rollback plan.
- **Mutable semantic layer**: preferences, capabilities, device profiles, world-model metadata, provider tuning; changes allowed via validated config edits.
- **Enforcement**: validators, schema checks, snapshot diff monitors, policy engine; failures block deploy/startup until resolved.

## Governance Flow (ties to `config/schemas/governance.json`)
- Record decisions when actions cross trust boundaries (kernel changes, provider priority shifts, policy overrides).
- Minimum record: `decision.id`, `agent_id`, `action`, `rationale`, `created_at`, `rule_verdicts[]`, `final_verdict`, `evidence_hash` (SHA-256). Store `audit_path` and optional `biblical_support` per schema.
- Sign/hash records where supported; reject unsigned governance events in production workflows.

## Policy as Code Checklist
- Map every policy to enforcement: lint/format rules, schema validation, CI gates, runtime startup checks, and status output.
- Measurable outcomes: lint clean, schema clean, tests passing, coverage floor met, performance/error budgets within limits.
- Rollback: define reversal steps per change (config reversion, snapshot restore).
- Automation: prefer scripts to enforce (pre-commit, CI jobs); avoid manual-only policies.

## Elite Testing Rubric
- Coverage floors: >=80% for critical libs/services; targeted integration tests for provider orchestration and kernel switching.
- Mutation/property tests for critical paths (providers, kernel selection, config loaders).
- Performance/error budgets: define SLAs per service; add regression tests for hot paths.
- Preflight for risky changes: schema validation, lint/format, unit + smoke tests, config diff review, rollback plan documented.

## Clean Code Reminders
- Comment the "why," not the obvious "what."
- Prefer composition over inheritance; keep modules cohesive and loosely coupled.
- Use clear commit messages (imperative, 50-char subject, wrap body at 72).
