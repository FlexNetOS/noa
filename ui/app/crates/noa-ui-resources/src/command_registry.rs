//! Command registry for managing commands

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A command definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: String,
    pub name: String,
    pub description: String,
    pub command_type: String,
    pub parameters: HashMap<String, String>,
}

/// Registry for managing commands
pub struct CommandRegistry {
    commands: HashMap<String, Command>,
}

impl CommandRegistry {
    /// Create a new CommandRegistry
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    /// Initialize the registry
    pub async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// Cleanup resources
    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Register a command
    pub fn register_command(&mut self, command: Command) -> Result<()> {
        self.commands.insert(command.id.clone(), command);
        Ok(())
    }

    /// Get a command by ID
    pub fn get_command(&self, id: &str) -> Option<Command> {
        self.commands.get(id).cloned()
    }

    /// List all commands
    pub fn list_commands(&self) -> Vec<Command> {
        self.commands.values().cloned().collect()
    }

    /// Remove a command
    pub fn remove_command(&mut self, id: &str) -> Option<Command> {
        self.commands.remove(id)
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}
