use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: String,
    pub message: String,
    pub metadata: serde_json::Value,
}

pub struct LogStore {
    logs: VecDeque<LogEntry>,
    max_size: usize,
}

impl LogStore {
    pub fn new() -> Self {
        Self {
            logs: VecDeque::new(),
            max_size: 10000,
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    pub async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn add_log(&mut self, entry: LogEntry) -> Result<()> {
        if self.logs.len() >= self.max_size {
            self.logs.pop_front();
        }
        self.logs.push_back(entry);
        Ok(())
    }

    pub fn get_logs(&self, level: Option<&str>, limit: usize) -> Vec<LogEntry> {
        self.logs
            .iter()
            .rev()
            .filter(|entry| level.map_or(true, |l| entry.level == l))
            .take(limit)
            .cloned()
            .collect()
    }

    pub fn search_logs(&self, query: &str, limit: usize) -> Vec<LogEntry> {
        self.logs
            .iter()
            .rev()
            .filter(|entry| entry.message.contains(query))
            .take(limit)
            .cloned()
            .collect()
    }
}
