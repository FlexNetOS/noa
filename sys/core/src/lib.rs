//! NOA Core Library
//!
//! Library crate for NOA core functionality

// Always-available core modules (required for `noa init`)
pub mod cli;
pub mod config;
pub mod db;
pub mod error;
pub mod init;
pub mod logging;
pub mod timestamp;

// Optional subsystems
#[cfg(feature = "full")]
pub mod api;
#[cfg(feature = "full")]
pub mod autonomy;
#[cfg(feature = "full")]
pub mod events;
#[cfg(feature = "full")]
pub mod healing;
#[cfg(feature = "full")]
pub mod memory;
#[cfg(feature = "full")]
pub mod modules;
#[cfg(feature = "full")]
pub mod providers;
#[cfg(feature = "full")]
pub mod neural;
#[cfg(feature = "full")]
pub mod learning;
#[cfg(feature = "full")]
pub mod observability;
#[cfg(feature = "full")]
pub mod services;
#[cfg(feature = "full")]
pub mod vector;
#[cfg(feature = "full")]
pub mod agents;
#[cfg(feature = "full")]
pub mod automation;
#[cfg(feature = "full")]
pub mod virtual_packages;
