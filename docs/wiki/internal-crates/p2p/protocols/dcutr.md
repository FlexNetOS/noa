# dcutr Protocol

Direct Connection Upgrade through Relay.

**Location**: `p2p/protocols/dcutr/`  
**Version**: 0.14.0  
**Crate**: `libp2p-dcutr`

## Overview

Upgrade relayed connections to direct connections:

- Hole punching coordination
- Works with relay protocol
- Simultaneous open technique

## Flow

```
┌────────┐         ┌────────┐         ┌────────┐
│ PeerA  │         │ Relay  │         │ PeerB  │
└───┬────┘         └───┬────┘         └───┬────┘
    │ [Relayed Connection]                │
    │<════════════════╪═════════════════=>│
    │                  │                  │
    │ CONNECT          │                  │
    │─────────────────>│─────────────────>│
    │                  │  CONNECT         │
    │<─────────────────│<─────────────────│
    │                  │                  │
    │ [Hole Punch Attempt]                │
    │<───────────────────────────────────>│
    │                  │                  │
    │ [Direct Connection]                 │
    │<═══════════════════════════════════>│
```

## Key Types

### Behaviour

```rust
pub struct Behaviour {
    // Tracks ongoing upgrades
    pending: HashMap<ConnectionId, UpgradeState>,
}
```

## NOA Usage

```rust
use libp2p::dcutr::Behaviour;

let dcutr = Behaviour::new(local_peer_id);

// Combined with relay client
let combined = Combined {
    relay_client,
    dcutr,
};
```

## See Also

- [relay](relay.md) — Circuit relay
- [autonat](autonat.md) — NAT detection
