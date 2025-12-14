# {topic}-rule.md

> **Template Instructions:** Replace `{topic}` with your subject area and `{TOPIC}` with uppercase prefix (e.g., `ENV`, `AUTH`, `LOG`). Delete this instruction block before finalizing.

> Executable rules for {topic description}. Each rule is atomic, testable, and enforceable. Violations block builds or deployments.

## Associated Goals

> Link to the goals these rules help achieve.

- **Goals Document:** [{topic}-goals.md](../01-goals/{topic}-goals.md)
- **Goal Enforcement:**

| Goal ID | Goal Name | Enforced By Rules |
|---------|-----------|-------------------|
| G-{TOPIC}-001 | {Goal name} | {TOPIC}-001, {TOPIC}-002 |
| G-{TOPIC}-002 | {Goal name} | {TOPIC}-003, {TOPIC}-004 |
| G-{TOPIC}-003 | {Goal name} | {TOPIC}-005, {TOPIC}-006 |

- **Goal Ledger:** [goal-ledger.md](../goal-ledger.md#{topic})

## Associated Policy

> Link to the policy these rules enforce.

- **Policy Document:** [{topic}-policy.md](../02-policy/{topic}-policy.md)
- **Policy Enforcement:**

| Policy Section | Section Name | Enforced By Rules |
|----------------|--------------|-------------------|
| §5.1 | {Critical subsection A} | {TOPIC}-001, {TOPIC}-002 |
| §5.2 | {Critical subsection B} | {TOPIC}-003 |
| §6 | {Standards section} | {TOPIC}-004, {TOPIC}-005 |
| §11 | Prohibitions | {TOPIC}-006 |

## Rule Index

> Summary table of all rules. Keep synchronized with detailed sections below.

| ID | Rule | Enforcement | Severity | Goal | Policy §|
|----|------|-------------|----------|------|---------|
| {TOPIC}-001 | {Rule name} | {Stage} | CRITICAL | G-{TOPIC}-001 | §5.1 |
| {TOPIC}-002 | {Rule name} | {Stage} | CRITICAL | G-{TOPIC}-001 | §5.1 |
| {TOPIC}-003 | {Rule name} | {Stage} | HIGH | G-{TOPIC}-002 | §5.2 |
| {TOPIC}-004 | {Rule name} | {Stage} | HIGH | G-{TOPIC}-002 | §6 |
| {TOPIC}-005 | {Rule name} | {Stage} | MEDIUM | G-{TOPIC}-003 | §6 |
| {TOPIC}-006 | {Rule name} | {Stage} | MEDIUM | G-{TOPIC}-003 | §11 |

**Severity Definitions:**
- **CRITICAL:** Violations block all progress. Immediate remediation required.
- **HIGH:** Violations block deployment. Must fix before release.
- **MEDIUM:** Violations generate warnings. Must fix before next sprint.
- **LOW:** Violations logged. Fix when convenient.

**Enforcement Stages:**
- **Pre-commit:** Blocked before code enters repository
- **Build:** Blocked during compilation/bundling
- **Lint/Review:** Flagged during code review
- **Deploy:** Blocked before deployment proceeds
- **Runtime:** Detected during application execution

---

## {TOPIC}-001: {Rule Name}

**Statement:** {Subject} SHALL/SHALL NOT {requirement with clear scope}.

**Rationale:** {Why this rule exists and what harm it prevents.}

**Supports:** Goal G-{TOPIC}-001 | Policy §5.1

**Detection:**
```bash
# {Description of detection method}
{command or script that detects violations}
# {Expected output for passing}
```

**Exceptions:** {List exceptions or "None."}

**Remediation:** {Specific steps to fix a violation.}

---

## {TOPIC}-002: {Rule Name}

**Statement:** {Subject} SHALL/SHALL NOT {requirement}.

**Rationale:** {Why this rule matters.}

**Supports:** Goal G-{TOPIC}-001 | Policy §5.1

**Detection:**
```bash
# {Detection description}
{detection command}
# {Expected behavior}
```

**Verification:**
```bash
# {Verification that rule is satisfied}
{verification command}
```

**Remediation:** {How to fix violations.}

---

## {TOPIC}-003: {Rule Name}

**Statement:** {Subject} SHALL {requirement with measurable criteria}.

**Rationale:** {Business or technical justification.}

**Supports:** Goal G-{TOPIC}-002 | Policy §5.2

**Detection:**
```bash
# {What this checks}
{detection script or command}
```

**Correct Pattern:**
```typescript
// ✓ Correct - {why this is correct}
{good code example}

// ✓ Correct - {alternative correct approach}
{another good example}

// ✗ Wrong - {why this is wrong}
{bad code example}
```

**Remediation:** {Step-by-step fix instructions.}

---

## {TOPIC}-004: {Rule Name}

**Statement:** {Subject} SHALL {requirement} at {stage/time}.

**Rationale:** {Justification for timing/stage requirement.}

**Supports:** Goal G-{TOPIC}-002 | Policy §6

**Detection:**
```typescript
// {Detection logic description}
{detection code}
```

**Implementation:**
```typescript
// {Reference implementation}
{implementation code showing how to comply}
```

**Remediation:** {Fix instructions.}

---

## {TOPIC}-005: {Rule Name}

**Statement:** {Subject} SHALL follow the pattern `{PATTERN}`.

**Rationale:** {Why consistency matters here.}

**Supports:** Goal G-{TOPIC}-003 | Policy §6

**Detection:**
```bash
# {Pattern validation}
{command to find violations}
# Must return empty (no violations)
```

**Valid Examples:**
- `{Good example 1}`
- `{Good example 2}`
- `{Good example 3}`

**Invalid Examples:**
- `{Bad example 1}` ({why it's wrong})
- `{Bad example 2}` ({why it's wrong})
- `{Bad example 3}` ({why it's wrong})

**Remediation:** {How to rename/restructure to comply.}

---

## {TOPIC}-006: {Rule Name}

**Statement:** {Subject} SHALL be synchronized with {other artifact}.

**Rationale:** {Why synchronization matters.}

**Supports:** Goal G-{TOPIC}-003 | Policy §11

**Detection:**
```bash
# {Cross-reference validation}
{script to compare/validate synchronization}
# Must return empty (no mismatches)
```

**{Artifact} Format:**
```{format}
# {Section description}
{example content}

# {Another section}
{example content}
```

**Remediation:** {How to synchronize.}

---

## Rule Enforcement Matrix

> Maps rules to enforcement stages. Must stay synchronized with Rule Index.

| Stage | Rules Enforced | Failure Action |
|-------|----------------|----------------|
| Pre-commit | {TOPIC}-001, {TOPIC}-002 | Commit blocked |
| Build | {TOPIC}-003, {TOPIC}-004 | Build fails |
| Lint/Review | {TOPIC}-005, {TOPIC}-006 | Warning / PR blocked |
| Deploy | {TOPIC}-00X | Deployment halted |
| Runtime | {TOPIC}-00X | App won't start / Alert |

---

## Tags

> Tags for tracking and cross-referencing rules.

### Required Tags

| Tag | Purpose | Usage |
|-----|---------|-------|
| `#{topic}` | Primary topic identifier | All related artifacts |
| `#rule-{TOPIC}-{NNN}` | Individual rule reference | Link to specific rule |
| `#severity-{level}` | Severity classification | {critical/high/medium/low} |
| `#enforcement-{stage}` | Enforcement stage | {pre-commit/build/lint/deploy/runtime} |

### Optional Tags

- `#goal-{id}` — Link to supported goal
- `#policy-section-{N}` — Link to enforced policy section
- `#violation-{id}` — Track violation instances
- `#exception-{id}` — Track approved rule exceptions

---

## Reward System

> Incentives for rule compliance and detection.

### Compliance Rewards

| Achievement | Criteria | Reward | Eligible |
|-------------|----------|--------|----------|
| Clean Build | Zero rule violations in sprint | {Reward} | {Team} |
| First Reporter | First to identify new rule need | {Reward} | {Anyone} |
| Rule Author | Submit accepted rule addition | {Reward} | {Anyone} |
| Automation Hero | Automate manual rule check | {Reward} | {Dev} |

### Violation Tracking

| Metric | Target | Current | Trend |
|--------|--------|---------|-------|
| CRITICAL violations/month | 0 | {N} | {↑/↓/→} |
| HIGH violations/month | < 5 | {N} | {↑/↓/→} |
| Mean time to remediation | < 24h | {N}h | {↑/↓/→} |
| False positive rate | < 5% | {N}% | {↑/↓/→} |

### Reward Types

- **Recognition:** Compliance badges, public acknowledgment
- **Development:** Training, learning budget allocation
- **Process:** Expedited reviews, reduced approval requirements
- **Team:** Sprint celebration, team recognition

---

## Quick Reference Commands

```bash
# Full {topic} rule check
npm run {topic}:validate

# Check specific rule
npm run {topic}:check-{rule-name}

# Auto-fix where possible
npm run {topic}:fix

# Generate compliance report
npm run {topic}:audit
```

---

## Adding New Rules

When adding a new rule to this document:

1. **Assign ID:** Use next sequential number: `{TOPIC}-{NNN}`
2. **Link Goal:** Identify which goal this rule supports
3. **Link Policy:** Identify which policy section this enforces
4. **Classify Severity:** CRITICAL > HIGH > MEDIUM > LOW
5. **Define Enforcement:** When/where is it checked?
6. **Write Statement:** Use SHALL/SHALL NOT, be specific
7. **Provide Detection:** Automated check that can run in CI
8. **Show Examples:** Good and bad patterns
9. **Document Remediation:** How to fix violations
10. **Update Index:** Add to Rule Index table with Goal and Policy columns
11. **Update Matrix:** Add to Enforcement Matrix
12. **Add Tags:** Create rule-specific tag

---

## RULE SET SUMMARY

```
RULESET: {Topic} Rules
VERSION: 1.0
LAST UPDATED: {YYYY-MM-DD}
TOTAL RULES: {N}
CRITICAL: {N} ({list IDs})
HIGH: {N} ({list IDs})
MEDIUM: {N} ({list IDs})
AUTOMATION: {Enforcement mechanisms}
TAGS: #{topic}, #rule-{TOPIC}-{NNN}, #severity-{level}, #enforcement-{stage}
GOALS: ../01-goals/{topic}-goals.md
POLICY: ../02-policy/{topic}-policy.md
LEDGER: ../goal-ledger.md#{topic}
```

---

## Template Checklist

Before finalizing this rules document, verify:

- [ ] All `{placeholders}` replaced with actual content
- [ ] All `{TOPIC}` prefixes use consistent uppercase abbreviation
- [ ] Associated goals linked with goal-rule mapping
- [ ] Associated policy linked with policy-rule mapping
- [ ] Rule Index matches detailed rule sections
- [ ] Rule Index includes Goal and Policy § columns
- [ ] Rule Enforcement Matrix matches Rule Index enforcement stages
- [ ] Each rule has: Statement, Supports, Detection, Remediation
- [ ] Detection scripts are executable (tested)
- [ ] Correct/incorrect patterns clearly marked with ✓/✗
- [ ] Severity levels appropriately assigned
- [ ] Tags created and documented
- [ ] Reward system defined
- [ ] Quick reference commands work
- [ ] LAST UPDATED has actual date
