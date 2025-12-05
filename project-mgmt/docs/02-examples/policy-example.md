# {topic}-policy.md

> **Template Instructions:** Replace `{topic}` with your subject area (e.g., `env`, `auth`, `logging`, `testing`). Delete this instruction block before finalizing.

> Governing policy for {topic description}. Applies to all projects, all environments, all agents and developers.

## 0) Scope and Priority

- **Applies to:** {What artifacts, processes, or systems this policy governs.}
- **Hierarchy:** (1) {Highest priority}; (2) {Second priority}; (3) {Third priority}.
- **Hard stop rule:** Any violation of §{N} ({Critical Section}) requires immediate remediation. Do not proceed until resolved.
- **Alignment:** Operates under the Universal Task Execution Policy. All evidence and verification requirements apply.

## 1) Associated Goals

> Link to the goals this policy supports. Policy exists to achieve goals.

- **Goals Document:** [{topic}-goals.md](../01-goals/{topic}-goals.md)
- **Supported Goals:**
  - G-{TOPIC}-001: {Goal name} — this policy enables via §{N}
  - G-{TOPIC}-002: {Goal name} — this policy enables via §{N}
  - G-{TOPIC}-003: {Goal name} — this policy enables via §{N}
- **Goal Ledger:** [goal-ledger.md](../goal-ledger.md#{topic})

## 2) Associated Rules

> Link to rules that enforce this policy. Rules make policy actionable.

- **Rules Document:** [{topic}-rule.md](../03-rules/{topic}-rule.md)
- **Enforcement Mapping:**

| Policy Section | Enforced By | Severity |
|----------------|-------------|----------|
| §3.1 {Subsection A} | {TOPIC}-001, {TOPIC}-002 | CRITICAL |
| §3.2 {Subsection B} | {TOPIC}-003 | HIGH |
| §4 {Standards} | {TOPIC}-004, {TOPIC}-005 | HIGH |
| §9 Prohibitions | {TOPIC}-006 | CRITICAL |

## 3) Definitions

> Define key terms to ensure consistent interpretation across the team.

- **{Term 1}:** {Clear, unambiguous definition.}
- **{Term 2}:** {Clear, unambiguous definition.}
- **{Term 3}:** {Clear, unambiguous definition.}
- **{Term 4}:** {Clear, unambiguous definition.}
- **{Term 5}:** {Clear, unambiguous definition.}

## 4) {Classification/Tiers/Categories}

> Define categories, tiers, or classifications relevant to this policy.

| {Category} | {Attribute 1} | {Attribute 2} | {Attribute 3} |
|------------|---------------|---------------|---------------|
| {Type 1} | {Value} | {Value} | {Value} |
| {Type 2} | {Value} | {Value} | {Value} |
| {Type 3} | {Value} | {Value} | {Value} |

**Cross-{category} rule:** {Rule that applies across categories.}

## 5) {Critical Section} (MANDATORY)

> The most important section. Violations require hard stops.

### 5.1 {Subsection A} Rules
- {Rule 1 with clear, actionable requirement.}
- {Rule 2 with verification method.}
- {Rule 3 with scope of application.}

### 5.2 {Subsection B} Rules
- {Rule 1.}
- {Rule 2.}
- {Rule 3.}

### 5.3 {Subsection C} Rules
- {Rule 1 with frequency or trigger.}
- {Rule 2 with threshold or limit.}
- {Rule 3 with escalation path.}

### 5.4 {Incident/Violation} Response
1. {Immediate action.}
2. {Investigation step.}
3. {Remediation step.}
4. {Documentation step.}
5. {Notification step.}

## 6) {Standards Section}

### 6.1 {Standard Type A}
```
{Pattern or format specification}

Examples:
  {Good example 1}
  {Good example 2}
  {Good example 3}
```

- {Guideline 1.}
- {Guideline 2.}
- {Guideline 3.}

### 6.2 Required Documentation
Every {artifact} MUST have:
1. {Documentation requirement 1.}
2. {Documentation requirement 2.}
3. {Documentation requirement 3.}

### 6.3 {Quality/Safety} Requirements
- {Requirement 1 with enforcement mechanism.}
- {Requirement 2 with verification method.}
- {Requirement 3 with failure behavior.}

## 7) {Structure/Organization} Standards

> Define expected file/folder/system structure.

```
{root}/
├── {path1}                 # {Description}
├── {path2}                 # {Description}
├── {folder}/
│   ├── {subpath1}          # {Description}
│   ├── {subpath2}          # {Description}
│   └── {subpath3}          # {Description}
└── {folder2}/
    └── {subpath}           # {Description}
```

## 8) {Process/Workflow} Rules

### 8.1 {Phase 1}
- {Rule for this phase.}
- {Another rule for this phase.}
- {Verification for this phase.}

### 8.2 {Phase 2} Requirements
- {Requirement 1.}
- {Requirement 2.}
- {Requirement 3.}

### 8.3 {Phase 3} Verification
Before marking {phase 3} complete:
- [ ] {Checklist item 1}
- [ ] {Checklist item 2}
- [ ] {Checklist item 3}
- [ ] {Checklist item 4}

## 9) Access Control

> Define who can do what.

| Action | {Role 1} | {Role 2} | {Role 3} | {Role 4} |
|--------|----------|----------|----------|----------|
| {Action 1} | ✓ | ✓ | ✓ | ✓ |
| {Action 2} | ✓ (conditional) | ✓ | ✓ | ✓ |
| {Action 3} | — | ✓ | ✓ | ✓ |
| {Action 4} | — | — | ✓ | ✓ |
| {Action 5} | — | — | ✓ | ✓ |

## 10) Validation Protocol

### 10.1 {Validation Stage 1}
```bash
# {Description of validation}
{command or script}
# {Expected outcome}
```

### 10.2 {Validation Stage 2}
```typescript
function validate{Topic}(): void {
  // {Validation logic description}
  const {items} = [{list}];
  const {failures} = {items}.filter({condition});
  if ({failures}.length > 0) {
    throw new Error(`{Error message}: ${{failures}.join(', ')}`);
  }
}
```

### 10.3 {Validation Stage 3}
- {Runtime or ongoing validation 1.}
- {Runtime or ongoing validation 2.}
- {Runtime or ongoing validation 3.}

## 11) Prohibitions

> Explicit list of forbidden actions. Use strong, clear language.

- NO {prohibited action 1}.
- NO {prohibited action 2}.
- NO {prohibited action 3}.
- NO {prohibited action 4}.
- NO {prohibited action 5}.
- NO {prohibited action 6}.

## 12) Exception Process

> How to handle legitimate exceptions to this policy.

Exceptions to this policy require:
1. Written justification with risk assessment.
2. Approval from {Approver 1} AND {Approver 2}.
3. Time-bounded scope (max {N} days).
4. Documented remediation plan.
5. Entry in exception register.

## 13) Compliance Verification

> How compliance is monitored and enforced.

| Check | Frequency | Owner | Evidence |
|-------|-----------|-------|----------|
| {Check 1} | {Frequency} | {Role} | {What proves compliance} |
| {Check 2} | {Frequency} | {Role} | {What proves compliance} |
| {Check 3} | {Frequency} | {Role} | {What proves compliance} |
| {Check 4} | {Frequency} | {Role} | {What proves compliance} |

## 14) Tags

> Tags for tracking and cross-referencing this policy.

### 14.1 Required Tags

| Tag | Purpose | Usage |
|-----|---------|-------|
| `#{topic}` | Primary topic identifier | All related artifacts |
| `#policy-{topic}` | Policy document reference | Link to this document |
| `#compliance-{level}` | Compliance classification | {mandatory/recommended/optional} |
| `#enforcement-{stage}` | When enforced | {pre-commit/build/deploy/runtime} |

### 14.2 Optional Tags

- `#section-{N}` — Reference specific policy section
- `#prohibition-{name}` — Reference specific prohibition
- `#exception-{id}` — Track approved exceptions
- `#audit-{date}` — Mark audit timestamps

## 15) Reward System

> Incentives for policy compliance and improvement.

### 15.1 Compliance Rewards

| Achievement | Criteria | Reward | Eligible |
|-------------|----------|--------|----------|
| Policy Champion | Zero violations for {N} months | {Reward} | {Who} |
| Audit Excellence | Pass compliance audit with no findings | {Reward} | {Team} |
| Policy Improvement | Submit accepted policy enhancement | {Reward} | {Anyone} |

### 15.2 Violation Consequences

| Violation Level | First Occurrence | Repeat | Chronic |
|-----------------|------------------|--------|---------|
| CRITICAL | {Consequence} | {Escalated} | {Severe} |
| HIGH | {Consequence} | {Escalated} | {Severe} |
| MEDIUM | {Consequence} | {Escalated} | {Escalated} |

### 15.3 Reward Types

- **Recognition:** Compliance badges, public acknowledgment
- **Development:** Training opportunities, conference attendance
- **Process:** Reduced audit frequency, expedited approvals
- **Team:** Celebration budget, team recognition

---

## POLICY SUMMARY

```
STATUS: ACTIVE
VERSION: 1.0
LAST UPDATED: {YYYY-MM-DD}
OWNER: {Owner team/role}
ENFORCEMENT: {Automated + Manual mechanisms}
VIOLATION CONSEQUENCE: {What happens on violation}
TAGS: #{topic}, #policy-{topic}, #compliance-{level}, #enforcement-{stage}
GOALS: ../01-goals/{topic}-goals.md
RULES: ../03-rules/{topic}-rule.md
LEDGER: ../goal-ledger.md#{topic}
```

---

## Template Checklist

Before finalizing this policy document, verify:

- [ ] All `{placeholders}` replaced with actual content
- [ ] Associated goals linked with supported goals listed
- [ ] Associated rules linked with enforcement mapping
- [ ] Scope and priority clearly defined
- [ ] All key terms defined in §3
- [ ] Critical section (§5) identifies hard-stop violations
- [ ] Access control matrix is complete
- [ ] Validation protocols are executable
- [ ] Prohibitions are explicit and unambiguous
- [ ] Exception process has clear approval chain
- [ ] Compliance checks have owners and frequencies
- [ ] Tags created and documented
- [ ] Reward system defined with criteria
- [ ] LAST UPDATED has actual date (not placeholder)
