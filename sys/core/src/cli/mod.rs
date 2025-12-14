//! NOA CLI Commands Module
//!
//! Implements all CLI commands for NOA.

pub mod init;
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

pub use memory::{MemoryArgs, execute_memory};
pub use p2p::{P2PArgs, execute_p2p};
