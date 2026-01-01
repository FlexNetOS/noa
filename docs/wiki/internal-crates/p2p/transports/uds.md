# uds Transport

Unix domain sockets.

**Location**: `p2p/transports/uds/`  
**Crate**: `libp2p-uds`

## Overview

Unix domain socket transport:

- Local IPC
- Low latency
- Unix/Linux only

## NOA Usage

For local agent communication:

```rust
use libp2p::uds;

let transport = uds::tokio::Transport::default();

// Address: /unix/path/to/socket
```

## See Also

- [tcp](tcp.md) — TCP transport
