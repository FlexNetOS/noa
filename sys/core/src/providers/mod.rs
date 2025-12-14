//! Provider framework (Phase 2.6 - Shared Provider Execution Memory)
//! Implements provider registry, shared memory bus, and collaborative orchestration stubs.

pub mod base;
pub mod registry;
pub mod health;
pub mod selector;
pub mod shared_memory;
pub mod context_manager;
pub mod sync;
pub mod persistence;
pub mod rate_limits;
pub mod backoff;

pub mod llama;
pub mod claude;
pub mod codex;
pub mod copilot;
pub mod git;
pub mod cursor;
pub mod abacus;

pub mod collaborative;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, MutexGuard};

/// Basic provider status representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderStatus {
    Ready,
    Busy,
    Degraded,
    Offline,
    Unknown,
}

/// Provider information stored in the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    pub id: String,
    pub kind: String,
    pub priority: i32,
    pub status: ProviderStatus,
    pub enabled: bool,
    pub capabilities: Vec<String>,
}

static PROVIDERS: OnceCell<Mutex<Vec<ProviderInfo>>> = OnceCell::new();

fn providers_state() -> MutexGuard<'static, Vec<ProviderInfo>> {
    PROVIDERS
        .get_or_init(|| Mutex::new(default_providers()))
        .lock()
        .expect("providers mutex poisoned")
}

fn default_providers() -> Vec<ProviderInfo> {
    vec![
        ProviderInfo {
            id: "llama.cpp".into(),
            kind: "local".into(),
            priority: 1,
            status: ProviderStatus::Ready,
            enabled: true,
            capabilities: vec!["reasoning".into(), "embedding".into()],
        },
        ProviderInfo {
            id: "cursor".into(),
            kind: "ide".into(),
            priority: 2,
            status: ProviderStatus::Ready,
            enabled: true,
            capabilities: vec!["code".into(), "context".into()],
        },
        ProviderInfo {
            id: "claude".into(),
            kind: "cloud".into(),
            priority: 3,
            status: ProviderStatus::Ready,
            enabled: true,
            capabilities: vec!["reasoning".into()],
        },
        ProviderInfo {
            id: "codex".into(),
            kind: "cloud".into(),
            priority: 4,
            status: ProviderStatus::Ready,
            enabled: true,
            capabilities: vec!["code".into()],
        },
        ProviderInfo {
            id: "copilot".into(),
            kind: "ide".into(),
            priority: 5,
            status: ProviderStatus::Ready,
            enabled: true,
            capabilities: vec!["autocomplete".into()],
        },
        ProviderInfo {
            id: "git".into(),
            kind: "local".into(),
            priority: 6,
            status: ProviderStatus::Ready,
            enabled: true,
            capabilities: vec!["vcs".into()],
        },
        ProviderInfo {
            id: "abacus".into(),
            kind: "cloud".into(),
            priority: 7,
            status: ProviderStatus::Ready,
            enabled: true,
            capabilities: vec!["analytics".into()],
        },
    ]
}

/// List all providers from in-memory registry
pub fn list_providers() -> Vec<ProviderInfo> {
    providers_state().clone()
}

/// Get provider by id
pub fn get_provider(id: &str) -> Option<ProviderInfo> {
    providers_state().iter().find(|p| p.id == id).cloned()
}

/// Update enabled flag for a provider
pub fn set_provider_enabled(id: &str, enabled: bool) -> bool {
    let mut guard = providers_state();
    if let Some(p) = guard.iter_mut().find(|p| p.id == id) {
        p.enabled = enabled;
        return true;
    }
    false
}

/// Update status for a provider
pub fn set_provider_status(id: &str, status: ProviderStatus) -> bool {
    let mut guard = providers_state();
    if let Some(p) = guard.iter_mut().find(|p| p.id == id) {
        p.status = status;
        return true;
    }
    false
}
