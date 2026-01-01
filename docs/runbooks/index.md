# NOA Runbooks

Operational runbooks for common procedures and incident response.

## Available Runbooks

### System Operations

| Runbook | Purpose | Trigger |
|---------|---------|---------|
| [system-startup](system-startup.md) | Start all NOA services | System boot |
| [system-shutdown](system-shutdown.md) | Graceful shutdown | Maintenance |

### Incident Response

| Runbook | Purpose | Severity |
|---------|---------|----------|
| [build-failure](build-failure.md) | Handle build failures | S2 |
| [agent-failure](agent-failure.md) | Agent execution failures | S2 |
| [p2p-connectivity](p2p-connectivity.md) | P2P network issues | S2 |

### Maintenance

| Runbook | Purpose | Schedule |
|---------|---------|----------|
| [database-backup](database-backup.md) | Backup SQLite database | Daily |
| [log-rotation](log-rotation.md) | Rotate and archive logs | Weekly |
| [cache-cleanup](cache-cleanup.md) | Clean model/pip caches | Monthly |

---

## Runbook Format

All runbooks follow this structure:

```yaml
id: string          # Unique identifier
title: string       # Human-readable title
triggers:           # What initiates this runbook
  - signal: string
    severity: S1|S2|S3|S4
prerequisites:      # Required capabilities/tools
  - capability: string
steps:              # Ordered procedure steps
  - kind: COMMAND|CHECK|LINK|NOTE
    value: string
verification:       # Last validation info
  last_dry_run: timestamp
  owner: string
```

---

*See [architecture/policy/wiki-pages-runbook.md](../architecture/policy/wiki-pages-runbook.md) for runbook governance.*
