//! NOA Kernel Abstraction Layer (NKAL)
//!
//! Provides a unified interface for kernel independence across different isolation modes.
//! Per NOA Constitution §3.11: Kernel Independence Layer (NKAL)
//!
//! # Isolation Modes
//!
//! - **Native**: Direct execution on host (Windows/Linux/macOS)
//! - **VM**: Full isolation via hypervisor (Hyper-V, KVM, Virtualization.framework)
//! - **Container**: Lightweight isolation via containers (Docker, Podman)
//! - **Sandbox**: Minimal isolation for testing (Windows Sandbox)
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    NOA Application Layer                     │
//! ├─────────────────────────────────────────────────────────────┤
//! │                    NKAL Interface (this module)              │
//! ├─────────────────────────────────────────────────────────────┤
//! │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │
//! │  │  Native  │  │    VM    │  │Container │  │ Sandbox  │    │
//! │  │  Driver  │  │  Driver  │  │  Driver  │  │  Driver  │    │
//! │  └──────────┘  └──────────┘  └──────────┘  └──────────┘    │
//! ├─────────────────────────────────────────────────────────────┤
//! │  Windows      Linux         Docker        Windows Sandbox   │
//! │  macOS        KVM/QEMU      Podman        (lightweight)     │
//! │               Hyper-V       rootless                        │
//! │               VirtFW                                        │
//! └─────────────────────────────────────────────────────────────┘
//! ```

use std::path::PathBuf;

/// Kernel isolation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelMode {
    /// Direct execution on host OS
    Native,
    /// Full VM isolation (Hyper-V, KVM, Virtualization.framework)
    VM,
    /// Container-based isolation (Docker, Podman)
    Container,
    /// Lightweight sandbox isolation
    Sandbox,
}

impl Default for KernelMode {
    fn default() -> Self {
        KernelMode::Native
    }
}

impl std::fmt::Display for KernelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KernelMode::Native => write!(f, "native"),
            KernelMode::VM => write!(f, "vm"),
            KernelMode::Container => write!(f, "container"),
            KernelMode::Sandbox => write!(f, "sandbox"),
        }
    }
}

impl std::str::FromStr for KernelMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "native" => Ok(KernelMode::Native),
            "vm" => Ok(KernelMode::VM),
            "container" => Ok(KernelMode::Container),
            "sandbox" => Ok(KernelMode::Sandbox),
            _ => Err(format!("Unknown kernel mode: {}", s)),
        }
    }
}

/// Platform-specific hypervisor type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hypervisor {
    /// Microsoft Hyper-V (Windows)
    HyperV,
    /// KVM/QEMU (Linux)
    KVM,
    /// Apple Virtualization.framework (macOS)
    VirtualizationFramework,
    /// No hypervisor available
    None,
}

/// Container runtime type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerRuntime {
    /// Docker (rootless preferred)
    Docker,
    /// Podman (rootless by default)
    Podman,
    /// containerd
    Containerd,
    /// No container runtime available
    None,
}

/// Platform capabilities detection result
#[derive(Debug, Clone)]
pub struct PlatformCapabilities {
    /// Host operating system
    pub os: String,
    /// CPU architecture
    pub arch: String,
    /// Available hypervisor
    pub hypervisor: Hypervisor,
    /// Available container runtime
    pub container_runtime: ContainerRuntime,
    /// Whether sandbox mode is available
    pub sandbox_available: bool,
    /// Current kernel mode
    pub current_mode: KernelMode,
    /// NOA root directory
    pub noa_root: PathBuf,
}

impl Default for PlatformCapabilities {
    fn default() -> Self {
        Self {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            hypervisor: Hypervisor::None,
            container_runtime: ContainerRuntime::None,
            sandbox_available: false,
            current_mode: KernelMode::Native,
            noa_root: PathBuf::new(),
        }
    }
}

/// NKAL configuration
#[derive(Debug, Clone)]
pub struct NkalConfig {
    /// Preferred kernel mode
    pub preferred_mode: KernelMode,
    /// Fallback mode if preferred is unavailable
    pub fallback_mode: KernelMode,
    /// VM image path (for VM mode)
    pub vm_image_path: Option<PathBuf>,
    /// Container image name (for Container mode)
    pub container_image: Option<String>,
    /// Auto-detect best available mode
    pub auto_detect: bool,
}

impl Default for NkalConfig {
    fn default() -> Self {
        Self {
            preferred_mode: KernelMode::Native,
            fallback_mode: KernelMode::Native,
            vm_image_path: None,
            container_image: None,
            auto_detect: true,
        }
    }
}

/// NKAL error types
#[derive(Debug)]
pub enum NkalError {
    /// Requested mode is not available on this platform
    ModeUnavailable(KernelMode),
    /// Hypervisor not found or not enabled
    HypervisorNotFound,
    /// Container runtime not found
    ContainerRuntimeNotFound,
    /// VM image not found
    VmImageNotFound(PathBuf),
    /// Configuration error
    ConfigError(String),
    /// IO error
    IoError(std::io::Error),
}

