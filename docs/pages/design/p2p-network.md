# P2P Network Design

NOA uses libp2p for peer-to-peer networking.

## Network Topology

```
      ┌───────┐
      │ Relay │
      └───┬───┘
          │
    ┌─────┴─────┐
    │           │
┌───▼───┐   ┌───▼───┐
│ NOA A │◄──│ NOA B │
└───────┘   └───┬───┘
                │
            ┌───▼───┐
            │ NOA C │
            └───────┘
```

## Protocols Used

| Protocol | Purpose |
|----------|---------|
| Gossipsub | Pub/sub messaging |
| Kademlia | DHT for discovery |
| Identify | Peer info exchange |
| Relay | NAT traversal |
| DCUtR | Direct connection upgrade |
| Ping | Liveness checks |

## Transport Stack

```
┌─────────────────────────────────────────┐
│             Application                  │
├─────────────────────────────────────────┤
│             Swarm                        │
├─────────────────────────────────────────┤
│  Gossipsub │ Kad │ Identify │ Relay    │
├─────────────────────────────────────────┤
│           QUIC Transport                 │
│     (includes encryption + muxing)       │
└─────────────────────────────────────────┘
```

## Use Cases

1. **Agent Federation**: Distribute work across peers
2. **Model Sharing**: P2P model distribution
3. **Knowledge Sync**: Share knowledge graphs
4. **Compute Offload**: Route tasks to available peers

## See Also

- [Architecture Overview](architecture.md)
- [P2P Crates](../../wiki/internal-crates/p2p/index.md)
