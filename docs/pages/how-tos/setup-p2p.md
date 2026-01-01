# How-To: Setup P2P Network

Connect NOA instances via peer-to-peer networking.

## Prerequisites

- NOA built with `full` feature
- Network access (port 4001 by default)
- At least one other NOA peer or bootstrap node

## Steps

### 1. Enable P2P

Edit `~/.noa/config/config.toml`:

```toml
[p2p]
enabled = true
listen_addresses = ["/ip4/0.0.0.0/tcp/4001"]

# Optional: Public IP if known
external_address = "/ip4/YOUR.PUBLIC.IP/tcp/4001"
```

### 2. Generate Identity

```bash
# NOA generates identity on first run
noa run

# View peer ID
noa p2p identity
# Output: 12D3KooW...
```

### 3. Add Bootstrap Peers

```toml
[p2p.bootstrap]
peers = [
    "/ip4/1.2.3.4/tcp/4001/p2p/12D3KooWBootstrap...",
]
```

Or dynamically:

```bash
noa p2p add-peer /ip4/1.2.3.4/tcp/4001/p2p/12D3KooW...
```

### 4. Configure NAT Traversal

```toml
[p2p.nat]
# Enable UPnP
upnp = true

# Use relay if behind NAT
relay = true
relay_servers = [
    "/ip4/relay.noa.network/tcp/4001/p2p/12D3KooWRelay..."
]
```

### 5. Verify Connectivity

```bash
# Check status
noa p2p status

# List peers
noa p2p peers

# Check NAT status
noa p2p nat-status
```

### 6. Join Topics

```bash
# Subscribe to gossip topic
noa p2p subscribe noa/agents/v1

# Publish message
noa p2p publish noa/agents/v1 "Hello, peers!"
```

## Firewall Configuration

| Port | Protocol | Purpose |
|------|----------|---------|
| 4001 | TCP/UDP | libp2p |
| 4001 | UDP | QUIC |

## Troubleshooting

### No Peers Found

1. Check bootstrap peers are online
2. Verify firewall allows port 4001
3. Enable relay if behind strict NAT

### Relay Not Working

1. Verify relay server is online
2. Check relay reservation: `noa p2p relay-status`
3. Force reconnect: `noa p2p relay-reconnect`

## See Also

- [P2P Network Design](../design/p2p-network.md)
- [P2P Connectivity Runbook](../../runbooks/p2p-connectivity.md)
