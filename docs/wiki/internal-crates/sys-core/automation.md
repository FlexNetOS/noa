# automation Module

Task automation and scheduling.

**Location**: `sys/core/src/automation/`  
**Feature**: `full`

## Overview

Provides automated task execution:

- Scheduled tasks (cron-like)
- Event-triggered automation
- Workflow definitions
- Retry policies

## Key Types

### Trigger

Automation trigger definition.

```rust
pub enum Trigger {
    Schedule(CronSchedule),
    Event { event_type: String, filter: Option<Filter> },
    Webhook { path: String },
    Manual,
}

pub struct CronSchedule {
    pub expression: String,  // "0 0 * * *" = daily
    pub timezone: Tz,
}
```

### Automation

Automation definition.

```rust
pub struct Automation {
    pub id: AutomationId,
    pub name: String,
    pub trigger: Trigger,
    pub actions: Vec<Action>,
    pub enabled: bool,
    pub retry_policy: RetryPolicy,
}

pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff: Duration,
    pub exponential: bool,
}
```

### Scheduler

Central automation scheduler.

```rust
pub struct Scheduler {
    automations: Vec<Automation>,
    runtime: tokio::runtime::Handle,
}

impl Scheduler {
    pub fn register(&mut self, automation: Automation);
    pub fn start(&self);
    pub fn stop(&self);
}
```

## Usage

```rust
use noa_core::automation::{Automation, Trigger, CronSchedule};

let backup_automation = Automation {
    name: "daily-backup".into(),
    trigger: Trigger::Schedule(CronSchedule {
        expression: "0 2 * * *".into(),  // 2 AM daily
        timezone: Tz::UTC,
    }),
    actions: vec![
        Action::ExecuteAgent {
            agent: AgentKind::FileIO,
            task: json!({"operation": "backup"}),
        },
    ],
    enabled: true,
    retry_policy: RetryPolicy::default(),
};

scheduler.register(backup_automation);
```

## See Also

- [agents module](agents.md) — Agent execution
- [events module](events.md) — Event triggers
