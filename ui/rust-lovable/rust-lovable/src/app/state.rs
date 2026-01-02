//! Application state management
//!
//! Provides global state management for the application.

use std::sync::Arc;
use std::sync::RwLock;

use super::App;

/// Global application state
pub struct AppState {
    /// The main application instance
    pub app: Arc<RwLock<App>>,
    /// Whether the app is initialized
    pub initialized: bool,
}

impl AppState {
    /// Create a new AppState
    pub fn new(app: App) -> Self {
        Self {
            app: Arc::new(RwLock::new(app)),
            initialized: false,
        }
    }

    /// Mark the app as initialized
    pub fn set_initialized(&mut self, value: bool) {
        self.initialized = value;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(App::default())
    }
}
