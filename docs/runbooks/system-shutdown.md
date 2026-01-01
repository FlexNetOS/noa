# System Shutdown Runbook

Gracefully stop all NOA services.

---

## Metadata

| Field | Value |
|-------|-------|
| **ID** | `system-shutdown` |
| **Trigger** | Maintenance, updates, incidents |
| **Impact** | All services become unavailable |
| **Owner** | Platform Team |
| **Escalation** | On-call SRE |
| **Last-Verified** | 2026-01-01 |

---

## Prerequisites

- [ ] Access to NOA host machine
- [ ] Running NOA instance
- [ ] No critical tasks in progress (check first)

---

## Steps

### 1. Check Running Tasks

```bash
# List active tasks
noa task list --status running

# If critical tasks are running, wait or cancel
noa task cancel <task-id>
```

### 2. Notify Dependents

```bash
# Optional: Notify connected clients
noa broadcast --message "System shutdown in 5 minutes"
```

### 3. Graceful Shutdown

```bash
# Signal graceful shutdown
noa shutdown

# Or send SIGTERM to daemon
kill -TERM $(cat ~/.noa/noa.pid)
```

### 4. Verify Shutdown

```bash
# Check process is gone
pgrep -f "noa run"

# Check port is free
lsof -i :8080
```

### 5. (Optional) Force Kill

Only if graceful shutdown fails:

```bash
# Force kill
kill -9 $(cat ~/.noa/noa.pid)

# Clean up stale files
rm -f ~/.noa/noa.pid ~/.noa/noa.sock
```

---

## Verification

- [ ] No NOA processes running
- [ ] Port 8080 is free
- [ ] PID file removed
- [ ] Clean shutdown in logs

---

## Post-Shutdown

1. **Backup database** (if needed):
   ```bash
   cp ~/.noa/data/noa.db ~/.noa/backups/noa-$(date +%Y%m%d).db
   ```

2. **Archive logs** (if needed):
   ```bash
   gzip ~/.noa/logs/noa.log
   ```

---

## See Also

- [system-startup.md](system-startup.md) — Start services
- [database-backup.md](database-backup.md) — Backup procedure
