# Identity Management - NOA System Core

**Version**: 1.0.0
**Location**: `sys/core/identity/`
**Purpose**: Trusted identity and capability management for NOA microkernel

---

## Overview

The NOA identity system provides:
- **Principal Management**: System, agent, and user identities
- **Role-Based Access Control (RBAC)**: Hierarchical role system
- **Capability-Based Authorization**: Fine-grained permission model
- **Authentication**: Multiple auth methods (service account, API key, OIDC)
- **Authorization**: Capability-based RBAC with default deny

---

## Architecture

### Identity Model

```
Principal (system/agent/user)
  ├─> Roles (system_admin, agent, user, readonly)
  │    └─> Capabilities (reasoning, code_generation, etc.)
  │         └─> Resource Types + Operations
  └─> Constraints (sandbox_required, audit_required, budget_limited)
```

### Principal Types

| Type | ID Pattern | Purpose |
|------|-----------|---------|
| System | `sys:noa` | NOA system processes |
| Agent | `agent:*` | AI agents (Claude, etc.) |
| User | `user:${USERNAME}` | Human users |

---

## Configuration

### Identity Configuration

Located at: `sys/core/identity/identity.json`

**Key Sections**:
```json
{
  "identity_providers": {
    "local": { /* Filesystem-based identities */ },
    "remote": { /* OIDC provider */ }
  },
  "principals": {
    "system": { /* System principal */ },
    "agent": { /* Default agent principal */ },
    "user": { /* User template */ }
  },
  "roles": {
    "system_admin": { /* Full access */ },
    "agent": { /* AI agent capabilities */ },
    "user": { /* Standard user */ },
    "readonly": { /* Monitoring only */ }
  },
  "capabilities": {
    "reasoning": { /* LLM inference */ },
    "code_generation": { /* Code generation */ },
    "code_execution": { /* Sandbox execution */ },
    /* ... 7 capabilities total */
  }
}
```

---

## Principals

### System Principal

**ID**: `sys:noa`
**Roles**: `system_admin`
**Capabilities**: All (`*`)

Used by NOA core services that require unrestricted access.

### Agent Principal

**ID**: `agent:default`
**Roles**: `agent`
**Capabilities**:
- `reasoning` - LLM inference
- `code_generation` - Generate code
- `code_execution` - Execute in sandbox
- `file_operations` - Read/write files
- `git_operations` - Version control
- `embeddings` - Vector operations
- `tool_discovery` - MCP tool access

**Constraints**:
- Sandbox required for code execution
- Audit required for all operations
- Budget limits enforced

### User Principal

**ID**: `user:${USERNAME}`
**Roles**: `user`
**Capabilities**:
- `reasoning` - LLM inference
- `code_generation` - Generate code
- `file_operations` - Read/write files
- `git_operations` - Version control

---

## Roles

### system_admin

**Description**: System administrator with full access
**Capabilities**: All (`*`)
**Use Cases**: NOA core processes, emergency operations

### agent

**Description**: AI agent with capability-based restrictions
**Capabilities**: 7 capabilities (reasoning, code_generation, code_execution, file_operations, git_operations, embeddings, tool_discovery)
**Use Cases**: Claude Code CLI, local LLMs, agent workflows

### user

**Description**: Human user with standard permissions
**Capabilities**: 4 capabilities (reasoning, code_generation, file_operations, git_operations)
**Use Cases**: Developers, operators

### readonly

**Description**: Read-only access for monitoring
**Capabilities**: 2 capabilities (reasoning, embeddings)
**Use Cases**: Dashboards, metrics, read-only API access

---

## Capabilities

### reasoning

**Description**: Access to LLM reasoning and inference
**Resource Types**: `llm`, `inference`
**Operations**: `query`, `stream`

Example:
```json
{
  "principal": "agent:default",
  "capability": "reasoning",
  "resource": "llm:claude-sonnet-4.5",
  "operation": "query"
}
```

### code_generation

**Description**: Generate code via LLM
**Resource Types**: `llm`, `codegen`
**Operations**: `generate`, `complete`

### code_execution

**Description**: Execute code in sandbox
**Resource Types**: `sandbox`, `runtime`
**Operations**: `execute`, `build`
**Requires**: `sandbox_required: true`

### file_operations

**Description**: Read and write files
**Resource Types**: `filesystem`
**Operations**: `read`, `write`, `delete`
**Path Restrictions**: `${NOA_ROOT}/**`

### git_operations

**Description**: Git version control operations
**Resource Types**: `git`
**Operations**: `clone`, `commit`, `push`, `pull`
**Requires**: `audit_required: true`

### embeddings

**Description**: Generate and query embeddings
**Resource Types**: `embeddings`, `vector_store`
**Operations**: `embed`, `search`

