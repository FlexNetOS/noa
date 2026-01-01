# Database Backup Runbook

Backup SQLite database.

---

## Metadata

| Field | Value |
|-------|-------|
| **ID** | `database-backup` |
| **Trigger** | Daily schedule, before maintenance |
| **Impact** | Minimal (read operations continue) |
| **Owner** | Platform Team |
| **Escalation** | On-call SRE |
| **Schedule** | Daily at 02:00 UTC |
| **Last-Verified** | 2026-01-01 |

---

## Prerequisites

- [ ] Access to NOA host machine
- [ ] Sufficient disk space for backup
- [ ] Backup destination configured

---

## Steps

### 1. Check Database Status

```bash
# Verify database exists
ls -la ~/.noa/data/noa.db

# Check database integrity
sqlite3 ~/.noa/data/noa.db "PRAGMA integrity_check;"
```

### 2. Create Backup

```bash
# Create backup directory
mkdir -p ~/.noa/backups

# Generate backup filename
BACKUP_FILE="noa-$(date +%Y%m%d-%H%M%S).db"

# Backup using SQLite online backup
sqlite3 ~/.noa/data/noa.db ".backup ~/.noa/backups/$BACKUP_FILE"
```

### 3. Verify Backup

```bash
# Check backup exists and has content
ls -la ~/.noa/backups/$BACKUP_FILE

# Verify backup integrity
sqlite3 ~/.noa/backups/$BACKUP_FILE "PRAGMA integrity_check;"

# Verify row counts match
sqlite3 ~/.noa/data/noa.db "SELECT COUNT(*) FROM agents;"
sqlite3 ~/.noa/backups/$BACKUP_FILE "SELECT COUNT(*) FROM agents;"
```

### 4. Compress Backup

```bash
# Compress for storage
gzip ~/.noa/backups/$BACKUP_FILE

# Verify compressed file
ls -la ~/.noa/backups/$BACKUP_FILE.gz
```

### 5. Cleanup Old Backups

```bash
# Keep only last 7 days
find ~/.noa/backups -name "noa-*.db.gz" -mtime +7 -delete

# List remaining backups
ls -la ~/.noa/backups/
```

---

## Automated Backup

Add to crontab:

```bash
# Edit crontab
crontab -e

# Add daily backup at 2 AM
0 2 * * * /path/to/noa-backup.sh >> ~/.noa/logs/backup.log 2>&1
```

Or use NOA automation:

```bash
noa automation create \
  --name "daily-backup" \
  --schedule "0 2 * * *" \
  --action "database-backup"
```

---

## Restore Procedure

See [database-restore.md](database-restore.md) for restore steps.

Quick restore:

```bash
# Stop NOA
noa shutdown

# Restore from backup
gunzip -c ~/.noa/backups/noa-YYYYMMDD-HHMMSS.db.gz > ~/.noa/data/noa.db

# Start NOA
noa run
```

---

## Verification

- [ ] Backup file exists
- [ ] Integrity check passes
- [ ] Row counts match source
- [ ] Old backups cleaned up

---

## See Also

- [log-rotation.md](log-rotation.md) — Log management
- [system-shutdown.md](system-shutdown.md) — Pre-backup shutdown
