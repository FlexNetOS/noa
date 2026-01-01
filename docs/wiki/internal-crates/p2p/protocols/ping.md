# ping Protocol

Liveness checks.

**Location**: `p2p/protocols/ping/`  
**Version**: 0.47.0  
**Crate**: `libp2p-ping`

## Overview

Simple ping/pong for connection health:

- Periodic liveness checks
- RTT measurement
- Connection keep-alive

## Key Types

### Behaviour

```rust
pub struct Behaviour {
    config: Config,
}
```

### Config

```rust
pub struct Config {
    pub interval: Duration,   // Default: 15s
    pub timeout: Duration,    // Default: 20s
}
```

### Event

```rust
pub struct Event {
    pub peer: PeerId,
    pub connection: ConnectionId,
    pub result: Result<Duration, Failure>,
}
```

## NOA Usage

```rust
use libp2p::ping::{Behaviour, Config};

let behaviour = Behaviour::new(Config::default());

// Handle events
match event.result {
    Ok(rtt) => info!("Ping {} RTT: {:?}", event.peer, rtt),
    Err(e) => warn!("Ping {} failed: {:?}", event.peer, e),
}
```

## See Also

- [identify](identify.md) — Peer info exchange
