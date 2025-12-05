# {topic}-rule.md

> Executable rules for {topic description}. Each rule is atomic, testable, and enforceable.

## Associated Goals

- **Goals Document:** [{topic}-goals.md](../01-goals/{topic}-goals.md)
- **Goal Enforcement:**

| Goal ID | Goal Name | Enforced By Rules |
|---------|-----------|-------------------|
| G-{TOPIC}-001 | {Goal name} | {TOPIC}-001, {TOPIC}-002 |
| G-{TOPIC}-002 | {Goal name} | {TOPIC}-003, {TOPIC}-004 |

- **Goal Ledger:** [goal-ledger.md](../goal-ledger.md#{topic})

## Associated Policy

- **Policy Document:** [{topic}-policy.md](../02-policy/{topic}-policy.md)
- **Policy Enforcement:**

| Policy Section | Section Name | Enforced By Rules |
|----------------|--------------|-------------------|
| §5.1 | {Subsection A} | {TOPIC}-001, {TOPIC}-002 |
| §5.2 | {Subsection B} | {TOPIC}-003 |
| §6 | {Standards} | {TOPIC}-004, {TOPIC}-005 |

## Rule Index

| ID | Rule | Enforcement | Severity | Goal | Policy §|
|----|------|-------------|----------|------|---------|
| {TOPIC}-001 | {Rule name} | {Stage} | CRITICAL | G-{TOPIC}-001 | §5.1 |
| {TOPIC}-002 | {Rule name} | {Stage} | CRITICAL | G-{TOPIC}-001 | §5.1 |
| {TOPIC}-003 | {Rule name} | {Stage} | HIGH | G-{TOPIC}-002 | §5.2 |
| {TOPIC}-004 | {Rule name} | {Stage} | HIGH | G-{TOPIC}-002 | §6 |
| {TOPIC}-005 | {Rule name} | {Stage} | MEDIUM | G-{TOPIC}-003 | §6 |
| {TOPIC}-006 | {Rule name} | {Stage} | MEDIUM | G-{TOPIC}-003 | §6 |

---

## {TOPIC}-001: {Rule Name}

**Statement:** {Subject} SHALL/SHALL NOT {requirement}.

**Supports:** Goal G-{TOPIC}-001 | Policy §5.1

**Detection:**
```bash
{detection command}
```

**Exceptions:** {Exceptions or "None."}

**Remediation:** {Fix steps.}

---

## {TOPIC}-002: {Rule Name}

**Statement:** {Subject} SHALL/SHALL NOT {requirement}.

**Supports:** Goal G-{TOPIC}-001 | Policy §5.1

**Detection:**
```bash
{detection command}
```

**Remediation:** {Fix steps.}

---

## {TOPIC}-003: {Rule Name}

**Statement:** {Subject} SHALL {requirement}.

**Supports:** Goal G-{TOPIC}-002 | Policy §5.2

**Detection:**
```bash
{detection command}
```

**Correct Pattern:**
```typescript
// ✓ Correct
{good example}

// ✗ Wrong
{bad example}
```

**Remediation:** {Fix steps.}

---

## {TOPIC}-004: {Rule Name}

**Statement:** {Subject} SHALL {requirement}.

**Supports:** Goal G-{TOPIC}-002 | Policy §6

**Detection:**
```typescript
{detection code}
```

**Remediation:** {Fix steps.}

---

## {TOPIC}-005: {Rule Name}

**Statement:** {Subject} SHALL follow pattern `{PATTERN}`.

**Supports:** Goal G-{TOPIC}-003 | Policy §6

**Detection:**
```bash
{detection command}
```

**Valid Examples:**
- `{Example 1}`
- `{Example 2}`

**Invalid Examples:**
- `{Example 1}` ({reason})
- `{Example 2}` ({reason})

**Remediation:** {Fix steps.}

---

## {TOPIC}-006: {Rule Name}

**Statement:** {Subject} SHALL {requirement}.

**Supports:** Goal G-{TOPIC}-003 | Policy §6

**Detection:**
```bash
{detection command}
```

**Remediation:** {Fix steps.}

---

## Rule Enforcement Matrix

| Stage | Rules Enforced | Failure Action |
|-------|----------------|----------------|
| Pre-commit | {TOPIC}-001, {TOPIC}-002 | Commit blocked |
| Build | {TOPIC}-003, {TOPIC}-004 | Build fails |
| Lint/Review | {TOPIC}-005, {TOPIC}-006 | Warning / PR blocked |
| Deploy | {TOPIC}-00X | Deployment halted |
| Runtime | {TOPIC}-00X | App won't start |

---

## Tags

| Tag | Purpose |
|-----|---------|
| `#{topic}` | Primary topic identifier |
| `#rule-{TOPIC}-{NNN}` | Individual rule reference |
| `#severity-{level}` | Severity classification |
| `#enforcement-{stage}` | Enforcement stage |

---

## Reward System

### Compliance Rewards

| Achievement | Criteria | Reward | Eligible |
|-------------|----------|--------|----------|
| {Achievement 1} | {Criteria} | {Reward} | {Who} |
| {Achievement 2} | {Criteria} | {Reward} | {Who} |

### Violation Tracking

| Metric | Target | Current |
|--------|--------|---------|
| CRITICAL violations/month | 0 | {N} |
| HIGH violations/month | < 5 | {N} |
| Mean time to remediation | < 24h | {N}h |

---

## Quick Reference Commands

```bash
# Full rule check
npm run {topic}:validate

# Auto-fix where possible
npm run {topic}:fix

# Audit report
npm run {topic}:audit
```

---

## RULE SET SUMMARY

```
RULESET: {Topic} Rules
VERSION: 1.0
LAST UPDATED: {YYYY-MM-DD}
TOTAL RULES: {N}
CRITICAL: {N} ({IDs})
HIGH: {N} ({IDs})
MEDIUM: {N} ({IDs})
AUTOMATION: {Mechanisms}
TAGS: #{topic}, #rule-{TOPIC}-{NNN}, #severity-{level}, #enforcement-{stage}
GOALS: ../01-goals/{topic}-goals.md
POLICY: ../02-policy/{topic}-policy.md
LEDGER: ../goal-ledger.md#{topic}
```
