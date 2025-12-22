# Clean Stable Foundation: Config/Schema/Policy Stabilization Plan (Prelaunch)

## Goal
Establish a clean, stable foundation across code quality, metadata, configs, settings, env, kernel, and related assets by creating a queryable, policy-driven asset registry and using it to drive concrete governance and corrections.

## Scope
This plan covers **all config-like assets** across the repository, not only `config/`.

## Steps

### 1) Freeze scope
- [x] Confirm the asset registry scope includes everything in the comprehensive list (not just `config/`).
- [x] Define success criteria:
  - [x] Queryable inventory exists and is complete
  - [x] Concrete policy exists and maps to registry fields
  - [x] Enforcement checks exist to prevent drift
  - [x] Deprecated/legacy sources are explicitly tracked
  - [x] Documentation is regenerated from the registry

---

### 2) Normalize the inventory table
- [x] Extend `docs/plans/config-audit-table.csv` from a list into a **typed asset registry**.
- [x] Standardize list separator conventions (recommend `;` for lists).
- [x] Add a row-level unique id column if needed (e.g., `asset_id`).

---

### 3) Add policy-aligned columns (must map to governance)
Add columns aligned to:
- CAS hybrid model (`docs/05-policy/config-cas.md`)
- Wiki/pages/runbook durability (`docs/05-policy/wiki-pages-runbook.md`)
- AppData containment (`docs/architecture/appdata-containment.md`)
- Kernel independence (`docs/architecture/kernel-independence.md`)
- Environment policy (`project-mgmt/docs/05-policy/env-policy.md`)
- ML/model provider best practices (`project-mgmt/docs/07-plans/ml_model_provider_best_practices.md`)

**Minimum new columns**:
- [x] `asset_type` (config|schema|policy|template|code_consumer|runtime_state|runtime_data|tooling|generated|binary|doc)
- [x] `layer` (immutable|mutable|cas|cache|logs|state|data)
- [x] `authority` (authoritative|derived|cache|generated|external)
- [x] `owner` (sys-core|sys-desktop|ai|containers|docs|project-mgmt|tooling)
- [x] `consumer_status` (none|planned|partial|implemented)
- [x] `planned_consumer` (module/script path or intended consumer)
- [x] `consumed_by` (code modules/scripts)
- [x] `depends_on` (schemas/env/configs)
- [x] `produces` (generated outputs)
- [x] `conflicts_with` (duplicate authorities/legacy sources)
- [x] `schema_ref` (schema path + `$id`)
- [x] `schema_status` (exists|missing|outdated|wrong_draft)
- [x] `merge_key` (if merged into `NoaConfig.raw`)
- [x] `canonical_pointer` (JSON pointers)
- [x] `reloadable` (yes/no)
- [x] `watch_path` (watched file path list)
- [x] `validation_mode` (hard_fail|warn|none)
- [x] `confidentiality` (public|internal|secret)
- [x] `has_secrets` (yes|no|unknown)
- [x] `tracked_in_git` (yes/no)
- [x] `env_vars_used` (list)
- [x] `env_example_covered` (yes/no)
- [x] `contained` (yes/no/NA)
- [x] `containment_mechanism` (APPDATA redirect|XDG redirect|wrapper|hardcoded risk|NA)
- [x] `kernel_mode_relevance` (native|vm|container|sandbox|NA)
- [x] `platform_scope` (windows|linux|macos|all|NA)
- [x] `reproducible` (yes/no)
- [x] `versioning` (semver|content-hash|none)
- [x] `lineage_required` (yes/no)
- [x] `doc_tier` (Draft|Reviewed|Deprecated)
- [x] `runbook_verified` (date/user/version)
- [x] `truth_link` (path/URL to canonical source)

---

### 4) Re-baseline existing CSV rows (fill what we already know)
- [x] Populate the new columns for the rows already in the CSV.
- [x] Explicitly label external/generated artifacts:
  - [x] Go telemetry files: `asset_type=generated`, `authority=external`, `layer=state`, `validation_mode=none`, `next_actions=exclude from policy`
  - [x] Next.js app config/telemetry files similarly.
- [x] Label IDE configs as `tooling` and non-runtime.

**Known issues to preserve in the registry** (do not lose these):
- [x] Merged raw keys vs typed consumption mismatch in `sys/core`:
  - [x] `database.yaml` merged as `database_config` but typed loader reads `raw["database"]`
  - [x] Observability/logging path(s) inconsistent across docs and code
- [x] Watch list mismatch:
  - [x] `sys/core/src/config/watch.rs` does not watch all files listed by merge map
- [x] Invalid placeholder configs:
  - [x] `.github/dependabot.yml` has placeholder `package-ecosystem: ""`

---

### 5) Complete coverage for remaining listed items (no omissions)
- [x] Add rows for each of the following groups not fully represented:
  - [x] `sys/core/crates/*/Cargo.toml` (each)
  - [x] `sys/kernel/**` scripts, profiles, and JSON/YAML configs (beyond `params/current.json`)
  - [x] All `ai/shared/resources/*` (resources, registries, schemas, DBs)
  - [x] All `ai/shared/models/*` and `README.md`
  - [x] All `ai/shared/agents/*` and `README.md`
  - [x] `ai/shared/skills/*.json`, `ai/shared/tools/*.json`, `ai/shared/workflows/*.yaml`
  - [x] `ai/providers/**/config.json` (all providers)
  - [x] binaries listed (track but classify as `binary` + likely `tracked_in_git=no`)

