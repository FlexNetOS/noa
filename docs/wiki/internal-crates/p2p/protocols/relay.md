# relay Protocol

Circuit Relay v2 for NAT traversal.

**Location**: `p2p/protocols/relay/`  
**Version**: 0.21.1  
**Crate**: `libp2p-relay`

## Overview

Relay connections through intermediate peers:

- Circuit Relay v2 protocol
- Reservation system
- Limited relay for NAT traversal
- Works with DCUtR

## Modes

### Client Mode

Connect through relay when direct connection fails.

```rust
use libp2p::relay::client::{Behaviour, Config};

let behaviour = Behaviour::new(keypair.public().to_peer_id(), Config::default());
```

### Server Mode

Act as a relay for other peers.

```rust
use libp2p::relay::{Behaviour, Config};

let config = Config {
    max_reservations: 128,
    max_circuits: 16,
    reservation_duration: Duration::from_secs(3600),
    ..Default::default()
};

let behaviour = Behaviour::new(local_peer_id, config);
```

## Reservation Flow

```
┌────────┐         ┌────────┐         ┌────────┐
│ Client │         │ Relay  │         │ Target │
└───┬────┘         └───┬────┘         └───┬────┘
    │   RESERVE        │                  │
    │─────────────────>│                  │
    │   RESERVATION_OK │                  │
    │<─────────────────│                  │
    │                  │  RESERVE         │
    │                  │<─────────────────│
    │                  │  RESERVATION_OK  │
    │                  │─────────────────>│
    │   CONNECT        │                  │
    │─────────────────>│                  │
    │                  │  CONNECT         │
    │                  │─────────────────>│
    │<═════════════════╪══════════════════│
    │     Relayed Connection              │
```

## NOA Usage

Used for:
- NAT traversal
- Mobile device connectivity
- Initial peer discovery

## See Also

- [dcutr](dcutr.md) — Direct connection upgrade
- [autonat](autonat.md) — NAT detection
