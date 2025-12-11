//! Cross-platform process helpers for kernel independence.
//!
//! Provides minimal wrappers for spawning commands while capturing output.

use crate::error::{NoaError, Result};
use std::process::Command;

/// Result of a process invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessResult {
    /// Exit code of the process (or -1 when unavailable).
    pub status: i32,
    /// Captured stdout.
    pub stdout: String,
    /// Captured stderr.
    pub stderr: String,
}

/// Process utilities.
#[derive(Debug, Default)]
pub struct ProcessOps;

impl ProcessOps {
    /// Run a command and capture output.
    pub fn run(&self, program: &str, args: &[&str]) -> Result<ProcessResult> {
        let output = Command::new(program).args(args).output().map_err(NoaError::from)?;

        Ok(ProcessResult {
            status: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    /// Best-effort availability probe by invoking `<program> --version`.
    pub fn is_available(&self, program: &str) -> bool {
        Command::new(program)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_echo_command() {
        let ops = ProcessOps::default();
        let result = if cfg!(windows) {
            ops.run("cmd", &["/C", "echo", "hello"])
        } else {
            ops.run("sh", &["-c", "echo hello"])
        }
        .unwrap();

        assert_eq!(result.status, 0);
        assert!(result.stdout.to_lowercase().contains("hello"));
    }
}
