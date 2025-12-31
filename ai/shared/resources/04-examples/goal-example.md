# {topic}-goals.md

> **Template Instructions:** Replace `{topic}` with your subject area (e.g., `env`, `auth`, `logging`, `testing`). Delete this instruction block before finalizing.

> Strategic objectives for {topic description}. Define what success looks like before defining how to achieve it.

## 1) Primary Goals

> **Ledger Requirement:** All primary goals MUST be registered in [goal-ledger.md](../goal-ledger.md) with a link back to this document.

### 1.1 {Goal Category 1}
- **{Objective A}:** {Description of what must be achieved and why it matters.}
- **{Objective B}:** {Another key objective with measurable outcome.}
- **{Objective C}:** {Third objective supporting the category.}

### 1.2 {Goal Category 2}
- **{Objective A}:** {Description with clear success criteria.}
- **{Objective B}:** {Complementary objective.}

### 1.3 {Goal Category 3}
- **{Objective A}:** {Developer/user-facing objective.}
- **{Objective B}:** {Operational objective.}

### 1.4 {Goal Category 4}
- **{Objective A}:** {Resilience or sustainability objective.}
- **{Objective B}:** {Long-term maintenance objective.}

## 2) Non-Goals (Explicit Exclusions)

> List things that are explicitly OUT OF SCOPE to prevent scope creep and set clear boundaries.

- **{Exclusion 1}:** {Why this is not a goal for this topic.}
- **{Exclusion 2}:** {Future scope or separate concern.}
- **{Exclusion 3}:** {Anti-pattern or approach to avoid.}

## 3) Constraints

> Hard limits that cannot be violated. These shape what solutions are acceptable.

| Constraint | Type | Rationale |
|------------|------|-----------|
| {Constraint 1} | {Technical/Budget/Time/Legal/Resource} | {Why this limit exists} |
| {Constraint 2} | {Type} | {Rationale} |
| {Constraint 3} | {Type} | {Rationale} |

**Constraint Types:**
- **Technical:** Platform, language, or architecture limitations
- **Budget:** Cost ceilings or resource allocations
- **Time:** Deadlines or scheduling boundaries
- **Legal:** Compliance, licensing, or regulatory requirements
- **Resource:** Team capacity or skill availability

## 4) Success Metrics

