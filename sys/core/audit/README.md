# Audit System - NOA System Core

**Version**: 1.0.0
**Location**: `sys/core/audit/`
**Purpose**: Comprehensive audit logging for NOA microkernel operations

---

## Overview

The NOA audit system provides:
- **Comprehensive Logging**: All security-relevant events
- **Tamper Protection**: Immutable audit logs
- **Compliance**: SOC2 and ISO27001 alignment
- **Sensitive Data Handling**: Auto-redaction of secrets and PII
- **Long-term Retention**: 90-day retention with archival

---

## Audit Event Categories

### 1. Authentication Events

**Events Logged**:
- `login_success` - Successful authentication
- `login_failure` - Failed authentication attempt
- `logout` - User/agent logout
- `session_expired` - Session timeout
- `token_refresh` - Token refresh operation

**Example Log Entry**:
```json
{
  "timestamp": "2026-01-02T10:15:30Z",
  "event_type": "authentication",
  "event": "login_success",
  "principal_id": "agent:default",
  "session_id": "sess_abc123",
  "metadata": {
    "auth_method": "service_account",
    "ip_address": "127.0.0.1"
  }
}
```

### 2. Authorization Events

**Events Logged**:
- `access_granted` - Authorization succeeded
- `access_denied` - Authorization failed
- `capability_checked` - Capability verification
- `role_evaluated` - Role-based access check

### 3. Capability Usage

**Capabilities Tracked**:
- `reasoning` - LLM inference usage
- `code_generation` - Code generation
- `code_execution` - Sandbox execution
- `file_operations` - File system access
- `git_operations` - Git commands
- `embeddings` - Vector operations
- `tool_discovery` - MCP tool usage

### 4. Code Execution Events

**Events Logged**:
- `sandbox_created` - Sandbox initialization
- `sandbox_destroyed` - Sandbox cleanup
- `code_executed` - Code execution
- `execution_failed` - Execution error
- `resource_limit_exceeded` - Resource limit hit

**Metadata Included**:
- Sandbox ID
- Exit code
- Duration (ms)
- Resource usage (CPU, memory)

### 5. File Operations

**Events Logged**:
- `file_read` - File read
- `file_write` - File write/create
- `file_delete` - File deletion
- `directory_create` - Directory creation
- `directory_delete` - Directory deletion

**Metadata Included**:
- File path
- File size
- File hash (optional)

### 6. Git Operations

**Events Logged**:
- `git_clone` - Repository clone
- `git_commit` - Commit creation
- `git_push` - Push to remote
- `git_pull` - Pull from remote
- `git_branch` - Branch operations
- `git_merge` - Merge operations

**Metadata Included**:
- Repository URL/path
- Branch name
- Commit hash
- Author

### 7. Policy Violations

**Events Logged**:
- `capability_denied` - Capability check failed
- `sandbox_violation` - Sandbox policy violated
- `budget_exceeded` - Budget limit hit
- `path_violation` - Path restriction violated
- `rate_limit_exceeded` - Rate limit hit

**Metadata Included**:
- Rule ID
- Policy name
- Principal ID
- Violation details

### 8. Budget Tracking

**Events Logged**:
- `budget_allocated` - Budget assigned
- `budget_spent` - Operation cost recorded
- `budget_warning` - Approaching limit
- `budget_exceeded` - Limit exceeded

**Metadata Included**:
- Amount (USD)
- Remaining budget
- Operation type

---

## Log Format

### JSON Structure

```json
{
  "timestamp": "2026-01-02T10:15:30.123Z",
  "trace_id": "trace_xyz789",
  "event_type": "code_execution",
  "event": "code_executed",
  "principal_id": "agent:default",
  "session_id": "sess_abc123",
  "capability": "code_execution",
  "resource": {
    "type": "sandbox",
    "id": "sandbox_build_001"
  },
  "operation": "execute",
  "result": "success",
  "metadata": {
    "sandbox_id": "sandbox_build_001",
    "exit_code": 0,
    "duration_ms": 1234,
    "resource_usage": {
      "cpu_percent": 45.2,
      "memory_mb": 512
    }
  }
}
```

### Timestamp Format

ISO 8601 with millisecond precision:
```
2026-01-02T10:15:30.123Z
```

---

## Storage and Rotation

### Log Storage

**Base Path**: `${NOA_ROOT}/sys/core/audit/logs/`

