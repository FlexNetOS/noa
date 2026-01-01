# How-To: Add Custom Agent

Create and register a custom agent in NOA.

## Overview

Custom agents extend NOA's capabilities by implementing the `Agent` trait.

## Steps

### 1. Create Agent Module

Create `my-agent/src/lib.rs`:

```rust
use noa_core::agents::{Agent, AgentConfig, Task, TaskResult};
use noa_core::error::NoaResult;
use async_trait::async_trait;

pub struct MyAgent {
    config: AgentConfig,
}

impl MyAgent {
    pub fn new(config: AgentConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl Agent for MyAgent {
    fn name(&self) -> &str {
        "my-agent"
    }
    
    fn capabilities(&self) -> Vec<&str> {
        vec!["custom-action"]
    }
    
    async fn execute(&self, task: Task) -> NoaResult<TaskResult> {
        // Implement your logic here
        let result = format!("Processed: {:?}", task.input);
        Ok(TaskResult::success(result))
    }
}
```

### 2. Create Module Manifest

Create `my-agent/module.toml`:

```toml
[module]
id = "my-agent"
name = "My Custom Agent"
version = "1.0.0"
type = "agent"

[dependencies]
noa-core = "^0.1"
```

### 3. Build Module

```bash
cd my-agent
cargo build --release
```

### 4. Install Module

```bash
# Copy to modules directory
cp target/release/libmy_agent.so ~/.noa/modules/

# Or use noa CLI
noa module install ./my-agent
```

### 5. Register Agent

```bash
# Register in configuration
noa agent register my-agent

# Or via API
curl -X POST http://localhost:8080/api/v1/agents \
  -H "Content-Type: application/json" \
  -d '{"kind": "custom", "name": "my-agent"}'
```

### 6. Test Agent

```bash
# Execute task
noa task run --agent my-agent --input '{"action": "test"}'
```

## Best Practices

1. **Handle errors gracefully**: Return meaningful errors
2. **Log actions**: Use tracing for observability
3. **Respect governance**: Check permissions before actions
4. **Clean up resources**: Implement proper shutdown

## See Also

- [Agent System Design](../design/agent-system.md)
- [Agents Module](../../wiki/internal-crates/sys-core/agents.md)
