use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxInstance {
    pub id: String,
    pub platform: String,
    pub requirements: Vec<String>,
    pub state: SandboxState,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub logs: Arc<Mutex<VecDeque<String>>>,
    pub execution_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SandboxState {
    Creating,
    Ready,
    Executing,
    Error,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
    pub exit_code: Option<i32>,
    pub execution_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxStatus {
    pub state: String,
    pub uptime: u64,
    pub memory_usage: u64,
    pub cpu_usage: f32,
    pub active_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInstallResult {
    pub success: bool,
    pub installed: Vec<String>,
    pub failed: Vec<String>,
    pub logs: Vec<String>,
}

impl SandboxInstance {
    pub async fn new(
        id: String,
        platform: String,
        requirements: Vec<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut instance = Self {
            id: id.clone(),
            platform,
            requirements: requirements.clone(),
            state: SandboxState::Creating,
            created_at: chrono::Utc::now(),
            logs: Arc::new(Mutex::new(VecDeque::new())),
            execution_count: 0,
        };
        
        // Log creation
        instance.log(format!("Creating sandbox {} for platform {}", id, instance.platform)).await;
        
        // Install requirements
        instance.install_requirements(&requirements).await?;
        
        instance.state = SandboxState::Ready;
        instance.log("Sandbox ready for execution".to_string()).await;
        
        Ok(instance)
    }
    
    pub async fn execute_code(
        &mut self,
        code: &str,
        language: &str,
        timeout: Option<u64>,
    ) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        self.state = SandboxState::Executing;
        self.execution_count += 1;
        
        let start_time = std::time::Instant::now();
        
        self.log(format!("Executing {} code (timeout: {:?}s)", language, timeout)).await;
        
        let result = match language {
            "rust" => self.execute_rust_code(code, timeout).await,
            "javascript" | "js" => self.execute_javascript_code(code, timeout).await,
            "python" | "py" => self.execute_python_code(code, timeout).await,
            "shell" | "bash" | "sh" => self.execute_shell_code(code, timeout).await,
            _ => Err(format!("Unsupported language: {}", language).into()),
        };
        
        let execution_time = start_time.elapsed().as_millis() as u64;
        
        self.state = SandboxState::Ready;
        
        result.map(|mut r| {
            r.execution_time = execution_time;
            r
        })
    }
    
    async fn execute_rust_code(
        &mut self,
        code: &str,
        timeout: Option<u64>,
    ) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        // Write code to temporary file
        let temp_dir = std::env::temp_dir().join(format!("rust_sandbox_{}", self.id));
        std::fs::create_dir_all(&temp_dir)?;
        
        let main_rs = temp_dir.join("main.rs");
        std::fs::write(&main_rs, code)?;
        
        // Execute with cargo
        let mut cmd = Command::new("cargo");
        cmd.arg("run")
            .arg("--manifest-path")
            .arg(temp_dir.join("Cargo.toml"))
            .current_dir(&temp_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        if let Some(timeout_secs) = timeout {
            cmd.env("RUST_BACKTRACE", "1");
        }
        
        let output = cmd.output()?;
        
        // Cleanup
        std::fs::remove_dir_all(temp_dir)?;
        
        Ok(ExecutionResult {
            success: output.status.success(),
            output: if output.stdout.is_empty() { None } else { Some(String::from_utf8_lossy(&output.stdout).to_string()) },
            error: if output.stderr.is_empty() { None } else { Some(String::from_utf8_lossy(&output.stderr).to_string()) },
            exit_code: output.status.code(),
            execution_time: 0, // Will be set by caller
        })
    }
    
    async fn execute_javascript_code(
        &mut self,
        code: &str,
        timeout: Option<u64>,
    ) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        let temp_dir = std::env::temp_dir().join(format!("js_sandbox_{}", self.id));
        std::fs::create_dir_all(&temp_dir)?;
        
        let js_file = temp_dir.join("script.js");
        std::fs::write(&js_file, code)?;
        
        let mut cmd = Command::new("node");
        cmd.arg(&js_file)
            .current_dir(&temp_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        let output = cmd.output()?;
        
        // Cleanup
        std::fs::remove_dir_all(temp_dir)?;
        
        Ok(ExecutionResult {
            success: output.status.success(),
            output: if output.stdout.is_empty() { None } else { Some(String::from_utf8_lossy(&output.stdout).to_string()) },
            error: if output.stderr.is_empty() { None } else { Some(String::from_utf8_lossy(&output.stderr).to_string()) },
            exit_code: output.status.code(),
            execution_time: 0,
        })
    }
    
    async fn execute_python_code(
        &mut self,
        code: &str,
        timeout: Option<u64>,
    ) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        let temp_dir = std::env::temp_dir().join(format!("py_sandbox_{}", self.id));
        std::fs::create_dir_all(&temp_dir)?;
        
        let py_file = temp_dir.join("script.py");
        std::fs::write(&py_file, code)?;
        
        let mut cmd = Command::new("python3");
        cmd.arg(&py_file)
            .current_dir(&temp_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        let output = cmd.output()?;
        
        // Cleanup
        std::fs::remove_dir_all(temp_dir)?;
        
        Ok(ExecutionResult {
            success: output.status.success(),
            output: if output.stdout.is_empty() { None } else { Some(String::from_utf8_lossy(&output.stdout).to_string()) },
            error: if output.stderr.is_empty() { None } else { Some(String::from_utf8_lossy(&output.stderr).to_string()) },
            exit_code: output.status.code(),
            execution_time: 0,
        })
    }
    
    async fn execute_shell_code(
        &mut self,
        code: &str,
        timeout: Option<u64>,
    ) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        let mut cmd = Command::new("sh");
        cmd.arg("-c")
            .arg(code)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        let output = cmd.output()?;
        
        Ok(ExecutionResult {
            success: output.status.success(),
            output: if output.stdout.is_empty() { None } else { Some(String::from_utf8_lossy(&output.stdout).to_string()) },
            error: if output.stderr.is_empty() { None } else { Some(String::from_utf8_lossy(&output.stderr).to_string()) },
            exit_code: output.status.code(),
            execution_time: 0,
        })
    }
    
    pub async fn install_packages(
        &mut self,
        packages: &[String],
        package_manager: &str,
    ) -> Result<PackageInstallResult, Box<dyn std::error::Error>> {
        self.log(format!("Installing packages: {:?} using {}", packages, package_manager)).await;
        
        let mut installed = Vec::new();
        let mut failed = Vec::new();
        let mut logs = Vec::new();
        
        for package in packages {
            match self.install_single_package(package, package_manager).await {
                Ok(_) => {
                    installed.push(package.clone());
                    logs.push(format!("Successfully installed {}", package));
                }
                Err(e) => {
                    failed.push(package.clone());
                    logs.push(format!("Failed to install {}: {}", package, e));
                }
            }
        }
        
        Ok(PackageInstallResult {
            success: failed.is_empty(),
            installed,
            failed,
            logs,
        })
    }
    
    async fn install_single_package(
        &mut self,
        package: &str,
        package_manager: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (cmd_name, args) = match package_manager {
            "npm" => ("npm", vec!["install", package]),
            "yarn" => ("yarn", vec!["add", package]),
            "pnpm" => ("pnpm", vec!["add", package]),
            _ => return Err(format!("Unsupported package manager: {}", package_manager).into()),
        };
        
        let output = Command::new(cmd_name)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;
        
        if output.status.success() {
            Ok(())
        } else {
            Err(format!("Package installation failed: {}", String::from_utf8_lossy(&output.stderr)).into())
        }
    }
    
    async fn install_requirements(&mut self, requirements: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        // Detect package manager
        let package_manager = self.detect_package_manager();
        
        if !requirements.is_empty() {
            self.install_packages(requirements, &package_manager).await?;
        }
        
        Ok(())
    }
    
    fn detect_package_manager(&self) -> String {
        // Simple detection - in production, this would check for lock files
        "npm".to_string()
    }
    
    pub async fn get_logs(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let logs = self.logs.lock().await;
        Ok(logs.iter().cloned().collect())
    }
    
    pub async fn get_status(&self) -> Result<SandboxStatus, Box<dyn std::error::Error>> {
        let uptime = (chrono::Utc::now() - self.created_at).num_seconds() as u64;
        
        // Get system resource usage (simplified)
        let memory_usage = self.get_memory_usage();
        let cpu_usage = self.get_cpu_usage();
        
        Ok(SandboxStatus {
            state: format!("{:?}", self.state).to_lowercase(),
            uptime,
            memory_usage,
            cpu_usage,
            active_connections: 1, // Simplified
        })
    }
    
    fn get_memory_usage(&self) -> u64 {
        // Simplified - in production, use system APIs
        1024 * 1024 * 100 // 100MB placeholder
    }
    
    fn get_cpu_usage(&self) -> f32 {
        // Simplified - in production, use system APIs
        15.5 // 15.5% placeholder
    }
    
    pub async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.log("Cleaning up sandbox".to_string()).await;
        // Cleanup temporary files, processes, etc.
        Ok(())
    }
    
    async fn log(&self, message: String) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}] {}", timestamp, message);
        
        let mut logs = self.logs.lock().await;
        logs.push_back(log_entry);
        
        // Keep only last 1000 logs
        if logs.len() > 1000 {
            logs.pop_front();
        }
        
        println!("Sandbox {}: {}", self.id, message);
    }
}