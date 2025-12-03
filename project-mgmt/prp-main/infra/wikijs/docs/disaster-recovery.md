# Wiki.js Disaster Recovery Procedures

**Purpose**: This document outlines comprehensive disaster recovery procedures for Wiki.js deployments to ensure business continuity and minimize downtime.

**Last Updated**: 2025-11-03
**Version**: 1.0
**Author**: Robo-DevOps/SRE

---

## Table of Contents
1. [Recovery Objectives](#recovery-objectives)
2. [Backup Strategy](#backup-strategy)
3. [Recovery Procedures](#recovery-procedures)
4. [Failover Scenarios](#failover-scenarios)
5. [Testing and Validation](#testing-and-validation)
6. [Contact and Escalation](#contact-and-escalation)

---

## Recovery Objectives

### Recovery Time Objective (RTO)
- **Critical Services**: 4 hours maximum
- **Non-Critical Services**: 24 hours maximum
- **Full System Recovery**: 8 hours maximum

### Recovery Point Objective (RPO)
- **Data Loss**: Maximum 1 hour of data loss
- **Configuration Changes**: Zero loss (real-time replication)
- **User Content**: Maximum 15 minutes of data loss

### Service Level Objectives
- **Availability**: 99.9% (8.76 hours downtime per year)
- **Data Integrity**: 100% verification required
- **Recovery Success Rate**: 95% or higher

---

## Backup Strategy

### Automated Backups

#### Database Backups
```bash
# Daily full backup (2 AM)
0 2 * * * /scripts/backup-db.sh

# Hourly incremental backup
0 * * * * /scripts/backup-incremental.sh

# Transaction log backup (every 15 minutes)
*/15 * * * * /scripts/backup-transaction-logs.sh
```

#### Application Backups
- **Configuration Files**: Every 6 hours
- **User Uploads**: Daily at 3 AM
- **Git Repository**: Real-time replication
- **SSL Certificates**: Weekly

#### Backup Storage
- **Local Storage**: 7 days retention
- **Regional Cloud Storage**: 30 days retention
- **Cross-Region Cloud Storage**: 90 days retention
- **Offsite Tape Backup**: 1 year retention (quarterly)

### Backup Verification

#### Automated Verification
```bash
#!/bin/bash
# backup-verification.sh
# Verify backup integrity and completeness

verify_backup() {
    local backup_file=$1

    # Check file exists
    if [[ ! -f "$backup_file" ]]; then
        echo "ERROR: Backup file $backup_file not found"
        return 1
    fi

    # Verify checksum
    if ! sha256sum -c "${backup_file}.sha256"; then
        echo "ERROR: Backup integrity check failed"
        return 1
    fi

    # Test restore to temporary location
    if ! psql -h localhost -U wikijs -d wikijs_test < "$backup_file"; then
        echo "ERROR: Backup restore test failed"
        return 1
    fi

    echo "SUCCESS: Backup verification completed"
    return 0
}
```

#### Manual Verification
- Weekly restore tests to staging environment
- Monthly full disaster recovery drill
- Quarterly cross-region restore verification

---

## Recovery Procedures

### Scenario 1: Database Corruption

#### Severity: HIGH
#### Impact: Complete loss of database functionality
#### Recovery Time: 2-4 hours

**Steps:**
1. **Immediate Response (0-15 minutes)**
   ```bash
   # Stop Wiki.js application
   kubectl scale deployment wikijs --replicas=0 -n wikijs

   # Isolate affected database
   kubectl get pods -n wikijs -l app=wikijs,component=database
   ```

2. **Assessment (15-30 minutes)**
   ```bash
   # Check database status
   kubectl exec -n wikijs deployment/postgres -- pg_isready -U wikijs

   # Review logs for corruption indicators
   kubectl logs -n wikijs deployment/postgres --tail=100
   ```

3. **Database Recovery (30 minutes - 2 hours)**
   ```bash
   # Restore from latest verified backup
   kubectl exec -n wikijs deployment/postgres -- psql -U wikijs -d wikijs < /backups/latest_backup.sql

   # Verify database integrity
   kubectl exec -n wikijs deployment/postgres -- pg_dump -U wikijs -d wikijs > /tmp/verify.sql
   ```

4. **Application Recovery (2-3 hours)**
   ```bash
   # Restart Wiki.js application
   kubectl scale deployment wikijs --replicas=2 -n wikijs

   # Monitor health checks
   kubectl get pods -n wikijs -l app=wikijs,component=application
   ```

5. **Verification (3-4 hours)**
   ```bash
   # Full system health check
   kubectl exec -n wikijs deployment/wikijs -- curl -f http://localhost:3000/health

   # Test critical functionality
   curl -f https://wiki.example.com/
   ```

### Scenario 2: Complete Pod Failure

#### Severity: MEDIUM
#### Impact: Application unavailable
#### Recovery Time: 30-60 minutes

**Steps:**
1. **Diagnosis (0-10 minutes)**
   ```bash
   # Check pod status
   kubectl get pods -n wikijs -o wide

   # Review pod logs
   kubectl logs -n wikijs -l app=wikijs,component=application --tail=50
   ```

2. **Recovery (10-30 minutes)**
   ```bash
   # Delete affected pods
   kubectl delete pods -n wikijs -l app=wikijs,component=application

   # Force rollout restart
   kubectl rollout restart deployment/wikijs -n wikijs

   # Monitor new pod startup
   kubectl rollout status deployment/wikijs -n wikijs --timeout=600s
   ```

3. **Verification (30-60 minutes)**
   ```bash
   # Health check
   kubectl get pods -n wikijs -l app=wikijs,component=application

   # Application health check
   curl -f https://wiki.example.com/health
   ```

### Scenario 3: Regional Outage

#### Severity: CRITICAL
#### Impact: Complete service unavailability
#### Recovery Time: 4-8 hours

**Steps:**
1. **Declaration (0-30 minutes)**
   ```bash
   # Confirm regional outage
   kubectl get nodes --show-labels

   # Check regional service status
   aws ec2 describe-instances --region us-east-1
   ```

2. **Failover Activation (30 minutes - 2 hours)**
   ```bash
   # Update DNS to point to failover region
   aws route53 change-resource-record-sets \
     --hosted-zone-id ZONE_ID \
     --change-batch file://dns-failover.json

   # Activate disaster recovery infrastructure
   kubectl apply -f k8s/disaster-recovery/ -n wikijs-dr
   ```

3. **Data Recovery (2-6 hours)**
   ```bash
   # Restore from cross-region backup
   aws s3 sync s3://wikijs-backups-us-east-1 s3://wikijs-backups-us-west-2

   # Restore database
   kubectl exec -n wikijs-dr deployment/postgres -- psql -U wikijs -d wikijs < /backups/cross-region-latest.sql
   ```

4. **Service Verification (6-8 hours)**
   ```bash
   # Comprehensive health checks
   kubectl get pods -n wikijs-dr
   kubectl get ingress -n wikijs-dr

   # End-to-end testing
   curl -f https://wiki-dr.example.com/
   ```

---

## Failover Scenarios

### Active-Passive Configuration

#### Primary Region (us-east-1)
- **Production Services**: Active
- **Database**: Primary PostgreSQL with streaming replication
- **Backups**: Real-time to secondary region
- **DNS**: Primary record pointing to this region

#### Secondary Region (us-west-2)
- **Production Services**: Passive (scaled to 0)
- **Database**: Standby PostgreSQL with replication
- **Backups**: Real-time from primary
- **DNS**: Failover record (low TTL)

### Automated Failover

#### Health Monitoring
```yaml
# health-check.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: failover-health-check
data:
  check.sh: |
    #!/bin/bash
    # Comprehensive health check for failover decision

    check_primary_health() {
        # Check application health
        if ! curl -f https://wiki.example.com/health; then
            return 1
        fi

        # Check database connectivity
        if ! kubectl exec -n wikijs deployment/postgres -- pg_isready -U wikijs; then
            return 1
        fi

        # Check critical pod status
        if ! kubectl get pods -n wikijs -l app=wikijs --field-selector=status.phase=Running | grep -q "2/2"; then
            return 1
        fi

        return 0
    }

    # Trigger failover if primary is unhealthy for 5 minutes
    if ! check_primary_health; then
        sleep 300
        if ! check_primary_health; then
            echo "Initiating failover to secondary region"
            kubectl apply -f /failover/activate-secondary.yaml
        fi
    fi
```

#### DNS Failover Configuration
```json
{
  "Comment": "Failover to secondary region",
  "Changes": [
    {
      "Action": "UPSERT",
      "ResourceRecordSet": {
        "Name": "wiki.example.com.",
        "Type": "A",
        "SetIdentifier": "secondary-region",
        "Region": "us-west-2",
        "HealthCheckId": "HEALTH_CHECK_ID",
        "AliasTarget": {
          "HostedZoneId": "ZONE_ID",
          "DNSName": "wiki-dr-us-west-2.elb.amazonaws.com",
          "EvaluateTargetHealth": true
        }
      }
    }
  ]
}
```

---

## Testing and Validation

### Monthly DR Drills

#### Drill Scenario 1: Database Corruption
1. **Preparation**: Schedule maintenance window
2. **Simulation**: Corrupt primary database
3. **Response**: Execute recovery procedures
4. **Validation**: Verify data integrity
5. **Documentation**: Record lessons learned

#### Drill Scenario 2: Regional Failover
1. **Preparation**: Coordinate with cloud provider
2. **Simulation**: Disable primary region
3. **Response**: Activate secondary region
4. **Validation**: End-to-end testing
5. **Documentation**: Update procedures

### Quarterly Full-Scale Test

#### Test Scope
- Complete regional failover
- Cross-region backup restoration
- Performance testing under failover conditions
- Security validation in DR environment

#### Success Criteria
- RTO < 4 hours
- RPO < 1 hour
- No data loss
- All services functional
- Performance within 80% of normal

---

## Contact and Escalation

### Incident Response Team

#### Primary Contacts
- **DevOps Lead**: +1-XXX-XXX-XXXX
- **Database Administrator**: +1-XXX-XXX-XXXX
- **Cloud Architect**: +1-XXX-XXX-XXXX
- **Security Officer**: +1-XXX-XXX-XXXX

#### Escalation Matrix
| Severity | Response Time | Escalation |
|----------|---------------|------------|
| Critical | 15 minutes | VP Engineering |
| High | 1 hour | DevOps Manager |
| Medium | 4 hours | Team Lead |
| Low | 24 hours | On-call Engineer |

### External Contacts
- **Cloud Provider Support**: AWS Support - 1-XXX-XXX-XXXX
- **Database Vendor**: PostgreSQL Enterprise Support
- **Security Incident**: security@company.com

### Communication Channels
- **Slack**: #wikijs-incidents
- **Email**: incidents@company.com
- **Phone**: Incident hotline +1-XXX-XXX-XXXX

---

## Appendix

### Backup Script Examples

#### Full Database Backup
```bash
#!/bin/bash
# backup-full.sh
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="/backups/wikijs_full_${TIMESTAMP}.sql"

# Create backup
pg_dump -h localhost -U wikijs -d wikijs -F c -b -v -f "$BACKUP_FILE"

# Compress backup
gzip "$BACKUP_FILE"

# Create checksum
sha256sum "${BACKUP_FILE}.gz" > "${BACKUP_FILE}.gz.sha256"

# Upload to cloud storage
aws s3 cp "${BACKUP_FILE}.gz" "s3://wikijs-backups/full/"
aws s3 cp "${BACKUP_FILE}.gz.sha256" "s3://wikijs-backups/full/"

echo "Full backup completed: ${BACKUP_FILE}.gz"
```

#### Incremental Backup
```bash
#!/bin/bash
# backup-incremental.sh
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="/backups/wikijs_incremental_${TIMESTAMP}.sql"

# Create incremental backup using WAL
pg_basebackup -h localhost -D "$BACKUP_FILE" -U wikijs -v -P -W

# Compress and upload
tar -czf "${BACKUP_FILE}.tar.gz" "$BACKUP_FILE"
aws s3 cp "${BACKUP_FILE}.tar.gz" "s3://wikijs-backups/incremental/"

echo "Incremental backup completed: ${BACKUP_FILE}.tar.gz"
```

### Recovery Checklists

#### Pre-Recovery Checklist
- [ ] Confirm incident scope and impact
- [ ] Notify stakeholders
- [ ] Verify backup availability
- [ ] Prepare recovery environment
- [ ] Document recovery start time

#### Post-Recovery Checklist
- [ ] Verify service functionality
- [ ] Run data integrity checks
- [ ] Update DNS records (if needed)
- [ ] Monitor system performance
- [ ] Document recovery completion
- [ ] Conduct post-mortem analysis
- [ ] Update DR procedures

---

**Document History**:
- 2025-11-03: Initial version (v1.0) - Robo-DevOps/SRE

**Next Review**: 2026-02-03 (Quarterly review scheduled)