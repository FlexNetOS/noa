# Agent Failure Runbook

Handle agent execution failures.

---

## Metadata

| Field | Value |
|-------|-------|
| **ID** | `agent-failure` |
| **Trigger** | Agent crash, task timeout, error response |
| **Impact** | Task not completed, user blocked |
| **Owner** | Platform Team |
| **Escalation** | On-call SRE |
| **Severity** | S2 |
| **Last-Verified** | 2026-01-01 |

---

## Prerequisites

- [ ] Access to NOA logs
- [ ] `noa` CLI access
- [ ] Agent ID or task ID

---

## Triage

### 1. Identify Failure Type

```bash
# Get agent status
noa agent status <agent-id>

# Get task details
noa task show <task-id>
```

| Status | Meaning | Action |
|--------|---------|--------|
| `Failed` | Agent crashed | Restart agent |
| `Timeout` | Task took too long | Increase timeout or optimize |
| `Rejected` | Policy violation | Check governance |
| `Paused` | Manually paused | Resume if appropriate |

### 2. Check Logs

```bash
# Agent logs
tail -100 ~/.noa/logs/agents/<agent-id>.log

# System logs
grep "ERROR\|WARN" ~/.noa/logs/noa.log | tail -50
```

---

## Common Issues

### Agent Crash

```bash
# Restart specific agent
noa agent restart <agent-id>

# Restart all agents
noa agent restart --all
```

### Task Timeout

```bash
# Increase timeout
noa config set agents.timeout_seconds 600

# Retry task
noa task retry <task-id>
```

### Policy Violation

```bash
# Check policy decision
noa governance check <agent-id> <action>

# View policies
noa governance list
```

### Resource Exhaustion

```bash
# Check memory usage
noa status --verbose

# Increase limits
noa config set agents.max_memory_mb 2048
```

---

## Recovery Steps

### 1. Stop Failed Agent

```bash
noa agent stop <agent-id>
```

### 2. Clear State (if needed)

```bash
# Clear agent state
noa agent reset <agent-id>

# Clear task queue
noa task clear --agent <agent-id>
```

### 3. Restart Agent

```bash
noa agent start <agent-id>
```

### 4. Verify Recovery

```bash
# Test with simple task
noa task run --agent <agent-id> --test

# Check status
noa agent status <agent-id>
```

---

## Verification

- [ ] Agent status is `Idle` or `Running`
- [ ] No errors in recent logs
- [ ] Test task completes successfully
- [ ] User can submit tasks

---

## Escalation

If issue persists:

1. Collect logs: `noa debug collect`
2. Check system resources: `noa status --all`
3. Escalate to on-call SRE

---

## See Also

- [build-failure.md](build-failure.md) — Build issues
- [system-startup.md](system-startup.md) — Restart services