> Define measurable outcomes. Each metric should be specific, measurable, and have a clear target.

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| {Metric 1} | {Target value} | {How it's measured} |
| {Metric 2} | {Target value} | {How it's measured} |
| {Metric 3} | {Target value} | {How it's measured} |
| {Metric 4} | {Target value} | {How it's measured} |
| {Metric 5} | {Target value} | {How it's measured} |

## 5) Stakeholders and Responsibilities

> Identify who is accountable for achieving these goals.

| Role | Responsibility |
|------|----------------|
| {Role 1} | {What they must do to support these goals} |
| {Role 2} | {Their specific responsibilities} |
| {Role 3} | {Their specific responsibilities} |
| {Role 4} | {Their specific responsibilities} |

## 6) Alignment with Universal Policy

> How these goals connect to the Universal Task Execution Policy.

- **Evidence rule:** {How claims related to this topic require evidence.}
- **Truth gate:** {What "complete/ready" means for this topic.}
- **Verification:** {How changes undergo triple-verification.}

## 7) Dependencies and Prerequisites

> What must be in place before these goals can be achieved.

### 7.1 Current Inventory

> What already exists that supports these goals.

| Asset | Status | Location | Notes |
|-------|--------|----------|-------|
| {Existing asset 1} | {Ready/Partial/Needs Update} | {Path or link} | {Current state} |
| {Existing asset 2} | {Status} | {Location} | {Notes} |
| {Existing asset 3} | {Status} | {Location} | {Notes} |

### 7.2 Needs

> What must be created, acquired, or resolved before proceeding.

| Need | Priority | Blocker? | Owner | Target Date |
|------|----------|----------|-------|-------------|
| {Need 1} | {Critical/High/Medium/Low} | {Yes/No} | {Who} | {Date} |
| {Need 2} | {Priority} | {Blocker?} | {Owner} | {Date} |
| {Need 3} | {Priority} | {Blocker?} | {Owner} | {Date} |

**Blocking Needs:** Must be resolved before any work begins.
**Non-Blocking Needs:** Can be addressed in parallel with goal work.

## 8) Associated Policy

> Link to the policy document that governs how these goals are achieved.

- **Policy Document:** [{topic}-policy.md](../02-policy/{topic}-policy.md)
- **Key Sections:**
  - §{N} - {Section name relevant to goal 1}
  - §{N} - {Section name relevant to goal 2}
  - §{N} - {Section name relevant to goal 3}

## 9) Associated Rules

> Link to the rules document that enforces these goals.

- **Rules Document:** [{topic}-rule.md](../03-rules/{topic}-rule.md)
- **Key Rules:**
  - {TOPIC}-001: {Rule name} — enforces {goal reference}
  - {TOPIC}-002: {Rule name} — enforces {goal reference}
  - {TOPIC}-003: {Rule name} — enforces {goal reference}

## 10) Tags

> **Tag Creation Requirement:** Every goals document MUST define tags for tracking and cross-referencing.

### 10.1 Required Tags

| Tag | Purpose | Usage |
|-----|---------|-------|
| `#{topic}` | Primary topic identifier | All related artifacts |
| `#goal-{topic}` | Goal document reference | Link to this document |
| `#priority-{level}` | Priority classification | {critical/high/medium/low} |
| `#status-{state}` | Current status | {draft/active/achieved/deprecated} |

### 10.2 Optional Tags

- `#milestone-{name}` — Associate with specific milestone
- `#stakeholder-{role}` — Associate with responsible party
- `#constraint-{type}` — Reference specific constraints
- `#dependency-{name}` — Track dependency relationships

## 11) Reward System

> Incentives for achieving goals and milestones.

### 11.1 Achievement Rewards

| Achievement | Criteria | Reward | Eligible |
|-------------|----------|--------|----------|
| {Achievement 1} | {What must be done} | {Reward type and value} | {Who can earn} |
| {Achievement 2} | {Criteria} | {Reward} | {Eligible} |
| {Achievement 3} | {Criteria} | {Reward} | {Eligible} |

### 11.2 Milestone Bonuses

| Milestone | Bonus Condition | Bonus |
|-----------|-----------------|-------|
| {Milestone 1} | Completed by {date} or under {budget} | {Bonus reward} |
| {Milestone 2} | {Condition} | {Bonus} |

### 11.3 Reward Types

- **Recognition:** Public acknowledgment, certificates, badges
- **Development:** Training, conference attendance, learning budget
- **Time:** Extra PTO, flex time, reduced on-call
- **Monetary:** Bonus, gift cards, team celebration budget
- **Choice:** Winner selects from reward menu

## 12) Timeline and Milestones

> Define phases or milestones for goal achievement.

| Phase | Milestone | Target Date | Reward Trigger |
|-------|-----------|-------------|----------------|
| Phase 1 | {Initial capability} | {Date} | {Yes/No} |
| Phase 2 | {Expanded capability} | {Date} | {Yes/No} |
| Phase 3 | {Full capability} | {Date} | {Yes/No} |

---

## SUMMARY

```
GOAL: {One-line description of the goal set}
SCOPE: {What/who this applies to}
PRIORITY: {Priority hierarchy, e.g., Security > Consistency > UX > Performance}
NORTH STAR: {The ultimate success indicator}
TAGS: #{topic}, #goal-{topic}, #priority-{level}, #status-{state}
POLICY: ../02-policy/{topic}-policy.md
RULES: ../03-rules/{topic}-rule.md
LEDGER: ../goal-ledger.md#{topic}
```

---

## Template Checklist

Before finalizing this goals document, verify:

- [ ] All `{placeholders}` replaced with actual content
- [ ] Primary goals are strategic (what), not tactical (how)
- [ ] Primary goals registered in [goal-ledger.md](../goal-ledger.md)
- [ ] Constraints documented with types and rationale
- [ ] Non-goals clearly exclude out-of-scope items
- [ ] Success metrics are measurable and have targets
- [ ] Stakeholders and responsibilities are assigned
- [ ] Alignment with Universal Policy is documented
- [ ] Current inventory assessed
- [ ] Needs identified with owners and priorities
- [ ] Associated policy linked with key sections
- [ ] Associated rules linked with enforcement mapping
- [ ] Tags created and documented
- [ ] Reward system defined with criteria
- [ ] Summary accurately captures the essence
