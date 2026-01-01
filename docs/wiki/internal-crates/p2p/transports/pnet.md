# pnet Transport

Private network PSK.

**Location**: `p2p/transports/pnet/`  
**Version**: 0.26.0  
**Crate**: `libp2p-pnet`

## Overview

Private network with pre-shared key:

- Network isolation
- Simple authentication
- 256-bit key

## NOA Usage

For isolated NOA deployments:

```rust
use libp2p::pnet::{PnetConfig, PreSharedKey};

let psk = PreSharedKey::from_bytes(&key_bytes)?;
let config = PnetConfig::new(psk);

let transport = config.layer_on(tcp_transport);
```

## See Also

- [noise](noise.md) — Encryption
