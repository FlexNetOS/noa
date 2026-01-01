# dns Transport

DNS resolution.

**Location**: `p2p/transports/dns/`  
**Version**: 0.44.0  
**Crate**: `libp2p-dns`

## Overview

DNS resolution for multiaddresses:

- Resolve `/dns4/` and `/dns6/`
- Async resolution
- Caching

## NOA Usage

```rust
use libp2p::dns;

let transport = dns::tokio::Transport::system(tcp::tokio::Transport::default())?;
```

## See Also

- [tcp](tcp.md) — TCP transport
