//! NOA CLI Commands Module
//!
//! Implements all CLI commands for NOA.

pub mod init;

#[cfg(feature = "full")]
pub mod start;
#[cfg(feature = "full")]
pub mod status;
#[cfg(feature = "full")]
pub mod stop;

pub mod db;

#[cfg(feature = "full")]
pub mod providers;
#[cfg(feature = "full")]
pub mod modules;
#[cfg(feature = "full")]
pub mod memory;
#[cfg(feature = "full")]
pub mod p2p;
#[cfg(feature = "full")]
pub mod plane;
#[cfg(feature = "full")]
pub mod promotion;
#[cfg(feature = "full")]
pub mod healing;
#[cfg(feature = "full")]
pub mod agents;
#[cfg(feature = "full")]
pub mod tasks;
#[cfg(feature = "full")]
pub mod goal;
#[cfg(feature = "full")]
pub mod logs;
#[cfg(feature = "full")]
pub mod capsule;
#[cfg(feature = "full")]
pub mod crm;

#[cfg(feature = "full")]
pub use memory::{execute_memory, MemoryArgs};
#[cfg(feature = "full")]
pub use p2p::{execute_p2p, P2PArgs};
