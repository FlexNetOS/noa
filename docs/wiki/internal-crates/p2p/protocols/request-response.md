# request-response Protocol

Request/response messaging patterns.

**Location**: `p2p/protocols/request-response/`  
**Crate**: `libp2p-request-response`

## Overview

Typed request/response communication:

- Codec-based serialization
- Automatic matching
- Timeout handling

## Key Types

### Behaviour

```rust
pub struct Behaviour<TCodec> {
    codec: TCodec,
    pending: HashMap<RequestId, ResponseChannel>,
}
```

### Codec Trait

```rust
pub trait Codec {
    type Protocol: AsRef<str>;
    type Request: Send;
    type Response: Send;
    
    fn read_request(&mut self, protocol: &Self::Protocol, io: &mut impl AsyncRead) -> Self::Request;
    fn read_response(&mut self, protocol: &Self::Protocol, io: &mut impl AsyncRead) -> Self::Response;
    fn write_request(&mut self, protocol: &Self::Protocol, io: &mut impl AsyncWrite, req: Self::Request);
    fn write_response(&mut self, protocol: &Self::Protocol, io: &mut impl AsyncWrite, res: Self::Response);
}
```

## NOA Usage

```rust
use libp2p::request_response::{Behaviour, Config, ProtocolSupport};

#[derive(Debug, Clone)]
struct TaskRequest { task_id: String }

#[derive(Debug, Clone)]
struct TaskResponse { result: Value }

let behaviour = Behaviour::with_codec(
    MyCodec,
    [(StreamProtocol::new("/noa/task/1"), ProtocolSupport::Full)],
    Config::default(),
);

// Send request
let request_id = behaviour.send_request(&peer_id, TaskRequest { task_id: "123".into() });

// Handle response
match event {
    Event::Message { peer, message: Message::Response { request_id, response } } => {
        handle_response(response);
    }
    _ => {}
}
```

## See Also

- [stream](stream.md) — Raw streams
- [gossipsub](gossipsub.md) — Pub/sub
