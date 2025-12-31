#!/bin/bash

# Wiki.js PostgreSQL Backup Script
# This script creates automated backups of the Wiki.js database

set -euo pipefail

# Configuration
BACKUP_DIR="${BACKUP_DIR:-/backups}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
DB_HOST="${DB_HOST:-db}"
DB_PORT="${DB_PORT:-5432}"
DB_NAME="${DB_NAME:-wikijs}"
DB_USER="${DB_USER:-wikijs}"
RETENTION_DAYS="${RETENTION_DAYS:-30}"

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Log function
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*" | tee -a "$BACKUP_DIR/backup.log"
}

# Health check
check_database_health() {
    log "Checking database health..."
    if pg_isready -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME"; then
        log "Database is healthy"
        return 0
    else
        log "ERROR: Database is not healthy"
        return 1
    fi
}

# Create backup
create_backup() {
    local backup_file="$BACKUP_DIR/wikijs_backup_$TIMESTAMP.sql"
    local compressed_file="$backup_file.gz"

    log "Starting database backup..."

    # Create backup
    if pg_dump -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" > "$backup_file"; then
        log "Backup created successfully: $backup_file"

        # Compress backup
        if gzip "$backup_file"; then
            log "Backup compressed successfully: $compressed_file"

            # Calculate checksum
            sha256sum "$compressed_file" > "$compressed_file.sha256"
            log "Checksum created: $compressed_file.sha256"

            # Verify backup integrity
            if gzip -t "$compressed_file"; then
                log "Backup integrity verified"
                echo "$compressed_file"
            else
                log "ERROR: Backup integrity check failed"
                rm -f "$compressed_file" "$compressed_file.sha256"
                return 1
            fi
        else
            log "ERROR: Failed to compress backup"
            rm -f "$backup_file"
            return 1
        fi
    else
        log "ERROR: Failed to create backup"
        return 1
    fi
}

# Clean old backups
cleanup_old_backups() {
    log "Cleaning up backups older than $RETENTION_DAYS days..."

    # Remove old backup files
    find "$BACKUP_DIR" -name "wikijs_backup_*.sql.gz" -mtime +$RETENTION_DAYS -delete
    find "$BACKUP_DIR" -name "wikijs_backup_*.sql.gz.sha256" -mtime +$RETENTION_DAYS -delete

    # Clean old log files (keep last 7 days)
    find "$BACKUP_DIR" -name "backup.log" -mtime +7 -delete

    log "Cleanup completed"
}

# Generate backup report
generate_report() {
    local backup_count=$(find "$BACKUP_DIR" -name "wikijs_backup_*.sql.gz" | wc -l)
    local total_size=$(du -sh "$BACKUP_DIR"/*.gz 2>/dev/null | awk '{sum+=$1} END {print sum "B"}' || echo "0B")

    log "Backup Report:"
    log "  Total backups: $backup_count"
    log "  Total size: $total_size"
    log "  Retention period: $RETENTION_DAYS days"
}

# Main backup function
main() {
    log "Starting Wiki.js backup process..."

    # Check database health first
    if ! check_database_health; then
        log "CRITICAL: Database health check failed. Backup aborted."
        exit 1
    fi

    # Create backup
    local backup_file
    if backup_file=$(create_backup); then
        log "SUCCESS: Backup completed successfully: $backup_file"

        # Clean old backups
        cleanup_old_backups

        # Generate report
        generate_report

        log "Backup process completed successfully"
    else
        log "CRITICAL: Backup process failed"
        exit 1
    fi
}

# Handle signals
trap 'log "Backup process interrupted"; exit 1' INT TERM

# Run main function
main "$@"