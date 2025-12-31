# NOA UI hive mind swarm protocol (v1)

This document defines the **wire protocol** used by platform binaries to stay in sync via the P2P hive mind swarm.

Scope:
- Device discovery + capability/version handshake
- State replication (oplog + snapshot)
- Release/binary sync (announce + fetch + verify)

## Versioning

- `PROTOCOL_VERSION`: monotonic integer. Bump on breaking changes.
- `APP_VERSION`: SemVer from the running binary/package.
- `BUILD_ID`: build metadata (git SHA or CI build id) used for “same build?” checks.

Rules:
- If `protocol_version` mismatches, **do not** attempt state sync.
- If `protocol_version` matches but `app_version` differs, state sync may still proceed, but feature flags/capabilities must be respected.

## GossipSub topics

All topics are namespaced and versioned:

- `noa-ui/v1/presence`
  - Periodic presence announcements (peer id, device id, capabilities)
- `noa-ui/v1/state/op`
  - Broadcast of state operations (append-only oplog entries)
- `noa-ui/v1/release/manifest`
  - Announce latest release manifest (platform artifacts + hashes)
- `noa-ui/v1/alert`
  - Optional: non-state alerts (e.g., protocol mismatch, required update)

## Request/Response endpoints

All request/response are CBOR (recommended) or JSON (dev) using the following protocol ids:

- `noa-ui/v1/handshake`
  - Request: `HandshakeRequest`
  - Response: `HandshakeResponse`

- `noa-ui/v1/state/ops/get`
  - Request: `GetOpsRequest`
  - Response: `GetOpsResponse`

- `noa-ui/v1/state/snapshot/get`
  - Request: `GetSnapshotRequest`
  - Response: `GetSnapshotResponse`

- `noa-ui/v1/release/manifest/get`
  - Request: `GetManifestRequest`
  - Response: `GetManifestResponse`

- `noa-ui/v1/release/artifact/get`
  - Request: `GetArtifactRequest`
  - Response: `GetArtifactResponse` (chunked)

## Message types (high level)

### Presence

- `PresenceAnnounce`
  - `device_id`, `app_version`, `build_id`, `protocol_version`
  - `capabilities` (supports_web, supports_desktop, supports_server, supports_update, supports_state_sync)
  - optional: `listen_addrs` for direct connections

### State replication

- `StateOp`
  - `op_id` = `(device_id, counter)`
  - `lamport` timestamp
  - `actor` (device_id)
  - `entity` (string)
  - `payload` (JSON value) — v1 uses JSON payload for agility

- `Snapshot`
  - `snapshot_id` (hash)
  - `state_version` (cursor)
  - `payload` (JSON value or CBOR blob)

### Release sync

- `ReleaseManifest`
  - `manifest_id` (hash)
  - `app_version`, `build_id`, `protocol_version`
  - `artifacts[]` { platform, arch, os, sha256, size, url? }
  - `signature` (future: required)

- `ArtifactChunk`
  - `artifact_id` (hash)
  - `offset`, `data`, `done`

## Security

- v1: hash verification mandatory; signatures strongly recommended.
- Keys must be stored under `$NOA_DATA/apps/noa-ui/p2p/keys/`.
