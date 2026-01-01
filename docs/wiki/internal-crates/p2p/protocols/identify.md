# identify Protocol

Peer identification and capability exchange.

**Location**: `p2p/protocols/identify/`  
**Version**: 0.47.0  
**Crate**: `libp2p-identify`

## Overview

Exchange peer information on connection:

- Protocol versions
- Agent string
- Listen addresses
- Observed address

## Key Types

### Behaviour

```rust
pub struct Behaviour {
    config: Config,
    events: VecDeque<Event>,
}
```

### Info

```rust
pub struct Info {
    pub public_key: PublicKey,
    pub protocol_version: String,
    pub agent_version: String,
    pub listen_addrs: Vec<Multiaddr>,
    pub protocols: Vec<StreamProtocol>,
    pub observed_addr: Multiaddr,
}
```

### Config

```rust
pub struct Config {
    pub protocol_version: String,
    pub agent_version: String,
    pub interval: Duration,
    pub push_listen_addr_updates: bool,
}
```

## NOA Usage

```rust
use libp2p::identify::{Behaviour, Config};

let config = Config::new(
    "/noa/1.0.0".into(),
    keypair.public(),
)
.with_agent_version("noa-core/0.1.0".into());

let behaviour = Behaviour::new(config);
```

## Events

| Event | Description |
|-------|-------------|
| `Received` | Info received from peer |
| `Sent` | Info sent to peer |
| `Pushed` | Updated info pushed |
| `Error` | Identification failed |

## See Also

- [kad](kad.md) — Peer discovery
- [autonat](autonat.md) — NAT detection
