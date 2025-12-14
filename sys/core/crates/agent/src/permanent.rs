//! Permanent agents that persist across sessions

use noa_common::Metadata;
use serde::{Deserialize, Serialize};

/// FileIO Agent - handles file system operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileIOAgent {
    pub metadata: Metadata,
    pub noa_root: String,
}

impl FileIOAgent {
    /// Create a new FileIO agent
    pub fn new(noa_root: &str) -> Self {
        Self {
            metadata: Metadata::new(),
            noa_root: noa_root.to_string(),
        }
    }

    /// Read a file within noa_root
    pub async fn read_file(&self, path: &str) -> noa_common::Result<Vec<u8>> {
        // Validate path is within noa_root
        if !path.starts_with(&self.noa_root) {
            return Err(noa_common::NoaError::Unauthorized(
                "Path outside noa_root".into(),
            ));
        }
        tokio::fs::read(path)
            .await
            .map_err(|e| noa_common::NoaError::Io { source: e })
    }

    /// Write a file within noa_root
    pub async fn write_file(&self, path: &str, contents: &[u8]) -> noa_common::Result<()> {
        if !path.starts_with(&self.noa_root) {
            return Err(noa_common::NoaError::Unauthorized(
                "Path outside noa_root".into(),
            ));
        }
        tokio::fs::write(path, contents)
            .await
            .map_err(|e| noa_common::NoaError::Io { source: e })
    }
}

/// Terminal Agent - executes shell commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalAgent {
    pub metadata: Metadata,
    pub timeout_secs: u64,
}

impl Default for TerminalAgent {
    fn default() -> Self {
        Self {
            metadata: Metadata::new(),
            timeout_secs: 30, // FR-008: 30s default timeout
        }
    }
}

impl TerminalAgent {
    /// Execute a shell command with timeout and capture stdout/stderr
    /// FR-008: Execute shell commands with timeout (default 30s), capture stdout/stderr
    pub async fn execute(
        &self,
        command: &str,
    ) -> noa_common::Result<TerminalExecutionResult> {
        use tokio::process::Command;
        use tokio::time::{timeout, Duration};

        let timeout_duration = Duration::from_secs(self.timeout_secs);

        // Determine shell based on platform
        let (shell, shell_arg) = if cfg!(windows) {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        let mut child = Command::new(shell)
            .arg(shell_arg)
            .arg(command)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| noa_common::NoaError::Io {
                source: std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to spawn command: {}", e),
                ),
            })?;

        // Take stdout and stderr before waiting
        let stdout_handle = child.stdout.take();
        let stderr_handle = child.stderr.take();
        let child_id = child.id();

        // Capture stdout and stderr concurrently
        let stdout_task = tokio::spawn(async move {
            let mut stdout = Vec::new();
            if let Some(mut handle) = stdout_handle {
                use tokio::io::AsyncReadExt;
                let _ = handle.read_to_end(&mut stdout).await;
            }
            stdout
        });

        let stderr_task = tokio::spawn(async move {
            let mut stderr = Vec::new();
            if let Some(mut handle) = stderr_handle {
                use tokio::io::AsyncReadExt;
                let _ = handle.read_to_end(&mut stderr).await;
            }
            stderr
        });

        // Wait for process with timeout
        let wait_result = timeout(timeout_duration, child.wait()).await;

        match wait_result {
            Ok(Ok(status)) => {
                let stdout = stdout_task.await.unwrap_or_default();
                let stderr = stderr_task.await.unwrap_or_default();

                Ok(TerminalExecutionResult {
                    exit_code: status.code().unwrap_or(-1),
                    stdout: String::from_utf8_lossy(&stdout).to_string(),
                    stderr: String::from_utf8_lossy(&stderr).to_string(),
                    timed_out: false,
                })
            }
            Ok(Err(e)) => {
                // Process error - still collect output (discard but ensure tasks complete)
                let _ = stdout_task.await;
                let _ = stderr_task.await;
                Err(noa_common::NoaError::Io {
                    source: std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Command execution failed: {}", e),
                    ),
                })
            }
            Err(_) => {
                // Timeout occurred - kill the process
                if let Some(id) = child_id {
                    #[cfg(unix)]
                    {
                        use std::process::Command as SyncCommand;
                        let _ = SyncCommand::new("kill").arg("-9").arg(id.to_string()).output();
                    }
                    #[cfg(windows)]
                    {
                        use std::process::Command as SyncCommand;
                        let _ = SyncCommand::new("taskkill")
                            .args(["/F", "/PID", &id.to_string()])
                            .output();
                    }
                }
                let stdout = stdout_task.await.unwrap_or_default();
                let stderr = stderr_task.await.unwrap_or_default();

                Ok(TerminalExecutionResult {
                    exit_code: -1,
                    stdout: String::from_utf8_lossy(&stdout).to_string(),
                    stderr: format!(
                        "{}\nCommand timed out after {} seconds",
                        String::from_utf8_lossy(&stderr),
                        self.timeout_secs
                    ),
                    timed_out: true,
                })
            }
        }
    }
}

