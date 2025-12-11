//! Feature flag framework (US10)
//!
//! Provides feature flag loading, evaluation, and persistence backed by
//! `config/features.json`. Flags are simple booleans grouped by prefix
//! (e.g., `connectors.github`).

#[path = "flags.rs"]
pub mod flags;
