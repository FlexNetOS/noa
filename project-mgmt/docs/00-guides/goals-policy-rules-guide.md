# Goals-Policy-Rules Framework Guide

> A comprehensive guide to creating, linking, and maintaining Goals, Policy, and Rules documents.

## Table of Contents

1. [Framework Overview](#1-framework-overview)
2. [When to Use Each Artifact](#2-when-to-use-each-artifact)
3. [Creation Workflow](#3-creation-workflow)
4. [Cross-Linking Requirements](#4-cross-linking-requirements)
5. [Naming Conventions](#5-naming-conventions)
6. [Tag System](#6-tag-system)
7. [Reward System Integration](#7-reward-system-integration)
8. [Maintenance Procedures](#8-maintenance-procedures)
9. [Common Mistakes to Avoid](#9-common-mistakes-to-avoid)
10. [Quick Reference](#10-quick-reference)

---

## 1) Framework Overview

### The Three Pillars

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        GOALS-POLICY-RULES FRAMEWORK                     │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│   ┌─────────────┐      ┌─────────────┐      ┌─────────────┐            │
│   │   GOALS     │      │   POLICY    │      │   RULES     │            │
│   │   (WHY)     │ ───► │   (WHAT)    │ ───► │   (HOW)     │            │
│   └─────────────┘      └─────────────┘      └─────────────┘            │
│                                                                         │
│   Strategic         Governing           Executable                      │
│   Objectives        Constraints         Enforcement                     │
│                                                                         │
│   "What we want     "What we must       "How we check                   │
│    to achieve"       follow"             compliance"                    │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Document Relationships

| Artifact | Purpose | Audience | Change Frequency |
|----------|---------|----------|------------------|
| **Goals** | Define success criteria and strategic objectives | Leadership, Stakeholders | Quarterly |
| **Policy** | Establish boundaries and required behaviors | All team members | Semi-annually |
| **Rules** | Provide automated enforcement mechanisms | Developers, CI/CD | As needed |

### Directory Structure

```
docs/
├── 00-guides/              # How-to guides (this document)
├── 00-examples/            # Annotated examples with instructions
├── 00-templates/           # Clean templates for copying
├── 01-goals/               # Goal documents
│   └── {topic}-goals.md
├── 02-policy/              # Policy documents
│   └── {topic}-policy.md
├── 03-rules/               # Rule documents
│   └── {topic}-rule.md
└── goal-ledger.md          # Central registry of all goals
```

---

## 2) When to Use Each Artifact

### Start with Goals When...

- Defining a new initiative or capability area
- Stakeholders need to align on success criteria
- You need to justify resource allocation
- Measuring outcomes is more important than prescribing methods

**Goals Document Answers:**
- What does success look like?
- How will we measure progress?
- What are our constraints?
- Who is responsible?

### Create Policy When...

- Multiple teams need consistent behavior
- Compliance or regulatory requirements exist
- You need to establish organizational standards
- Exceptions need a formal approval process

**Policy Document Answers:**
- What behaviors are required?
- What is explicitly prohibited?
- How do we handle exceptions?
- Who has authority to do what?

### Define Rules When...

- Behaviors can be automatically verified
- You need CI/CD enforcement
- Violations should block progress
- Consistency can be checked programmatically

**Rules Document Answers:**
- How do we detect violations?
- What's the exact enforcement mechanism?
- What are valid patterns?
- How do we fix violations?

### Decision Flowchart

```
                    ┌─────────────────────┐
                    │ New topic/initiative│
                    └──────────┬──────────┘
                               │
                               ▼
                    ┌─────────────────────┐
              ┌─────│ Need strategic      │─────┐
              │ YES │ objectives?         │ NO  │
              │     └─────────────────────┘     │
              ▼                                 │
     ┌─────────────────┐                        │
     │ Create Goals    │                        │
     │ Register in     │                        │
     │ Ledger          │                        │
     └────────┬────────┘                        │
              │                                 │
              ▼                                 ▼
     ┌─────────────────────┐         ┌─────────────────────┐
     │ Need governing      │   YES   │ Existing goals      │
     │ constraints?        │◄────────│ cover this?         │
     └──────────┬──────────┘         └─────────────────────┘
                │ YES
                ▼
     ┌─────────────────┐
     │ Create Policy   │
     │ Link to Goals   │
     └────────┬────────┘
              │
              ▼
     ┌─────────────────────┐
     │ Can enforce         │
     │ automatically?      │
     └──────────┬──────────┘
                │ YES
                ▼
     ┌─────────────────┐
     │ Create Rules    │
     │ Link to Policy  │
     │ Link to Goals   │
     └─────────────────┘
```

---

## 3) Creation Workflow

### Step 1: Register Goals First

**Always start with the goal ledger.** Before creating any documents:

1. Open `docs/goal-ledger.md`
2. Assign goal IDs using format `G-{TOPIC}-{NNN}`
3. Add entries to the Active Goals table
4. Commit the ledger update

```markdown
| ID | Goal | Topic | Priority | Status | Document | Owner |
|----|------|-------|----------|--------|----------|-------|
| G-AUTH-001 | Secure user authentication | Auth | Critical | Active | [auth-goals.md](01-goals/auth-goals.md) | Security |
| G-AUTH-002 | Session management | Auth | High | Active | [auth-goals.md](01-goals/auth-goals.md) | Security |
```

### Step 2: Create Goals Document

1. Copy template: `00-templates/goal-template.md` → `01-goals/{topic}-goals.md`
2. Replace all `{topic}` with your topic (lowercase)
3. Replace all `{TOPIC}` with uppercase version
4. Fill in primary goals (reference the IDs from ledger)
5. Define constraints, metrics, and stakeholders
6. Add policy and rules links (even if documents don't exist yet)

### Step 3: Create Policy Document

1. Copy template: `00-templates/policy-template.md` → `02-policy/{topic}-policy.md`
2. Replace placeholders with topic-specific content
3. Link to goals document in §1
4. Define the enforcement mapping to rules in §2
5. Write critical section with hard-stop violations
6. Document prohibitions explicitly

### Step 4: Create Rules Document

1. Copy template: `00-templates/rules-template.md` → `03-rules/{topic}-rule.md`
2. Link to goals in Associated Goals section
3. Link to policy in Associated Policy section
4. Create rules that enforce policy sections
5. Add detection scripts (must be executable)
6. Verify Rule Index matches Enforcement Matrix

### Step 5: Verify Cross-Links

Run this checklist after creating all three documents:

```
□ Goals → Policy link works
□ Goals → Rules link works
□ Goals → Ledger link works
□ Policy → Goals link works
□ Policy → Rules link works
□ Policy → Ledger link works
□ Rules → Goals link works
□ Rules → Policy link works
□ Rules → Ledger link works
□ Ledger has all goal IDs
□ All section references (§N) are accurate
□ All rule IDs match across documents
```

---

## 4) Cross-Linking Requirements

### Mandatory Links

Every document MUST include these cross-references:

| Document | Required Links |
|----------|---------------|
| Goals | Policy (§8), Rules (§9), Ledger (§1) |
| Policy | Goals (§1), Rules (§2), Ledger (SUMMARY) |
| Rules | Goals (Associated Goals), Policy (Associated Policy), Ledger (SUMMARY) |

### Link Format Standards

**Goal-to-Policy Link:**
```markdown
## 8) Associated Policy

- **Policy Document:** [auth-policy.md](../02-policy/auth-policy.md)
- **Key Sections:**
  - §5 - Authentication Requirements
  - §6 - Session Standards
```

**Policy-to-Rules Link:**
```markdown
## 2) Associated Rules

- **Rules Document:** [auth-rule.md](../03-rules/auth-rule.md)
- **Enforcement Mapping:**

| Policy Section | Enforced By | Severity |
|----------------|-------------|----------|
| §5.1 Password Rules | AUTH-001, AUTH-002 | CRITICAL |
| §5.2 Token Rules | AUTH-003 | HIGH |
```

**Rules-to-Goals Link:**
```markdown
## Associated Goals

- **Goals Document:** [auth-goals.md](../01-goals/auth-goals.md)
- **Goal Enforcement:**

| Goal ID | Goal Name | Enforced By Rules |
|---------|-----------|-------------------|
| G-AUTH-001 | Secure authentication | AUTH-001, AUTH-002 |
| G-AUTH-002 | Session management | AUTH-003, AUTH-004 |
```

### Traceability Matrix

For complex topics, maintain a traceability matrix:

```markdown
## Traceability Matrix

| Goal | Policy Section | Rules | Detection | Test |
|------|----------------|-------|-----------|------|
| G-AUTH-001 | §5.1, §5.2 | AUTH-001, AUTH-002 | pre-commit | auth.test.ts |
| G-AUTH-002 | §6.1 | AUTH-003 | build | session.test.ts |
| G-AUTH-003 | §11 (prohibitions) | AUTH-004, AUTH-005 | runtime | audit.test.ts |
```

---

## 5) Naming Conventions

### Topic Names

| Type | Format | Example |
|------|--------|---------|
| Topic (lowercase) | `{topic}` | `auth`, `env`, `logging` |
| Topic (uppercase) | `{TOPIC}` | `AUTH`, `ENV`, `LOG` |

**Rules for topic names:**
- Use lowercase for file names and links
- Use UPPERCASE for IDs and prefixes
- Keep topics short (3-10 characters)
- Use common abbreviations

### ID Formats

| ID Type | Format | Example |
|---------|--------|---------|
| Goal ID | `G-{TOPIC}-{NNN}` | `G-AUTH-001` |
| Rule ID | `{TOPIC}-{NNN}` | `AUTH-001` |
| Exception ID | `EX-{TOPIC}-{NNN}` | `EX-AUTH-001` |
| Violation ID | `V-{TOPIC}-{NNN}` | `V-AUTH-001` |

### File Naming

```
{topic}-goals.md      # Goals document
{topic}-policy.md     # Policy document
{topic}-rule.md       # Rules document (singular "rule")
```

**Note:** Use `rule` (singular) not `rules` for consistency with "goal" and "policy".

---

## 6) Tag System

### Tag Hierarchy

```
Primary Tags (required):
├── #{topic}                    # Topic identifier
├── #goal-{topic}               # Goal document
├── #policy-{topic}             # Policy document
└── #rule-{TOPIC}-{NNN}         # Individual rule

Classification Tags:
├── #priority-{level}           # critical/high/medium/low
├── #status-{state}             # draft/active/achieved/deprecated
├── #severity-{level}           # critical/high/medium/low
├── #compliance-{level}         # mandatory/recommended/optional
└── #enforcement-{stage}        # pre-commit/build/lint/deploy/runtime

Association Tags:
├── #milestone-{name}           # Milestone association
├── #stakeholder-{role}         # Stakeholder association
└── #dependency-{name}          # Dependency tracking
```

### Tag Usage by Document

| Document | Required Tags | Optional Tags |
|----------|---------------|---------------|
| Goals | `#{topic}`, `#goal-{topic}`, `#priority-{level}`, `#status-{state}` | `#milestone-*`, `#stakeholder-*` |
| Policy | `#{topic}`, `#policy-{topic}`, `#compliance-{level}`, `#enforcement-{stage}` | `#section-*`, `#exception-*` |
| Rules | `#{topic}`, `#rule-{TOPIC}-{NNN}`, `#severity-{level}`, `#enforcement-{stage}` | `#goal-*`, `#policy-section-*` |

### Tag Placement

Tags appear in two locations:

1. **In SUMMARY block** (machine-readable):
```markdown
## SUMMARY

```
TAGS: #auth, #goal-auth, #priority-critical, #status-active
```
```

2. **In Tags section** (human-readable with descriptions):
```markdown
## Tags

| Tag | Purpose |
|-----|---------|
| `#auth` | Primary topic identifier |
| `#goal-auth` | Goal document reference |
```

---

## 7) Reward System Integration

### Reward Categories

| Category | Goals | Policy | Rules |
|----------|-------|--------|-------|
| **Achievement** | Milestone completion | Zero violations | Clean builds |
| **Excellence** | Exceeding metrics | Audit excellence | Automation |
| **Improvement** | Goal refinement | Policy enhancement | Rule contribution |
| **Compliance** | N/A | Sustained compliance | Low violation rate |

### Defining Rewards

**In Goals Document:**
```markdown
## 11) Reward System

### 11.1 Achievement Rewards

| Achievement | Criteria | Reward | Eligible |
|-------------|----------|--------|----------|
| Milestone Champion | Complete Phase 1 by target date | Team lunch | Project team |
| Metric Master | Exceed target metric by 20% | $100 bonus | Individual |

### 11.2 Milestone Bonuses

| Milestone | Bonus Condition | Bonus |
|-----------|-----------------|-------|
| MVP Launch | Completed 2 weeks early | Extra PTO day |
```

**In Policy Document:**
```markdown
## 15) Reward System

### 15.1 Compliance Rewards

| Achievement | Criteria | Reward | Eligible |
|-------------|----------|--------|----------|
| Policy Champion | Zero violations for 6 months | Badge + Recognition | Individual |
| Audit Excellence | Pass audit with no findings | Team celebration | Team |

### 15.2 Violation Consequences

| Level | First | Repeat | Chronic |
|-------|-------|--------|---------|
| CRITICAL | Immediate review | Formal warning | Performance action |
| HIGH | Discussion | Written feedback | Training required |
```

**In Rules Document:**
```markdown
## Reward System

### Compliance Rewards

| Achievement | Criteria | Reward | Eligible |
|-------------|----------|--------|----------|
| Clean Build | Zero violations in sprint | Recognition | Team |
| First Reporter | Identify new rule need | Gift card | Anyone |
| Rule Author | Submit accepted rule | Learning budget | Developer |
```

### Aligning Rewards Across Documents

Ensure reward triggers align:

| Goal Milestone | Policy Compliance | Rule Achievement |
|----------------|-------------------|------------------|
| "Phase 1 Complete" triggers | "Zero violations during Phase 1" | "All rules passing" |
| "Metric Target Met" triggers | N/A | "Detection accuracy > 95%" |

---

## 8) Maintenance Procedures

### Regular Reviews

| Review Type | Frequency | Owner | Scope |
|-------------|-----------|-------|-------|
| Goal Progress | Weekly | Project Lead | Status updates, blockers |
| Policy Compliance | Monthly | Tech Lead | Violation trends, exceptions |
| Rule Effectiveness | Monthly | DevOps | False positive rate, coverage |
| Cross-Link Audit | Quarterly | Documentation Owner | Link validity, sync |
| Full Framework Review | Annually | Leadership | Relevance, completeness |

### Update Procedures

**Updating Goals:**
1. Update goal-ledger.md first
2. Update the goals document
3. Check if policy sections need updates
4. Check if rules need updates
5. Update LAST UPDATED in all affected documents

**Updating Policy:**
1. Update the policy document
2. Update enforcement mapping if sections changed
3. Update rules if enforcement changed
4. Verify goals still align
5. Update LAST UPDATED

**Updating Rules:**
1. Update the rules document
2. Update Rule Index and Enforcement Matrix together
3. Verify policy section references
4. Verify goal ID references
5. Test detection scripts
6. Update LAST UPDATED

### Deprecation Process

When deprecating goals, policies, or rules:

1. **Mark as deprecated** (don't delete immediately):
```markdown
> ⚠️ **DEPRECATED:** This {goal/policy/rule} is deprecated as of {date}.
> Superseded by: {link to replacement}
> Removal date: {date}
```

2. **Update ledger** (for goals):
```markdown
### Deprecated Goals

| ID | Goal | Deprecated Date | Reason | Superseded By |
|----|------|-----------------|--------|---------------|
| G-AUTH-001 | Old auth goal | 2025-12-01 | Replaced | G-AUTH-010 |
```

3. **Update cross-references** in related documents

4. **Remove after grace period** (typically 30-90 days)

---

## 9) Common Mistakes to Avoid

### ❌ Mistake 1: Creating Rules Without Goals

**Wrong:**
```
1. Create rules for password complexity
2. (No goals document exists)
```

**Right:**
```
1. Register G-AUTH-001 "Secure authentication" in ledger
2. Create auth-goals.md with success criteria
3. Create auth-policy.md with password requirements
4. Create auth-rule.md with AUTH-001 password rule
```

### ❌ Mistake 2: Inconsistent IDs

**Wrong:**
```
Goals: G-Auth-001 (mixed case)
Rules: auth-001 (lowercase)
Policy: References AUTH_001 (underscore)
```

**Right:**
```
Goals: G-AUTH-001
Rules: AUTH-001
Policy: References AUTH-001
```

### ❌ Mistake 3: One-Way Links

**Wrong:**
```
Goals → Policy (link exists)
Policy → Goals (no link)
```

**Right:**
```
Goals → Policy (link exists)
Policy → Goals (link exists)
Goals → Rules (link exists)
Rules → Goals (link exists)
Policy → Rules (link exists)
Rules → Policy (link exists)
```

### ❌ Mistake 4: Stale Section References

**Wrong:**
```
# In rules document
**Supports:** Goal G-AUTH-001 | Policy §3.1

# But policy was renumbered and §3.1 is now §5.1
```

**Right:**
- Update all section references when renumbering
- Use the cross-link audit checklist after any structural changes

### ❌ Mistake 5: Detection Scripts That Don't Run

**Wrong:**
```bash
# Pseudocode that won't execute
check if passwords are strong
```

**Right:**
```bash
# Actual executable command
rg -l "password.*=.*['\"][^'\"]{0,7}['\"]" --type ts src/
# Must return empty (no weak passwords)
```

### ❌ Mistake 6: Rule Index vs Matrix Mismatch

**Wrong:**
```
Rule Index: AUTH-001 | Enforcement: Pre-commit
Matrix: Build | AUTH-001, AUTH-002 | Build fails
```

**Right:**
```
Rule Index: AUTH-001 | Enforcement: Pre-commit
Matrix: Pre-commit | AUTH-001, AUTH-002 | Commit blocked
```

### ❌ Mistake 7: Missing Ledger Registration

**Wrong:**
```
1. Create auth-goals.md with G-AUTH-001
2. (Never update goal-ledger.md)
```

**Right:**
```
1. Add G-AUTH-001 to goal-ledger.md
2. Create auth-goals.md referencing G-AUTH-001
3. Update ledger cross-reference indexes
```

---

## 10) Quick Reference

### File Locations

| Artifact | Path |
|----------|------|
| Goal Ledger | `docs/goal-ledger.md` |
| Goals | `docs/01-goals/{topic}-goals.md` |
| Policy | `docs/02-policy/{topic}-policy.md` |
| Rules | `docs/03-rules/{topic}-rule.md` |
| Templates | `docs/00-templates/` |
| Examples | `docs/00-examples/` |

### ID Quick Reference

```
Goal ID:      G-{TOPIC}-{NNN}     Example: G-AUTH-001
Rule ID:      {TOPIC}-{NNN}       Example: AUTH-001
Exception:    EX-{TOPIC}-{NNN}    Example: EX-AUTH-001
```

### Section Number Quick Reference

| Goals | Policy | Rules |
|-------|--------|-------|
| §1 Primary Goals | §0 Scope | Associated Goals |
| §2 Non-Goals | §1 Associated Goals | Associated Policy |
| §3 Constraints | §2 Associated Rules | Rule Index |
| §4 Success Metrics | §3 Definitions | Individual Rules |
| §5 Stakeholders | §4 Categories | Enforcement Matrix |
| §6 Universal Policy | §5 Critical Section | Tags |
| §7 Dependencies | §6 Standards | Reward System |
| §8 Associated Policy | §7-10 Process/Access | Commands |
| §9 Associated Rules | §11 Prohibitions | |
| §10 Tags | §12 Exceptions | |
| §11 Reward System | §13 Compliance | |
| §12 Timeline | §14 Tags | |
| | §15 Reward System | |

### Checklist: New Topic

```
□ 1. Register goals in goal-ledger.md
□ 2. Copy goal-template.md → 01-goals/{topic}-goals.md
□ 3. Fill in goals document
□ 4. Copy policy-template.md → 02-policy/{topic}-policy.md
□ 5. Fill in policy document
□ 6. Link policy to goals
□ 7. Copy rules-template.md → 03-rules/{topic}-rule.md
□ 8. Fill in rules document
□ 9. Link rules to policy and goals
□ 10. Update ledger cross-reference indexes
□ 11. Verify all links work
□ 12. Test all detection scripts
□ 13. Update LAST UPDATED in all documents
```

### Checklist: Cross-Link Audit

```
□ All goal IDs in ledger match documents
□ Goals → Policy links work
□ Goals → Rules links work
□ Policy → Goals links work
□ Policy → Rules enforcement mapping accurate
□ Rules → Goals mapping accurate
□ Rules → Policy section references accurate
□ Rule Index matches Enforcement Matrix
□ All section references (§N) are current
□ All LAST UPDATED dates are current
```

---

## Appendix: Template Quick Copy Commands

```bash
# Create new topic documents (replace 'newtopic' with your topic)
cp docs/00-templates/goal-template.md docs/01-goals/newtopic-goals.md
cp docs/00-templates/policy-template.md docs/02-policy/newtopic-policy.md
cp docs/00-templates/rules-template.md docs/03-rules/newtopic-rule.md

# Find and replace placeholders (example using sed)
sed -i 's/{topic}/newtopic/g' docs/01-goals/newtopic-goals.md
sed -i 's/{TOPIC}/NEWTOPIC/g' docs/01-goals/newtopic-goals.md
```

---

**Document Version:** 1.0
**Last Updated:** 2025-12-04
**Maintainer:** Documentation Team
