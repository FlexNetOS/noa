//! NOA Mobile Companion Stub
//! Phase 13: P2P-only mobile companion to participate in hive-mind.

pub mod p2p_client;
pub mod ui;

pub use p2p_client::{MobileP2PClient, MobileP2PClientConfig};
pub use ui::{CompanionUiState, UiStatus};
