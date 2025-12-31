use crate::agents::base::BaseAgent;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::time::Duration;

/// Terminal command request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalCommand {
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub timeout_secs: Option<u64>,
}

/// Terminal command result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub error: Option<String>,
}

pub struct TerminalAgent {
    allowed_commands: Option<Vec<String>>,
    default_timeout: Duration,
}

impl TerminalAgent {
    pub fn new() -> Self {
        Self {
            allowed_commands: None,
            default_timeout: Duration::from_secs(60),
        }
    }

    /// Create agent with whitelist of allowed commands
    pub fn with_whitelist(commands: Vec<String>) -> Self {
        Self {
            allowed_commands: Some(commands),
            default_timeout: Duration::from_secs(60),
        }
    }

    /// Create agent with custom timeout
    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
            allowed_commands: None,
            default_timeout: timeout,
        }
    }

    /// Check if a command is allowed
    fn is_command_allowed(&self, command: &str) -> bool {
        if let Some(whitelist) = &self.allowed_commands {
            whitelist.iter().any(|c| c == command)
        } else {
            // If no whitelist, allow common safe commands
            let safe_commands = [
                "ls", "dir", "pwd", "echo", "cat", "type",
                "find", "grep", "which", "where",
                "git", "cargo", "npm", "node", "python",
                "rustc", "rustup", "dotnet", "cmd", "powershell",
            ];
            safe_commands.contains(&command)
        }
    }

    /// Execute a terminal command
    pub fn execute_command(&self, cmd: TerminalCommand) -> Result<TerminalResult> {
        // Security check
        if !self.is_command_allowed(&cmd.command) {
            return Ok(TerminalResult {
                success: false,
                exit_code: None,
                stdout: String::new(),
                stderr: String::new(),
                error: Some(format!("Command '{}' not in whitelist", cmd.command)),
            });
        }

        let mut command = Command::new(&cmd.command);
        command.args(&cmd.args);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        // Set working directory
        if let Some(dir) = &cmd.working_dir {
            command.current_dir(dir);
        }

        // Set environment variables
        if let Some(env) = &cmd.env {
            for (key, value) in env {
                command.env(key, value);
            }
        }

        // Execute with timeout
        let timeout = cmd
            .timeout_secs
            .map(Duration::from_secs)
            .unwrap_or(self.default_timeout);

        match command.spawn() {
            Ok(mut child) => {
                // Wait for completion with timeout
                use std::thread;
                use std::time::Instant;

                let start = Instant::now();
                let check_interval = Duration::from_millis(100);

                loop {
                    match child.try_wait() {
                        Ok(Some(status)) => {
                            // Process completed
                            let output = child.wait_with_output()?;
                            return Ok(TerminalResult {
                                success: status.success(),
                                exit_code: status.code(),
                                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                                error: None,
                            });
                        }
                        Ok(None) => {
                            // Still running
                            if start.elapsed() >= timeout {
                                // Timeout reached, kill process
                                let _ = child.kill();
                                return Ok(TerminalResult {
                                    success: false,
                                    exit_code: None,
                                    stdout: String::new(),
                                    stderr: String::new(),
                                    error: Some(format!(
                                        "Command timed out after {} seconds",
                                        timeout.as_secs()
                                    )),
                                });
                            }
                            thread::sleep(check_interval);
                        }
                        Err(e) => {
                            return Ok(TerminalResult {
                                success: false,
                                exit_code: None,
                                stdout: String::new(),
                                stderr: String::new(),
                                error: Some(format!("Failed to wait for command: {}", e)),
                            });
                        }
                    }
                }
            }
            Err(e) => Ok(TerminalResult {
                success: false,
                exit_code: None,
                stdout: String::new(),
                stderr: String::new(),
                error: Some(format!("Failed to spawn command: {}", e)),
            }),
        }
    }

    /// Execute a simple command string
    pub fn execute_simple(&self, command_str: &str) -> Result<TerminalResult> {
        let parts: Vec<&str> = command_str.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(TerminalResult {
                success: false,
                exit_code: None,
                stdout: String::new(),
                stderr: String::new(),
                error: Some("Empty command".into()),
            });
        }

        let cmd = TerminalCommand {
            command: parts[0].to_string(),
            args: parts[1..].iter().map(|s| s.to_string()).collect(),
            working_dir: None,
            env: None,
            timeout_secs: None,
        };

        self.execute_command(cmd)
    }
}

impl Default for TerminalAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseAgent for TerminalAgent {
    fn name(&self) -> &str {
        "terminal"
    }

    fn description(&self) -> &str {
        "Executes shell commands in a controlled, secure environment with timeouts and whitelisting"
    }

    fn capabilities(&self) -> Vec<String> {
        vec![
            "shell".into(),
            "diagnostics".into(),
            "build".into(),
            "test".into(),
            "git".into(),
        ]
    }

    fn execute(&self, task: &str) -> Result<String> {
        // Try to parse as JSON command first
        match serde_json::from_str::<TerminalCommand>(task) {
            Ok(cmd) => {
                let result = self.execute_command(cmd)?;
                Ok(serde_json::to_string(&result)?)
            }
            Err(_) => {
                // Fallback: treat as simple command string
                let result = self.execute_simple(task)?;
                Ok(serde_json::to_string(&result)?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_agent_echo() {
        let agent = TerminalAgent::new();
        
        #[cfg(target_os = "windows")]
        let cmd = TerminalCommand {
            command: "cmd".into(),
            args: vec!["/C".into(), "echo".into(), "Hello".into(), "World".into()],
            working_dir: None,
            env: None,
            timeout_secs: Some(5),
        };

        #[cfg(not(target_os = "windows"))]
        let cmd = TerminalCommand {
            command: "echo".into(),
            args: vec!["Hello".into(), "World".into()],
            working_dir: None,
            env: None,
            timeout_secs: Some(5),
        };

        let result = agent.execute_command(cmd).unwrap();
        assert!(result.success);
        assert!(result.stdout.contains("Hello"));
    }

    #[test]
    fn test_terminal_agent_whitelist() {
        #[cfg(target_os = "windows")]
        let allowed = vec!["cmd".into()];
        #[cfg(not(target_os = "windows"))]
        let allowed = vec!["echo".into()];

        let agent = TerminalAgent::with_whitelist(allowed);
        
        // Allowed command
        #[cfg(target_os = "windows")]
        let result = agent.execute_simple("cmd /C echo test").unwrap();
        #[cfg(not(target_os = "windows"))]
        let result = agent.execute_simple("echo test").unwrap();
        
        assert!(result.success);
        
        // Disallowed command
        let result = agent.execute_simple("git status").unwrap();
        assert!(!result.success);
        assert!(result.error.is_some());
    }
}

