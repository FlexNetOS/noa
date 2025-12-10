//! NOA Kernel Module
//!
//! Provides kernel independence and isolation capabilities.

pub mod abstraction;
pub mod file;
pub mod process;
pub mod network;
pub mod platform;
pub mod nkal;

pub use abstraction::{DefaultKernel, KernelAbstraction};
pub use file::{FileOps, FileStat};
pub use network::NetworkOps;
pub use nkal::*;
pub use platform::PlatformInfo;
pub use process::{ProcessOps, ProcessResult};
