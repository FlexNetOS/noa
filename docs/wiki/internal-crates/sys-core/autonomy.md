# autonomy Module

Self-governance and policy enforcement.

**Location**: `sys/core/src/autonomy/`  
**Feature**: `full`

## Overview

Implements NOA's constitutional governance:

- Policy definition and enforcement
- Agent permission management
- Resource quotas and limits
- Audit logging

## Key Types

### Governor

Central governance authority.

```rust
pub struct Governor {
    constitution: Constitution,
    policies: Vec<Policy>,
    audit_log: AuditLog,
}

impl Governor {
    pub fn check_permission(&self, agent: &Agent, action: &Action) -> Decision;
    pub fn enforce_quota(&self, agent: &Agent, resource: Resource) -> QuotaResult;
    pub fn log_action(&mut self, entry: AuditEntry);
}
```

### Policy

Governance policy definition.

```rust
pub struct Policy {
    pub id: PolicyId,
    pub name: String,
    pub rules: Vec<Rule>,
    pub priority: u32,
    pub enabled: bool,
}

pub struct Rule {
    pub condition: Condition,
    pub effect: Effect,
}

pub enum Effect {
    Allow,
    Deny,
    RequireApproval,
    RateLimit(Duration),
}
```

### Decision

Policy evaluation result.

```rust
pub enum Decision {
    Allowed,
    Denied { reason: String },
    RequiresApproval { approver: String },
    RateLimited { retry_after: Duration },
}
```

## Constitutional Compliance

All agents must comply with [CONSTITUTION.md](../../../../CONSTITUTION.md):

1. **Data Sovereignty**: User data stays local
2. **Transparency**: Actions are auditable
3. **Consent**: Explicit permission for external access
4. **Minimal Authority**: Least-privilege principle

## Policy Examples

### File System Policy

```rust
Policy {
    name: "file-system-access",
    rules: vec![
        Rule {
            condition: Condition::PathPrefix("~/.ssh"),
            effect: Effect::Deny,
        },
        Rule {
            condition: Condition::PathPrefix("~/.noa"),
            effect: Effect::Allow,
        },
    ],
}
```

### Network Policy

```rust
Policy {
    name: "network-access",
    rules: vec![
        Rule {
            condition: Condition::ExternalDomain("*"),
            effect: Effect::RequireApproval,
        },
        Rule {
            condition: Condition::Localhost,
            effect: Effect::Allow,
        },
    ],
}
```

## Usage

```rust
use noa_core::autonomy::{Governor, Action, Resource};

async fn example(governor: &Governor, agent: &Agent) -> NoaResult<()> {
    let action = Action::FileWrite { path: "/tmp/test.txt".into() };
    
    match governor.check_permission(agent, &action) {
        Decision::Allowed => {
            // Proceed with action
        }
        Decision::Denied { reason } => {
            return Err(NoaError::Unauthorized(reason));
        }
        _ => { /* Handle other cases */ }
    }
    
    Ok(())
}
```

## See Also

- [agents module](agents.md) — Agent definitions
- [events module](events.md) — Event logging
- [CONSTITUTION.md](../../../../CONSTITUTION.md) — Governance framework
