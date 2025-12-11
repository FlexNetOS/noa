use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

/// Minimal UI status to reflect companion connectivity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UiStatus {
    Offline,
    Connecting,
    Online,
    Error(String),
}

/// Lightweight UI state container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionUiState {
    pub status: UiStatus,
    pub connected_to: Option<String>,
    pub notifications: VecDeque<String>,
}

impl CompanionUiState {
    pub fn new() -> Self {
        Self {
            status: UiStatus::Offline,
            connected_to: None,
            notifications: VecDeque::with_capacity(10),
        }
    }

    pub fn set_status(&mut self, status: UiStatus) {
        self.status = status;
    }

    pub fn set_connected_to(&mut self, relay: impl Into<String>) {
        self.connected_to = Some(relay.into());
    }

    pub fn push_notification(&mut self, note: impl Into<String>) {
        if self.notifications.len() >= 10 {
            self.notifications.pop_front();
        }
        self.notifications.push_back(note.into());
    }

    pub fn connection_banner(&self) -> String {
        match (&self.status, &self.connected_to) {
            (UiStatus::Online, Some(relay)) => format!("Connected to {}", relay),
            (UiStatus::Connecting, Some(relay)) => format!("Connecting to {}", relay),
            (UiStatus::Error(msg), _) => format!("Error: {}", msg),
            _ => "Offline".to_string(),
        }
    }
}

impl Default for CompanionUiState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_notifications() {
        let mut ui = CompanionUiState::new();
        ui.push_notification("hello");
        assert_eq!(ui.notifications.len(), 1);
    }

    #[test]
    fn renders_banner() {
        let mut ui = CompanionUiState::new();
        ui.set_status(UiStatus::Connecting);
        ui.set_connected_to("relay.test");
        assert!(ui.connection_banner().contains("relay.test"));
    }
}
