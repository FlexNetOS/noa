# mdns Protocol

Local network peer discovery.

**Location**: `p2p/protocols/mdns/`  
**Version**: 0.48.0  
**Crate**: `libp2p-mdns`

## Overview

Discover peers on local network via mDNS:

- Zero configuration
- LAN discovery
- IPv4/IPv6 support

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
    pub ttl: Duration,
    pub query_interval: Duration,
    pub enable_ipv6: bool,
}
```

### Event

```rust
pub enum Event {
    Discovered(Vec<(PeerId, Multiaddr)>),
    Expired(Vec<(PeerId, Multiaddr)>),
}
```

## NOA Usage

```rust
use libp2p::mdns::{tokio::Behaviour, Config};

let behaviour = Behaviour::new(Config::default(), local_peer_id)?;

// Handle discovery
match event {
    Event::Discovered(peers) => {
        for (peer, addr) in peers {
            info!("Discovered {} at {}", peer, addr);
            swarm.dial(addr)?;
        }
    }
    Event::Expired(peers) => {
        for (peer, _) in peers {
            info!("Peer {} expired", peer);
        }
    }
}
```

## See Also

- [rendezvous](rendezvous.md) — Internet discovery
- [kad](kad.md) — DHT discovery
