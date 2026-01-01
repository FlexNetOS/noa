# Rust Lovable - Enhanced Operations Runbook

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
11. [Advanced Features](#advanced-features)
12. [Integration Management](#integration-management)

## System Overview

Rust Lovable is a comprehensive cross-platform UI building platform that combines conversational AI with dynamic code generation. The system consists of:

- **Frontend**: Dioxus-based web/desktop/mobile UI with real-time collaboration
- **Backend**: Rust-based API server with Tauri integration and WebSocket support
- **AI Engine**: Multi-provider conversational AI with streaming capabilities
- **Sandbox**: Secure code execution environment with Vite integration
- **Database**: SQLite/PostgreSQL for persistence with connection pooling
- **Streaming**: Server-sent events for real-time updates
- **Monitoring**: Comprehensive observability and health checks

### Architecture Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Web UI        │    │  Desktop App    │    │  Mobile App     │
│   (Dioxus)      │    │   (Tauri)       │    │   (Web)         │
└────────┬────────┘    └────────┬────────┘    └────────┬────────┘
         │                      │                      │
         └──────────────────────┼──────────────────────┘
                                │
                    ┌───────────▼───────────┐
                    │    API Server         │
                    │    (Rust/Axum)        │
                    └───────────┬───────────┘
                                │
    ┌──────────────────────┬────┼────┬──────────────────────┐
    │                      │    │    │                      │
┌───▼────┐  ┌───────▼───────┐  │  ┌───▼────┐  ┌───────────▼────────┐
│   AI   │  │   Sandbox     │  │  │   UI   │  │   File System      │
│ Engine │  │ Environment   │  │  │ Generator│  │   & Streaming      │
└────────┘  └───────────────┘  │  └────────┘  └────────────────────┘
                                │
                    ┌───────────▼───────────┐
                    │   Database & Cache    │
                    │ (SQLite/Redis/PG)     │
                    └───────────────────────┘
```

## Installation and Setup

### Prerequisites

- **OS**: Linux (Ubuntu 20.04+, RHEL 8+, Arch), macOS (10.15+), Windows 10+
- **CPU**: 2+ cores, x86_64 or ARM64 with SSE/AVX support
- **RAM**: 4GB minimum, 8GB recommended, 16GB+ for large projects
- **Storage**: 2GB free space, SSD recommended
- **Network**: Internet connection for AI APIs and package installation
- **GPU**: Optional, for accelerated AI processing

### Enhanced Single-Click Installation

**Linux/macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/yourusername/rust-lovable/main/install-v2.sh | bash
```

**Windows (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/yourusername/rust-lovable/main/install-v2.ps1 | iex
```

### Hardware Detection and Optimization

The installer automatically detects and optimizes for:

- **CPU Features**: SSE, AVX, AVX2, AVX512 support
- **Memory**: Available RAM and swap configuration
- **GPU**: CUDA/OpenCL availability for AI acceleration
- **Storage**: Disk type (SSD/HDD) and available space
- **Network**: Bandwidth and latency characteristics
- **Development Tools**: Available compilers and package managers

### Manual Installation with Custom Configuration

```bash
# Clone repository
git clone https://github.com/yourusername/rust-lovable.git
cd rust-lovable

# Run hardware detection
./scripts/detect_hardware.sh > hardware.json

# Create custom configuration
cp config.toml.example config.toml
# Edit config.toml with your hardware-specific settings

# Build with optimizations
cargo build --release --features "web desktop mobile"

# Install system-wide
sudo cp target/release/rust-lovable /usr/local/bin/
```

### Configuration

Create configuration file at `~/.config/rust-lovable/config.toml`:

```toml
[general]
platform = "universal"
auto_update = true
install_location = "/home/user/.rust-lovable"

[performance]
# Auto-detected based on hardware
worker_threads = 4
max_sandboxes = 5
cache_enabled = true
max_cache_size = "4GB"
enable_gpu_acceleration = true

[ai]
provider = "openai"
api_key = "your-api-key"
model = "gpt-4"
timeout = 30
max_concurrent_requests = 4
streaming_enabled = true

[sandbox]
max_sandboxes = 5
execution_timeout = 60
memory_limit_mb = 2048
cpu_limit_percent = 80
enable_vite_integration = true

[database]
type = "sqlite"
path = "/home/user/.local/share/rust-lovable/data.db"
max_connections = 40
pool_size = 10

[streaming]
enable_server_sent_events = true
max_connections_per_user = 5
heartbeat_interval = 30

[monitoring]
enable_metrics = true
metrics_port = 9090
health_check_interval = 60

[security]
enable_https = true
rate_limit = 100
max_request_size = "50MB"
cors_origins = ["https://app.rust-lovable.com"]

[logging]
level = "info"
file_path = "/home/user/.local/share/rust-lovable/logs/app.log"
max_size_mb = 100
max_files = 10
structured = true
```

## Daily Operations

### Starting the Service

```bash
# Development mode with hot reload
rust-lovable --dev --port 8080

# Production mode with optimizations
rust-lovable --release --port 8080

# With custom configuration
rust-lovable --config /path/to/config.toml

# Background service
rust-lovable --daemon --pid-file /var/run/rust-lovable.pid
```

### Service Management

**Systemd (Linux):**
```bash
# Start service
systemctl --user start rust-lovable

# Enable auto-start
systemctl --user enable rust-lovable

# Check status
systemctl --user status rust-lovable

# View logs
journalctl --user -u rust-lovable -f

# Restart service
systemctl --user restart rust-lovable

# Stop service
systemctl --user stop rust-lovable
```

**LaunchAgent (macOS):**
```bash
# Load service
launchctl load ~/Library/LaunchAgents/rust-lovable.plist

# Check status
launchctl list | grep rust-lovable

# Unload service
launchctl unload ~/Library/LaunchAgents/rust-lovable.plist
```

**Windows Service:**
```powershell
# Install service
rust-lovable --install-service

# Start service
Start-Service rust-lovable

# Check status
Get-Service rust-lovable

# Stop service
Stop-Service rust-lovable
```

### Health Checks

```bash
# Basic health check
curl http://localhost:8080/health

# Detailed status
curl http://localhost:8080/api/status

# Component health checks
curl http://localhost:8080/api/health/ai
curl http://localhost:8080/api/health/database
curl http://localhost:8080/api/health/sandbox
curl http://localhost:8080/api/health/streaming

# Metrics endpoint
curl http://localhost:8080/metrics

# Custom health monitor
rust-lovable-monitor
```

### Log Management

**Log Locations:**
- Linux: `~/.local/share/rust-lovable/logs/`
- macOS: `~/Library/Application Support/rust-lovable/logs/`
- Windows: `%APPDATA%\rust-lovable\logs\`

**Log Rotation:**
```bash
# Configure logrotate (Linux)
sudo cp scripts/logrotate.conf /etc/logrotate.d/rust-lovable

# Manual log rotation
rust-lovable --rotate-logs
```

**Log Analysis:**
```bash
# View recent errors
tail -f ~/.local/share/rust-lovable/logs/app.log | grep ERROR

# Search for specific patterns
grep -i "sandbox" ~/.local/share/rust-lovable/logs/app.log

# Structured log analysis
jq '.level == "ERROR"' ~/.local/share/rust-lovable/logs/app.json
```

## Monitoring and Alerting

### Key Metrics

| Metric | Description | Threshold | Alert Level |
|--------|-------------|-----------|-------------|
| Response Time | API response time | < 2s (OK), 2-5s (Warning), > 5s (Critical) | Critical |
| Error Rate | Failed requests | < 1% (OK), 1-5% (Warning), > 5% (Critical) | Critical |
| Memory Usage | RAM utilization | < 80% (OK), 80-95% (Warning), > 95% (Critical) | Warning |
| CPU Usage | Processor utilization | < 90% (OK), 90-95% (Warning), > 95% (Critical) | Warning |
| Active Sandboxes | Running sandboxes | < max-5 (OK), max-5 to max (Warning), > max (Critical) | Warning |
| AI API Latency | AI provider response | < 5s (OK), 5-10s (Warning), > 10s (Critical) | Critical |
| Streaming Connections | WebSocket connections | < 100 (OK), 100-500 (Warning), > 500 (Critical) | Warning |
| Database Connections | Pool utilization | < 80% (OK), 80-95% (Warning), > 95% (Critical) | Critical |
| Disk Space | Available storage | > 20% (OK), 10-20% (Warning), < 10% (Critical) | Critical |

### Monitoring Setup

**Prometheus + Grafana:**
```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'rust-lovable'
    static_configs:
      - targets: ['localhost:9090']
    scrape_interval: 5s
    metrics_path: /metrics
    
  - job_name: 'rust-lovable-health'
    static_configs:
      - targets: ['localhost:8080']
    scrape_interval: 30s
    metrics_path: /api/health
```

**Grafana Dashboard:**
```json
{
  "dashboard": {
    "title": "Rust Lovable Monitoring",
    "panels": [
      {
        "title": "Request Rate",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])"
          }
        ]
      },
      {
        "title": "Response Time",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))"
          }
        ]
      },
      {
        "title": "Error Rate",
        "targets": [
          {
            "expr": "rate(http_requests_total{status=~\"5..\"}[5m])"
          }
        ]
      }
    ]
  }
}
```

**Custom Metrics Endpoint:**
```bash
curl http://localhost:9090/metrics
```

### Alert Configuration

**Critical Alerts (Immediate Response):**
- Service down for > 2 minutes
- Error rate > 5% for > 5 minutes
- Memory usage > 95% for > 10 minutes
- Disk space < 10%
- AI API failures > 10% for > 5 minutes

**Warning Alerts (Business Hours):**
- Response time > 5s for > 10 minutes
- Memory usage > 80% for > 30 minutes
- CPU usage > 90% for > 15 minutes
- Sandbox queue backup > 10
- Database connections > 80%

**Info Alerts (Log Only):**
- Response time > 2s for > 5 minutes
- Memory usage > 70%
- CPU usage > 80%

### Alert Channels

**Email Alerts:**
```bash
# Configure email notifications
echo "admin@company.com" > ~/.rust-lovable/alert-email
echo "devops@company.com" >> ~/.rust-lovable/alert-email
```

**Slack Integration:**
```bash
# Configure Slack webhook
echo "https://hooks.slack.com/services/YOUR/WEBHOOK/URL" > ~/.rust-lovable/slack-webhook
```

**PagerDuty Integration:**
```bash
# Configure PagerDuty integration key
echo "YOUR_PAGERDUTY_INTEGRATION_KEY" > ~/.rust-lovable/pagerduty-key
```

## Troubleshooting

### Common Issues

#### 1. Service Won't Start

**Symptoms:** Service fails to start or crashes immediately

**Diagnosis:**
```bash
# Check service status
systemctl --user status rust-lovable

