# MCP Connectors

This directory contains MCP (Model Context Protocol) connectors that bridge NOA's gateway to various providers and tools.

## Overview

Connectors are the implementation layer that translates between:
- **NOA's gateway** (agentgateway-based MCP proxy)
- **Providers** (Claude Code, Codex, Llama.cpp, etc.)
- **Tools** (Spec-kit, code-scan, build-test, etc.)

Each connector implements the MCP protocol and provides a specific capability boundary.

## Directory Structure

```
connectors/
├─ provider-connectors/       # Connectors to AI providers
│  ├─ claude-code.json        # Claude Code CLI connector
│  ├─ codex.json              # OpenAI Codex connector
│  ├─ llama-cpp.json          # Local llama.cpp connector
│  └─ ...
└─ tool-connectors/           # Connectors to tools
   ├─ spec-kit.json           # Spec-kit tool connector
   ├─ code-scan.json          # Code scanning tool connector
   └─ ...
```

## Connector Types

### Provider Connectors

Connect to AI model providers (local or remote):

- **claude-code.json** - Anthropic Claude Code CLI
- **codex.json** - OpenAI Codex CLI
- **llama-cpp.json** - Local llama.cpp inference
- **copilot.json** - GitHub Copilot (future)
- **ollama.json** - Ollama local models (future)

### Tool Connectors

Connect to specialized tools:

- **spec-kit** - Specification and documentation tools
- **code-scan** - Code analysis and scanning
- **build-test** - Build and test automation
- **db** - Database operations (read/write)
- **vector** - Vector database operations
- **cas** - Content-addressed storage

## MCP Protocol Support

All connectors implement the MCP protocol with support for:

### Core Features

- **Tools** - Callable functions/operations
- **Resources** - Data sources and files
- **Prompts** - Template-based interactions
- **Streaming** - Real-time response streaming

### Transports

- **HTTP** - Standard HTTP/HTTPS requests
- **SSE** - Server-Sent Events for streaming
- **Child Process** - Local process execution
- **WebSocket** - Bidirectional communication (future)

## Configuration Schema

Each connector follows this schema:

```json
{
  "version": "1.0.0",
  "description": "Connector description",
  "metadata": {
    "version": "1.0.0",
    "updated_at": "2026-01-02T00:00:00Z"
  },
  "connector": {
    "id": "unique-connector-id",
    "name": "Human Readable Name",
    "type": "provider|tool",
    "provider_id": "provider-path",
    "protocol": "mcp",
    "transport": "http|sse|child-process"
  },
  "capabilities": ["list", "of", "capabilities"],
  "command": {
    "executable": "path-to-binary",
    "args": ["arg1", "arg2"],
    "env": {},
    "working_dir": "work-dir",
    "timeout_seconds": 300
  },
  "mcp_config": {
    "version": "1.0",
    "tools_enabled": true,
    "resources_enabled": true,
    "prompts_enabled": true
  },
  "routing": {
    "locality": "local|remote",
    "fallback_provider": "provider-id",
    "sticky_sessions": true|false
  },
  "rate_limiting": {
    "enabled": true,
    "max_rps": 100
  },
  "monitoring": {
    "enabled": true,
    "track_latency": true
  }
}
```

## Creating a New Connector

### Step 1: Create Configuration File

```bash
# For provider connector
cp provider-connectors/template.json provider-connectors/my-provider.json

# For tool connector
cp tool-connectors/template.json tool-connectors/my-tool.json
```

### Step 2: Configure Connector

Edit the JSON file with your connector details:

1. **Set unique ID and name**
2. **Define capabilities**
3. **Configure command execution**
4. **Set MCP protocol options**
5. **Configure routing and fallback**
6. **Set rate limits**

### Step 3: Register Connector

Connectors are auto-discovered by the gateway registry:

```bash
# Registry will scan and register automatically
# Or manually trigger registration
curl -X POST http://localhost:9090/registry/scan
```

### Step 4: Test Connector

```bash
# Test MCP endpoint
curl -X POST http://localhost:9090/mcp/invoke \
  -H "Content-Type: application/json" \
  -d '{
    "connector": "my-provider",
    "tool": "generate_code",
    "params": {"prompt": "Hello world in Python"}
  }'
```

## Connector Lifecycle

