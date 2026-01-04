# Third-Party Tool Integration Guide

**Version**: 1.0.0
**Last Updated**: 2026-01-02

---

## Overview

This guide covers integration of third-party tools into the NOA framework. Phase 6 focuses on four key integrations:

1. **MCP SDK** - Model Context Protocol implementation
2. **Qdrant** - Vector database for embeddings
3. **SQLx** - Async SQL database toolkit
4. **libp2p** - Peer-to-peer networking

---

## 1. MCP SDK Integration

### Purpose

Implements the Model Context Protocol (MCP) for standardized tool and resource discovery.

### configsuration

**Location**: `tools/third-party/mcp-sdk/configs.json`

### Installation

```bash
# Add to Cargo.toml
[dependencies]
serde = "1.0"
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"
```

### Key Components

#### 1. MCP Server

Exposes NOA tools as MCP-compliant endpoints:

```rust
// Example server implementation
use mcp_sdk::Server;

let server = Server::new("noa-mcp-server", "1.0.0")
    .with_tool("read_file", read_file_handler)
    .with_tool("write_file", write_file_handler)
    .with_resource("cas_objects", cas_resource_handler);
```

#### 2. MCP Client

Connects to external MCP servers:

```rust
// Example client implementation
use mcp_sdk::Client;

let client = Client::connect("stdio", command).await?;
let tools = client.list_tools().await?;
```

#### 3. Tool Registration

Auto-registers tools from resource registry:

```json
{
  "tool_registration": {
    "auto_discover": true,
    "registry_path": "${NOA_ROOT}/data/resources/registry.json",
    "tool_prefix": "noa:"
  }
}
```

### Integration Points

- **Gateway**: `gateway/mcp/` - Protocol implementation
- **Tools**: `tools/` - Tool registration
- **Providers**: `providers/` - Provider MCP endpoints

### Use Cases

1. **Tool Discovery**: List available tools via MCP
2. **Remote Tool Execution**: Execute tools on remote MCP servers
3. **Resource Access**: Access CAS objects via MCP resources
4. **Provider Integration**: Connect Claude Code CLI, Codex via MCP

---

## 2. Qdrant Integration

### Purpose

Vector database for semantic search, code search, and RAG (Retrieval-Augmented Generation).

### configsuration

**Location**: `tools/third-party/qdrant/configs.json`

### Installation

```bash
# Add to Cargo.toml
[dependencies]
qdrant-client = "1.7.0"

# Start Qdrant server
docker run -p 6333:6333 -p 6334:6334 qdrant/qdrant
```

### Collections

#### 1. noa_embeddings (General Purpose)

```json
{
  "name": "noa_embeddings",
  "vector_size": 768,
  "distance": "Cosine"
}
```

**Use Cases**:
- Document similarity search
- Semantic code search
- Knowledge base retrieval

#### 2. noa_code_embeddings (Code-Specific)

```json
{
  "name": "noa_code_embeddings",
  "vector_size": 384,
  "distance": "Cosine",
  "payload_schema": {
    "file_path": "keyword",
    "language": "keyword",
    "function_name": "text"
  }
}
```

**Use Cases**:
- Find similar functions
- Code completion context
- Duplicate code detection

#### 3. noa_document_embeddings (RAG)

```json
{
  "name": "noa_document_embeddings",
  "vector_size": 768,
  "distance": "Cosine",
  "payload_schema": {
    "title": "text",
    "content": "text",
    "tags": "keyword"
  }
}
```

**Use Cases**:
- Retrieval-augmented generation
- Documentation search
- Context injection for LLMs

### Operations

#### Upsert Embeddings

```bash
# Example: Insert code embedding
curl -X PUT "http://localhost:6334/collections/noa_code_embeddings/points" \
  -H "Content-Type: application/json" \
  -d '{
    "points": [{
      "id": 1,
      "vector": [0.1, 0.2, ..., 0.3],
      "payload": {
        "file_path": "/src/main.rs",
        "language": "rust",
        "function_name": "main"
      }
    }]
  }'
```

#### Search

```bash
# Example: Search for similar code
curl -X POST "http://localhost:6334/collections/noa_code_embeddings/points/search" \
  -H "Content-Type: application/json" \
  -d '{
    "vector": [0.1, 0.2, ..., 0.3],
    "limit": 10,
    "score_threshold": 0.7
  }'
```

### Integration Points

- **Embedding Cache**: `providers/shared/embedding-cache/` - Caches embeddings
- **Agent Templates**: Embedding agent uses Qdrant for storage
- **RAG Pipeline**: Document retrieval for context injection

---

## 3. SQLx Integration

### Purpose

Async SQL database for persistent storage, audit archival, and analytics.

### configsuration

**Location**: `tools/third-party/sqlx/configs.json`

### Installation

```bash
# Add to Cargo.toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "sqlite"] }

# Install SQLx CLI
cargo install sqlx-cli
```

