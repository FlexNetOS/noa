# Log Rotation Runbook

Rotate and archive NOA logs.

---

## Metadata

| Field | Value |
|-------|-------|
| **ID** | `log-rotation` |
| **Trigger** | Weekly schedule, disk space alert |
| **Impact** | None (log writing continues) |
| **Owner** | Platform Team |
| **Escalation** | On-call SRE |
| **Schedule** | Weekly on Sunday at 03:00 UTC |
| **Last-Verified** | 2026-01-01 |

---

## Prerequisites

- [ ] Access to NOA host machine
- [ ] `logrotate` installed (Linux) or equivalent

---

## Log Locations

| Log | Path | Retention |
|-----|------|-----------|
| Main | `~/.noa/logs/noa.log` | 30 days |
| Agent | `~/.noa/logs/agents/*.log` | 14 days |
| API | `~/.noa/logs/api.log` | 7 days |
| Debug | `~/.noa/logs/debug.log` | 3 days |

---

## Manual Rotation

### 1. Rotate Main Log

```bash
# Rename current log
mv ~/.noa/logs/noa.log ~/.noa/logs/noa.log.$(date +%Y%m%d)

# Signal NOA to reopen log file
kill -HUP $(cat ~/.noa/noa.pid)

# Compress old log
gzip ~/.noa/logs/noa.log.$(date +%Y%m%d)
```

### 2. Rotate Agent Logs

```bash
# Rotate all agent logs
for log in ~/.noa/logs/agents/*.log; do
  mv "$log" "$log.$(date +%Y%m%d)"
  gzip "$log.$(date +%Y%m%d)"
done

# Signal agents to reopen logs
noa agent reload --all
```

### 3. Cleanup Old Logs

```bash
# Remove logs older than retention
find ~/.noa/logs -name "*.log.*.gz" -mtime +30 -delete
find ~/.noa/logs/agents -name "*.log.*.gz" -mtime +14 -delete
```

---

## Automated Rotation (Linux)

Create `/etc/logrotate.d/noa`:

```
/home/*/.noa/logs/noa.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    postrotate
        /bin/kill -HUP $(cat /home/*/.noa/noa.pid 2>/dev/null) 2>/dev/null || true
    endscript
}

/home/*/.noa/logs/agents/*.log {
    weekly
    rotate 2
    compress
    missingok
    notifempty
}
```

---

## Automated Rotation (Windows)

PowerShell script:

```powershell
# noa-log-rotate.ps1
$LogDir = "$env:USERPROFILE\.noa\logs"
$RetentionDays = 30

# Compress logs older than 1 day
Get-ChildItem "$LogDir\*.log" | Where-Object { 
    $_.LastWriteTime -lt (Get-Date).AddDays(-1) 
} | ForEach-Object {
    Compress-Archive -Path $_.FullName -DestinationPath "$($_.FullName).zip"
    Remove-Item $_.FullName
}

# Delete archives older than retention
Get-ChildItem "$LogDir\*.zip" | Where-Object { 
    $_.LastWriteTime -lt (Get-Date).AddDays(-$RetentionDays) 
} | Remove-Item
```

---

## Verification

- [ ] Current log file is active and writable
- [ ] Old logs are compressed
- [ ] Logs beyond retention are deleted
- [ ] Disk space recovered

---

## See Also

- [database-backup.md](database-backup.md) — Database backup
