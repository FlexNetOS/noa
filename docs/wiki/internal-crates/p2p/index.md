# p2p Crate (libp2p Fork)

NOA's peer-to-peer networking stack, forked from `libp2p/rust-libp2p`.

**Location**: `p2p/`  
**Fork**: [FlexNetOS/rust-libp2p](https://github.com/FlexNetOS/rust-libp2p)  
**Upstream**: [libp2p/rust-libp2p](https://github.com/libp2p/rust-libp2p)  
**Edition**: Rust 2021  
**MSRV**: 1.83.0

## Crate Categories

### Core (`p2p/core/`)

The foundation of all libp2p networking:

| Crate | Version | Purpose |
|-------|---------|---------|
| `libp2p-core` | 0.43.1 | Transport, peer identity, multiaddress |
| `libp2p-identity` | 0.2.13 | Cryptographic identities (Ed25519, secp256k1) |

### Protocols (`p2p/protocols/`)

High-level networking behaviors:

| Crate | Version | Purpose |
|-------|---------|---------|
| [gossipsub](protocols/gossipsub.md) | 0.50.0 | Pub/sub messaging |
| [kad](protocols/kad.md) | 0.49.0 | Kademlia DHT |
| [identify](protocols/identify.md) | 0.47.0 | Peer identification |
| [relay](protocols/relay.md) | 0.21.1 | Circuit relay v2 |
| [dcutr](protocols/dcutr.md) | 0.14.0 | Direct connection upgrade |
| [autonat](protocols/autonat.md) | 0.15.0 | NAT detection |
| [ping](protocols/ping.md) | 0.47.0 | Liveness checks |
| [request-response](protocols/request-response.md) | 0.x | Request/response patterns |
| [stream](protocols/stream.md) | 0.x | Multiplexed streams |
| [rendezvous](protocols/rendezvous.md) | 0.x | Peer discovery |
| [mdns](protocols/mdns.md) | 0.48.0 | Local network discovery |
| [floodsub](protocols/floodsub.md) | 0.47.0 | Simple pub/sub (deprecated) |
| [upnp](protocols/upnp.md) | 0.x | UPnP port mapping |
| [perf](protocols/perf.md) | 0.4.0 | Performance testing |

### Transports (`p2p/transports/`)

Network transport implementations:

| Crate | Version | Purpose |
|-------|---------|---------|
| [quic](transports/quic.md) | 0.13.0 | QUIC transport (recommended) |
| [tcp](transports/tcp.md) | 0.x | TCP transport |
| [websocket](transports/websocket.md) | 0.x | WebSocket transport |
| [webrtc](transports/webrtc.md) | 0.x | WebRTC transport |
| [noise](transports/noise.md) | 0.46.1 | Noise protocol encryption |
| [tls](transports/tls.md) | 0.x | TLS 1.3 encryption |
| [dns](transports/dns.md) | 0.44.0 | DNS resolution |
| [pnet](transports/pnet.md) | 0.26.0 | Private network PSK |
| [plaintext](transports/plaintext.md) | 0.43.0 | Unencrypted (testing only) |
| [uds](transports/uds.md) | 0.x | Unix domain sockets |

#### WASM Variants

| Crate | Purpose |
|-------|---------|
| `websocket-websys` | WebSocket for browser |
| `webrtc-websys` | WebRTC for browser |
| `webtransport-websys` | WebTransport for browser |

### Muxers (`p2p/muxers/`)

Stream multiplexing:

| Crate | Version | Purpose |
|-------|---------|---------|
| [yamux](muxers/yamux.md) | 0.x | Yamux multiplexer (recommended) |
| [mplex](muxers/mplex.md) | 0.43.1 | Mplex multiplexer (deprecated) |

### Misc (`p2p/misc/`)

Utility crates:

| Crate | Purpose |
|-------|---------|
| `allow-block-list` | Peer filtering |
| `connection-limits` | Connection throttling |
| `memory-connection-limits` | Memory-based limits |
| `metrics` | Prometheus metrics |
| `peer-store` | Peer address book |
| `multistream-select` | Protocol negotiation |
| `server` | Standalone server utilities |

### Swarm (`p2p/swarm/`)

High-level orchestration:

| Crate | Purpose |
|-------|---------|
| `swarm` | Main swarm driver |
| `swarm-derive` | Derive macros for behaviours |
| `swarm-test` | Testing utilities |

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Application                              │
├─────────────────────────────────────────────────────────────────┤
│                           Swarm                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                    NetworkBehaviour                      │   │
│   │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐   │   │
│   │  │ Gossipsub│ │   Kad    │ │  Relay   │ │ Identify │   │   │
│   │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘   │   │
│   │       └────────────┴────────────┴────────────┘          │   │
│   └─────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│                         Transport                                │
│   ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐          │
│   │   QUIC   │ │   TCP    │ │ WebSocket│ │  WebRTC  │          │
│   └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘          │
│        └────────────┴────────────┴────────────┘                 │
├─────────────────────────────────────────────────────────────────┤
│                     Encryption (Noise/TLS)                       │
├─────────────────────────────────────────────────────────────────┤
│                     Multiplexer (Yamux)                          │
└─────────────────────────────────────────────────────────────────┘
```

## NOA Integration

NOA uses p2p for:

1. **Agent Federation**: Distributed agent execution across peers
2. **Model Sharing**: P2P model weight distribution
3. **Knowledge Sync**: Gossip-based knowledge graph synchronization
4. **Compute Offload**: DHT-based task routing to available peers

## Usage

```rust
use libp2p::{
    gossipsub, identify, kad, noise, quic, swarm, yamux,
    Multiaddr, PeerId, Swarm,
};

async fn create_swarm() -> Swarm<MyBehaviour> {
    let keypair = libp2p::identity::Keypair::generate_ed25519();
    
    libp2p::SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_quic()
        .with_behaviour(|key| MyBehaviour::new(key))
        .build()
}
```

---

*See [libp2p docs](https://docs.rs/libp2p) for upstream documentation*
