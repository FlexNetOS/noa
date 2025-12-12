//! NOA Kernel Module
//!
//! Provides kernel independence and isolation capabilities.

pub mod abstraction;
pub mod file;
pub mod network;
pub mod nkal;
pub mod platform;
pub mod process;

pub use abstraction::{DefaultKernel, KernelAbstraction};
pub use file::{FileOps, FileStat};
pub use network::NetworkOps;
pub use nkal::*;
pub use platform::PlatformInfo;
pub use process::{ProcessOps, ProcessResult};
