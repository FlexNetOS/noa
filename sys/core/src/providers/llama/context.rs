//! Context forwarding for llama.cpp

use crate::error::Result;

pub fn forward_context(_from: &str, _to: &str, _content: &str) -> Result<()> {
    // Stub: wire context into llama runtime
    Ok(())
}
