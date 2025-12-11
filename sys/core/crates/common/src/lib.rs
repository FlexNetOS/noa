//! NOA Common Library
//!
//! Shared types, utilities, and traits used across all NOA crates.

pub mod error;
pub mod hash;
pub mod types;

pub use error::{NoaError, Result};
pub use types::*;
