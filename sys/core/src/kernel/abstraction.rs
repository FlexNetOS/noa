//! Kernel abstraction trait used by secondary adapters and higher layers.
//!
//! Provides a thin wrapper around file, process, network, and platform helpers
//! to keep isolation-friendly logic behind a single interface.

use super::file::FileOps;
use super::network::NetworkOps;
use super::platform::PlatformInfo;
use super::process::ProcessOps;

/// Trait describing the capabilities exposed by the kernel layer.
pub trait KernelAbstraction {
    /// File operations (read/write/stat).
    fn file(&self) -> &FileOps;
    /// Process operations (spawn/run).
    fn process(&self) -> &ProcessOps;
    /// Network utilities.
    fn network(&self) -> &NetworkOps;
    /// Platform metadata.
    fn platform(&self) -> &PlatformInfo;
}

/// Default implementation backed by the standard library.
#[derive(Debug, Default)]
pub struct DefaultKernel {
    file: FileOps,
    process: ProcessOps,
    network: NetworkOps,
    platform: PlatformInfo,
}

impl DefaultKernel {
    /// Create a kernel abstraction with freshly detected platform info.
    pub fn new() -> Self {
        Self {
            platform: PlatformInfo::detect(),
            ..Default::default()
        }
    }
}

impl KernelAbstraction for DefaultKernel {
    fn file(&self) -> &FileOps {
        &self.file
    }

    fn process(&self) -> &ProcessOps {
        &self.process
    }

    fn network(&self) -> &NetworkOps {
        &self.network
    }

    fn platform(&self) -> &PlatformInfo {
        &self.platform
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_kernel_provides_all_components() {
        let kernel = DefaultKernel::new();
        assert!(!kernel.platform().os.is_empty());
        assert!(!kernel.platform().arch.is_empty());
    }
}
