# Configuration Deprecation Workflow (Prelaunch)

**Document ID**: ARCH-CONFIG-DEPRECATION-001  
**Status**: Draft

## Purpose

Define how config sources are deprecated and migrated without ambiguity.

## Registry Fields

Add/maintain the following fields in `docs/plans/config-audit-table.csv`:
- `conflicts_with`: list of other known sources for the same domain
- `next_actions`: migration steps
- `notes`: rationale and timing

For deprecations, add these conventions:
- Set `authority=derived` for legacy copies that are no longer canonical.
- Set `consumer_status=none` once no consumers remain.
- Set `doc_tier=Deprecated` for documentation sources.

## Required Metadata (for each deprecated source)

- Replacement source
- Migration steps
- Target version/date

## Enforcement

- Any `authority=authoritative` row that has a competing source MUST list it in `conflicts_with`.
- Any non-authoritative competing source MUST list the authoritative source in `conflicts_with`.
