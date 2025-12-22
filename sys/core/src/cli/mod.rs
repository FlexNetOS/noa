//! NOA CLI Commands Module
//!
//! Implements all CLI commands for NOA.

pub mod init;
<<<<<<< HEAD
=======
pub mod start;
pub mod status;
pub mod stop;
pub mod db;
pub mod providers;
pub mod modules;
pub mod memory;
pub mod p2p;
pub mod plane;
pub mod promotion;
pub mod healing;
pub mod agents;
pub mod tasks;
pub mod goal;
pub mod logs;
pub mod capsule;
pub mod crm;
pub mod digest;
>>>>>>> d68c02c8d3 (WIP on develop: 18ff7fc8 Remove large build artifacts from git tracking)

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
pub mod digest;

#[cfg(feature = "full")]
pub use memory::{execute_memory, MemoryArgs};
#[cfg(feature = "full")]
pub use p2p::{execute_p2p, P2PArgs};
