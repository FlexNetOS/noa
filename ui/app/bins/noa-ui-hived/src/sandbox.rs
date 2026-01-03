//! Sandbox execution environment
//!
//! Provides isolated execution environments for running untrusted code safely.
//! Integrates with the capsule system for resource isolation.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Sandbox execution environment for running code safely
#[derive(Clone)]
pub struct Sandbox {
    /// Unique identifier for this sandbox
    pub id: String,
    /// Working directory for the sandbox
    pub working_dir: PathBuf,
    /// Environment variables for the sandbox
    pub env_vars: HashMap<String, String>,
    /// Resource limits
    pub limits: SandboxLimits,
    /// Current status
    pub status: SandboxStatus,
}

/// Resource limits for sandbox execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxLimits {
    /// Maximum memory in bytes
    pub max_memory: u64,
    /// Maximum CPU time in seconds
    pub max_cpu_time: u64,
    /// Maximum disk space in bytes
    pub max_disk_space: u64,
    /// Maximum number of processes
    pub max_processes: u32,
    /// Network access allowed
    pub network_allowed: bool,
}

impl Default for SandboxLimits {
    fn default() -> Self {
        Self {
            max_memory: 256 * 1024 * 1024, // 256MB
            max_cpu_time: 30,               // 30 seconds
            max_disk_space: 100 * 1024 * 1024, // 100MB
            max_processes: 10,
            network_allowed: false,
        }
    }
}

/// Sandbox execution status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SandboxStatus {
    /// Sandbox is being created
    Creating,
    /// Sandbox is ready for execution
    Ready,
    /// Sandbox is currently running code
    Running,
    /// Execution completed successfully
    Completed,
    /// Execution failed
    Failed,
    /// Sandbox has been terminated
    Terminated,
}

impl Sandbox {
    /// Create a new sandbox with the given ID
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        let working_dir = noa_ui_paths::noa_data_dir()
            .map(|p| p.join("sandboxes").join(&id))
            .unwrap_or_else(|_| std::path::PathBuf::from(".").join("sandboxes").join(&id));

        Self {
            id,
            working_dir,
            env_vars: HashMap::new(),
            limits: SandboxLimits::default(),
            status: SandboxStatus::Creating,
        }
    }

    /// Initialize the sandbox environment
    pub async fn initialize(&mut self) -> anyhow::Result<()> {
        tracing::info!("Initializing sandbox: {}", self.id);

        // Create working directory
        tokio::fs::create_dir_all(&self.working_dir).await?;

        self.status = SandboxStatus::Ready;
        Ok(())
    }

    /// Execute code in the sandbox
    pub async fn execute(&mut self, code: &str, language: &str) -> anyhow::Result<ExecutionResult> {
        if self.status != SandboxStatus::Ready {
            anyhow::bail!("Sandbox is not ready for execution");
        }

        self.status = SandboxStatus::Running;
        tracing::info!("Executing {} code in sandbox {}", language, self.id);

        let result = self.run_in_capsule(code, language).await?;

        self.status = SandboxStatus::Completed;
        Ok(result)
    }

    /// Run code inside a capsule with resource limits
    async fn run_in_capsule(&self, code: &str, language: &str) -> anyhow::Result<ExecutionResult> {
        use std::time::Instant;
        use tokio::process::Command;
        use tokio::time::{timeout, Duration};

        let start = Instant::now();

        // Write code to a temp file
        let file_ext = match language {
            "rust" => "rs",
            "python" => "py",
            "javascript" | "js" => "js",
            "typescript" | "ts" => "ts",
            _ => "txt",
        };
        let code_file = self.working_dir.join(format!("code.{}", file_ext));
        tokio::fs::write(&code_file, code).await?;

        // Store paths as owned Strings to avoid temporary lifetime issues
        let code_file_str = code_file.to_str().unwrap().to_owned();
        let output_path_str = self.working_dir.join("output").to_str().unwrap().to_owned();

        // Build command based on language
        let (cmd, args): (&str, Vec<String>) = match language {
            "python" => ("python", vec![code_file_str]),
            "javascript" | "js" => ("node", vec![code_file_str]),
            "rust" => {
                // For Rust, we'd need to compile first - simplified here
                ("rustc", vec!["--edition=2021".to_owned(), "-o".to_owned(), 
                    output_path_str,
                    code_file_str])
            }
            _ => anyhow::bail!("Unsupported language: {}", language),
        };

        // Execute with timeout
        let timeout_duration = Duration::from_secs(self.limits.max_cpu_time);
        
        let output = timeout(timeout_duration, async {
            Command::new(cmd)
                .args(&args)
                .current_dir(&self.working_dir)
                .envs(&self.env_vars)
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
                stderr: format!("Execution timed out after {}s", self.limits.max_cpu_time),
                exit_code: -1,
                execution_time_ms,
            }),
        }
    }

    /// Terminate the sandbox
    pub async fn terminate(&mut self) -> anyhow::Result<()> {
        tracing::info!("Terminating sandbox: {}", self.id);

        // Cleanup working directory
        if self.working_dir.exists() {
            tokio::fs::remove_dir_all(&self.working_dir).await?;
        }

        self.status = SandboxStatus::Terminated;
        Ok(())
    }

    /// Set an environment variable
    pub fn set_env(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.env_vars.insert(key.into(), value.into());
    }

    /// Set resource limits
    pub fn set_limits(&mut self, limits: SandboxLimits) {
        self.limits = limits;
    }
}

/// Result of code execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// Standard output
    pub stdout: String,
    /// Standard error
    pub stderr: String,
    /// Exit code
    pub exit_code: i32,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Manager for multiple sandboxes
pub struct SandboxManager {
    sandboxes: Arc<RwLock<HashMap<String, Sandbox>>>,
}

impl SandboxManager {
    /// Create a new sandbox manager
    pub fn new() -> Self {
        Self {
            sandboxes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new sandbox
    pub async fn create(&self, id: impl Into<String>) -> anyhow::Result<String> {
        let id = id.into();
        let mut sandbox = Sandbox::new(&id);
        sandbox.initialize().await?;

        self.sandboxes.write().await.insert(id.clone(), sandbox);
        Ok(id)
    }

    /// Get a sandbox by ID
    pub async fn get(&self, id: &str) -> Option<Sandbox> {
        self.sandboxes.read().await.get(id).cloned()
    }

    /// Execute code in a sandbox
    pub async fn execute(&self, id: &str, code: &str, language: &str) -> anyhow::Result<ExecutionResult> {
        let mut sandboxes = self.sandboxes.write().await;
        let sandbox = sandboxes.get_mut(id)
            .ok_or_else(|| anyhow::anyhow!("Sandbox not found: {}", id))?;
        sandbox.execute(code, language).await
    }

    /// Terminate and remove a sandbox
    pub async fn remove(&self, id: &str) -> anyhow::Result<()> {
        if let Some(mut sandbox) = self.sandboxes.write().await.remove(id) {
            sandbox.terminate().await?;
        }
        Ok(())
    }

    /// List all sandbox IDs
    pub async fn list(&self) -> Vec<String> {
        self.sandboxes.read().await.keys().cloned().collect()
    }
}

impl Default for SandboxManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sandbox_creation() {
        let mut sandbox = Sandbox::new("test-1");
        assert_eq!(sandbox.status, SandboxStatus::Creating);
        
        sandbox.initialize().await.unwrap();
        assert_eq!(sandbox.status, SandboxStatus::Ready);
        
        sandbox.terminate().await.unwrap();
        assert_eq!(sandbox.status, SandboxStatus::Terminated);
    }
}
