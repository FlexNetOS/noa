# rendezvous Protocol

Peer discovery via rendezvous points.

**Location**: `p2p/protocols/rendezvous/`  
**Crate**: `libp2p-rendezvous`

## Overview

Register and discover peers at rendezvous points:

- Namespace-based registration
- TTL-based expiration
- Lightweight discovery

## Modes

### Client Mode

Register and discover peers.

```rust
use libp2p::rendezvous::client::{Behaviour, Config};

let behaviour = Behaviour::new(keypair);

// Register with namespace
behaviour.register(Namespace::from_static("noa/agents"), rendezvous_peer, None)?;

// Discover peers
behaviour.discover(Some(Namespace::from_static("noa/agents")), None, None, rendezvous_peer);
```

### Server Mode

Act as rendezvous point.

```rust
use libp2p::rendezvous::server::{Behaviour, Config};

let behaviour = Behaviour::new(Config::default());
```

## NOA Usage

Used for initial peer discovery before DHT bootstrap.

## See Also

- [kad](kad.md) — DHT discovery
- [mdns](mdns.md) — Local discovery
