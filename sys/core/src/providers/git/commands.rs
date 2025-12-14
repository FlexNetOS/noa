use crate::error::Result;

pub fn run_git_command(cmd: &str) -> Result<String> {
    Ok(format!("git executed: {}", cmd))
}
