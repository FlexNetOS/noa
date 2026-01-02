//! Application module
//!
//! Contains the core App struct and application state management.

pub mod state;

use std::sync::Arc;
use std::sync::RwLock;

use crate::core::conversational_ai::{AIProvider, ConversationalAI};
use crate::core::project_manager::ProjectManager;
use crate::core::ui_generator::UIGenerator;

/// Main application struct that holds all core components
pub struct App {
    /// Conversational AI engine
    pub ai: Arc<RwLock<ConversationalAI>>,
    /// Project manager
    pub project_manager: Arc<RwLock<ProjectManager>>,
    /// UI generator
    pub ui_generator: Arc<RwLock<UIGenerator>>,
}

impl App {
    /// Create a new App instance with default local AI provider
    pub fn new() -> Self {
        let default_provider = AIProvider::Local {
            endpoint: "http://localhost:11434".to_string(),
        };
        Self {
            ai: Arc::new(RwLock::new(ConversationalAI::new(default_provider))),
            project_manager: Arc::new(RwLock::new(ProjectManager::new(std::path::PathBuf::from(
                "projects",
            )))),
            ui_generator: Arc::new(RwLock::new(UIGenerator::new())),
        }
    }

    /// Initialize the application
    pub async fn initialize(&self) -> anyhow::Result<()> {
        tracing::info!("Initializing Rust Lovable application...");
        tracing::info!("Application initialized successfully");
        Ok(())
    }

    /// Shutdown the application gracefully
    pub async fn shutdown(&self) -> anyhow::Result<()> {
        tracing::info!("Shutting down application...");
        tracing::info!("Application shutdown complete");
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