impl std::fmt::Display for NkalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NkalError::ModeUnavailable(mode) => {
                write!(f, "Kernel mode '{}' is not available on this platform", mode)
            }
            NkalError::HypervisorNotFound => write!(f, "No hypervisor found or enabled"),
            NkalError::ContainerRuntimeNotFound => write!(f, "No container runtime found"),
            NkalError::VmImageNotFound(path) => write!(f, "VM image not found: {:?}", path),
            NkalError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            NkalError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for NkalError {}

impl From<std::io::Error> for NkalError {
    fn from(e: std::io::Error) -> Self {
        NkalError::IoError(e)
    }
}

/// Main NKAL interface
pub struct Nkal {
    config: NkalConfig,
    capabilities: PlatformCapabilities,
}

impl Nkal {
    /// Create a new NKAL instance with default configuration
    pub fn new() -> Result<Self, NkalError> {
        let config = NkalConfig::default();
        let capabilities = Self::detect_capabilities()?;

        Ok(Self {
            config,
            capabilities,
        })
    }

    /// Create NKAL with custom configuration
    pub fn with_config(config: NkalConfig) -> Result<Self, NkalError> {
        let capabilities = Self::detect_capabilities()?;

        Ok(Self {
            config,
            capabilities,
        })
    }

    /// Detect platform capabilities
    pub fn detect_capabilities() -> Result<PlatformCapabilities, NkalError> {
        let mut caps = PlatformCapabilities::default();

        // Detect hypervisor
        caps.hypervisor = Self::detect_hypervisor();

        // Detect container runtime
        caps.container_runtime = Self::detect_container_runtime();

        // Detect sandbox availability
        caps.sandbox_available = Self::detect_sandbox();

        // Detect NOA root
        caps.noa_root = Self::detect_noa_root()?;

        Ok(caps)
    }

    /// Detect available hypervisor
    fn detect_hypervisor() -> Hypervisor {
        match std::env::consts::OS {
            "windows" => {
                // Check for Hyper-V
                if Self::check_hyperv_available() {
                    Hypervisor::HyperV
                } else {
                    Hypervisor::None
                }
            }
            "linux" => {
                // Check for KVM
                if Self::check_kvm_available() {
                    Hypervisor::KVM
                } else {
                    Hypervisor::None
                }
            }
            "macos" => {
                // Virtualization.framework available on Apple Silicon and recent Intel Macs
                if Self::check_virtfw_available() {
                    Hypervisor::VirtualizationFramework
                } else {
                    Hypervisor::None
                }
            }
            _ => Hypervisor::None,
        }
    }