# Check logs
journalctl --user -u rust-lovable -n 100

# Validate configuration
rust-lovable --validate-config

# Check dependencies
rust-lovable --check-deps

# Test binary
rust-lovable --version
```

**Solutions:**
- Verify configuration file syntax: `toml-validator ~/.config/rust-lovable/config.toml`
- Check file permissions: `ls -la ~/.rust-lovable/`
- Ensure all dependencies are installed: `./scripts/check-deps.sh`
- Review system requirements: `./scripts/detect_hardware.sh`
- Check for port conflicts: `netstat -tlnp | grep 8080`

#### 2. AI Provider Errors

**Symptoms:** AI requests failing or timing out

**Diagnosis:**
```bash
# Test AI provider connectivity
curl -X POST http://localhost:8080/api/v1/ai/test \
  -H "Content-Type: application/json" \
  -d '{"message": "test"}'

# Check AI provider logs
grep -i "ai\|openai\|anthropic" ~/.rust-lovable/logs/app.log

# Test network connectivity
ping api.openai.com
```

**Solutions:**
- Verify API key validity and permissions
- Check rate limits and usage quotas
- Try alternative AI provider
- Review network connectivity and firewall settings
- Update AI provider configuration

#### 3. Sandbox Execution Failures

**Symptoms:** Code execution errors or timeouts

**Diagnosis:**
```bash
# Check sandbox status
curl http://localhost:8080/api/v1/sandboxes/{id}/status

