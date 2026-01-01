# stream Protocol

Multiplexed byte streams.

**Location**: `p2p/protocols/stream/`  
**Crate**: `libp2p-stream`

## Overview

Raw bidirectional streams:

- Protocol negotiation
- Multiplexed connections
- Custom protocols

## Key Types

### Behaviour

```rust
pub struct Behaviour {
    supported_protocols: HashSet<StreamProtocol>,
}
```

### Control

```rust
pub struct Control {
    // Open streams to peers
}

impl Control {
    pub async fn open_stream(&mut self, peer: PeerId, protocol: StreamProtocol) -> Result<Stream>;
    pub async fn accept_stream(&mut self) -> Result<(PeerId, Stream)>;
}
```

## NOA Usage

```rust
use libp2p::stream::{Behaviour, Control};

let behaviour = Behaviour::new();
let control = behaviour.new_control();

// Open stream
let stream = control.open_stream(peer_id, StreamProtocol::new("/noa/data/1")).await?;

// Read/write
stream.write_all(b"hello").await?;
let mut buf = vec![0u8; 1024];
let n = stream.read(&mut buf).await?;
```

## See Also

- [request-response](request-response.md) — Typed messaging
