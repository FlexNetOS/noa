# ADR-002: libp2p for Networking

## Status

Accepted

## Context

NOA needs peer-to-peer networking for:
- Distributed compute
- Model sharing
- Knowledge synchronization
- Agent federation

Options considered:
1. Custom protocol
2. libp2p
3. ZeroMQ
4. gRPC

## Decision

Use libp2p (Rust implementation) for P2P networking.

## Rationale

1. **Modular**: Pick protocols à la carte
2. **NAT traversal**: Built-in relay and hole punching
3. **Encryption**: Noise/TLS by default
4. **Transport agnostic**: TCP, QUIC, WebSocket, WebRTC
5. **Proven**: Used by IPFS, Filecoin, Polkadot
6. **Active development**: Well-maintained Rust implementation

## Consequences

### Positive
- Rich protocol ecosystem
- Browser compatibility (WASM)
- Strong security defaults
- Good documentation

### Negative
- Learning curve
- Complex configuration
- Large dependency tree

## Mitigations

- Sensible defaults in configuration
- Comprehensive documentation
- Maintain our own fork for stability

## References

- [libp2p Documentation](https://docs.libp2p.io/)
- [rust-libp2p](https://github.com/libp2p/rust-libp2p)
