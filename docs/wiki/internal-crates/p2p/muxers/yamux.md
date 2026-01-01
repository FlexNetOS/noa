# yamux Muxer

Yamux stream multiplexer.

**Location**: `p2p/muxers/yamux/`  
**Crate**: `libp2p-yamux`

## Overview

Recommended stream multiplexer:

- Low overhead
- Flow control
- Keep-alive

## Config

```rust
pub struct Config {
    pub receive_window: u32,
    pub max_buffer_size: usize,
    pub max_num_streams: usize,
}
```

## NOA Usage

```rust
use libp2p::yamux;

let transport = tcp_transport
    .authenticate(noise::Config::new(&keypair)?)
    .multiplex(yamux::Config::default());
```

## See Also

- [mplex](mplex.md) — Mplex muxer (deprecated)