    /// Check if Hyper-V is available (Windows)
    fn check_hyperv_available() -> bool {
        #[cfg(target_os = "windows")]
        {
            // Check via PowerShell or registry
            std::process::Command::new("powershell")
                .args(["-Command", "(Get-WindowsOptionalFeature -FeatureName Microsoft-Hyper-V-All -Online).State -eq 'Enabled'"])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "True")
                .unwrap_or(false)
        }
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }

    /// Check if KVM is available (Linux)
    fn check_kvm_available() -> bool {
        #[cfg(target_os = "linux")]
        {
            std::path::Path::new("/dev/kvm").exists()
        }
        #[cfg(not(target_os = "linux"))]
        {
            false
        }
    }

    /// Check if Virtualization.framework is available (macOS)
    fn check_virtfw_available() -> bool {
        #[cfg(target_os = "macos")]
        {
            // Available on macOS 11+ with Apple Silicon or Intel with VT-x
            std::process::Command::new("sysctl")
                .args(["-n", "kern.hv_support"])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "1")
                .unwrap_or(false)
        }
        #[cfg(not(target_os = "macos"))]
        {
            false
        }
    }

    /// Detect available container runtime
    fn detect_container_runtime() -> ContainerRuntime {
        // Check for Podman first (preferred for rootless)
        if std::process::Command::new("podman")
            .arg("--version")
            .output()
            .is_ok()
        {
            return ContainerRuntime::Podman;
        }

        // Check for Docker
        if std::process::Command::new("docker")
            .arg("--version")
            .output()
            .is_ok()
        {
            return ContainerRuntime::Docker;
        }

        ContainerRuntime::None
    }

    /// Detect sandbox availability
    fn detect_sandbox() -> bool {
        match std::env::consts::OS {
            "windows" => {
                // Windows Sandbox available on Windows 10/11 Pro/Enterprise
                std::process::Command::new("powershell")
                    .args(["-Command", "(Get-WindowsOptionalFeature -FeatureName Containers-DisposableClientVM -Online).State -eq 'Enabled'"])
                    .output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "True")
                    .unwrap_or(false)
            }
            "linux" => {
                // Linux: bubblewrap or firejail
                std::process::Command::new("bwrap")
                    .arg("--version")
                    .output()
                    .is_ok()
                    || std::process::Command::new("firejail")
                        .arg("--version")
                        .output()
                        .is_ok()
            }
            _ => false,
        }
    }

    /// Detect NOA root directory
    fn detect_noa_root() -> Result<PathBuf, NkalError> {
        // Check environment variable
        if let Ok(root) = std::env::var("NOA_ROOT") {
            return Ok(PathBuf::from(root));
        }

        // Check for .noa marker file
        let mut current = std::env::current_dir()?;
        loop {
            let marker = current.join(".noa");
            if marker.exists() {
                return Ok(current);
            }
            if !current.pop() {
                break;
            }
        }

        Err(NkalError::ConfigError(
            "NOA_ROOT not found. Set NOA_ROOT environment variable or ensure .noa marker exists."
                .to_string(),
        ))
    }

    /// Get current platform capabilities
    pub fn capabilities(&self) -> &PlatformCapabilities {
        &self.capabilities
    }

    /// Get current configuration
    pub fn config(&self) -> &NkalConfig {
        &self.config
    }

    /// Set kernel mode
    pub fn set_mode(&mut self, mode: KernelMode) -> Result<(), NkalError> {
        // Validate mode is available
        match mode {
            KernelMode::Native => {
                // Always available
            }
            KernelMode::VM => {
                if self.capabilities.hypervisor == Hypervisor::None {
                    return Err(NkalError::ModeUnavailable(mode));
                }
            }
            KernelMode::Container => {
                if self.capabilities.container_runtime == ContainerRuntime::None {
                    return Err(NkalError::ModeUnavailable(mode));
                }
            }
            KernelMode::Sandbox => {
                if !self.capabilities.sandbox_available {
                    return Err(NkalError::ModeUnavailable(mode));
                }
            }
        }

        self.config.preferred_mode = mode;
        Ok(())
    }

    /// Get the best available kernel mode
    pub fn best_available_mode(&self) -> KernelMode {
        // Priority: VM > Container > Sandbox > Native
        if self.capabilities.hypervisor != Hypervisor::None {
            KernelMode::VM
        } else if self.capabilities.container_runtime != ContainerRuntime::None {
            KernelMode::Container
        } else if self.capabilities.sandbox_available {
            KernelMode::Sandbox
        } else {
            KernelMode::Native
        }
    }

    /// Initialize the kernel layer with current configuration
    pub fn initialize(&self) -> Result<(), NkalError> {
        let mode = if self.config.auto_detect {
            self.best_available_mode()
        } else {
            self.config.preferred_mode
        };

        match mode {
            KernelMode::Native => self.init_native(),
            KernelMode::VM => self.init_vm(),
            KernelMode::Container => self.init_container(),
            KernelMode::Sandbox => self.init_sandbox(),
        }
    }

    fn init_native(&self) -> Result<(), NkalError> {
        // Native mode: minimal setup, just validate NOA_ROOT
        if !self.capabilities.noa_root.exists() {
            return Err(NkalError::ConfigError(format!(
                "NOA_ROOT does not exist: {:?}",
                self.capabilities.noa_root
            )));
        }
        Ok(())
    }

    fn init_vm(&self) -> Result<(), NkalError> {
        // VM mode: check hypervisor and image
        if self.capabilities.hypervisor == Hypervisor::None {
            return Err(NkalError::HypervisorNotFound);
        }

        if let Some(ref image_path) = self.config.vm_image_path {
            if !image_path.exists() {
                return Err(NkalError::VmImageNotFound(image_path.clone()));
            }
        }

        Ok(())
    }

    fn init_container(&self) -> Result<(), NkalError> {
        // Container mode: check runtime
        if self.capabilities.container_runtime == ContainerRuntime::None {
            return Err(NkalError::ContainerRuntimeNotFound);
        }
        Ok(())
    }

    fn init_sandbox(&self) -> Result<(), NkalError> {
        // Sandbox mode: check availability
        if !self.capabilities.sandbox_available {
            return Err(NkalError::ModeUnavailable(KernelMode::Sandbox));
        }
        Ok(())
    }
}

impl Default for Nkal {
    fn default() -> Self {
        Self::new().expect("Failed to initialize NKAL with defaults")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_mode_display() {
        assert_eq!(KernelMode::Native.to_string(), "native");
        assert_eq!(KernelMode::VM.to_string(), "vm");
        assert_eq!(KernelMode::Container.to_string(), "container");
        assert_eq!(KernelMode::Sandbox.to_string(), "sandbox");
    }

    #[test]
    fn test_kernel_mode_parse() {
        assert_eq!("native".parse::<KernelMode>().unwrap(), KernelMode::Native);
        assert_eq!("VM".parse::<KernelMode>().unwrap(), KernelMode::VM);
        assert!("invalid".parse::<KernelMode>().is_err());
    }

    #[test]
    fn test_default_config() {
        let config = NkalConfig::default();
        assert_eq!(config.preferred_mode, KernelMode::Native);
        assert!(config.auto_detect);
    }
}