# Review sandbox logs
curl http://localhost:8080/api/v1/sandboxes/{id}/logs

# Check resource usage
ps aux | grep sandbox

# Test sandbox manually
rust-lovable --test-sandbox
```

**Solutions:**
- Increase timeout limits in configuration
- Check resource availability (CPU, memory, disk)
- Review code for infinite loops or resource exhaustion
- Restart sandbox environment
- Check for malicious code patterns

#### 4. Database Connection Issues

**Symptoms:** Data persistence failures, connection errors

**Diagnosis:**
```bash
# Test database connection
rust-lovable --test-database

# Check database logs
tail -f ~/.rust-lovable/logs/database.log

# Check database file
ls -la ~/.local/share/rust-lovable/data.db

# Test database integrity
sqlite3 ~/.local/share/rust-lovable/data.db "PRAGMA integrity_check;"
```

**Solutions:**
- Verify database file permissions
- Check disk space availability
- Repair corrupted database
- Switch to PostgreSQL for better reliability
- Restore from backup

#### 5. Streaming Connection Issues

**Symptoms:** Real-time updates not working, WebSocket errors

**Diagnosis:**
```bash
# Check WebSocket connectivity
wscat -c ws://localhost:8080/ws

# Check streaming endpoints
curl -N http://localhost:8080/api/v1/stream/ai/generate

# Review streaming logs
grep -i "stream\|websocket" ~/.rust-lovable/logs/app.log

# Test streaming manually
rust-lovable --test-streaming
```

**Solutions:**
- Check WebSocket support in browser/client
- Verify streaming endpoint configuration
- Review CORS settings
- Check for network proxies or firewalls
- Restart streaming service

#### 6. Vite Integration Errors

**Symptoms:** Build failures, hot reload not working

**Diagnosis:**
```bash
# Check Vite errors
curl http://localhost:8080/api/v1/vite/{id}/errors

# Check build logs
curl http://localhost:8080/api/v1/vite/{id}/logs

# Test Vite manually
rust-lovable --test-vite

