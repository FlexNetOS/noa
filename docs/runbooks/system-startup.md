# System Startup Runbook

Start all NOA services in correct order.

---

## Metadata

| Field | Value |
|-------|-------|
| **ID** | `system-startup` |
| **Trigger** | System boot, manual restart |
| **Impact** | All services unavailable until complete |
| **Owner** | Platform Team |
| **Escalation** | On-call SRE |
| **Last-Verified** | 2026-01-01 |

---

## Prerequisites

- [ ] Access to NOA host machine
- [ ] `noa` CLI installed
- [ ] Configuration files present in `~/.noa/config/`
- [ ] SQLite database initialized

---

## Steps

### 1. Verify Environment

```bash
# Check NOA installation
noa --version

# Verify config exists
ls -la ~/.noa/config/

# Check database
ls -la ~/.noa/data/noa.db
```

### 2. Start Core Services

```bash
# Start in foreground (development)
noa run

# Start as daemon (production)
noa run --daemon
```

### 3. Verify Health

```bash
# Health check
curl http://localhost:8080/health

# Expected response
{"status":"healthy","services":{"db":"ok","agents":"ok"}}
```

### 4. Verify Agents

```bash
# List running agents
noa agent list

# Expected output
ID          KIND            STATUS
commander   CommanderChief  Running
file-io     FileIO          Idle
terminal    Terminal        Idle
```

---

## Verification

- [ ] Health endpoint returns `healthy`
- [ ] All core agents are listed
- [ ] No errors in logs: `tail -f ~/.noa/logs/noa.log`

---

## Rollback

If startup fails:

1. Check logs: `tail -100 ~/.noa/logs/noa.log`
2. Verify config: `noa config validate`
3. Restart with debug: `RUST_LOG=debug noa run`

---

## See Also

- [system-shutdown.md](system-shutdown.md) — Graceful shutdown
- [build-failure.md](build-failure.md) — Build issues