**File Structure**:
```
logs/
├─ authentication/
│  └─ 2026-01-02.json
├─ authorization/
│  └─ 2026-01-02.json
├─ code_execution/
│  └─ 2026-01-02.json
├─ file_operations/
│  └─ 2026-01-02.json
├─ git_operations/
│  └─ 2026-01-02.json
└─ policy_violations/
   └─ 2026-01-02.json
```

### Log Rotation

**Triggered When**:
- File size exceeds 100MB
- Daily at midnight UTC

**Process**:
1. Close current log file
2. Compress to `.gz`
3. Move to archive
4. Create new log file

### Retention

- **Active Logs**: 90 days in `logs/`
- **Archived Logs**: Moved to `archive/` after 90 days
- **Long-term**: Archive to S3/backup after 1 year

---

## Sensitive Data Handling

### Secret Redaction

**Auto-Redacted Fields**:
- `password`
- `token`
- `api_key`
- `secret`
- `private_key`

**Example**:
```json
{
  "metadata": {
    "api_key": "[REDACTED]",
    "password": "[REDACTED]"
  }
}
```

### PII Hashing

**Hashed Fields**:
- `email`
- `username`
- `ip_address`

**Example**:
```json
{
  "principal_id": "user:alice",
  "metadata": {
    "email_hash": "sha256:abc123...",
    "ip_address_hash": "sha256:def456..."
  }
}
```

---

## Compliance

### SOC2 Type II

Audit logs support SOC2 compliance:
- **Tamper Protection**: Immutable logs
- **Access Logging**: All access events recorded
- **Change Tracking**: File/configs modifications logged
- **Retention**: 90-day minimum retention

### ISO 27001

Audit logs support ISO 27001:
- **Security Event Logging**: Comprehensive event coverage
- **Log Protection**: Tamper-proof storage
- **Review Process**: Queryable audit trail
- **Retention Policy**: Defined retention periods

---

## Usage Examples

### Query Audit Logs

```bash
# View today's authentication events
cat sys/core/audit/logs/authentication/$(date +%Y-%m-%d).json | jq .

# Find failed login attempts
cat sys/core/audit/logs/authentication/*.json | \
  jq 'select(.event == "login_failure")'

# Track code executions for specific principal
cat sys/core/audit/logs/code_execution/*.json | \
  jq 'select(.principal_id == "agent:default")'

# Find policy violations in last 7 days
find sys/core/audit/logs/policy_violations -name "*.json" -mtime -7 -exec cat {} \; | \
  jq 'select(.event == "capability_denied")'
```

### Track Budget Usage

```bash
# Calculate total budget spent today
cat sys/core/audit/logs/budget_tracking/$(date +%Y-%m-%d).json | \
  jq 'select(.event == "budget_spent") | .metadata.amount_usd' | \
  awk '{sum += $1} END {print sum}'
```

### Git Operation Audit

```bash
# List all commits today
cat sys/core/audit/logs/git_operations/$(date +%Y-%m-%d).json | \
  jq 'select(.event == "git_commit") | {
    timestamp,
    author: .metadata.author,
    commit: .metadata.commit_hash
  }'
```

---

## Integration

### With Identity System

Audit logs reference principals from `sys/core/identity/`:
```json
{
  "principal_id": "agent:default"  # References sys/core/identity/identity.json
}
```

### With Enforcement System

Policy violations logged from `sys/core/enforcement/`:
```json
{
  "event": "capability_denied",
  "metadata": {
    "rule_id": "cap-001",  # References sys/core/enforcement/policy.json
    "policy_name": "capability_enforcement"
  }
}
```

---

## Security

### Tamper Protection

- Logs written once, never modified
- Checksums generated for each log file
- Chain of custody maintained via hashing

### Access Control

- Audit logs readable only by `system_admin` role
- No delete permissions (immutable)
- Separate backup process for disaster recovery

### Encryption

- Logs encrypted at rest (AES-256)
- In-transit encryption (TLS 1.3)
- Key management via `sys/core/secrets/`

---

## Monitoring and Alerting

### Alert Conditions

1. **Repeated Login Failures**: 5+ failures in 5 minutes
2. **Policy Violations**: 10+ violations in 1 hour
3. **Budget Exceeded**: Any budget limit exceeded
4. **Suspicious File Access**: Access to secret files

### Alert Destinations

- Email to admin
- Slack notification
- PagerDuty (critical only)

---

## References

- [Identity System](../identity/README.md)
- [Enforcement System](../enforcement/README.md)
- [NOA Policy Framework](../../../ai/shared/resources/policy/01_CONSTITUTION.md)

---

**Version**: 1.0.0
**Last Updated**: 2026-01-02
