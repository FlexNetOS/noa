//! Log store for log entries

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// A log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub metadata: serde_json::Value,
}

/// Store for log entries
pub struct LogStore {
    logs: VecDeque<LogEntry>,
    max_size: usize,
}

impl LogStore {
    /// Create a new LogStore
    pub fn new() -> Self {
        Self {
            logs: VecDeque::new(),
            max_size: 10000,
        }
    }

    /// Initialize the store
    pub async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    /// Cleanup resources
    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    /// Add a log entry
    pub fn add_log(&mut self, entry: LogEntry) -> Result<()> {
        if self.logs.len() >= self.max_size {
            self.logs.pop_front();
        }
        self.logs.push_back(entry);
        Ok(())
    }

    /// Get logs, optionally filtered by level
    pub fn get_logs(&self, level: Option<&str>, limit: usize) -> Vec<LogEntry> {
        self.logs
            .iter()
            .rev()
            .filter(|entry| level.map_or(true, |l| entry.level == l))
            .take(limit)
            .cloned()
            .collect()
    }

    /// Search logs by message content
    pub fn search_logs(&self, query: &str, limit: usize) -> Vec<LogEntry> {
        self.logs
            .iter()
            .rev()
            .filter(|entry| entry.message.contains(query))
            .take(limit)
            .cloned()
            .collect()
    }

    /// Get logs since a timestamp
    pub fn get_logs_since(&self, since: DateTime<Utc>, limit: usize) -> Vec<LogEntry> {
        self.logs
            .iter()
            .rev()
            .filter(|entry| entry.timestamp >= since)
            .take(limit)
            .cloned()
            .collect()
    }

    /// Clear all logs
    pub fn clear(&mut self) {
        self.logs.clear();
    }
}

impl Default for LogStore {
    fn default() -> Self {
        Self::new()
    }
}
