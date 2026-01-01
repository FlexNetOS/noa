# perf Protocol

Performance testing.

**Location**: `p2p/protocols/perf/`  
**Version**: 0.4.0  
**Crate**: `libp2p-perf`

## Overview

Measure connection performance:

- Throughput testing
- Latency measurement
- Connection benchmarking

## Usage

Primarily for debugging and benchmarking, not production use.

```rust
use libp2p::perf::{client::Behaviour as PerfClient, RunParams};

let behaviour = PerfClient::default();

// Run performance test
let params = RunParams {
    to_send: 1024 * 1024,   // 1MB
    to_receive: 1024 * 1024,
};
behaviour.perf(peer_id, params)?;
```

## See Also

- [ping](ping.md) — Simple latency
