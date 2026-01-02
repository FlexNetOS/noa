# AgentGateway Integration Documentation

## Overview

This directory contains the **AgentGateway** Rust workspace, which serves as the foundation for NOA's MCP (Model Context Protocol) and A2A (Agent-to-Agent) gateway infrastructure.

AgentGateway provides the complete connectivity solution for agentic AI within the NOA ecosystem.

## What is AgentGateway?

AgentGateway is an open-source data plane optimized for agentic AI connectivity. It provides:

- **MCP (Model Context Protocol) Support** - Industry-standard protocol for agent-tool communication
- **A2A (Agent-to-Agent) Protocol** - Google's agent interoperability protocol
- **RBAC & Security** - Robust role-based access control
- **Dynamic Configuration** - xDS-based configuration without downtime
- **Multi-Tenant Support** - Isolated environments for different users/orgs
- **High Performance** - Written in Rust for maximum efficiency
- **Observability** - Built-in metrics and monitoring

## Architecture Integration

AgentGateway fits into the NOA unified architecture as follows:

```
NOA Gateway Layer
├─ gateway/mcp/agentgateway/     ← Core agentgateway (this directory)
├─ gateway/mcp/proxy/            ← Single ingress using agentgateway
├─ gateway/mcp/registry/         ← Tool/server discovery
├─ gateway/mcp/routing/          ← Locality-based routing
├─ gateway/mcp/authz/            ← Capability-based authorization
└─ gateway/mcp/connectors/       ← MCP tool servers
```

### How It Works

1. **Agents** make requests through the NOA orchestrator
2. **Gateway Proxy** (using agentgateway) receives the requests
3. **Routing Layer** determines locality (local → personal → regional → org)
4. **Authz Layer** checks capabilities and permissions
5. **Registry** resolves tool/server endpoints
6. **Connectors** execute the actual tool calls via MCP
7. **Results** flow back through the gateway with audit trails

## Crates

The AgentGateway workspace contains 9 crates:

### Core Crates

- **`agentgateway`** - Main gateway implementation with LLM support
- **`agentgateway-app`** - Application layer and CLI
- **`core`** - Core utilities and shared types

### Protocol & SDK Crates

- **`a2a-sdk`** - Agent-to-Agent SDK for A2A protocol
- **`hbone`** - H-BONE (HTTP-Based Overlay Network Environment) protocol
- **`xds`** - xDS (Discovery Service) for dynamic configuration

### Utility Crates

- **`celx`** - CEL (Common Expression Language) support for policies
- **`hyper-util-fork`** - HTTP utilities
- **`xtask`** - Build automation tasks

## Building

### Prerequisites

- Rust 1.90+ (defined in `rust-toolchain.toml`)
- Cargo workspace support

### Build Commands

```bash
# Build all crates in release mode
cd gateway/mcp/agentgateway
cargo build --release

# Build specific crate
cargo build --release --bin agentgateway-app

# Run tests
cargo test

# Run with specific features
cargo build --release --features "tls,metrics"
```

### Build Profiles

- **`quick-release`** - Faster incremental builds for development
- **`release`** - Full optimizations with LTO for production
- **`bench`** - Profiling-enabled benchmarks

## Configuration

AgentGateway is configured through the NOA config system:

### Main Config Location

`/n/noa/configs/semantic/capabilities/agentgateway.json` (to be created)

### Integration with NOA Configs

- **Base Layer** (`configs/base/`) - Immutable schema definitions
- **Semantic Layer** (`configs/semantic/`) - Runtime configuration
- **Enforcement Layer** (`configs/enforcement/`) - Policy validation

### Key Configuration Areas

1. **Proxy Configuration** - `gateway/mcp/proxy/config.json`
2. **Registry Configuration** - `gateway/mcp/registry/config.json`
3. **Routing Configuration** - `gateway/mcp/routing/config.json`
4. **Authorization Configuration** - `gateway/mcp/authz/config.json`

## Running AgentGateway

### Development Mode

```bash
cd gateway/mcp/agentgateway
cargo run --bin agentgateway-app -- --config ${NOA_ROOT}/configs/semantic/capabilities/agentgateway.json
```

### Production Mode

```bash
cd gateway/mcp/agentgateway
cargo build --release
./target/release/agentgateway-app --config ${NOA_ROOT}/configs/semantic/capabilities/agentgateway.json
```

