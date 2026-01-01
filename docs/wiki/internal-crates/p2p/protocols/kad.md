# kad Protocol

Kademlia Distributed Hash Table.

**Location**: `p2p/protocols/kad/`  
**Version**: 0.49.0  
**Crate**: `libp2p-kad`

## Overview

Kademlia DHT for distributed key-value storage:

- Peer discovery
- Content routing
- Distributed storage
- XOR-based routing

## Key Types

### Behaviour

```rust
pub struct Behaviour<TStore> {
    kbuckets: KBucketsTable<PeerId, AddressRecord>,
    store: TStore,
}
```

### Record

```rust
pub struct Record {
    pub key: Key,
    pub value: Vec<u8>,
    pub publisher: Option<PeerId>,
    pub expires: Option<Instant>,
}
```

### Config

```rust
pub struct Config {
    pub query_timeout: Duration,
    pub record_ttl: Option<Duration>,
    pub record_replication_interval: Duration,
    pub record_publication_interval: Duration,
    pub replication_factor: NonZeroUsize,
}
```

## Operations

| Operation | Description |
|-----------|-------------|
| `get_record` | Retrieve value by key |
| `put_record` | Store key-value pair |
| `get_closest_peers` | Find peers near key |
| `bootstrap` | Join network via known peers |

## NOA Usage

Used for:
- Peer discovery
- Model weight distribution
- Task routing

```rust
use libp2p::kad::{Behaviour, Config, store::MemoryStore, Mode};

let store = MemoryStore::new(local_peer_id);
let behaviour = Behaviour::with_config(local_peer_id, store, Config::default());

// Bootstrap
for addr in bootstrap_peers {
    behaviour.add_address(&peer_id, addr);
}
behaviour.bootstrap()?;

// Store record
let key = Key::new(&b"model/qwen2.5"[..]);
behaviour.put_record(Record::new(key, model_hash), Quorum::One)?;
```

## See Also

- [identify](identify.md) — Peer identification
- [gossipsub](gossipsub.md) — Pub/sub messaging
