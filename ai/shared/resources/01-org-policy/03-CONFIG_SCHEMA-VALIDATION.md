# Configuration Schema Validation (Prelaunch)

**Document ID**: ARCH-CONFIG-VALIDATION-001  
**Status**: Draft  

## Decision

- **JSON Schema draft**: **2020-12**
- **Canonical `$id` policy**:
  - Every schema file MUST have a stable `$id`.
  - `$id` MUST be a repo-relative URI of the form: `noa://schemas/<path>`.
    - Example: `noa://schemas/config/desktop-apps.json`

## Environment Variable Substitution

NOA config loaders MUST support (at minimum):

- `${VAR}`
- `${VAR:-default}`

Rules:
1. Substitution is performed **before** schema validation.
2. If `${VAR}` is used and `VAR` is missing, validation MUST fail (`hard_fail`) for `authority=authoritative` assets.
3. For `${VAR:-default}`, the `default` value is used when `VAR` is missing.
4. Substitution applies to string scalar values only (no implicit casting).

## Registry Mapping

For every schema in `config/schemas/**`:
- `asset_type=schema`
- `authority=authoritative`
- `validation_mode=hard_fail`
- `schema_status=exists`

For every config asset:
- `schema_ref` must be either:
  - a schema path (in repo) + `$id`, or
  - an explicit exception documented in `notes`.

## Enforcement

This architecture is enforced by `scripts/config/validate-config-audit-table.ps1` and follow-on schema lint checks (to be added).