### Databases

#### PostgreSQL (Production)

```bash
# Connection string
DATABASE_URL=postgresql://localhost/noa

# Create database
sqlx database create

# Run migrations
sqlx migrate run
```

**Tables**:
- `audit_logs` - Archived audit events
- `budget_tracking` - Cost tracking
- Additional analytics tables

#### SQLite (Development/Local)

```bash
# Database path
${NOA_ROOT}/data/db/noa.db

# Auto-creates on first connection
```

**Tables**:
- `agent_registry` - Active agents
- `model_registry` - Model deployments
- `cache_stats` - Cache usage stats

### Schema Migrations

**Location**: `data/db/migrations/`

```sql
-- migrations/20260102000001_create_audit_logs.sql
CREATE TABLE audit_logs (
    id BIGSERIAL PRIMARY KEY,
    timestamp TIMESTAMP NOT NULL,
    event_type VARCHAR(50) NOT NULL,
    principal_id VARCHAR(100) NOT NULL,
    metadata JSONB,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_audit_timestamp ON audit_logs(timestamp);
CREATE INDEX idx_audit_principal ON audit_logs(principal_id);
```

### Operations

#### Audit Archival

**Schedule**: Daily at 3 AM

```rust
// Archive logs older than 90 days
let threshold = now() - Duration::days(90);
let logs = read_audit_logs_since(threshold)?;

for log in logs {
    sqlx::query!(
        "INSERT INTO audit_logs (timestamp, event_type, principal_id, metadata)
         VALUES ($1, $2, $3, $4)",
        log.timestamp,
        log.event_type,
        log.principal_id,
        log.metadata
    )
    .execute(&pool)
    .await?;
}
```

#### Budget Queries

```sql
-- Get budget summary for principal
SELECT
    principal_id,
    SUM(amount_usd) as total_spent,
    COUNT(*) as operation_count
FROM budget_tracking
WHERE principal_id = 'agent:default'
  AND timestamp >= NOW() - INTERVAL '30 days'
GROUP BY principal_id;
```

### Integration Points

- **Audit System**: `sys/core/audit/` - Archives logs to database
- **Budget Tracking**: Stores budget data for analytics
- **Registry Persistence**: Backs up registry to SQL

---

## 4. libp2p Integration

### Purpose

Peer-to-peer networking for distributed NOA instances, agent discovery, and model sharing.

### configsuration

**Location**: `tools/third-party/libp2p/configs.json`

### Installation

```bash
# Add to Cargo.toml
[dependencies]
libp2p = { version = "0.53", features = [
    "tcp",
    "websocket",
    "noise",
    "yamux",
    "kad",
    "mdns",
    "gossipsub",
    "request-response"
] }
```

### Protocols

#### 1. Kademlia (DHT)

**Purpose**: Peer and content discovery

```rust
use libp2p::kad::{Kademlia, Kademliaconfigs};

let mut kad = Kademlia::new(peer_id, store);
kad.bootstrap()?;
```

**Use Cases**:
- Discover agent instances
- Find model providers
- DHT-based CAS object discovery

#### 2. mDNS

**Purpose**: Local network discovery

```rust
use libp2p::mdns::Mdns;

let mdns = Mdns::new(Default::default())?;
```

**Use Cases**:
- Discover NOA instances on LAN
- Zero-configs local collaboration

#### 3. GossipSub

**Purpose**: Pub/sub messaging

```rust
use libp2p::gossipsub::{Gossipsub, Gossipsubconfigs};

let mut gossipsub = Gossipsub::new(
    libp2p::gossipsub::MessageAuthenticity::Signed(local_key),
    Gossipsubconfigs::default()
)?;

gossipsub.subscribe(&"noa/agents/events")?;
```

**Topics**:
- `noa/agents/events` - Agent lifecycle events
- `noa/models/updates` - Model deployment notifications
- `noa/tasks/broadcast` - Task coordination

#### 4. Request-Response

**Purpose**: RPC-style communication

```rust
use libp2p::request_response::{RequestResponse, ProtocolSupport};

let protocol = RequestResponseCodec::new("/noa/rpc/1.0.0");
let behaviour = RequestResponse::new(protocol, ProtocolSupport::Full);
```

**Use Cases**:
- Request model from peer
- Execute remote agent task
- Fetch CAS object from peer

### Use Cases

#### Agent Discovery

```rust
// Publish agent availability
kad.start_providing(agent_id.into())?;

// Discover agents
let providers = kad.get_providers(capability.into());
```

#### Model Sharing

```rust
// Provide model
kad.start_providing(model_hash.into())?;

// Request model from peer
let request = FetchModelRequest { hash: model_hash };
behaviour.send_request(&peer_id, request);
```

#### Task Coordination

```rust
// Broadcast task
gossipsub.publish("noa/tasks/broadcast", task_json.as_bytes())?;

// Subscribe to task updates
gossipsub.subscribe(&"noa/tasks/broadcast")?;
```

