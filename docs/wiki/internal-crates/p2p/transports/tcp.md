# tcp Transport

TCP transport.

**Location**: `p2p/transports/tcp/`  
**Crate**: `libp2p-tcp`

## Overview

TCP-based transport:

- Widely compatible
- Requires separate encryption (Noise/TLS)
- Requires separate muxer (Yamux)

## NOA Usage

```rust
use libp2p::tcp;

let transport = tcp::tokio::Transport::new(tcp::Config::default());

// Add encryption and muxing
let transport = transport
    .upgrade(Version::V1)
    .authenticate(noise::Config::new(&keypair)?)
    .multiplex(yamux::Config::default())
    .boxed();
```

## See Also

- [quic](quic.md) — QUIC transport (recommended)
- [noise](noise.md) — Encryption
- [yamux](../muxers/yamux.md) — Multiplexing