**Granularity rule**:
- [x] Keep one directory summary row AND per-file rows for high-value/authoritative assets (agents/tools/skills/workflows/provider configs)

---

### 6) Make the policy concrete (derived from direction docs)
Draft a single governance doc (or section) that maps policy directly onto table fields.

Non-negotiable rules to encode:
- [x] From `docs/05-policy/config-cas.md`: immutable vs mutable vs CAS vs cache/state/logs/data rules
- [x] From `docs/05-policy/wiki-pages-runbook.md`: doc tiers, runbook verification, “where truth lives” link field
- [x] From `docs/architecture/appdata-containment.md`: FR-001 containment requirements + env redirection rules
- [x] From `docs/architecture/kernel-independence.md`: kernel mode precedence + platform scope rules
- [x] From `project-mgmt/docs/05-policy/env-policy.md`: secrets handling, `.env` rules, `.env.example` coverage, no secret logging
- [x] From `project-mgmt/docs/07-plans/ml_model_provider_best_practices.md`: determinism, pinned versions, lineage requirements, provider routing observability

---

### 7) Define a canonical configuration pipeline (prelaunch-ready)
Because there are **no current consumers**, define planned consumers explicitly.

- [x] Specify how authoritative configs are loaded and merged (e.g., `ConfigLoader` + merge map)
- [x] Specify schema validation approach:
  - [x] choose JSON Schema draft and enforce consistent `$id` usage
  - [x] ensure env-var pattern supports `${VAR}` and `${VAR:-default}`
- [x] Specify hot reload watch derivation rules:
  - [x] watch list must be derived from merge map
- [x] Define deprecation workflow:
  - [x] `replaced_by`, `migration_steps`, target version/date

---

### 8) Resolve authority conflicts and deprecations first
Use the registry to identify and resolve duplicates.

- [x] Identify duplicates/overlaps for provider configuration:
  - [x] `config/ai-providers.json`
  - [x] `config/providers/default.yaml`
  - [x] `ai/providers/**/config.json`
  - [ ] `sys/core/src/providers/mod.rs` default in-memory providers
- [ ] Define “one source of truth” for each domain and record it in the registry.
- [ ] Add deprecation rows and migration steps for legacy sources.

---

### 9) Update documentation from the registry (prevent drift)
- [x] Treat CSV registry as the source-of-truth.
- [x] Regenerate/update these docs from registry:
  - [x] `config/README.md` quick reference (generated as `config/README.generated.md`)
  - [x] policy docs section stating the registry is canonical (“where truth lives”)
- [ ] Add doc tiering fields and maintain them (Draft/Reviewed/Deprecated).

---

### 10) Add automated checks (even before consumers exist)
Add enforcement scripts/tests that validate:
- [ ] Every `authority=authoritative` asset has a schema or an explicit exception.
- [x] Secrets policy violations: `tracked_in_git=yes` and `has_secrets=yes` must fail.
- [x] Containment violations: any path outside `NOA_ROOT` (FR-001) flagged.
- [ ] Schema consistency:
  - [ ] draft consistency
  - [ ] env substitution pattern consistency (`${VAR}` and `${VAR:-default}`)
- [x] Reload consistency:
  - [x] any `reloadable=yes` must be watched
  - [x] watch list must match merge map

Output should be a report artifact (CSV diff + failure list).

---

### 11) Iterate: policy review ? table update ? enforcement
- [ ] Review the policy with stakeholders.
- [ ] Apply bulk updates to registry rows.
- [x] Implement enforcement scripts.
- [x] Re-run until registry stabilizes and becomes the single “truth map”.

## Files
- `docs/plans/config-audit-table.csv` (modify) - expand into typed asset registry
- `docs/05-policy/config-cas.md` (reference) - immutable/mutable/CAS model
- `docs/05-policy/wiki-pages-runbook.md` (reference) - doc durability + runbook standards
- `docs/architecture/appdata-containment.md` (reference) - FR-001 containment requirements
- `docs/architecture/kernel-independence.md` (reference) - kernel mode governance
- `project-mgmt/docs/05-policy/env-policy.md` (reference) - env/secrets rules
- `project-mgmt/docs/07-plans/ml_model_provider_best_practices.md` (reference) - determinism/versioning/observability rules
- `docs/05-policy/asset-registry-governance.md` (new) - field-mapped governance policy for the registry
- `docs/architecture/config-schema-validation.md` (new) - schema draft + env substitution contract
- `docs/architecture/config-deprecation-workflow.md` (new) - deprecation workflow contract
- `scripts/config/find-provider-authority-conflicts.ps1` (new) - provider authority overlap report
- `scripts/config/generate-config-readme.ps1` (new) - generate quick reference from registry
- `config/README.generated.md` (generated) - generated registry-based quick reference
