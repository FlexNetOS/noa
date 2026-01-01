# quic Transport

QUIC transport (recommended).

**Location**: `p2p/transports/quic/`  
**Version**: 0.13.0  
**Crate**: `libp2p-quic`

## Overview

QUIC-based transport:

- Built-in encryption (TLS 1.3)
- Multiplexing included
- Fast connection setup
- NAT-friendly (UDP)

## Key Types

### Config

```rust
pub struct Config {
    pub handshake_timeout: Duration,
    pub max_idle_timeout: u32,
    pub keep_alive_interval: Duration,
    pub max_concurrent_stream_limit: u32,
}
```

## NOA Usage

Recommended transport for most deployments.

```rust
use libp2p::quic;

let transport = quic::tokio::Transport::new(quic::Config::new(&keypair));
```

## See Also

- [tcp](tcp.md) — TCP transport
- [noise](noise.md) — Encryption layer