# Check package dependencies
curl http://localhost:8080/api/v1/sandboxes/{id}/packages/detect
```

**Solutions:**
- Install missing packages automatically
- Restart Vite development server
- Check for syntax errors in code
- Review build configuration
- Clear cache and rebuild

### Debug Mode

Enable debug logging:
```bash
RUST_LOG=debug rust-lovable
```

Enable verbose mode:
```bash
rust-lovable --verbose
```

Enable tracing:
```bash
RUST_LOG=trace rust-lovable 2>&1 | tee debug.log
```

### Performance Profiling

```bash
# CPU profiling
perf record -g -- ./rust-lovable
perf report

# Memory profiling
valgrind --tool=memcheck --leak-check=full ./rust-lovable

# Heap profiling
heaptrack ./rust-lovable

# Benchmarking
cargo bench

# Load testing
wrk -t12 -c400 -d30s http://localhost:8080/
```

## Backup and Recovery

### Backup Strategy

**Automated Backup:**
```bash
# Create backup script
cat > ~/.local/bin/backup-rust-lovable.sh << 'EOF'
#!/bin/bash
BACKUP_DIR="$HOME/.rust-lovable/backups"
CONFIG_DIR="$HOME/.config/rust-lovable"
DATA_DIR="$HOME/.local/share/rust-lovable"

mkdir -p "$BACKUP_DIR"
DATE=$(date +%Y%m%d_%H%M%S)

# Backup configuration
tar -czf "$BACKUP_DIR/config_$DATE.tar.gz" -C "$CONFIG_DIR" .

# Backup database
cp "$DATA_DIR/data.db" "$BACKUP_DIR/data_$DATE.db"

# Backup projects
tar -czf "$BACKUP_DIR/projects_$DATE.tar.gz" -C "$DATA_DIR/projects" . 2>/dev/null || true

# Cleanup old backups (keep last 7 days)
find "$BACKUP_DIR" -name "*.tar.gz" -mtime +7 -delete
find "$BACKUP_DIR" -name "*.db" -mtime +7 -delete

echo "Backup completed: $DATE"
EOF

chmod +x ~/.local/bin/backup-rust-lovable.sh

# Add to crontab
echo "0 2 * * * $HOME/.local/bin/backup-rust-lovable.sh" | crontab -
```

**Manual Backup:**
```bash
# Full system backup
tar -czf rust-lovable-backup-$(date +%Y%m%d).tar.gz \
  ~/.config/rust-lovable/ \
  ~/.local/share/rust-lovable/ \
  /usr/local/bin/rust-lovable

# Database backup only
sqlite3 ~/.local/share/rust-lovable/data.db ".backup backup.db"

# Configuration backup
cp -r ~/.config/rust-lovable/ config-backup/
```

### Recovery Procedures

**Database Recovery:**
```bash
# Restore from backup
cp ~/.rust-lovable/backups/data_20231201_120000.db ~/.local/share/rust-lovable/data.db

# Repair corrupted database
rust-lovable --repair-database

# Verify database integrity
sqlite3 ~/.local/share/rust-lovable/data.db "PRAGMA integrity_check;"
```

**Configuration Recovery:**
```bash
# Restore configuration
tar -xzf ~/.rust-lovable/backups/config_20231201_120000.tar.gz -C ~/.config/rust-lovable/

# Validate configuration
rust-lovable --validate-config
```

**Full System Recovery:**
```bash
# 1. Install Rust Lovable on new system
curl -sSL https://raw.githubusercontent.com/yourusername/rust-lovable/main/install-v2.sh | bash

# 2. Stop service
systemctl --user stop rust-lovable

# 3. Restore from backup
tar -xzf rust-lovable-backup-20231201.tar.gz -C /

# 4. Verify installation
rust-lovable --check-installation

# 5. Start service
systemctl --user start rust-lovable

# 6. Test functionality
rust-lovable --test-all
```

### Disaster Recovery

**Complete System Failure:**
1. **Immediate Response:**
   - Assess damage and scope
   - Activate disaster recovery team
   - Communicate with stakeholders

2. **Recovery Process:**
   - Provision new infrastructure
   - Install Rust Lovable from scratch
   - Restore from most recent backup
   - Verify functionality and data integrity
   - Update DNS and load balancer configurations

3. **Post-Recovery:**
   - Conduct post-mortem analysis
   - Update disaster recovery procedures
   - Implement improvements

## Scaling and Performance

### Horizontal Scaling

**Load Balancer Configuration:**
```nginx
upstream rust_lovable {
    least_conn;
    server 127.0.0.1:8080 weight=3;
    server 127.0.0.1:8081 weight=2;
    server 127.0.0.1:8082 weight=1;
    keepalive 32;
}

