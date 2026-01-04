# Wiki / Pages / Runbooks (Runbook Standard)

## Purpose

Keep documentation durable under pressure by separating:

- **Wiki**: navigation and architecture map (stable index)
- **Pages**: knowledge blocks (design, how-tos, ADRs)
- **Runbooks**: pressure-tested actions (incident remediation)

## Definitions

### Wiki (Map)
The wiki is the **system map**, not the system memory.

Required content:
- canonical hierarchy: `Home ? Systems Catalog ? Services ? {Runbooks, How-tos, ADRs, Diagrams}`
- ownership + on-call rotation link
- dependencies
- SLO/SLA pointers
- dashboards/alerts
- �where the truth lives� links

### Pages (Knowledge)
Pages are **granular, composable knowledge blocks**.

Allowed content:
- design docs, diagrams, vendor notes, configs examples
- known gotchas
- rationale / ADRs

Quality tiers:
- `Draft`
- `Reviewed`
- `Deprecated`

### Runbooks (Action)
Runbooks are **pages with teeth**: they must work at 03:00 on a phone.

Quality tiers:
- `Verified` (required for alert-linked runbooks)
- `Not Verified` (allowed, but may not be linked from alerts)

## Hard rule (non-negotiable)

**Alerts must deep-link to a runbook whose first screen contains:**
1. **Trigger** (alert name + symptoms)
2. **Escalation** (who/when/how)
3. **First 3 actions** (immediate checks/remediation)

## Runbook template (required header)

Every runbook must begin with:

- **Trigger(s):** exact alert name(s), thresholds, symptoms
- **Impact/Severity:** what breaks and customer/system impact
- **Scope:** what this runbook covers / does not cover
- **Owner:** team + primary maintainer
- **Escalation Path:** who next, when, and how
- **Last Verified:** date + verifier + version/context verified against
- **Automation:** command(s) or job link(s) (if available)

## Runbook structure (recommended)

1. **Triage (5 minutes)**
   - what to check first
   - how to confirm impact
   - where to look (logs/metrics/traces)

2. **Mitigation**
   - safest reversible actions first
   - explicit rollback steps

3. **Recovery**
   - how to restore service
   - how to validate recovery

4. **Post-incident**
   - follow-ups (tickets, gaps, runbook improvements)

## Playbooks vs Runbooks

- **Playbook**: coordination/comms (roles, timeline, stakeholders)
- **Runbook**: technical remediation steps (commands, checks, rollback)

## Verification and anti-rot policy

- Runbooks change with the system.
- Any PR that changes behavior addressed by a runbook must update the runbook.
- Critical runbooks must be re-verified on a cadence (monthly/quarterly depending on change rate).

## Docs-as-code recommendation

For offline-first, auditable documentation:
- maintain runbooks and pages alongside code
- rely on Git history for traceability of changes and verification

## Tooling (quick, open source)

The policy above is tooling-agnostic, but these options are low-effort to adopt:

- **MkDocs + Material for MkDocs**: fast docs-as-code site, good search, mobile-friendly.
  - https://www.mkdocs.org/
  - https://squidfunk.github.io/mkdocs-material/

- **Backstage TechDocs** (service catalog + docs discovery): docs-as-code rendered per service from the catalog.
  - https://backstage.io/docs/features/techdocs/

- **Rundeck Community (OSS)** (runbook automation runner): execute scripted diagnostics/remediations from a central job runner.
  - https://docs.rundeck.com/

If adopting one thing first: generate a static docs site (MkDocs), then add a catalog layer (Backstage) when service count grows.

## NOA-specific notes

- Prefer linking alert ? runbook ? automation where feasible.
- Use `NOA_ROOT`-relative paths in commands (portable, self-contained).
