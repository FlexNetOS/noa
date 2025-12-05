# Goal Ledger

> Central registry of all primary goals across the project. Every goal document MUST register its primary goals here.

## Purpose

This ledger provides:
- **Single source of truth** for all project goals
- **Cross-reference links** to detailed goal documents
- **Status tracking** across all goal areas
- **Priority visibility** for resource allocation
- **Dependency mapping** between goal areas

---

## Goal Registry

### Active Goals

| ID | Goal | Topic | Priority | Status | Document | Owner |
|----|------|-------|----------|--------|----------|-------|
| G-ENV-001 | Zero secret leakage | Environment | Critical | Active | [env-goals.md](01-goals/env-goals.md) | Security |
| G-ENV-002 | Environment parity | Environment | High | Active | [env-goals.md](01-goals/env-goals.md) | DevOps |
| G-ENV-003 | Type-safe configuration | Environment | High | Active | [env-goals.md](01-goals/env-goals.md) | Dev Lead |
| G-ENV-004 | Zero-downtime rotation | Environment | Medium | Active | [env-goals.md](01-goals/env-goals.md) | DevOps |
| {G-XXX-NNN} | {Goal name} | {Topic} | {Priority} | {Status} | [{topic}-goals.md](01-goals/{topic}-goals.md) | {Owner} |

### Achieved Goals

| ID | Goal | Topic | Achieved Date | Document | Evidence |
|----|------|-------|---------------|----------|----------|
| {G-XXX-NNN} | {Goal name} | {Topic} | {YYYY-MM-DD} | [{topic}-goals.md](01-goals/{topic}-goals.md) | {Link to proof} |

### Deprecated Goals

| ID | Goal | Topic | Deprecated Date | Reason | Superseded By |
|----|------|-------|-----------------|--------|---------------|
| {G-XXX-NNN} | {Goal name} | {Topic} | {YYYY-MM-DD} | {Why deprecated} | {New goal ID or N/A} |

---

## Goal ID Convention

```
G-{TOPIC}-{NNN}

Where:
  G       = Goal prefix
  TOPIC   = Uppercase topic abbreviation (ENV, AUTH, LOG, TEST, etc.)
  NNN     = Sequential number within topic (001, 002, 003...)

Examples:
  G-ENV-001  = First environment goal
  G-AUTH-003 = Third authentication goal
  G-LOG-002  = Second logging goal
```

---

## Status Definitions

| Status | Meaning | Next Action |
|--------|---------|-------------|
| **Draft** | Goal defined but not approved | Review and approve |
| **Active** | Approved and being pursued | Track progress |
| **Blocked** | Cannot progress due to dependency | Resolve blocker |
| **Achieved** | Goal met with evidence | Move to Achieved table |
| **Deprecated** | No longer relevant | Document reason |

---

## Priority Definitions

| Priority | Meaning | Resource Allocation |
|----------|---------|---------------------|
| **Critical** | Failure blocks all progress | Dedicated resources, daily tracking |
| **High** | Required for release | Prioritized resources, weekly tracking |
| **Medium** | Important but not blocking | Scheduled resources, bi-weekly tracking |
| **Low** | Nice to have | As-available resources, monthly tracking |

---

## Cross-Reference Index

### By Topic

| Topic | Goal Count | Document | Policy | Rules |
|-------|------------|----------|--------|-------|
| Environment | 4 | [env-goals.md](01-goals/env-goals.md) | [env-policy.md](02-policy/env-policy.md) | [env-rule.md](03-rules/env-rule.md) |
| {Topic} | {N} | [{topic}-goals.md](01-goals/{topic}-goals.md) | [{topic}-policy.md](02-policy/{topic}-policy.md) | [{topic}-rule.md](03-rules/{topic}-rule.md) |

### By Priority

| Priority | Goals |
|----------|-------|
| Critical | G-ENV-001 |
| High | G-ENV-002, G-ENV-003 |
| Medium | G-ENV-004 |
| Low | — |

### By Owner

| Owner | Goals |
|-------|-------|
| Security | G-ENV-001 |
| DevOps | G-ENV-002, G-ENV-004 |
| Dev Lead | G-ENV-003 |

---

## Dependency Map

> Goals that depend on other goals.

| Goal | Depends On | Dependency Type |
|------|------------|-----------------|
| G-ENV-003 | G-ENV-001 | Prerequisite |
| G-ENV-004 | G-ENV-002 | Prerequisite |
| {Goal ID} | {Goal ID(s)} | {Prerequisite/Parallel/Sequential} |

---

## Adding Goals to the Ledger

When creating a new goals document:

1. **Assign IDs:** Use next sequential number for the topic
2. **Register:** Add each primary goal to the Active Goals table
3. **Link:** Ensure Document column links to the goals file
4. **Cross-ref:** Update By Topic, By Priority, and By Owner indexes
5. **Dependencies:** Add any goal dependencies to Dependency Map

### Registration Checklist

- [ ] Goal ID follows `G-{TOPIC}-{NNN}` convention
- [ ] Goal added to Active Goals table
- [ ] Document link is correct and working
- [ ] Owner assigned
- [ ] Priority set
- [ ] Status set (usually "Active" for new goals)
- [ ] Topic added to Cross-Reference Index if new
- [ ] Dependencies documented if any exist

---

## Ledger Maintenance

| Action | Frequency | Owner |
|--------|-----------|-------|
| Review active goals | Weekly | Project Lead |
| Update statuses | As changes occur | Goal Owner |
| Archive achieved goals | Upon achievement | Goal Owner |
| Audit cross-references | Monthly | Project Lead |
| Dependency review | Quarterly | Tech Lead |

---

## LEDGER SUMMARY

```
TOTAL GOALS: {N}
ACTIVE: {N}
ACHIEVED: {N}
DEPRECATED: {N}
BLOCKED: {N}

LAST UPDATED: {YYYY-MM-DD}
MAINTAINER: {Role/Name}
```