### Docker (if needed)

```bash
cd gateway/mcp/agentgateway
docker build -t noa-agentgateway .
docker run -p 9090:9090 noa-agentgateway
```

## MCP Integration

AgentGateway provides native MCP support through the `rmcp` crate:

### MCP Transport Support

- **SSE (Server-Sent Events)** - Streaming responses
- **HTTP** - Standard request/response
- **Child Process** - Local tool execution
- **WebSocket** (future)

### Creating MCP Connectors

See `gateway/mcp/connectors/` for examples of MCP tool servers that integrate with agentgateway.

## A2A Integration

Agent-to-Agent protocol support enables NOA agents to communicate with:

- Google's A2A ecosystem
- Other A2A-compatible agent frameworks
- Cross-platform agent collaboration

## Security & RBAC

AgentGateway includes a robust RBAC system:

### Capability-Based Access Control

Defined in `gateway/mcp/authz/config.json`:

- **Capabilities** - What actions can be performed
- **Providers** - Who can use which providers
- **Budgets** - Rate limits and cost controls

### Authentication Methods

- **API Keys** - For external tool access
- **mTLS** - For service-to-service
- **OAuth** (via desktop apps) - For user authentication

## Monitoring & Observability

### Metrics

AgentGateway exports metrics compatible with:

- Prometheus (via `prometheus-client`)
- OpenTelemetry (via `opentelemetry-otlp`)

### Logging

Structured logging via `tracing`:

```bash
# Set log level
export RUST_LOG=agentgateway=debug,info

# Run with verbose logging
cargo run --release 2>&1 | tee gateway.log
```

### Health Checks

Built-in health endpoints for monitoring:

- `/health` - Basic health check
- `/ready` - Readiness probe
- `/metrics` - Prometheus metrics

## Dynamic Configuration (xDS)

AgentGateway supports xDS for dynamic configuration updates:

- **No Downtime** - Updates applied without restarts
- **Gradual Rollout** - Canary deployments
- **Version Pinning** - Rollback capability

## UI Dashboard

AgentGateway includes a built-in UI for exploration and debugging:

- **Agent-to-Agent Visualization** - See A2A communication flows
- **Tool Call Inspector** - Debug MCP tool calls
- **Performance Metrics** - Real-time dashboards

Access at: `http://localhost:9090/ui` (when running)

## Integration Checklist

- [x] AgentGateway workspace copied to `gateway/mcp/agentgateway/`
- [x] Directory structure created
- [ ] Main config created (`configs/semantic/capabilities/agentgateway.json`)
- [ ] Proxy layer configured
- [ ] Registry configured with tool discovery
- [ ] Routing configured with locality preferences
- [ ] Authorization configured with capabilities
- [ ] First MCP connector created
- [ ] Build validated (`cargo build --release`)
- [ ] Tests passing (`cargo test`)
- [ ] Integration test with NOA orchestrator

## Troubleshooting

### Build Errors

If you encounter build errors:

```bash
# Clean and rebuild
cargo clean
cargo build --release

# Update dependencies
cargo update
```

### Common Issues

**Issue:** Missing dependencies
**Solution:** Ensure all workspace members are present

**Issue:** xDS connection errors
**Solution:** Check `xds` crate configuration

**Issue:** MCP transport failures
**Solution:** Verify `rmcp` transport configuration

## Next Steps

1. **Create Main Config** - Define agentgateway runtime config
2. **Build Connectors** - Create MCP tool servers for providers
3. **Configure Routing** - Set up locality-based routing
4. **Enable Authz** - Implement capability-based access control
5. **Add Monitoring** - Set up metrics collection
6. **Integration Test** - Test with NOA orchestrator

## Resources

- **AgentGateway GitHub:** https://github.com/agentgateway/agentgateway
- **AgentGateway Docs:** https://agentgateway.dev/docs
- **MCP Specification:** https://modelcontextprotocol.io
- **A2A Protocol:** https://developers.googleblog.com/en/a2a-a-new-era-of-agent-interoperability
- **NOA Architecture:** `/n/noa/README.md`

## License

AgentGateway is licensed under Apache 2.0.

---

**Integration Status:** ✅ Deployed | ⏳ Configuration Pending | 📋 Testing Required