/// Result of terminal command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalExecutionResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub timed_out: bool,
}

/// RAG Agent - retrieves relevant context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGAgent {
    pub metadata: Metadata,
    pub relevance_threshold: f32,
}

impl Default for RAGAgent {
    fn default() -> Self {
        Self {
            metadata: Metadata::new(),
            relevance_threshold: 0.8, // FR-008: >80% relevance
        }
    }
}

/// Microservice Management Agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroserviceManagementAgent {
    pub metadata: Metadata,
    pub deploy_timeout_secs: u64,
    pub health_check_interval_secs: u64,
}

impl Default for MicroserviceManagementAgent {
    fn default() -> Self {
        Self {
            metadata: Metadata::new(),
            deploy_timeout_secs: 10,    // FR-008: deploys within 10s
            health_check_interval_secs: 1, // FR-008: health check <1s
        }
    }
}

impl MicroserviceManagementAgent {
    /// Deploy a service
    /// FR-008: Deploy/stop services within 10s
    pub async fn deploy(
        &self,
        service_name: &str,
        _service_config: &ServiceConfig,
    ) -> noa_common::Result<DeploymentResult> {
        use tokio::time::{timeout, Duration, Instant};

        let start = Instant::now();
        let timeout_duration = Duration::from_secs(self.deploy_timeout_secs);

        // TODO: Implement actual deployment logic (Docker, Kubernetes, etc.)
        // For now, simulate deployment
        let deploy_future = async {
            // Simulate deployment steps
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok(DeploymentResult {
                service_name: service_name.to_string(),
                deployed: true,
                deployment_time_ms: start.elapsed().as_millis() as u64,
            })
        };

        match timeout(timeout_duration, deploy_future).await {
            Ok(result) => result,
            Err(_) => Err(noa_common::NoaError::Agent(format!(
                "Service {} deployment timed out after {} seconds",
                service_name, self.deploy_timeout_secs
            ))),
        }
    }

    /// Perform health check on a service
    /// FR-008: Health check within 1s
    pub async fn health_check(
        &self,
        service_name: &str,
    ) -> noa_common::Result<HealthCheckResult> {
        use tokio::time::{timeout, Duration, Instant};

        let start = Instant::now();
        let timeout_duration = Duration::from_secs(self.health_check_interval_secs);

        // TODO: Implement actual health check logic (HTTP endpoint, process check, etc.)
        let health_future = async {
            // Simulate health check
            tokio::time::sleep(Duration::from_millis(50)).await;
            Ok(HealthCheckResult {
                service_name: service_name.to_string(),
                healthy: true,
                response_time_ms: start.elapsed().as_millis() as u64,
            })
        };

        match timeout(timeout_duration, health_future).await {
            Ok(result) => result,
            Err(_) => Ok(HealthCheckResult {
                service_name: service_name.to_string(),
                healthy: false,
                response_time_ms: self.health_check_interval_secs * 1000,
            }),
        }
    }
}

/// Service configuration for deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub image: String,
    pub ports: Vec<u16>,
    pub env: std::collections::HashMap<String, String>,
}

/// Result of service deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentResult {
    pub service_name: String,
    pub deployed: bool,
    pub deployment_time_ms: u64,
}

/// Result of health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub service_name: String,
    pub healthy: bool,
    pub response_time_ms: u64,
}

