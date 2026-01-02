# Agent Sandbox

Isolated execution environment for AI agents within the NOA platform.

## Structure

```
sandbox/agents/
├── README.md           # This file
├── capsules/           # Capsule definitions for agent isolation
│   ├── base.json       # Base capsule template
│   ├── code-agent.json # Code generation agent capsule
│   ├── chat-agent.json # Chat/conversational agent capsule
│   └── task-agent.json # Task execution agent capsule
├── policies/           # Security policies for agents
│   ├── default.rego    # Default OPA policy
│   └── restricted.rego # Restricted execution policy
├── runtime/            # Agent runtime configuration
│   ├── limits.json     # Resource limits
│   └── permissions.json# Capability permissions
└── templates/          # Agent templates
    ├── planning.md     # Planning agent template
    ├── execution.md    # Execution agent template
    └── review.md       # Review agent template
```

## Capsule Isolation

Agents run in isolated capsules with:

1. **Resource Limits**: CPU, memory, disk, network
2. **Capability Permissions**: File access, network access, tool access
3. **Policy Enforcement**: OPA-based policy evaluation

## Agent Types

| Type | Purpose | Capabilities |
|------|---------|--------------|
| `planning` | Research, outline multi-step plans | Read-only filesystem, no network |
| `execution` | Execute code, make changes | Full filesystem, restricted network |
| `code-generation` | Generate code snippets | Write to temp, no execution |
| `chat` | Conversational interface | No filesystem, API access only |
| `review` | Review changes, QA | Read-only filesystem, no execution |

## Usage

### Register an agent

```bash
curl -X POST http://localhost:9999/agents \
  -H "Content-Type: application/json" \
  -d '{"id": "planning-001", "name": "Planning Agent"}'
```

### Start a sandboxed agent

```bash
noa-ctl agent start --capsule code-agent --id planning-001
```

### Check agent status

```bash
noa-ctl agent status planning-001
```

## Security Model

1. **Principle of Least Privilege**: Agents receive minimum required permissions
2. **Audit Logging**: All agent actions are logged
3. **Policy Enforcement**: Actions validated against OPA policies before execution
4. **Resource Isolation**: Each agent runs in isolated capsule