### tool_discovery

**Description**: Discover and invoke MCP tools
**Resource Types**: `mcp`, `tools`
**Operations**: `list`, `invoke`

---

## Authentication

### Supported Methods

| Method | Use Case | Configuration |
|--------|----------|---------------|
| Service Account | System processes | Local identity file |
| API Key | Programmatic access | Key stored in secrets |
| OIDC | SSO for users | Remote provider |

### Session Management

- **Duration**: 3600 seconds (1 hour)
- **Refresh**: Enabled
- **MFA**: Optional (disabled by default)

---

## Authorization

### Model: Capability-Based RBAC

**Decision Flow**:
```
1. Authenticate principal (system/agent/user)
2. Load principal's roles
3. Resolve capabilities from roles
4. Check capability for resource + operation
5. Apply constraints (sandbox, audit, budget)
6. ALLOW or DENY (default: DENY)
```

### Default Deny

All access is denied unless explicitly granted via capability.

### Capability Inheritance

Roles can inherit capabilities from parent roles (not currently used but supported).

### Audit Denied Access

All denied authorization attempts are logged to audit system.

---

## Usage Examples

### Check Principal Capabilities

```bash
# List capabilities for agent principal
jq '.principals.agent.capabilities' sys/core/identity/identity.json

# Output:
# [
#   "reasoning",
#   "code_generation",
#   "code_execution",
#   "file_operations",
#   "git_operations",
#   "embeddings",
#   "tool_discovery"
# ]
```

### Verify Capability Access

```bash
# Check if agent can execute code
PRINCIPAL="agent:default"
CAPABILITY="code_execution"

# Get principal capabilities
jq -r ".principals.agent.capabilities | .[] | select(. == \"$CAPABILITY\")" \
  sys/core/identity/identity.json

# If output matches, access is granted
```

### Create New Principal

```bash
# Add new agent principal
jq '.principals.agent_custom = {
  "id": "agent:custom",
  "type": "agent",
  "name": "Custom Agent",
  "roles": ["agent"],
  "capabilities": ["reasoning", "code_generation"],
  "created_at": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'"
}' sys/core/identity/identity.json > /tmp/identity.json

mv /tmp/identity.json sys/core/identity/identity.json
```

---

## Integration

### With Policy Enforcement

Identity capabilities are enforced by `sys/core/enforcement/`:
```
sys/core/identity (defines WHO and WHAT)
  ↓
sys/core/enforcement (enforces WHEN and HOW)
  ↓
sys/core/audit (logs WHO did WHAT)
```

### With MCP Gateway

MCP gateway uses identity for authorization:
```
gateway/mcp/authz/config.json references sys/core/identity/identity.json
```

Capabilities map to MCP tool access:
- `tool_discovery` → List available tools
- `reasoning` → Invoke LLM tools
- `code_execution` → Invoke sandbox tools

---

## Security Considerations

### Least Privilege

Principals should have minimum capabilities needed:
- Agents: Only capabilities for their specific tasks
- Users: Standard permissions, no code_execution
- System: Full access, use sparingly

### Sandbox Enforcement

`code_execution` capability requires `sandbox_required: true` constraint:
```json
{
  "constraints": {
    "sandbox_required": true
  }
}
```

Enforcement layer MUST verify sandbox isolation before allowing execution.

### Audit Trail

All operations requiring `audit_required: true` are logged:
```json
{
  "git_operations": {
    "requires_audit": true
  }
}
```

Audit logs stored in `sys/core/audit/logs/`.

---

## Troubleshooting

### "Access Denied" Errors

**Symptom**: Principal cannot access resource

**Diagnosis**:
```bash
# 1. Check principal exists
jq '.principals | keys' sys/core/identity/identity.json

# 2. Check principal's capabilities
jq '.principals.agent.capabilities' sys/core/identity/identity.json

# 3. Check capability definition
jq '.capabilities.code_execution' sys/core/identity/identity.json
```

**Resolution**:
- Add missing capability to principal's role
- Verify resource type matches capability
- Check operation is allowed for capability

### Principal Not Found

**Symptom**: "Principal not found: agent:custom"

**Diagnosis**:
```bash
# List all principals
jq '.principals | keys' sys/core/identity/identity.json
```

**Resolution**:
- Create principal in identity.json
- Use correct principal ID pattern
- Restart identity service (if applicable)

---

## References

- [NOA Policy Framework](../../ai/shared/resources/policy/01_CONSTITUTION.md)
- [MCP Authorization](../../gateway/mcp/authz/config.json)
- [Enforcement System](../enforcement/README.md)
- [Audit System](../audit/README.md)

---

**Version**: 1.0.0
**Last Updated**: 2026-01-02
