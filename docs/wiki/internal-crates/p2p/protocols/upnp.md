# upnp Protocol

UPnP port mapping.

**Location**: `p2p/protocols/upnp/`  
**Crate**: `libp2p-upnp`

## Overview

Automatic port forwarding via UPnP:

- Router discovery
- Port mapping creation
- Lease renewal

## NOA Usage

```rust
use libp2p::upnp::tokio::Behaviour;

let behaviour = Behaviour::default();

// Events
match event {
    Event::NewExternalAddr(addr) => {
        info!("External address: {}", addr);
    }
    Event::GatewayNotFound => {
        warn!("No UPnP gateway found");
    }
    Event::ExpiredExternalAddr(addr) => {
        info!("Address expired: {}", addr);
    }
}
```

## See Also

- [autonat](autonat.md) — NAT detection