1. **Discovery** - Registry scans connector directories
2. **Validation** - Config validated against schema
3. **Registration** - Added to registry database
4. **Initialization** - Command executed (if needed)
5. **Ready** - Available for routing
6. **Monitoring** - Health checks and metrics
7. **Deregistration** - Removed when disabled

## Routing Integration

Connectors integrate with the routing layer:

```
Request → Gateway Proxy → Routing Layer → Connector Selection → Execution
```

### Routing Rules

1. **Capability Match** - Filter by required capabilities
2. **Locality Preference** - Prefer local over remote
3. **Load Balancing** - Distribute across available connectors
4. **Fallback** - Use fallback on failure
5. **Circuit Breaking** - Disable unhealthy connectors

## Authorization Integration

Connectors respect capability-based authorization:

```
Request → Authz Check → Capability Validation → Connector Allowed → Execute
```

Only connectors with allowed capabilities can be invoked.

## Monitoring

All connectors export metrics:

- **Latency** - Request/response time
- **Success Rate** - % successful calls
- **Token Usage** - For AI providers
- **Error Rate** - Failed requests
- **Throughput** - Requests per second

Metrics exported to: `${NOA_ROOT}/data/logs/connector-metrics.json`

## Best Practices

### Performance

- Use local connectors for latency-sensitive operations
- Enable KV caching for repeated context
- Configure appropriate timeouts
- Use streaming for long responses

### Reliability

- Set fallback providers
- Configure circuit breakers
- Implement retry logic
- Monitor health checks

### Security

- Use environment variables for secrets
- Enable authorization
- Audit all tool calls
- Sandbox code execution

### Monitoring

- Track all key metrics
- Set up alerts for failures
- Monitor resource usage
- Log important events

## Troubleshooting

### Connector Not Found

**Symptom:** Registry can't find connector
**Solution:**
- Check file location in `connectors/` directory
- Verify JSON syntax
- Check registry scan logs

### Command Execution Fails

**Symptom:** Connector can't execute command
**Solution:**
- Verify executable path
- Check environment variables
- Ensure binary has execute permissions
- Review command logs

### MCP Protocol Errors

**Symptom:** MCP communication fails
**Solution:**
- Verify MCP version compatibility
- Check transport configuration
- Review message size limits
- Inspect protocol logs

### Rate Limiting

**Symptom:** Requests rejected
**Solution:**
- Check rate limit config
- Review burst settings
- Monitor actual RPS
- Adjust limits if needed

## Examples

### Invoke Claude Code Connector

```bash
curl -X POST http://localhost:9090/mcp/invoke \
  -H "Content-Type: application/json" \
  -d '{
    "connector": "claude-code-connector",
    "tool": "reasoning",
    "params": {
      "prompt": "Explain how quicksort works",
      "max_tokens": 1000
    }
  }'
```

### Invoke Llama.cpp Local Connector

```bash
curl -X POST http://localhost:9090/mcp/invoke \
  -H "Content-Type: application/json" \
  -d '{
    "connector": "llama-cpp-connector",
    "tool": "chat_completion",
    "params": {
      "messages": [
        {"role": "user", "content": "Hello!"}
      ],
      "stream": true
    }
  }'
```

## Integration with NOA

Connectors integrate with the full NOA architecture:

```
┌─────────────────────────────────────────────────┐
│ NOA Orchestrator                                │
└────────────────┬────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────┐
│ Gateway MCP Proxy (AgentGateway)                │
├─────────────────────────────────────────────────┤
│ • Routing Layer (locality-based)                │
│ • Authz Layer (capability-based)                │
│ • Registry (discovery & trust)                  │
└────────────────┬────────────────────────────────┘
                 │
        ┌────────┴────────┐
        │                 │
┌───────▼──────┐  ┌──────▼────────┐
│ Provider     │  │ Tool          │
│ Connectors   │  │ Connectors    │
├──────────────┤  ├───────────────┤
│ • Claude     │  │ • Spec-kit    │
│ • Codex      │  │ • Code-scan   │
│ • Llama.cpp  │  │ • Build-test  │
└──────────────┘  └───────────────┘
```

## Next Steps

1. **Implement wrapper scripts** for child-process connectors
2. **Add health check endpoints** for all connectors
3. **Create integration tests** for each connector
4. **Set up monitoring dashboards** for metrics
5. **Document tool-specific APIs** for each connector

---

**Status:** ✅ Structure Created | ⏳ Implementations Pending | 📋 Testing Required