### Integration Points

- **CAS**: Distributed object storage and retrieval
- **Agent Registry**: Discover agents across network
- **Model Registry**: Share models between instances

---

## Integration Testing

### Test Suite

**Location**: `scripts/tests/test-integrations.sh`

```bash
#!/bin/bash
# Test third-party integrations

# Test MCP SDK
echo "Testing MCP SDK..."
# TODO: MCP server/client tests

# Test Qdrant
echo "Testing Qdrant..."
curl -f http://localhost:6334/collections || echo "Qdrant not running"

# Test SQLx
echo "Testing SQLx..."
sqlx database drop -y && sqlx database create && sqlx migrate run

# Test libp2p
echo "Testing libp2p..."
# TODO: libp2p network tests

echo "All integration tests complete"
```

### Manual Testing

#### MCP SDK

```bash
# Start MCP server
cargo run --bin noa-mcp-server

# Test tool listing
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | \
  cargo run --bin noa-mcp-client
```

#### Qdrant

```bash
# Create collection
curl -X PUT "http://localhost:6334/collections/test" \
  -H "Content-Type: application/json" \
  -d '{"vectors":{"size":768,"distance":"Cosine"}}'

# Insert point
curl -X PUT "http://localhost:6334/collections/test/points" \
  -H "Content-Type: application/json" \
  -d '{"points":[{"id":1,"vector":[0.1,0.2,0.3,...]}]}'

# Search
curl -X POST "http://localhost:6334/collections/test/points/search" \
  -H "Content-Type: application/json" \
  -d '{"vector":[0.1,0.2,0.3,...],"limit":5}'
```

#### SQLx

```bash
# Run migrations
cd /n/noa
sqlx migrate run --database-url sqlite:data/db/noa.db

# Query database
sqlite3 data/db/noa.db "SELECT * FROM agent_registry;"
```

#### libp2p

```bash
# Start first peer
LIBP2P_TCP_PORT=9000 cargo run --bin noa-p2p-node

# Start second peer
LIBP2P_TCP_PORT=9010 cargo run --bin noa-p2p-node

# Check peer discovery via logs
```

---

## Security Considerations

### MCP SDK

- **Authentication**: Require authentication for MCP requests
- **Capability Enforcement**: Validate capabilities before tool execution
- **Audit**: Log all MCP tool executions

### Qdrant

- **API Key**: Use `QDRANT_API_KEY` for production
- **Network Isolation**: Run Qdrant on private network
- **Access Control**: Limit collection access by capability

### SQLx

- **Prepared Statements**: Always use prepared statements (automatic with SQLx)
- **Connection Pooling**: Limit max connections
- **Encryption**: Use SSL/TLS for PostgreSQL connections

### libp2p

- **Transport Encryption**: Enable Noise protocol
- **Peer Authentication**: Verify peer IDs
- **Message Signing**: Sign and verify all messages
- **Rate Limiting**: Limit incoming connections

---

## Performance Tuning

### Qdrant

```json
{
  "hnsw_configs": {
    "m": 16,              // Connections per layer (higher = better recall, slower)
    "ef_construct": 100,  // Construction time (higher = better index)
    "full_scan_threshold": 10000  // Switch to exact search below this
  }
}
```

### SQLx

```json
{
  "max_connections": 10,
  "min_connections": 2,
  "idle_timeout_seconds": 600,
  "connect_timeout_seconds": 30
}
```

### libp2p

```json
{
  "max_connections": 50,
  "max_incoming_per_second": 10,
  "dht_replication_factor": 20
}
```

---

## Troubleshooting

### MCP SDK

**Issue**: Tool not discovered
```bash
# Check registry
jq '.tool_definitions' data/resources/registry.json

# Verify MCP server running
curl http://localhost:8080/health
```

### Qdrant

**Issue**: Collection not found
```bash
# List collections
curl http://localhost:6334/collections

# Create collection
curl -X PUT http://localhost:6334/collections/noa_embeddings \
  -H "Content-Type: application/json" \
  -d '{"vectors":{"size":768,"distance":"Cosine"}}'
```

### SQLx

**Issue**: Migration failed
```bash
# Reset database
sqlx database drop -y
sqlx database create
sqlx migrate run

# Check migration status
sqlx migrate info
```

### libp2p

**Issue**: Peer not connecting
```bash
# Check listening addresses
# Should see: Listening on /ip4/0.0.0.0/tcp/9000

# Verify firewall
sudo ufw allow 9000/tcp

# Check bootstrap peers
# Ensure at least one bootstrap peer is reachable
```

---

## References

- [MCP Specification](https://modelcontextprotocol.io/specification)
- [Qdrant Documentation](https://qdrant.tech/documentation/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [libp2p Documentation](https://docs.libp2p.io/)

---

**Last Updated**: 2026-01-02
