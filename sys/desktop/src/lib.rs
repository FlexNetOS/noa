//! NOA Desktop Containment Layer (NDCL)
//! Exposes proxy/auth modules used by desktop app wrappers.

#[path = "../proxy/proxy.rs"]
pub mod proxy;

#[path = "../proxy/config.rs"]
pub mod proxy_config;

#[path = "../auth/oauth_proxy.rs"]
pub mod oauth_proxy;

#[path = "../auth/vault.rs"]
pub mod vault;
