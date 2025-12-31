# Rust Lovable - Operations Runbook

## Table of Contents

1. [System Overview](#system-overview)
2. [Installation and Setup](#installation-and-setup)
3. [Daily Operations](#daily-operations)
4. [Monitoring and Alerting](#monitoring-and-alerting)
5. [Troubleshooting](#troubleshooting)
6. [Backup and Recovery](#backup-and-recovery)
7. [Scaling and Performance](#scaling-and-performance)
8. [Security](#security)
9. [API Reference](#api-reference)
10. [Emergency Procedures](#emergency-procedures)

## System Overview

Rust Lovable is a cross-platform UI building platform that combines conversational AI with dynamic code generation. The system consists of:

- **Frontend**: Dioxus-based web/desktop/mobile UI
- **Backend**: Rust-based API server with Tauri integration
- **AI Engine**: Multi-provider conversational AI
- **Sandbox**: Secure code execution environment
- **Database**: SQLite/PostgreSQL for persistence

### Architecture Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Web UI        │    │  Desktop App    │    │  Mobile App     │
│   (Dioxus)      │    │   (Tauri)       │    │   (Dioxus)      │
└────────┬────────┘    └────────┬────────┘    └────────┬────────┘
         │                      │                      │
         └──────────────────────┼──────────────────────┘
                                │
                    ┌───────────▼───────────┐
                    │    API Server         │
                    │    (Rust/Axum)        │
                    └───────────┬───────────┘
                                │
         ┌──────────────────────┼──────────────────────┐
         │                      │                      │
    ┌────▼────┐          ┌───────▼───────┐      ┌───────▼───────┐
    │   AI    │          │   Sandbox     │      │  Database     │
    │ Engine  │          │  Environment  │      │ (SQLite/PG)   │
    └─────────┘          └───────────────┘      └───────────────┘
```

## Installation and Setup

### Prerequisites

- **OS**: Linux (Ubuntu 20.04+), macOS (10.15+), Windows 10+
- **CPU**: 2+ cores, x86_64 or ARM64
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 2GB free space
- **Network**: Internet connection for AI APIs

### Single-Click Installation

```bash
# Linux/macOS
curl -sSL https://raw.githubusercontent.com/yourusername/rust-lovable/main/install.sh | bash

# Windows (PowerShell)
iwr -useb https://raw.githubusercontent.com/yourusername/rust-lovable/main/install.ps1 | iex
```

### Manual Installation

1. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Clone Repository**:
   ```bash
   git clone https://github.com/yourusername/rust-lovable.git
   cd rust-lovable
   ```

3. **Build and Install**:
   ```bash
   cargo build --release
   sudo cp target/release/rust-lovable /usr/local/bin/
   ```

### Configuration

Create configuration file at `~/.config/rust-lovable/config.toml`:

```toml
[general]
platform = "universal"
auto_update = true

[ai]
provider = "openai"
api_key = "your-api-key"
model = "gpt-4"
timeout = 30

[sandbox]
max_sandboxes = 5
execution_timeout = 60

[database]
type = "sqlite"
path = "~/.rust-lovable/data.db"
```

## Daily Operations

### Starting the Service

```bash
# Development mode
rust-lovable --dev

# Production mode
rust-lovable --release

# With specific config
rust-lovable --config /path/to/config.toml
```

### Service Management

**Systemd (Linux)**:
```bash
# Start service
systemctl --user start rust-lovable

# Enable auto-start
systemctl --user enable rust-lovable

# Check status
systemctl --user status rust-lovable

# View logs
journalctl --user -u rust-lovable -f
```

**LaunchAgent (macOS)**:
```bash
# Load service
launchctl load ~/Library/LaunchAgents/rust-lovable.plist

# Check status
launchctl list | grep rust-lovable
```

### Health Checks

```bash
# Basic health check
curl http://localhost:8080/health

# Detailed status
curl http://localhost:8080/api/status

# Component health
curl http://localhost:8080/api/health/ai
curl http://localhost:8080/api/health/database
curl http://localhost:8080/api/health/sandbox
```

### Log Management

**Log Locations**:
- Linux: `~/.local/share/rust-lovable/logs/`
- macOS: `~/Library/Application Support/rust-lovable/logs/`
- Windows: `%APPDATA%\rust-lovable\logs\`

**Log Rotation**:
```bash
# Configure logrotate (Linux)
sudo cp scripts/logrotate.conf /etc/logrotate.d/rust-lovable
```

## Monitoring and Alerting

### Key Metrics

| Metric | Description | Threshold |
|--------|-------------|-----------|
| Response Time | API response time | < 2s |
| Error Rate | Failed requests | < 1% |
| Memory Usage | RAM utilization | < 80% |
| CPU Usage | Processor utilization | < 90% |
| Active Sandboxes | Running sandboxes | < max_sandboxes |
| AI API Latency | AI provider response | < 5s |

### Monitoring Setup

**Prometheus + Grafana**:
```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'rust-lovable'
    static_configs:
      - targets: ['localhost:8080']
```

**Custom Metrics Endpoint**:
```bash
curl http://localhost:8080/metrics
```

### Alerts Configuration

**Critical Alerts**:
- Service down
- High error rate (>5%)
- Memory usage >95%
- Disk space <10%
- AI API failures

**Warning Alerts**:
- High response time (>5s)
- Memory usage >80%
- CPU usage >90%
- Sandbox queue backup

## Troubleshooting

### Common Issues

#### 1. Service Won't Start

**Symptoms**: Service fails to start or crashes immediately

**Diagnosis**:
```bash
# Check logs
journalctl --user -u rust-lovable -n 100

# Check configuration
rust-lovable --validate-config

# Check dependencies
rust-lovable --check-deps
```

**Solutions**:
- Verify configuration file syntax
- Check file permissions
- Ensure all dependencies are installed
- Review system requirements

#### 2. AI Provider Errors

**Symptoms**: AI requests failing or timing out

**Diagnosis**:
```bash
# Test AI provider
curl -X POST http://localhost:8080/api/ai/test \
  -H "Content-Type: application/json" \
  -d '{"message": "test"}'
```

**Solutions**:
- Verify API key validity
- Check rate limits
- Try alternative AI provider
- Review network connectivity

#### 3. Sandbox Execution Failures

**Symptoms**: Code execution errors or timeouts

**Diagnosis**:
```bash
# Check sandbox status
curl http://localhost:8080/api/sandbox/status

# Review sandbox logs
curl http://localhost:8080/api/sandbox/logs
```

**Solutions**:
- Increase timeout limits
- Check resource availability
- Review code for infinite loops
- Restart sandbox environment

#### 4. Database Connection Issues

**Symptoms**: Data persistence failures

**Diagnosis**:
```bash
# Test database connection
rust-lovable --test-database

# Check database logs
tail -f ~/.rust-lovable/logs/database.log
```

**Solutions**:
- Verify database file permissions
- Check disk space
- Repair corrupted database
- Switch to PostgreSQL for better reliability

### Debug Mode

Enable debug logging:
```bash
RUST_LOG=debug rust-lovable
```

Enable verbose mode:
```bash
rust-lovable --verbose
```

### Performance Profiling

```bash
# CPU profiling
perf record -g -- ./rust-lovable
perf report

# Memory profiling
valgrind --tool=memcheck ./rust-lovable

# Benchmarking
cargo bench
```

## Backup and Recovery

### Database Backup

**Automated Backup**:
```bash
# Create backup script
cat > ~/.local/bin/backup-rust-lovable.sh << 'EOF'
#!/bin/bash
BACKUP_DIR="$HOME/.rust-lovable/backups"
mkdir -p "$BACKUP_DIR"
DATE=$(date +%Y%m%d_%H%M%S)

cp ~/.rust-lovable/data.db "$BACKUP_DIR/data_$DATE.db"
find "$BACKUP_DIR" -name "*.db" -mtime +7 -delete
EOF

chmod +x ~/.local/bin/backup-rust-lovable.sh

# Add to crontab
echo "0 2 * * * $HOME/.local/bin/backup-rust-loable.sh" | crontab -
```

**Manual Backup**:
```bash
# Backup database
cp ~/.rust-lovable/data.db ~/.rust-lovable/data.db.backup

# Backup configuration
tar -czf rust-lovable-config-backup.tar.gz ~/.config/rust-lovable/
```

### Recovery Procedures

**Database Recovery**:
```bash
# Restore from backup
cp ~/.rust-lovable/backups/data_20231201_120000.db ~/.rust-lovable/data.db

# Repair corrupted database
rust-lovable --repair-database
```

**Configuration Recovery**:
```bash
# Restore configuration
tar -xzf rust-lovable-config-backup.tar.gz -C ~/
```

### Disaster Recovery

**Complete System Recovery**:
1. Install Rust Lovable on new system
2. Restore configuration files
3. Restore database from backup
4. Restart services
5. Verify functionality

## Scaling and Performance

### Horizontal Scaling

**Load Balancer Configuration**:
```nginx
upstream rust_lovable {
    server 127.0.0.1:8080;
    server 127.0.0.1:8081;
    server 127.0.0.1:8082;
}

server {
    listen 80;
    location / {
        proxy_pass http://rust_lovable;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

**Multi-Instance Setup**:
```bash
# Start multiple instances
rust-lovable --port 8080 --instance-id 1
rust-lovable --port 8081 --instance-id 2
rust-lovable --port 8082 --instance-id 3
```

### Performance Tuning

**Database Optimization**:
```sql
-- Create indexes
CREATE INDEX idx_components_project_id ON components(project_id);
CREATE INDEX idx_conversations_user_id ON conversations(user_id);

-- Vacuum and analyze
VACUUM ANALYZE;
```

**Memory Optimization**:
```toml
# config.toml
[performance]
cache_size = "2GB"
max_connections = 100
worker_threads = 4
```

**Sandbox Optimization**:
```toml
[sandbox]
max_sandboxes = 10
reuse_sandboxes = true
prewarm_sandboxes = 2
```

### Caching Strategy

**Redis Caching**:
```bash
# Install Redis
sudo apt-get install redis-server

# Configure Redis
redis-cli config set maxmemory 2gb
redis-cli config set maxmemory-policy allkeys-lru
```

**Application-Level Caching**:
```rust
// Cache AI responses
let cache_key = format!("ai:{}:{}", provider, hash(&request));
if let Some(cached) = cache.get(&cache_key) {
    return cached;
}
```

## Security

### Security Best Practices

1. **API Key Management**:
   - Use environment variables
   - Rotate keys regularly
   - Implement key encryption at rest

2. **Sandbox Security**:
   - Isolate sandbox environments
   - Limit resource usage
   - Monitor for malicious code

3. **Network Security**:
   - Use HTTPS only
   - Implement rate limiting
   - Enable CORS properly

4. **Data Security**:
   - Encrypt sensitive data
   - Implement proper access controls
   - Regular security audits

### Security Configuration

```toml
# config.toml
[security]
enable_https = true
rate_limit = 100  # requests per minute
max_request_size = "10MB"
enable_cors = true
cors_origins = ["https://app.rust-lovable.com"]

[audit]
enable_logging = true
log_file = "~/.rust-lovable/logs/audit.log"
retention_days = 90
```

### Vulnerability Scanning

```bash
# Dependency scanning
cargo audit

# Container scanning
docker run --rm -v "$PWD":/app clair-scanner rust-lovable:latest

# SAST scanning
cargo install cargo-geiger
cargo geiger
```

## API Reference

### Authentication

All API requests require authentication:

```http
POST /api/v1/endpoint
Authorization: Bearer your-api-key
Content-Type: application/json
```

### Core Endpoints

#### Project Management

**Create Project**:
```http
POST /api/v1/projects
{
  "name": "My Project",
  "description": "Project description",
  "platform": "universal"
}
```

**List Projects**:
```http
GET /api/v1/projects
```

**Get Project**:
```http
GET /api/v1/projects/{project_id}
```

**Update Project**:
```http
PUT /api/v1/projects/{project_id}
{
  "name": "Updated Name",
  "description": "Updated description"
}
```

**Delete Project**:
```http
DELETE /api/v1/projects/{project_id}
```

#### AI Integration

**Process Message**:
```http
POST /api/v1/ai/process
{
  "project_id": "project-123",
  "message": "Create a blue button",
  "context": {}
}
```

**Generate Code**:
```http
POST /api/v1/ai/generate
{
  "project_id": "project-123",
  "request": {
    "type": "create_component",
    "component_type": "button",
    "properties": {}
  }
}
```

#### Sandbox Management

**Create Sandbox**:
```http
POST /api/v1/sandboxes
{
  "platform": "web",
  "requirements": ["react", "tailwind"]
}
```

**Execute Code**:
```http
POST /api/v1/sandboxes/{sandbox_id}/execute
{
  "code": "console.log('Hello World')",
  "language": "javascript",
  "timeout": 30
}
```

**Get Sandbox Status**:
```http
GET /api/v1/sandboxes/{sandbox_id}/status
```

#### Component Management

**Create Component**:
```http
POST /api/v1/projects/{project_id}/components
{
  "type": "button",
  "properties": {
    "text": "Click Me",
    "color": "blue"
  }
}
```

**List Components**:
```http
GET /api/v1/projects/{project_id}/components
```

**Update Component**:
```http
PUT /api/v1/projects/{project_id}/components/{component_id}
{
  "properties": {
    "text": "Updated Text"
  }
}
```

**Delete Component**:
```http
DELETE /api/v1/projects/{project_id}/components/{component_id}
```

### WebSocket API

**Real-time Updates**:
```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onmessage = (event) => {
  const update = JSON.parse(event.data);
  console.log('Received update:', update);
};
```

### Error Handling

**Error Response Format**:
```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "Invalid request format",
    "details": {
      "field": "name",
      "issue": "required"
    }
  }
}
```

## Emergency Procedures

### Service Outage

1. **Immediate Response**:
   ```bash
   # Check service status
   systemctl --user status rust-lovable
   
   # Restart service
   systemctl --user restart rust-lovable
   
   # Check logs
   journalctl --user -u rust-lovable -n 100
   ```

2. **Root Cause Analysis**:
   - Review recent changes
   - Check system resources
   - Analyze error logs
   - Test dependencies

3. **Recovery**:
   - Rollback if necessary
   - Apply fixes
   - Test thoroughly
   - Monitor closely

### Data Corruption

1. **Stop Services**:
   ```bash
   systemctl --user stop rust-lovable
   ```

2. **Assess Damage**:
   ```bash
   # Check database integrity
   sqlite3 ~/.rust-lovable/data.db "PRAGMA integrity_check;"
   ```

3. **Recovery**:
   ```bash
   # Restore from backup
   cp ~/.rust-lovable/backups/latest.db ~/.rust-lovable/data.db
   
   # Repair if possible
   rust-lovable --repair-database
   ```

4. **Verification**:
   ```bash
   # Test functionality
   rust-lovable --test-all
   
   # Start services
   systemctl --user start rust-lovable
   ```

### Security Incident

1. **Immediate Response**:
   - Isolate affected systems
   - Preserve evidence
   - Notify security team

2. **Investigation**:
   - Review access logs
   - Check for unauthorized changes
   - Analyze system state

3. **Recovery**:
   - Patch vulnerabilities
   - Reset compromised credentials
   - Restore from clean backups

4. **Post-Incident**:
   - Document findings
   - Update security measures
   - Conduct post-mortem

### Contact Information

**Emergency Contacts**:
- DevOps Team: devops@rust-lovable.com
- Security Team: security@rust-lovable.com
- On-Call Engineer: +1-555-0123

**Escalation Matrix**:
1. Level 1: Service degradation
2. Level 2: Service outage
3. Level 3: Security incident
4. Level 4: Complete system failure

---

This runbook is a living document. Please keep it updated as the system evolves.