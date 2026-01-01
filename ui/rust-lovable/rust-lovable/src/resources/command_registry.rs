use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: String,
    pub name: String,
    pub description: String,
    pub command_type: String,
    pub parameters: HashMap<String, String>,
}

pub struct CommandRegistry {
    commands: HashMap<String, Command>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn register_command(&mut self, command: Command) -> Result<()> {
        self.commands.insert(command.id.clone(), command);
        Ok(())
    }

    pub fn get_command(&self, id: &str) -> Option<Command> {
        self.commands.get(id).cloned()
    }

    pub fn list_commands(&self) -> Vec<Command> {
        self.commands.values().cloned().collect()
    }
}