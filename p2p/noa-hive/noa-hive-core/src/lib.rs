//! NOA-Hive Core Types
//!
//! Core types and primitives for the NOA-Hive P2P coordination layer.
//! Derived from hyveos-core with NOA-specific modifications.

pub mod error;
pub mod message;
pub mod peer;
pub mod topic;

pub use error::{Error, Result};
pub use peer::PeerId;
pub use topic::Topic;

/// Protocol version for wire compatibility.
pub const PROTOCOL_VERSION: &str = "noa-hive/1.0.0";

/// Default protocol prefix for topics and DHT keys.
pub const PROTOCOL_PREFIX: &str = "noa-hive/v1";
