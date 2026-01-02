# NOA-Hive: P2P Device Coordination Layer

NOA-Hive is a software-only P2P coordination layer for the NOA platform, derived from [p2p-industries/hyveos](https://github.com/p2p-industries/hyveos) with modifications for NOA's requirements.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         NOA Application                         │
├─────────────────────────────────────────────────────────────────┤
│                      noa-hive-sdk (gRPC)                        │
├─────────────────────────────────────────────────────────────────┤
│  noa-hive-runtime  │  noa-hive-stack  │  noa-hive-core         │
├─────────────────────────────────────────────────────────────────┤
│                 libp2p (GossipSub, Kademlia, R/R)               │
├─────────────────────────────────────────────────────────────────┤
│           loro (CRDT)           │        iroh (blobs)           │
└─────────────────────────────────────────────────────────────────┘
```

## Crates

| Crate | Purpose | Derived From |
|-------|---------|--------------|
| `noa-hive-core` | Core types, PeerId, messages | `hyveos-core` |
| `noa-hive-stack` | P2P network stack (GossipSub, DHT, R/R) | `p2p-stack` |
| `noa-hive-runtime` | Daemon runtime, service management | `runtime` |
| `noa-hive-config` | Configuration management | `config` |
| `noa-hive-sdk` | Client SDK for applications | `hyveos-sdk` |

## Key Differences from hyveos

1. **Software-only**: Removed mesh-hardware dependencies (`batman-neighbours-*`, `ifaddr`, `ifwatcher`, `macaddress`)
2. **Naming**: Renamed to `noa-hive-*` namespace
3. **State sync**: Added `loro` CRDT for collaborative state synchronization
4. **Binary distribution**: Added `iroh` for efficient binary/model distribution
5. **Protocol compatibility**: Wire protocol remains compatible for potential upstream contributions

## Capabilities

### From libp2p
- **GossipSub**: Pub/Sub messaging for state broadcasts
- **Kademlia DHT**: Distributed key-value store for discovery
- **Request-Response**: Direct peer communication
- **Identify**: Peer identification and capability exchange

### From loro
- **CRDT State**: Conflict-free replicated data types for UI state
- **Collaborative Editing**: Real-time multi-device state sync

### From iroh
- **Blob Transfer**: Efficient large file/model distribution
- **Content Addressing**: Deduplication via content hashes

## Wire Protocol

See [HIVE_PROTOCOL.md](../../ui/app/HIVE_PROTOCOL.md) for the wire protocol specification.

### Topics (GossipSub)
- `noa-hive/v1/presence` - Device presence announcements
- `noa-hive/v1/state/op` - CRDT operations for state sync
- `noa-hive/v1/release/manifest` - Binary release notifications

### DHT Keys (Kademlia)
- `/noa-hive/device/{peer_id}` - Device metadata
- `/noa-hive/model/{model_hash}` - Model location index
- `/noa-hive/state/{room_id}` - State room membership

## Usage

### As a Daemon (noa-hived)

```bash
# Start the daemon
noa-hived --config /etc/noa/hive.toml

# Check status
noa-ctl whoami
# => 🤖 You are { 12D3KooW... }

# Discover peers
noa-ctl peers list
```

### As a Library

```rust
use noa_hive_sdk::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::connect("http://127.0.0.1:50051").await?;
    
    // Get our peer ID
    let peer_id = client.whoami().await?;
    println!("Connected as: {}", peer_id);
    
    // Publish to a topic
    client.pubsub()
        .publish("noa-hive/v1/presence", b"hello")
        .await?;
    
    // Store in DHT
    client.dht()
        .put("/noa-hive/device/config", b"value")
        .await?;
    
    Ok(())
}
```

## Configuration

```toml
# /etc/noa/hive.toml

[network]
listen_addrs = ["/ip4/0.0.0.0/tcp/0", "/ip4/0.0.0.0/udp/0/quic-v1"]
bootstrap_peers = []
enable_mdns = true
enable_relay = true

[storage]
data_dir = "${NOA_ROOT}/data/hive"
state_db = "state.db"

[grpc]
listen_addr = "127.0.0.1:50051"

[loro]
enable = true
sync_interval_ms = 100

[iroh]
enable = true
blob_store = "${NOA_ROOT}/cache/blobs"
```

## License

MIT OR Apache-2.0 (same as hyveos)

## Attribution

This project is derived from [hyveos](https://github.com/p2p-industries/hyveos) by P2P Industries. We maintain wire protocol compatibility and intend to contribute improvements upstream where applicable.
