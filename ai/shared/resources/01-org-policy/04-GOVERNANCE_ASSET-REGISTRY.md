# Asset Registry Governance Policy (Prelaunch)

**Document ID**: POL-REGISTRY-001  
**Version**: 0.1.0  
**Status**: Draft  

## Purpose

Define a concrete governance policy that maps directly to the fields in `docs/plans/config-audit-table.csv` and can be enforced prelaunch.

## Source of Truth

- The asset registry CSV is the canonical inventory of config-like assets.
- Each row must be traceable via `truth_link`.

## Field Contract

Required fields for every row:
- `asset_id`
- `location`
- `asset_type`
- `layer`
- `authority`
- `owner`
- `validation_mode`
- `truth_link`

## Policy Mapping

### 1) CAS / Hybrid Configuration Model (`docs/05-policy/config-cas.md`)

Policy-to-field mapping:
- `layer`
  - `immutable`: never mutated at runtime; changes must be reviewed.
  - `mutable`: edited by humans/tools; validated where possible.
  - `cas`: immutable blobs addressed by content hash.
  - `cache`: regenerable artifacts.
  - `logs`: append-only logs.
  - `state`: mutable state with invariants.
  - `data`: persistent datasets.
- `authority`
  - `authoritative`: the canonical source for a domain.
  - `derived`: computed from authoritative inputs.
  - `generated`: produced by tools/build systems.
  - `external`: owned outside NOA; tracked for awareness.

Enforcement:
- `authority=authoritative` implies `validation_mode!=none`.
- `layer in (cache,logs)` implies `validation_mode=none`.

### 2) Docs durability tiers (`docs/05-policy/wiki-pages-runbook.md`)

Policy-to-field mapping:
- `asset_type=doc`
- `doc_tier in (Draft,Reviewed,Deprecated)`
- `runbook_verified` required for operational runbooks.
- `truth_link` required for all docs.

### 3) AppData containment (FR-001) (`docs/architecture/appdata-containment.md`)

Policy-to-field mapping:
- `contained`
  - `yes`: stored under `NOA_ROOT`.
  - `no`: outside `NOA_ROOT` (must be flagged).
  - `NA`: not applicable.
- `containment_mechanism`
  - `APPDATA redirect` / `XDG redirect` / `wrapper` / `hardcoded risk` / `NA`

Enforcement:
- Any `location` under `data/appdata/**` must have `contained=yes`.

### 4) Kernel independence (`docs/architecture/kernel-independence.md`)

Policy-to-field mapping:
- `kernel_mode_relevance in (native,vm,container,sandbox,NA)`
- `platform_scope in (windows,linux,macos,all,NA)`

Enforcement:
- If `kernel_mode_relevance != NA`, `platform_scope` must be set.

### 5) Environment policy / secrets (`project-mgmt/docs/05-policy/env-policy.md`)

Policy-to-field mapping:
- `confidentiality in (public,internal,secret)`
- `has_secrets in (yes,no,unknown)`
- `tracked_in_git in (yes,no)`
- `.env.example` must exist and be tracked.
- `env_vars_used` and `env_example_covered` describe documentation coverage.

Enforcement:
- If `has_secrets=yes` then `tracked_in_git=no`.

### 6) Provider best practices (`project-mgmt/docs/07-plans/ml_model_provider_best_practices.md`)

Policy-to-field mapping:
- `reproducible`
- `versioning`
- `lineage_required`
- `consumer_status`
- `planned_consumer` / `consumed_by`

Enforcement:
- Provider configs should be `versioning=semver` or `content-hash`.
- Provider-related authoritative configs should be `lineage_required=yes`.

## Enforcement Tooling

The following scripts are the required enforcement surface:
- `scripts/config/remove-bom.ps1`
- `scripts/config/normalize-config-audit-csv.ps1`
- `scripts/config/rebaseline-registry.ps1`
- `scripts/config/validate-config-audit-table.ps1`

## Change Control

- Any change to an `authority=authoritative` row must update `truth_link` and keep `validation_mode` consistent.
- Any new config-like file must be added to the registry.
