# websocket Transport

WebSocket transport.

**Location**: `p2p/transports/websocket/`  
**Crate**: `libp2p-websocket`

## Overview

WebSocket-based transport:

- Browser compatible
- Firewall friendly (port 80/443)
- Works over HTTP(S)

## NOA Usage

```rust
use libp2p::{websocket, tcp, noise, yamux};

let ws_config = websocket::Config::default();
let transport = websocket::WsConfig::new(tcp::tokio::Transport::default())
    .upgrade(Version::V1)
    .authenticate(noise::Config::new(&keypair)?)
    .multiplex(yamux::Config::default())
    .boxed();
```

## See Also

- [webrtc](webrtc.md) — WebRTC transport
- [websocket-websys](../index.md) — Browser variant