server {
    listen 80;
    location / {
        proxy_pass http://rust_lovable;
        proxy_http_version 1.1;
        proxy_set_header Connection "";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
    
    location /ws {
        proxy_pass http://rust_lovable;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

**Multi-Instance Setup:**
```bash
# Start multiple instances with different configurations
rust-lovable --port 8080 --instance-id 1 --config config1.toml
rust-lovable --port 8081 --instance-id 2 --config config2.toml
rust-lovable --port 8082 --instance-id 3 --config config3.toml

# Use process manager
pm2 start ecosystem.config.js
```

### Performance Tuning

**Database Optimization:**
```sql
-- Create indexes
CREATE INDEX idx_components_project_id ON components(project_id);
CREATE INDEX idx_conversations_user_id ON conversations(user_id);
CREATE INDEX idx_sandboxes_created_at ON sandboxes(created_at);
CREATE INDEX idx_files_project_id ON files(project_id);

-- Vacuum and analyze
VACUUM ANALYZE;

-- Enable WAL mode for better concurrency
PRAGMA journal_mode=WAL;
PRAGMA synchronous=NORMAL;

-- Set cache size
PRAGMA cache_size=10000;
PRAGMA temp_store=memory;
```

**Memory Optimization:**
```toml
# config.toml
[performance]
cache_size = "8GB"
max_connections = 200
worker_threads = 8
enable_connection_pooling = true
connection_pool_size = 20

[ai]
enable_response_caching = true
cache_ttl = 3600
batch_requests = true
max_batch_size = 10

[sandbox]
enable_sandbox_pooling = true
prewarm_sandboxes = 3
max_sandbox_reuse = 100
```

**CPU Optimization:**
```rust
// Use CPU-specific optimizations
#[cfg(target_feature = "avx2")]
compile_error!("This code requires AVX2 support");

// Enable parallel processing
use rayon::prelude::*;

fn process_components_parallel(components: Vec<Component>) -> Vec<ProcessedComponent> {
    components.par_iter()
        .map(|c| process_component(c))
        .collect()
}
```

### Caching Strategy

**Redis Caching:**
```bash
# Install Redis
sudo apt-get install redis-server

# Configure Redis
redis-cli config set maxmemory 2gb
redis-cli config set maxmemory-policy allkeys-lru
redis-cli config set save "900 1 300 10 60 10000"
```

**Application-Level Caching:**
```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct Cache<K, V> {
    data: Arc<RwLock<HashMap<K, (V, Instant)>>>,
    ttl: Duration,
}

impl<K: Eq + std::hash::Hash, V> Cache<K, V> {
    pub fn new(ttl: Duration) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }
    
    pub async fn get(&self, key: &K) -> Option<V> {
        let data = self.data.read().await;
        if let Some((value, timestamp)) = data.get(key) {
            if timestamp.elapsed() < self.ttl {
                return Some(value.clone());
            }
        }
        None
    }
    
    pub async fn insert(&self, key: K, value: V) {
        let mut data = self.data.write().await;
        data.insert(key, (value, Instant::now()));
    }
}
```

### Load Testing

```bash
# Install load testing tools
npm install -g artillery

# Create load test configuration
cat > load-test.yml << 'EOF'
config:
  target: 'http://localhost:8080'
  phases:
    - duration: 60
      arrivalRate: 10
    - duration: 120
      arrivalRate: 50
    - duration: 60
      arrivalRate: 10
  
scenarios:
  - name: "Create Project"
    requests:
      - post:
          url: "/api/v1/projects"
          json:
            name: "Load Test Project"
            platform: "web"
  
  - name: "Process AI Message"
    requests:
      - post:
          url: "/api/v1/ai/process"
          json:
            project_id: "test-project"
            message: "Create a button"
EOF

# Run load test
artillery run load-test.yml
```

## Security

### Security Best Practices

1. **API Key Management:**
   - Use environment variables or secure vaults
   - Rotate keys regularly
   - Implement key encryption at rest
   - Use different keys for different environments

2. **Sandbox Security:**
   - Isolate sandbox environments
   - Implement resource limits
   - Monitor for malicious code patterns
   - Use seccomp/AppArmor for additional isolation

3. **Network Security:**
   - Use HTTPS only in production
   - Implement rate limiting
   - Enable CORS with strict origins
   - Use WebSocket authentication

4. **Data Security:**
   - Encrypt sensitive data at rest
   - Implement proper access controls
   - Regular security audits
   - Use parameterized queries

### Security Configuration

```toml
# config.toml
[security]
enable_https = true
rate_limit = 100  # requests per minute
max_request_size = "50MB"
enable_cors = true
cors_origins = ["https://app.rust-lovable.com"]
cors_max_age = 86400

[authentication]
enable_jwt = true
jwt_secret = "your-secret-key"
jwt_expiration = 3600
enable_api_keys = true

[authorization]
enable_rbac = true
default_role = "user"
admin_roles = ["admin", "superadmin"]

[audit]
enable_logging = true
log_file = "/home/user/.local/share/rust-lovable/logs/audit.log"
log_level = "info"
retention_days = 90
include_requests = true
include_responses = false

[encryption]
enable_database_encryption = true
encryption_key = "your-encryption-key"
enable_file_encryption = false
```

### Vulnerability Scanning

```bash
# Dependency scanning
cargo audit
cargo outdated

# Container scanning
docker run --rm -v "$PWD":/app clair-scanner rust-lovable:latest

# SAST scanning
cargo install cargo-geiger
cargo geiger

# DAST scanning
# Use tools like OWASP ZAP or Burp Suite
```

### Security Headers

```rust
use axum::http::HeaderValue;
use tower_http::cors::CorsLayer;
use tower_http::security::SecurityLayer;

let app = Router::new()
    .layer(
        SecurityLayer::new()
            .x_frame_options("DENY")
            .x_content_type_options("nosniff")
            .x_xss_protection("1; mode=block")
            .strict_transport_security("max-age=31536000; includeSubDomains")
    )
    .layer(
        CorsLayer::new()
            .allow_origin("https://app.rust-lovable.com".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers([CONTENT_TYPE, AUTHORIZATION])
    );
```

## API Reference

### Authentication

All API requests require authentication:

```http
POST /api/v1/endpoint
Authorization: Bearer your-api-key
Content-Type: application/json
X-API-Version: 1.0
```

### Core Endpoints

#### Project Management

**Create Project:**
```http
POST /api/v1/projects
{
  "name": "My Project",
  "description": "Project description",
  "platform": "universal",
  "template": "modern"
}
```

**List Projects:**
```http
GET /api/v1/projects?page=1&limit=10&sort=created_at&order=desc
```

**Get Project:**
```http
GET /api/v1/projects/{project_id}
```

**Update Project:**
```http
PUT /api/v1/projects/{project_id}
{
  "name": "Updated Name",
  "description": "Updated description",
  "settings": {
    "theme": "dark",
    "language": "en"
  }
}
```

**Delete Project:**
```http
DELETE /api/v1/projects/{project_id}
```

#### AI Integration

**Process Message:**
```http
POST /api/v1/ai/process
{
  "project_id": "project-123",
  "message": "Create a blue button with text 'Submit'",
  "context": {
    "current_page": "landing",
    "selected_component": "hero_section"
  }
}
```

**Streaming Message Processing:**
```http
GET /api/v1/stream/ai/process/{project_id}
Authorization: Bearer your-api-key
Accept: text/event-stream
```

**Generate Code:**
```http
POST /api/v1/ai/generate
{
  "project_id": "project-123",
  "component_id": "component-456",
  "platform": "web",
  "style": "modern"
}
```

**Analyze Edit Intent:**
```http
POST /api/v1/ai/analyze-intent
{
  "project_id": "project-123",
  "message": "Make this button bigger",
  "context": {
    "current_component": "button-789"
  }
}
```

#### Sandbox Management

**Create Sandbox:**
```http
POST /api/v1/sandboxes
{
  "platform": "web",
  "requirements": ["react", "tailwind", "typescript"],
  "template": "vite-react-ts"
}
```

**Execute Code:**
```http
POST /api/v1/sandboxes/{sandbox_id}/execute
{
  "code": "console.log('Hello World')",
  "language": "javascript",
  "timeout": 30,
  "input": "test input"
}
```

**Get Sandbox Status:**
```http
GET /api/v1/sandboxes/{sandbox_id}/status
```

**Get Sandbox Files:**
```http
GET /api/v1/sandboxes/{sandbox_id}/files?path=/src
```

**Read File:**
```http
POST /api/v1/sandboxes/{sandbox_id}/files/read
{
  "file_path": "/src/App.tsx"
}
```

**Write File:**
```http
POST /api/v1/sandboxes/{sandbox_id}/files/write
{
  "file_path": "/src/App.tsx",
  "content": "import React from 'react';\n...",
  "create_if_not_exists": true
}
```

**Detect and Install Packages:**
```http
POST /api/v1/sandboxes/{sandbox_id}/packages/detect
{
  "code": "import React from 'react';\nimport lodash from 'lodash';",
  "file_path": "/src/App.tsx"
}
```

#### Vite Integration

**Check Vite Errors:**
```http
GET /api/v1/vite/{sandbox_id}/errors
```

**Report Vite Error:**
```http
POST /api/v1/vite/{sandbox_id}/report
{
  "message": "Module not found: Can't resolve 'react'",
  "file": "/src/App.tsx",
  "line": 3,
  "column": 8,
  "severity": "error"
}
```

**Stream Vite Logs:**
```http
GET /api/v1/stream/vite/{sandbox_id}/logs
Accept: text/event-stream
```

**Restart Vite:**
```http
POST /api/v1/vite/{sandbox_id}/restart
```

#### Component Management

**Create Component:**
```http
POST /api/v1/projects/{project_id}/components
{
  "component_type": "button",
  "properties": {
    "text": "Click Me",
    "color": "blue",
    "size": "medium"
  },
  "parent_id": "container-123"
}
```

**List Components:**
```http
GET /api/v1/projects/{project_id}/components
```

**Update Component:**
```http
PUT /api/v1/projects/{project_id}/components/{component_id}
{
  "properties": {
    "text": "Updated Text",
    "color": "red"
  }
}
```

**Delete Component:**
```http
DELETE /api/v1/projects/{project_id}/components/{component_id}
```

#### Export and Deployment

**Create ZIP:**
```http
POST /api/v1/export/zip
{
  "project_id": "project-123",
  "include_node_modules": false,
  "include_build": true,
  "format": "zip"
}
```

**Export to GitHub:**
```http
POST /api/v1/export/github
{
  "project_id": "project-123",
  "repo_name": "my-awesome-project",
  "github_token": "ghp_xxxxxxxxxxxx",
  "is_private": false,
  "description": "Built with Rust Lovable"
}
```

**Deploy to Vercel:**
```http
POST /api/v1/export/vercel
{
  "project_id": "project-123",
  "vercel_token": "xxxxxxxxxxxx",
  "project_name": "my-awesome-project",
  "framework": "react"
}
```

#### Streaming Endpoints

**AI Code Generation Stream:**
```http
GET /api/v1/stream/ai/generate/{project_id}
Accept: text/event-stream
```

**Apply AI Changes Stream:**
```http
GET /api/v1/stream/apply/{project_id}/{component_id}
Accept: text/event-stream
```

### WebSocket API

**Real-time Updates:**
```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onopen = () => {
  console.log('Connected to Rust Lovable');
  ws.send(JSON.stringify({
    type: 'subscribe',
    project_id: 'project-123'
  }));
};

ws.onmessage = (event) => {
  const update = JSON.parse(event.data);
  console.log('Received update:', update);
  
  switch (update.type) {
    case 'component_updated':
      // Handle component update
      break;
    case 'ai_response':
      // Handle AI response
      break;
    case 'vite_error':
      // Handle Vite error
      break;
  }
};

ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};

ws.onclose = () => {
  console.log('Disconnected from Rust Lovable');
};
```

### Error Handling

**Error Response Format:**
```json
{
  "success": false,
  "error": {
    "code": "INVALID_REQUEST",
    "message": "Invalid request format",
    "details": {
      "field": "name",
      "issue": "required",
      "suggestion": "Please provide a project name"
    },
    "timestamp": "2024-01-01T12:00:00Z",
    "request_id": "req_123456"
  }
}
```

## Emergency Procedures

### Service Outage

1. **Immediate Response (0-5 minutes):**
   ```bash
   # Check service status
   systemctl --user status rust-lovable
   
   # Attempt immediate restart
   systemctl --user restart rust-lovable
   
   # Check if service recovers
   curl -f http://localhost:8080/health
   ```

2. **Investigation (5-15 minutes):**
   ```bash
   # Check logs for errors
   journalctl --user -u rust-lovable -n 1000 | grep -i error
   
   # Check system resources
   htop
   df -h
   
   # Check network connectivity
   ping 8.8.8.8
   curl -I https://api.openai.com
   ```

3. **Recovery (15-30 minutes):**
   - If simple restart fails, check configuration
   - If configuration issue, restore from backup
   - If hardware issue, failover to backup system
   - If software bug, rollback to previous version

4. **Communication:**
   - Update status page
   - Notify stakeholders
   - Post incident report

### Data Corruption

1. **Immediate Response:**
   ```bash
   # Stop service
   systemctl --user stop rust-lovable
   
   # Assess damage
   sqlite3 ~/.local/share/rust-lovable/data.db "PRAGMA integrity_check;"
   
   # Check backup availability
   ls -la ~/.rust-lovable/backups/
   ```

2. **Recovery Process:**
   ```bash
   # Restore from backup
   cp ~/.rust-lovable/backups/data_20231201_120000.db ~/.local/share/rust-lovable/data.db
   
   # Repair if possible
   rust-lovable --repair-database
   
   # Verify integrity
   sqlite3 ~/.local/share/rust-lovable/data.db "PRAGMA integrity_check;"
   
   # Start service
   systemctl --user start rust-lovable
   
   # Test functionality
   rust-lovable --test-all
   ```

### Security Incident

1. **Immediate Response:**
   - Isolate affected systems
   - Preserve evidence
   - Notify security team
   - Change all API keys and passwords

2. **Investigation:**
   - Review access logs
   - Check for unauthorized changes
   - Analyze system state
   - Determine scope of breach

3. **Recovery:**
   - Patch vulnerabilities
   - Reset compromised credentials
   - Restore from clean backups
   - Implement additional security measures

4. **Post-Incident:**
   - Document findings
   - Update security procedures
   - Conduct security audit
   - Train team on new procedures

### Contact Information

**Emergency Contacts:**
- DevOps Team: devops@rust-lovable.com
- Security Team: security@rust-lovable.com
- On-Call Engineer: +1-555-0123
- Emergency Escalation: +1-555-0456

**Escalation Matrix:**
1. Level 1: Service degradation (15 minutes)
2. Level 2: Service outage (30 minutes)
3. Level 3: Security incident (immediate)
4. Level 4: Complete system failure (immediate)

## Advanced Features

### Real-Time Collaboration

**Enable Collaboration:**
```toml
[collaboration]
enabled = true
max_users_per_project = 10
enable_cursor_tracking = true
enable_voice_chat = false
```

**Collaboration Events:**
```javascript
// Subscribe to collaboration events
ws.send(JSON.stringify({
  type: 'subscribe_collaboration',
  project_id: 'project-123',
  user_id: 'user-456'
}));

// Handle collaboration events
ws.onmessage = (event) => {
  const update = JSON.parse(event.data);
  
  switch (update.type) {
    case 'user_joined':
      // Show user joined notification
      break;
    case 'user_left':
      // Show user left notification
      break;
    case 'cursor_moved':
      // Update cursor position
      break;
    case 'component_locked':
      // Show component as locked
      break;
    case 'component_unlocked':
      // Show component as unlocked
      break;
  }
};
```

### Custom Component Library

**Create Custom Component:**
```rust
use rust_lovable::core::ui_generator::{ComponentDefinition, PropertyDefinition, PropertyType};

let custom_component = ComponentDefinition {
    name: "CustomButton".to_string(),
    description: "A custom button with advanced features".to_string(),
    properties: vec![
        PropertyDefinition {
            name: "text".to_string(),
            property_type: PropertyType::String,
            default_value: Some("Click Me".into()),
            required: true,
            description: "Button text".to_string(),
        },
        PropertyDefinition {
            name: "variant".to_string(),
            property_type: PropertyType::Enum(vec![
                "primary".to_string(),
                "secondary".to_string(),
                "danger".to_string(),
            ]),
            default_value: Some("primary".into()),
            required: false,
            description: "Button variant".to_string(),
        },
    ],
    supported_platforms: vec![PlatformTarget::Web, PlatformTarget::Desktop, PlatformTarget::Mobile],
    code_templates: HashMap::new(),
};
```

### Plugin Development

**Create Plugin:**
```rust
use rust_lovable::plugin::{Plugin, PluginContext};

pub struct CustomPlugin;

impl Plugin for CustomPlugin {
    fn name(&self) -> String {
        "Custom Plugin".to_string()
    }
    
    fn version(&self) -> String {
        "1.0.0".to_string()
    }
    
    fn initialize(&self, context: &mut PluginContext) -> Result<(), PluginError> {
        // Initialize plugin
        context.register_component(custom_component);
        context.register_api_endpoint("/api/v1/custom", custom_handler);
        Ok(())
    }
    
    fn cleanup(&self) -> Result<(), PluginError> {
        // Cleanup resources
        Ok(())
    }
}
```

## Integration Management

### Future AI Integrations

**ruv-FANN Integration:**
```toml
[integrations.ruv-fann]
enabled = true
model_path = "/opt/ruv-fann/models"
acceleration = "cuda"
```

**ruvector Integration:**
```toml
[integrations.ruvector]
enabled = true
cluster_nodes = ["localhost:8081", "localhost:8082"]
replication_factor = 3
```

**QuDAG Integration:**
```toml
[integrations.qudag]
enabled = true
consensus_nodes = ["node1:9001", "node2:9002", "node3:9003"]
blockchain_backend = "rocksdb"
```

### GitHub Integration

**Automatic Repository Sync:**
```bash
# Configure GitHub integration
echo "ghp_xxxxxxxxxxxx" > ~/.rust-lovable/github-token

# Enable auto-sync
echo "true" > ~/.rust-lovable/github-auto-sync
```

**GitHub Actions Integration:**
```yaml
# .github/workflows/deploy.yml
name: Deploy from Rust Lovable

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Download from Rust Lovable
        run: |
          curl -X GET "https://api.rust-lovable.com/api/v1/export/zip" \
            -H "Authorization: Bearer ${{ secrets.RUST_LOVABLE_TOKEN }}" \
            --output project.zip
          unzip project.zip
      
      - name: Deploy to Vercel
        run: |
          npx vercel --token ${{ secrets.VERCEL_TOKEN }} --prod
```

### SSO Integration

**OIDC Configuration:**
```toml
[sso.oidc]
enabled = true
provider = "auth0"
client_id = "your-client-id"
client_secret = "your-client-secret"
issuer_url = "https://your-domain.auth0.com"
redirect_url = "https://app.rust-lovable.com/callback"
scopes = ["openid", "profile", "email"]
```

**SAML Configuration:**
```toml
[sso.saml]
enabled = true
provider = "okta"
entity_id = "https://your-domain.okta.com"
sso_url = "https://your-domain.okta.com/app/saml/sso"
certificate_file = "/path/to/certificate.pem"
```

---

This runbook is a living document. Please keep it updated as the system evolves and new features are added.