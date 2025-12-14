# {topic}-policy.md

> Governing policy for {topic description}. Applies to all projects, all environments, all agents and developers.

## 0) Scope and Priority

- **Applies to:** {What this policy governs.}
- **Hierarchy:** (1) {Priority 1}; (2) {Priority 2}; (3) {Priority 3}.
- **Hard stop rule:** Any violation of §5 requires immediate remediation.
- **Alignment:** Operates under the Universal Task Execution Policy.

## 1) Associated Goals

- **Goals Document:** [{topic}-goals.md](../01-goals/{topic}-goals.md)
- **Supported Goals:**
  - G-{TOPIC}-001: {Goal name} — enabled via §{N}
  - G-{TOPIC}-002: {Goal name} — enabled via §{N}
- **Goal Ledger:** [goal-ledger.md](../goal-ledger.md#{topic})

## 2) Associated Rules

- **Rules Document:** [{topic}-rule.md](../03-rules/{topic}-rule.md)
- **Enforcement Mapping:**

| Policy Section | Enforced By | Severity |
|----------------|-------------|----------|
| §5.1 {Subsection} | {TOPIC}-001, {TOPIC}-002 | CRITICAL |
| §5.2 {Subsection} | {TOPIC}-003 | HIGH |
| §6 {Standards} | {TOPIC}-004 | HIGH |

## 3) Definitions

- **{Term 1}:** {Definition.}
- **{Term 2}:** {Definition.}
- **{Term 3}:** {Definition.}
- **{Term 4}:** {Definition.}

## 4) {Categories/Tiers}

| {Category} | {Attribute 1} | {Attribute 2} | {Attribute 3} |
|------------|---------------|---------------|---------------|
| {Type 1} | {Value} | {Value} | {Value} |
| {Type 2} | {Value} | {Value} | {Value} |
| {Type 3} | {Value} | {Value} | {Value} |

**Cross-category rule:** {Rule applying across categories.}

## 5) {Critical Section} (MANDATORY)

### 5.1 {Subsection A} Rules
- {Rule 1.}
- {Rule 2.}
- {Rule 3.}

### 5.2 {Subsection B} Rules
- {Rule 1.}
- {Rule 2.}
- {Rule 3.}

### 5.3 {Subsection C} Rules
- {Rule 1.}
- {Rule 2.}
- {Rule 3.}

### 5.4 Violation Response
1. {Immediate action.}
2. {Investigation.}
3. {Remediation.}
4. {Documentation.}
5. {Notification.}

## 6) {Standards Section}

### 6.1 {Standard Type}
```
{Pattern or format}

Examples:
  {Example 1}
  {Example 2}
```

- {Guideline 1.}
- {Guideline 2.}
- {Guideline 3.}

### 6.2 Required Documentation
Every {artifact} MUST have:
1. {Requirement 1.}
2. {Requirement 2.}
3. {Requirement 3.}

### 6.3 {Quality} Requirements
- {Requirement 1.}
- {Requirement 2.}
- {Requirement 3.}

## 7) Structure Standards

```
{root}/
├── {path1}                 # {Description}
├── {path2}                 # {Description}
├── {folder}/
│   ├── {subpath1}          # {Description}
│   └── {subpath2}          # {Description}
└── {folder2}/
    └── {subpath}           # {Description}
```

## 8) {Process} Rules

### 8.1 {Phase 1}
- {Rule 1.}
- {Rule 2.}

### 8.2 {Phase 2}
- {Requirement 1.}
- {Requirement 2.}

### 8.3 {Phase 3} Verification
Before marking complete:
- [ ] {Check 1}
- [ ] {Check 2}
- [ ] {Check 3}

## 9) Access Control

| Action | {Role 1} | {Role 2} | {Role 3} | {Role 4} |
|--------|----------|----------|----------|----------|
| {Action 1} | ✓ | ✓ | ✓ | ✓ |
| {Action 2} | — | ✓ | ✓ | ✓ |
| {Action 3} | — | — | ✓ | ✓ |
| {Action 4} | — | — | ✓ | ✓ |

## 10) Validation Protocol

### 10.1 {Stage 1} Checks
```bash
{validation command}
```

### 10.2 {Stage 2} Validation
```typescript
function validate(): void {
  // {validation logic}
}
```

### 10.3 {Stage 3} Verification
- {Verification 1.}
- {Verification 2.}

## 11) Prohibitions

- NO {prohibition 1}.
- NO {prohibition 2}.
- NO {prohibition 3}.
- NO {prohibition 4}.
- NO {prohibition 5}.

## 12) Exception Process

Exceptions require:
1. Written justification with risk assessment.
2. Approval from {Approver 1} AND {Approver 2}.
3. Time-bounded scope (max {N} days).
4. Documented remediation plan.
5. Entry in exception register.

## 13) Compliance Verification

| Check | Frequency | Owner | Evidence |
|-------|-----------|-------|----------|
| {Check 1} | {Frequency} | {Role} | {Evidence} |
| {Check 2} | {Frequency} | {Role} | {Evidence} |
| {Check 3} | {Frequency} | {Role} | {Evidence} |

## 14) Tags

| Tag | Purpose |
|-----|---------|
| `#{topic}` | Primary topic identifier |
| `#policy-{topic}` | Policy document reference |
| `#compliance-{level}` | Compliance classification |
| `#enforcement-{stage}` | When enforced |

## 15) Reward System

### 15.1 Compliance Rewards

| Achievement | Criteria | Reward | Eligible |
|-------------|----------|--------|----------|
| {Achievement 1} | {Criteria} | {Reward} | {Who} |
| {Achievement 2} | {Criteria} | {Reward} | {Who} |

### 15.2 Violation Consequences

| Violation Level | First | Repeat | Chronic |
|-----------------|-------|--------|---------|
| CRITICAL | {Consequence} | {Escalated} | {Severe} |
| HIGH | {Consequence} | {Escalated} | {Severe} |

---

## POLICY SUMMARY

```
STATUS: ACTIVE
VERSION: 1.0
LAST UPDATED: {YYYY-MM-DD}
OWNER: {Owner}
ENFORCEMENT: {Mechanisms}
VIOLATION CONSEQUENCE: {Consequence}
TAGS: #{topic}, #policy-{topic}, #compliance-{level}, #enforcement-{stage}
GOALS: ../01-goals/{topic}-goals.md
RULES: ../03-rules/{topic}-rule.md
LEDGER: ../goal-ledger.md#{topic}
```
