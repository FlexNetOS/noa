# P2P Connectivity Runbook

Handle P2P network issues.

---

## Metadata

| Field | Value |
|-------|-------|
| **ID** | `p2p-connectivity` |
| **Trigger** | Peer unreachable, gossip failures |
| **Impact** | Distributed features unavailable |
| **Owner** | Platform Team |
| **Escalation** | On-call SRE |
| **Severity** | S2 |
| **Last-Verified** | 2026-01-01 |

---

## Prerequisites

- [ ] Access to NOA logs
- [ ] Network access to test connectivity
- [ ] Understanding of libp2p

---

## Triage

### 1. Check P2P Status

```bash
# Get peer count
noa p2p status

# List connected peers
noa p2p peers
```

### 2. Check NAT Status

```bash
# Get NAT detection result
noa p2p nat-status
```

| Status | Meaning | Action |
|--------|---------|--------|
| `Public` | Directly reachable | No action needed |
| `Private` | Behind NAT | Relay should be active |
| `Unknown` | Detection failed | Check network config |

---

## Common Issues

### No Peers Found

```bash
# Add bootstrap peers manually
noa p2p add-peer /ip4/1.2.3.4/tcp/4001/p2p/12D3Koo...

# Check bootstrap configuration
noa config get p2p.bootstrap_peers
```

### Relay Not Working

```bash
# Check relay status
noa p2p relay-status

# Force relay reconnection
noa p2p relay-reconnect
```

### Firewall Issues

```bash
# Check listening ports
netstat -an | grep LISTEN | grep -E "4001|8080"

# Verify UPnP
noa p2p upnp-status
```

---

## Verification

- [ ] `noa p2p status` shows healthy
- [ ] Peer count > 0
- [ ] NAT status is known
- [ ] Gossip messages flowing

---

## See Also

- [system-startup.md](system-startup.md) — Start services
- [wiki/internal-crates/p2p](../wiki/internal-crates/p2p/index.md) — P2P documentation
