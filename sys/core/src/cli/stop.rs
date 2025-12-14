//! NOA Stop Command
//!
//! Stops NOA services gracefully.

use std::path::PathBuf;
use std::fs;

use clap::Args;
use tracing::{info, warn};

use crate::error::Result;

/// Arguments for the stop command
#[derive(Args, Debug)]
pub struct StopArgs {
    /// Force stop without graceful shutdown
    #[arg(short, long)]
    pub force: bool,

    /// Timeout for graceful shutdown in seconds
    #[arg(short, long, default_value = "30")]
    pub timeout: u64,

    /// Stop specific service only
    #[arg(short, long)]
    pub service: Option<String>,
}

/// Execute the stop command
pub async fn execute(args: StopArgs) -> Result<()> {
    info!(
        force = args.force,
        timeout = args.timeout,
        service = ?args.service,
        "Stopping NOA services"
    );

    // Find PID file
    let noa_root = std::env::var("NOA_ROOT").unwrap_or_else(|_| ".".to_string());
    let pid_file = PathBuf::from(&noa_root).join("data/state/noa.pid");

    if !pid_file.exists() {
        println!("NOA is not running (no PID file found)");
        return Ok(());
    }

    // Read PID
    let pid_str = fs::read_to_string(&pid_file)?;
    let pid: u32 = pid_str.trim().parse().map_err(|_| {
        crate::error::NoaError::Internal {
            message: "Invalid PID file content".to_string(),
            source: None,
        }
    })?;

    println!("Stopping NOA (PID: {})...", pid);

    if args.force {
        // Force kill
        stop_process_force(pid)?;
    } else {
        // Graceful shutdown
        stop_process_graceful(pid, args.timeout).await?;
    }

    // Remove PID file
    if pid_file.exists() {
        fs::remove_file(&pid_file)?;
    }

    println!("NOA stopped successfully");
    Ok(())
}

/// Gracefully stop a process
async fn stop_process_graceful(pid: u32, timeout_secs: u64) -> Result<()> {
    info!(pid = pid, timeout = timeout_secs, "Sending graceful shutdown signal");

    // Send SIGTERM on Unix, use other mechanisms on Windows
    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;

        let pid = Pid::from_raw(pid as i32);
        kill(pid, Signal::SIGTERM).map_err(|e| {
            crate::error::NoaError::Internal {
                message: format!("Failed to send SIGTERM: {}", e),
                source: None,
            }
        })?;
    }

    #[cfg(windows)]
    {
        // On Windows, we'd use taskkill or similar
        let output = std::process::Command::new("taskkill")
            .args(["/PID", &pid.to_string()])
            .output()
            .map_err(|e| crate::error::NoaError::Internal {
                message: format!("Failed to stop process: {}", e),
                source: Some(Box::new(e)),
            })?;

        if !output.status.success() {
            warn!("taskkill returned non-zero exit code");
        }
    }

    // Wait for process to terminate
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(timeout_secs);

    while start.elapsed() < timeout {
        if !is_process_running(pid) {
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    warn!(pid = pid, "Process did not stop within timeout, forcing");
    stop_process_force(pid)
}

/// Force stop a process
fn stop_process_force(pid: u32) -> Result<()> {
    info!(pid = pid, "Force stopping process");

    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;

        let pid = Pid::from_raw(pid as i32);
        kill(pid, Signal::SIGKILL).map_err(|e| {
            crate::error::NoaError::Internal {
                message: format!("Failed to send SIGKILL: {}", e),
                source: None,
            }
        })?;
    }

    #[cfg(windows)]
    {
        let output = std::process::Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string()])
            .output()
            .map_err(|e| crate::error::NoaError::Internal {
                message: format!("Failed to force stop process: {}", e),
                source: Some(Box::new(e)),
            })?;

        if !output.status.success() {
            return Err(crate::error::NoaError::Internal {
                message: "taskkill failed".to_string(),
                source: None,
            });
        }
    }

    Ok(())
}

/// Check if a process is running
fn is_process_running(pid: u32) -> bool {
    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;

        let pid = Pid::from_raw(pid as i32);
        kill(pid, None).is_ok()
    }

    #[cfg(windows)]
    {
        use std::process::Command;

        let output = Command::new("tasklist")
            .args(["/FI", &format!("PID eq {}", pid), "/NH"])
            .output();

        match output {
            Ok(out) => {
                let output_str = String::from_utf8_lossy(&out.stdout);
                output_str.contains(&pid.to_string())
            }
            Err(_) => false,
        }
    }
}

