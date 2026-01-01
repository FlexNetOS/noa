# noise Transport

Noise protocol encryption.

**Location**: `p2p/transports/noise/`  
**Version**: 0.46.1  
**Crate**: `libp2p-noise`

## Overview

Noise Framework encryption:

- XX handshake pattern
- Perfect forward secrecy
- Mutual authentication

## NOA Usage

Used with TCP transport:

```rust
use libp2p::noise;

let config = noise::Config::new(&keypair)?;

let transport = tcp::Transport::default()
    .upgrade(Version::V1)
    .authenticate(config)
    .multiplex(yamux::Config::default());
```

## See Also

- [tls](tls.md) — TLS encryption
- [tcp](tcp.md) — TCP transport
