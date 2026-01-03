//! Capsule integration for sandbox execution
//!
//! Provides integration with the NOA capsule system for stronger isolation.

use std::path::PathBuf;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

use crate::sandbox::{ExecutionResult, SandboxLimits};

/// Capsule executor for running code in isolated containers
pub struct CapsuleExecutor {
    /// Path to capsule definition file
    capsule_def: PathBuf,
    /// Execution timeout
    timeout: Duration,
    /// Whether to use the capsule system (falls back to direct execution if false)
    use_capsule: bool,
}

/// Capsule definition loaded from YAML
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapsuleDefinition {
    pub name: String,
    pub isolation: IsolationConfig,
    pub mounts: Vec<MountConfig>,
}

/// Isolation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsolationConfig {
    /// Memory limit (e.g., "256Mi")
    pub memory_limit: String,
    /// CPU limit (e.g., 1.0)
    pub cpu_limit: f64,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Network access mode
    pub network: NetworkMode,
    /// Filesystem access mode
    pub filesystem: FilesystemMode,
}

/// Network access mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NetworkMode {
    None,
    Host,
    Bridge,
}

/// Filesystem access mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FilesystemMode {
    Readonly,
    Readwrite,
}

/// Mount configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountConfig {
    /// Host path or volume name
    pub source: String,
    /// Container path
    pub target: String,
    /// Read-only mount
    #[serde(default)]
    pub readonly: bool,
}

impl CapsuleExecutor {
    /// Create a new capsule executor
    pub fn new(capsule_def: PathBuf) -> Self {
        Self {
            capsule_def,
            timeout: Duration::from_secs(30),
            use_capsule: true,
        }
    }

    /// Create executor with custom timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Disable capsule mode (use direct execution)
    pub fn without_capsule(mut self) -> Self {
        self.use_capsule = false;
        self
    }

    /// Load capsule definition from file
    pub async fn load_definition(&self) -> anyhow::Result<CapsuleDefinition> {
        let content = tokio::fs::read_to_string(&self.capsule_def).await?;
        let def: CapsuleDefinition = serde_yaml::from_str(&content)?;
        Ok(def)
    }

    /// Execute code in a capsule
    pub async fn execute(
        &self,
        code: &str,
        language: &str,
        working_dir: &std::path::Path,
        limits: &SandboxLimits,
    ) -> anyhow::Result<ExecutionResult> {
        if !self.use_capsule {
            return self.execute_direct(code, language, working_dir).await;
        }

        // Check if container runtime is available
        if !self.is_container_runtime_available().await {
            tracing::warn!("Container runtime not available, falling back to direct execution");
            return self.execute_direct(code, language, working_dir).await;
        }

        self.execute_in_container(code, language, working_dir, limits).await
    }

    /// Check if a container runtime (Docker/Podman) is available
    async fn is_container_runtime_available(&self) -> bool {
        // Try docker first
        if Command::new("docker")
            .arg("--version")
            .output()
            .await
            .is_ok()
        {
            return true;
        }

        // Try podman as fallback
        Command::new("podman")
            .arg("--version")
            .output()
            .await
            .is_ok()
    }

    /// Execute code directly without container isolation
    async fn execute_direct(
        &self,
        code: &str,
        language: &str,
        working_dir: &std::path::Path,
    ) -> anyhow::Result<ExecutionResult> {
        use std::time::Instant;
        use tokio::time::timeout;

        let start = Instant::now();

        // Write code to temp file
        let file_ext = match language {
            "rust" => "rs",
            "python" => "py",
            "javascript" | "js" => "js",
            _ => "txt",
        };
        let code_file = working_dir.join(format!("code.{}", file_ext));
        tokio::fs::write(&code_file, code).await?;

        // Build command
        let (cmd, args): (&str, Vec<String>) = match language {
            "python" => ("python", vec![code_file.to_string_lossy().to_string()]),
            "javascript" | "js" => ("node", vec![code_file.to_string_lossy().to_string()]),
            _ => anyhow::bail!("Unsupported language: {}", language),
        };

        // Execute with timeout
        let output = timeout(self.timeout, async {
            Command::new(cmd)
                .args(&args)
                .current_dir(working_dir)
                .output()
                .await
        }).await;

        let execution_time_ms = start.elapsed().as_millis() as u64;

        match output {
            Ok(Ok(output)) => Ok(ExecutionResult {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                exit_code: output.status.code().unwrap_or(-1),
                execution_time_ms,
            }),
            Ok(Err(e)) => Ok(ExecutionResult {
                stdout: String::new(),
                stderr: format!("Execution error: {}", e),
                exit_code: -1,
                execution_time_ms,
            }),
            Err(_) => Ok(ExecutionResult {
                stdout: String::new(),
                stderr: "Execution timed out".to_string(),
                exit_code: -1,
                execution_time_ms,
            }),
        }
    }

