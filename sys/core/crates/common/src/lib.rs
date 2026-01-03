//! NOA Common Library
//!
//! Shared types, utilities, and traits used across all NOA crates.

pub mod error;
pub mod types;
pub mod hash;

#[cfg(feature = "compression")]
pub mod compression;

pub use error::{NoaError, Result};
pub use types::*;

