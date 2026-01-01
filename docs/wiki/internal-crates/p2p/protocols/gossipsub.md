# gossipsub Protocol

Pub/sub messaging with mesh networking.

**Location**: `p2p/protocols/gossipsub/`  
**Version**: 0.50.0  
**Crate**: `libp2p-gossipsub`

## Overview

GossipSub is a pubsub protocol for efficient message propagation:

- Topic-based subscriptions
- Mesh-based message routing
- Duplicate detection
- Score-based peer management

## Key Types

### Behaviour

```rust
pub struct Behaviour<D, F> {
    // Message routing table
    mesh: HashMap<TopicHash, HashSet<PeerId>>,
    // Pending messages
    pending: VecDeque<Message>,
}
```

### Message

```rust
pub struct Message {
    pub source: Option<PeerId>,
    pub data: Vec<u8>,
    pub sequence_number: u64,
    pub topic: TopicHash,
}
```

### Config

```rust
pub struct Config {
    pub mesh_n: usize,              // Target mesh size (default: 6)
    pub mesh_n_low: usize,          // Min mesh size (default: 4)
    pub mesh_n_high: usize,         // Max mesh size (default: 12)
    pub gossip_lazy: usize,         // Peers for gossip (default: 6)
    pub heartbeat_interval: Duration,
    pub history_length: usize,
    pub history_gossip: usize,
}
```

## NOA Usage

Used for:
- Agent federation messages
- Knowledge graph sync
- Task distribution

```rust
use libp2p::gossipsub::{Behaviour, Config, MessageAuthenticity, Topic};

let config = Config::default();
let behaviour = Behaviour::new(
    MessageAuthenticity::Signed(keypair),
    config,
)?;

// Subscribe to topic
let topic = Topic::new("noa/agents/v1");
behaviour.subscribe(&topic)?;

// Publish message
behaviour.publish(topic, b"hello")?;
```

## See Also

- [kad](kad.md) — DHT routing
- [floodsub](floodsub.md) — Simple pubsub (deprecated)