    /// Execute code inside a container
    async fn execute_in_container(
        &self,
        code: &str,
        language: &str,
        working_dir: &std::path::Path,
        limits: &SandboxLimits,
    ) -> anyhow::Result<ExecutionResult> {
        use std::time::Instant;
        use tokio::time::timeout;

        let start = Instant::now();

        // Write code to temp file
        let file_ext = match language {
            "rust" => "rs",
            "python" => "py",
            "javascript" | "js" => "js",
            _ => "txt",
        };
        let code_file = working_dir.join(format!("code.{}", file_ext));
        tokio::fs::write(&code_file, code).await?;

        // Select container image based on language
        let image = match language {
            "python" => "python:3.11-slim",
            "javascript" | "js" => "node:20-slim",
            "rust" => "rust:1.75-slim",
            _ => anyhow::bail!("Unsupported language: {}", language),
        };

        // Build container command
        let working_dir_str = working_dir.to_string_lossy();
        let memory_limit = format!("{}m", limits.max_memory / (1024 * 1024));
        let cpu_limit = format!("{:.2}", 1.0); // 1 CPU

        let run_cmd = match language {
            "python" => format!("python /workspace/code.{}", file_ext),
            "javascript" | "js" => format!("node /workspace/code.{}", file_ext),
            "rust" => format!("rustc /workspace/code.{} -o /tmp/out && /tmp/out", file_ext),
            _ => anyhow::bail!("Unsupported language: {}", language),
        };

        let mut cmd = Command::new("docker");
        cmd.args([
            "run",
            "--rm",
            "--network", if limits.network_allowed { "bridge" } else { "none" },
            "--memory", &memory_limit,
            "--cpus", &cpu_limit,
            "--read-only",
            "--tmpfs", "/tmp:rw,noexec,nosuid,size=64m",
            "-v", &format!("{}:/workspace:ro", working_dir_str),
            "-w", "/workspace",
            image,
            "sh", "-c", &run_cmd,
        ]);

        // Execute with timeout
        let timeout_duration = Duration::from_secs(limits.max_cpu_time);
        let output = timeout(timeout_duration, cmd.output()).await;

        let execution_time_ms = start.elapsed().as_millis() as u64;

        match output {
            Ok(Ok(output)) => Ok(ExecutionResult {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                exit_code: output.status.code().unwrap_or(-1),
                execution_time_ms,
            }),
            Ok(Err(e)) => Ok(ExecutionResult {
                stdout: String::new(),
                stderr: format!("Container execution error: {}", e),
                exit_code: -1,
                execution_time_ms,
            }),
            Err(_) => Ok(ExecutionResult {
                stdout: String::new(),
                stderr: format!("Execution timed out after {}s", limits.max_cpu_time),
                exit_code: -1,
                execution_time_ms,
            }),
        }
    }
}

impl Default for CapsuleExecutor {
    fn default() -> Self {
        let capsule_def = noa_ui_paths::noa_root()
            .map(|p| p.join("containers").join("capsules").join("noa-sandbox.yaml"))
            .unwrap_or_else(|_| std::path::PathBuf::from(".").join("noa-sandbox.yaml"));
        Self::new(capsule_def)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capsule_executor_creation() {
        let executor = CapsuleExecutor::default();
        assert!(executor.use_capsule);
        assert_eq!(executor.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_network_mode_serialization() {
        assert_eq!(
            serde_yaml::to_string(&NetworkMode::None).unwrap().trim(),
            "none"
        );
    }
}
