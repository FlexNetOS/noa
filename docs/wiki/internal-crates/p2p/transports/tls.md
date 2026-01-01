# tls Transport

TLS 1.3 encryption.

**Location**: `p2p/transports/tls/`  
**Crate**: `libp2p-tls`

## Overview

TLS 1.3 encryption with libp2p extensions:

- Standard TLS 1.3
- Certificate-based auth
- ALPN negotiation

## NOA Usage

```rust
use libp2p::tls;

let config = tls::Config::new(&keypair)?;

let transport = tcp::Transport::default()
    .upgrade(Version::V1)
    .authenticate(config)
    .multiplex(yamux::Config::default());
```

## See Also

- [noise](noise.md) — Noise encryption
- [tcp](tcp.md) — TCP transport
