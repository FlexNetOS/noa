//! NOA CLI Commands Module
//!
//! Implements all CLI commands for NOA.

pub mod agents;
pub mod ask;
pub mod capsule;
pub mod config;
pub mod connectors;
pub mod crm;
pub mod db;
pub mod digest;
pub mod features;
pub mod goal;
pub mod healing;
pub mod improve;
pub mod init;
pub mod logs;
pub mod memory;
pub mod models;
pub mod modules;
pub mod p2p;
pub mod plane;
pub mod promotion;
pub mod providers;
pub mod speckit;
pub mod start;
pub mod status;
pub mod stop;
pub mod tasks;

pub use memory::{execute_memory, MemoryArgs};
pub use p2p::{execute_p2p, P2PArgs};
