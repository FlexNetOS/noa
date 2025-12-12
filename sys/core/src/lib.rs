//! NOA Core Library
//!
//! Library crate for NOA core functionality

// Module declarations for library crate
pub mod adapters;
pub mod agents;
pub mod api;
pub mod auth;
pub mod autonomy;
pub mod cecca;
pub mod cli;
pub mod config;
#[path = "connectors/mod.rs"]
pub mod connectors;
pub mod db;
pub mod error;
pub mod export;
pub mod features;
pub mod governance;
pub mod graphs;
pub mod healing;
pub mod init;
pub mod kernel;
pub mod knowledge;
pub mod learning;
pub mod logging;
pub mod memory;
pub mod modules;
pub mod neural;
pub mod nkal;
pub mod observability;
pub mod orchestration;
pub mod platform;
pub mod policy;
pub mod predict;
pub mod providers;
pub mod regression;
pub mod resources;
pub mod self_improve;
pub mod services;
pub mod vector;
pub mod vhdx;
