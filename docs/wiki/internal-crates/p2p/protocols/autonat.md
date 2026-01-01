# autonat Protocol

NAT status detection.

**Location**: `p2p/protocols/autonat/`  
**Version**: 0.15.0  
**Crate**: `libp2p-autonat`

## Overview

Determine NAT status and reachability:

- Probe reachability via peers
- Determine public/private status
- Trigger relay usage when needed

## NAT Status

| Status | Description | Action |
|--------|-------------|--------|
| `Public` | Directly reachable | Accept connections |
| `Private` | Behind NAT | Use relay |
| `Unknown` | Not yet determined | Probe peers |

## Key Types

### Behaviour

```rust
pub struct Behaviour {
    config: Config,
    servers: Vec<PeerId>,
    nat_status: NatStatus,
}
```

### Config

```rust
pub struct Config {
    pub retry_interval: Duration,
    pub refresh_interval: Duration,
    pub boot_delay: Duration,
    pub throttle_server_period: Duration,
}
```

## NOA Usage

```rust
use libp2p::autonat::{Behaviour, Config, NatStatus};

let behaviour = Behaviour::new(local_peer_id, Config::default());

// Handle events
match event {
    Event::StatusChanged { old, new } => {
        match new {
            NatStatus::Public(addr) => {
                info!("Public at {}", addr);
            }
            NatStatus::Private => {
                info!("Behind NAT, using relay");
            }
            _ => {}
        }
    }
    _ => {}
}
```

## See Also

- [relay](relay.md) — Circuit relay
- [dcutr](dcutr.md) — Direct connection upgrade
