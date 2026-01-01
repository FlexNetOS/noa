# webrtc Transport

WebRTC transport.

**Location**: `p2p/transports/webrtc/`  
**Crate**: `libp2p-webrtc`

## Overview

WebRTC-based transport:

- Browser-to-browser connections
- NAT traversal (ICE)
- Data channels

## NOA Usage

```rust
use libp2p::webrtc;

let transport = webrtc::tokio::Transport::new(
    keypair,
    webrtc::tokio::Certificate::generate(&mut thread_rng())?,
);
```

## See Also

- [websocket](websocket.md) — WebSocket transport
- [webrtc-websys](../index.md) — Browser variant
