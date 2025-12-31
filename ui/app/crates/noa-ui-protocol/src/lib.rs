use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u32 = 1;

pub mod topics {
    pub const PRESENCE: &str = "noa-ui/v1/presence";
    pub const STATE_OP: &str = "noa-ui/v1/state/op";
    pub const RELEASE_MANIFEST: &str = "noa-ui/v1/release/manifest";
    pub const ALERT: &str = "noa-ui/v1/alert";
}

pub mod endpoints {
    pub const HANDSHAKE: &str = "noa-ui/v1/handshake";

    pub const STATE_OPS_GET: &str = "noa-ui/v1/state/ops/get";
    pub const STATE_SNAPSHOT_GET: &str = "noa-ui/v1/state/snapshot/get";

    pub const RELEASE_MANIFEST_GET: &str = "noa-ui/v1/release/manifest/get";
    pub const RELEASE_ARTIFACT_GET: &str = "noa-ui/v1/release/artifact/get";
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Capabilities {
    pub supports_desktop: bool,
    pub supports_web: bool,
    pub supports_mobile: bool,
    pub supports_server: bool,
    pub supports_state_sync: bool,
    pub supports_updates: bool,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            supports_desktop: true,
            supports_web: true,
            supports_mobile: false,
            supports_server: false,
            supports_state_sync: true,
            supports_updates: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PresenceAnnounce {
    pub device_id: String,
    pub protocol_version: u32,
    pub app_version: String,
    pub build_id: String,
    pub capabilities: Capabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HandshakeRequest {
    pub device_id: String,
    pub protocol_version: u32,
    pub app_version: String,
    pub build_id: String,
    pub capabilities: Capabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HandshakeResponse {
    pub accepted: bool,
    pub protocol_version: u32,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OpId {
    pub device_id: String,
    pub counter: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StateOp {
    pub op_id: OpId,
    pub lamport: u64,
    pub entity: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetOpsRequest {
    pub after: Option<OpId>,
    pub max: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetOpsResponse {
    pub ops: Vec<StateOp>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetSnapshotRequest {
    pub want_snapshot_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetSnapshotResponse {
    pub snapshot_id: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseArtifact {
    pub artifact_id: String,
    pub platform: String,
    pub os: String,
    pub arch: String,
    pub sha256: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleaseManifest {
    pub manifest_id: String,
    pub protocol_version: u32,
    pub app_version: String,
    pub build_id: String,
    pub artifacts: Vec<ReleaseArtifact>,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetManifestRequest {
    pub want_manifest_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetManifestResponse {
    pub manifest: Option<ReleaseManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetArtifactRequest {
    pub artifact_id: String,
    pub offset: u64,
    pub max_bytes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetArtifactResponse {
    pub artifact_id: String,
    pub offset: u64,
    pub data: Vec<u8>,
    pub done: bool,
}
