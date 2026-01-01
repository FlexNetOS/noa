# API Endpoints

REST API reference for NOA.

## Base URL

```
http://localhost:8080/api/v1
```

## Authentication

Currently uses local-only access. Future versions will support API keys.

## Endpoints

### Health

#### GET /health

Check server health.

**Response:**
```json
{
  "status": "ok",
  "version": "0.1.0",
  "uptime_seconds": 3600
}
```

### Agents

#### GET /agents

List all agents.

**Response:**
```json
{
  "agents": [
    {
      "id": "commander-chief",
      "kind": "commander-chief",
      "status": "running",
      "capabilities": ["plan", "delegate", "verify"]
    }
  ]
}
```

#### GET /agents/{id}

Get agent details.

**Path Parameters:**
- `id` - Agent ID

**Response:**
```json
{
  "id": "file-io",
  "kind": "file-io",
  "status": "idle",
  "capabilities": ["read", "write", "search"],
  "stats": {
    "tasks_completed": 42,
    "avg_duration_ms": 150
  }
}
```

#### POST /agents

Register a new agent.

**Request Body:**
```json
{
  "kind": "custom",
  "name": "my-agent",
  "config": {}
}
```

### Tasks

#### POST /tasks

Execute a task.

**Request Body:**
```json
{
  "agent": "file-io",
  "action": "read",
  "input": {
    "path": "./README.md"
  }
}
```

**Response:**
```json
{
  "task_id": "task_abc123",
  "status": "completed",
  "result": {
    "content": "# NOA\n..."
  }
}
```

#### GET /tasks/{id}

Get task status.

**Path Parameters:**
- `id` - Task ID

**Response:**
```json
{
  "task_id": "task_abc123",
  "status": "completed",
  "started_at": "2024-01-15T10:30:00Z",
  "completed_at": "2024-01-15T10:30:01Z",
  "result": {}
}
```

#### DELETE /tasks/{id}

Cancel a task.

**Path Parameters:**
- `id` - Task ID

### Chat

#### POST /chat

Send a chat message.

**Request Body:**
```json
{
  "messages": [
    {"role": "user", "content": "Hello"}
  ],
  "model": "qwen2.5-coder-7b",
  "stream": false
}
```

**Response:**
```json
{
  "message": {
    "role": "assistant",
    "content": "Hello! How can I help you?"
  },
  "usage": {
    "prompt_tokens": 10,
    "completion_tokens": 8
  }
}
```

#### POST /chat (Streaming)

Send a streaming chat message.

**Request Body:**
```json
{
  "messages": [
    {"role": "user", "content": "Hello"}
  ],
  "stream": true
}
```

**Response:** Server-Sent Events
```
data: {"delta": "Hello"}
data: {"delta": "!"}
data: {"done": true}
```

### Embeddings

#### POST /embeddings

Generate embeddings.

**Request Body:**
```json
{
  "input": "Hello, world!",
  "model": "nomic-embed-text"
}
```

**Response:**
```json
{
  "embeddings": [[0.1, 0.2, ...]]
}
```

### Models

#### GET /models

List available models.

**Response:**
```json
{
  "models": [
    {
      "id": "qwen2.5-coder-7b",
      "type": "chat",
      "size_bytes": 4500000000
    }
  ]
}
```

### P2P

#### GET /p2p/status

Get P2P network status.

**Response:**
```json
{
  "peer_id": "12D3KooW...",
  "connected_peers": 5,
  "listen_addresses": ["/ip4/0.0.0.0/tcp/4001"],
  "external_address": "/ip4/1.2.3.4/tcp/4001"
}
```

## Error Responses

All errors follow this format:

```json
{
  "error": {
    "code": "AGENT_NOT_FOUND",
    "message": "Agent 'foo' not found",
    "details": {}
  }
}
```

### Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `INVALID_REQUEST` | 400 | Bad request |
| `AGENT_NOT_FOUND` | 404 | Agent not found |
| `TASK_NOT_FOUND` | 404 | Task not found |
| `TASK_TIMEOUT` | 408 | Task timed out |
| `RATE_LIMITED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Server error |

## See Also

- [CLI Reference](cli.md)
- [API Module](../wiki/internal-crates/sys-core/api.md)
