# NOA Runbooks

Operational runbooks for common procedures and incident response.

## Available Runbooks

### System Operations

| Runbook | Purpose | Trigger |
|---------|---------|---------|
| [system-startup.md](system-startup.md) | Start all NOA services | System boot |
| [system-shutdown.md](system-shutdown.md) | Graceful shutdown | Maintenance |

### Incident Response

| Runbook | Purpose | Severity |
|---------|---------|----------|
| [build-failure.md](build-failure.md) | Handle build failures | S2 |
| [agent-failure.md](agent-failure.md) | Agent execution failures | S2 |

### Maintenance

| Runbook | Purpose | Schedule |
|---------|---------|----------|
| [database-backup.md](database-backup.md) | Backup SQLite database | Daily |
| [log-rotation.md](log-rotation.md) | Rotate and archive logs | Weekly |

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
