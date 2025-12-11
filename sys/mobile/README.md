# NOA Mobile Companion (Phase 13)

Stub mobile companion that participates in the P2P hive-mind. Current scope:

- Lightweight Rust crate in `sys/mobile` (not yet shipped to app stores).
- P2P client stub (`src/p2p_client.rs`) that tracks connection/heartbeat state only.
- Minimal UI state model (`src/ui/`) for companion mode surfaces.

Build locally:

```bash
cd sys/mobile
cargo check
```

Future work: integrate real transport, mobile UI toolkits, and platform-native packaging.
