# Signal Reference Guide

## Complete Signal System for PRP Orchestrator

### Core Architecture Principles

1. **Signals are ALWAYS `[XX]` format** - Two letters in square brackets
2. **Scanner emits EVENTS only** - Processes signals with guidelines
3. **Inspector CLASSIFIES only** - Never initiates actions
4. **Orchestrator PROCESSES only** - Never emits signals
5. **User interactions are DIRECT** - No signal overhead
6. **Ephemeral cycle system** - `[HF]` initiates all orchestration

---

## System Signals (Critical Infrastructure)

### `[HF]` - Health Feedback (Ephemeral)
- **Source**: System Monitor → Orchestrator
- **When**: Every orchestration cycle start
- **Priority**: 1 (Highest)
- **Data**: Current system status, active PRPs, agent states, resource usage
- **Resolution**: Extract priorities, select highest priority task, execute
- **Ephemeral**: Yes - Not stored, generated each cycle

### `[AS]` - User Signal (Direct)
- **Source**: User → Orchestrator
- **When**: Direct user input/commands
- **Priority**: 2
- **Data**: User message/command
- **Resolution**: Process user command directly in current cycle context
- **Special**: No signal emission - direct processing

### `[AE]` - Emergency
- **Source**: System → Admin
- **When**: Critical failures, security incidents
- **Priority**: 3
- **Data**: Emergency details, impact assessment
- **Resolution**: Nudge tool → Immediate user response required
- **Follow-up**: Crisis management procedures

### `[AD]` - Decision
- **Source**: System → Admin
- **When**: Critical architectural/strategic decisions needed
- **Priority**: 4
- **Data**: Decision context, options, recommendations
- **Resolution**: Nudge tool → Admin guidance required
- **Follow-up**: Continue with admin direction

### `[AA]` - Critical
- **Source**: System → Admin
- **When**: System-wide issues requiring immediate attention
- **Priority**: 5
- **Data**: Critical issue details, system impact
- **Resolution**: Nudge tool → Urgent action required
- **Follow-up**: System recovery procedures

### `[FATAL]` - System Fatal Error
- **Source**: Scanner/Inspector → Orchestrator
- **When**: System corruption, unrecoverable errors
- **Priority**: 1 (System critical)
- **Data**: Error details, corruption scope
- **Resolution**: Immediate cycle termination
- **Special**: Signal lost after storage sync - prevents cascade failures

---

## Development Cycle Signals

### Agent Work Progress Signals

#### `[Bb]` - Blocker
- **Source**: Robo-Agent → Inspector
- **When**: Technical dependency/configuration blocks progress
- **Priority**: 4 (High)
- **Data**: Blocker details, context, impact assessment
- **Resolution Path**:
  1. Inspector classifies blocker severity
  2. Orchestrator asks agent to write details to PRP
  3. Agent writes `[Bb] Detailed blocker description` to PRP
  4. Next cycle: Prioritize unblocking task
- **PRP Context**: Blocker becomes part of PRP record

#### `[Cc]` - Complete
- **Source**: Robo-Agent → Inspector
- **When**: Task or phase successfully completed
- **Priority**: 5 (Medium)
- **Data**: Completion details, results, artifacts
- **Resolution Path**:
  1. Inspector validates completion criteria
  2. Update PRP progress and status
  3. Select next task or mark PRP phase complete
- **PRP Context**: Marks task completion in PRP progress log

#### `[crash]` - Crash
- **Source**: Agent Process → Inspector
- **When**: Agent process termination/failure
- **Priority**: 4 (High)
- **Data**: Crash details, error context, last activity
- **Resolution Path**:
  1. Inspector detects process failure
  2. Orchestrator logs failure, updates agent status
  3. Next cycle: Assess respawnability or task reassignment
- **PRP Context**: Crash recorded in PRP if relevant

#### `[idle]` - Idle
- **Source**: Agent Process → Inspector
- **When**: Agent inactivity timeout detected
- **Priority**: 6 (Low)
- **Data**: Idle duration, last activity, current task state
- **Resolution Path**:
  1. Inspector detects prolonged inactivity
  2. Orchestrator reassigns agent to available task
  3. Optimize resource allocation
- **PRP Context**: Usually not recorded unless affects deliverables

### Quality Assurance Signals

#### `[Do]` - Definition of Done
- **Source**: Robo-AQA → Inspector
- **When**: DoD criteria met or validation needed
- **Priority**: 5 (Medium)
- **Data**: DoD checklist status, validation results
- **Resolution Path**:
  1. Inspector validates all DoD criteria met
  2. Move PRP to review phase
  3. Prepare for release considerations
- **PRP Context**: DoD completion marked in PRP status

#### `[Dd]` - Definition of Ready
- **Source**: Robbo-System-Analyst → Inspector
- **When**: DoR criteria met or requirements validation
- **Priority**: 5 (Medium)
- **Data**: DoR checklist status, readiness assessment
- **Resolution Path**:
  1. Inspector validates all DoR criteria met
  2. Move PRP to implementation phase
  3. Assign to appropriate agent
- **PRP Context**: DoR completion marked in PRP status

#### `[Tt]` - Test Verification
- **Source**: Robo-AQA → Inspector
- **When**: Tests ready for verification
- **Priority**: 6 (Low)
- **Data**: Test coverage results, quality metrics
- **Resolution Path**:
  1. Inspector analyzes test completeness
  2. Verify 100% PRP requirement coverage
  3. Identify gaps or improvements needed
