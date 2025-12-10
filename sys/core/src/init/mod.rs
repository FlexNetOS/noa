//! NOA Initialization Module
//!
//! Implements US1: Initialize NOA Seed Environment
//! §3.1: Self-Contained & Autonomous - creates complete directory structure
//! §3.2: Local-First & Offline-Capable - works fully offline

pub mod config;
pub mod database;
pub mod paths;
pub mod structure;

#[cfg(test)]
mod tests;

pub use config::ConfigGenerator;
pub use database::DatabaseInitializer;
pub use paths::NoaPaths;
pub use structure::DirectoryStructure;