- **PRP Context**: Test verification status recorded

### Workflow Integration Signals

#### `[Pr]` - Pull Request
- **Source**: Git System → Inspector
- **When**: PR created, updated, status changed
- **Priority**: 6 (Low)
- **Data**: PR details, changes, reviews, CI status
- **Resolution Path**:
  1. Inspector processes with PR guideline
  2. Analyze PR status, changes, reviews
  3. Determine next actions (approve, request changes, comment)
  4. Execute actions via GitHub tools
- **PRP Context**: PR workflow integration

---

## Admin Communication Signals

### `[af]` - Feedback Request
- **Source**: Orchestrator → Admin
- **When**: Decision needed on design, approach, implementation
- **Priority**: 4 (High)
- **Data**: Decision context, options, recommendations
- **Admin Action**: Provide direction to continue work
- **Follow-up**: Continue with admin guidance
- **Tool**: Nudge tool with reply expectation

### `[AE]` - Emergency (Admin)
- **Source**: System → Admin
- **When**: Critical failures, security incidents
- **Priority**: 3 (Very High)
- **Data**: Emergency details, impact assessment
- **Admin Action**: Immediate intervention required
- **Follow-up**: Crisis management procedures
- **Tool**: Nudge tool with urgent priority

### `[AD]` - Decision (Admin)
- **Source**: System → Admin
- **When**: Architectural choices, strategic decisions needed
- **Priority**: 4 (High)
- **Data**: Decision context, options, analysis
- **Admin Action**: Provide direction for critical choices
- **Follow-up**: Architectural decision implementation
- **Tool**: Nudge tool with decision context

### `[AA]` - Critical (Admin)
- **Source**: System → Admin
- **When**: System-wide issues requiring immediate attention
- **Priority**: 5 (Medium)
- **Data**: Critical issue details, system impact
- **Admin Action**: Urgent response needed
- **Follow-up**: System recovery procedures
- **Tool**: Nudge tool with critical alert

---

## Signal Resolution Scenarios

### Scenario 1: Normal Development Cycle
```
[HF] → Extract priorities → Select PRP task → Checkout worktree/branch
    ↓
Spawn robo-developer → Agent works → [Cc] completion signal
    ↓
Inspector validates → Update PRP → Next [HF] cycle
```

### Scenario 2: Blocker Resolution
```
Agent encounters issue → Writes [Bb] to PRP
    ↓
Scanner detects → Inspector processes (skip duplicate)
    ↓
Next [HF]: Prioritize unblocking → Spawn specialized agent
    ↓
Resolve blocker → Continue normal development cycle
```

### Scenario 3: User Intervention
```
User sends command → Direct to Orchestrator (NO signal)
    ↓
Process in current cycle context
    ↓
If decision needed: [af] → Nudge tool → Admin
    ↓
Admin responds → Direct to Orchestrator (NO signal)
    ↓
Continue with updated instructions
```

### Scenario 4: System Recovery
```
[FATAL] detected → Immediate cycle termination
    ↓
System corruption → Signal lost after storage sync
    ↓
Manual recovery → User restarts system
    ↓
New [HF] cycle → Assess damage → Recovery actions
```

### Scenario 5: Quality Gates
```
[Do] signal → Inspector validates DoD criteria
    ↓
Move to review phase → Prepare for release
    ↓
[Pr] signal → PR workflow processing
    ↓
Execute actions via GitHub tools → Monitor progress
```

---

## Signal Priority Matrix

| Priority | Signals | Processing Time | Response Required |
|----------|---------|-----------------|-------------------|
| 1 | `[FATAL]`, `[HF]` | Immediate | System action |
| 2 | `[AS]` | Immediate | Direct processing |
| 3 | `[AE]` | < 1 minute | Admin emergency |
| 4 | `[AD]`, `[Bb]`, `[crash]` | < 5 minutes | Admin decision / System action |
| 5 | `[AA]`, `[Cc]`, `[Do]`, `[Dd]` | < 15 minutes | Admin action / System processing |
| 6 | `[Pr]`, `[Tt]`, `[idle]` | < 30 minutes | Routine processing |

---

## Implementation Guidelines

### For Scanner
- Emit EVENTS only, not signals
- Process signal patterns with guidelines
- Update PRP CLI storage sync
- Handle special guideline adapters (PR, CI, terminal)

### For Inspector
- FIFO queue processing only
- Classify signals, never initiate actions
- Early prediction: skip duplicate signal processing
- Use guidelines for data gathering and LLM processing

### For Orchestrator
- Process signals by priority only
- Never emit signals
- Use tools for actions and communications
- Work within PRP context and memory
- Handle user interactions directly

### For Robo-Agents
- Emit signals only within PRP context
- Work on assigned tasks within PRP scope
- Signal completion, blockers, failures appropriately
- Follow signal guidelines for communication

### For User Interactions
- Direct to Orchestrator, no signal overhead
- Process in current cycle context
- Critical commands interrupt current tasks
- Non-critical queued for next cycle

---

## Signal Anti-Patterns

### DO NOT:
- Emit signals for internal processes
- Use Orchestrator to emit signals
- Create signals outside `[XX]` format
- Handle user input via signals
- Store ephemeral signals persistently
- Process duplicate signals multiple times

### ALWAYS:
- Use signals for exceptional coordination events
- Follow established signal flow patterns
- Maintain signal priority processing order
- Handle fatal signals gracefully (lose them)
- Work within PRP context for all signal data
- Use tools for actions, not signals