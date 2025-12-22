//! Unified Priority Queue
//!
//! Implements unified priority queue for goals from all sources.
//! FR-066-070: Autonomous Goal Generation
//!
//! T634: Implement unified priority queue

use crate::error::{Result, NoaError};
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Queue entry with priority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityEntry {
    pub id: Uuid,
    pub priority: i32,
    pub source: String,
    pub data: serde_json::Value,
}

impl PartialEq for PriorityEntry {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.id == other.id
    }
}

impl Eq for PriorityEntry {}

impl PartialOrd for PriorityEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first, then by ID for stability
        match self.priority.cmp(&other.priority) {
            std::cmp::Ordering::Equal => self.id.cmp(&other.id),
            ord => ord,
        }
    }
}

/// Unified priority queue
pub struct PriorityQueue {
    queue: Arc<RwLock<BinaryHeap<PriorityEntry>>>,
    max_size: Option<usize>,
}

impl PriorityQueue {
    /// Create a new priority queue
    pub fn new(max_size: Option<usize>) -> Self {
        Self {
            queue: Arc::new(RwLock::new(BinaryHeap::new())),
            max_size,
        }
    }

    /// Push an entry to the queue
    pub async fn push(&self, entry: PriorityEntry) -> Result<()> {
        let mut queue = self.queue.write().await;

        if let Some(max) = self.max_size {
            if queue.len() >= max {
                return Err(NoaError::Internal {
                    message: format!("Priority queue is full (max: {})", max),
                    source: None,
                });
            }
        }

        queue.push(entry);
        Ok(())
    }

    /// Pop the highest priority entry
    pub async fn pop(&self) -> Option<PriorityEntry> {
        let mut queue = self.queue.write().await;
        queue.pop()
    }

    /// Peek at the highest priority entry without removing it
    pub async fn peek(&self) -> Option<PriorityEntry> {
        let queue = self.queue.read().await;
        queue.peek().cloned()
    }

    /// Get queue size
    pub async fn len(&self) -> usize {
        self.queue.read().await.len()
    }

    /// Check if queue is empty
    pub async fn is_empty(&self) -> bool {
        self.queue.read().await.is_empty()
    }

    /// Clear the queue
    pub async fn clear(&self) {
        let mut queue = self.queue.write().await;
        queue.clear();
    }

    /// Get all entries (sorted by priority)
    pub async fn all_entries(&self) -> Vec<PriorityEntry> {
        let queue = self.queue.read().await;
        let mut entries: Vec<PriorityEntry> = queue.iter().cloned().collect();
        entries.sort();
        entries
    }

    /// Remove entry by ID
    pub async fn remove(&self, id: Uuid) -> bool {
        let mut queue = self.queue.write().await;
        let initial_len = queue.len();
        *queue = queue.iter().filter(|e| e.id != id).cloned().collect();
        queue.len() < initial_len
    }
}

impl Default for PriorityQueue {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_priority_ordering() {
        let queue = PriorityQueue::new(None);

        queue.push(PriorityEntry {
            id: Uuid::new_v4(),
            priority: 5,
            source: "test".to_string(),
            data: serde_json::json!({}),
        }).await.unwrap();

        queue.push(PriorityEntry {
            id: Uuid::new_v4(),
            priority: 10,
            source: "test".to_string(),
            data: serde_json::json!({}),
        }).await.unwrap();

        queue.push(PriorityEntry {
            id: Uuid::new_v4(),
            priority: 1,
            source: "test".to_string(),
            data: serde_json::json!({}),
        }).await.unwrap();

        // Should pop highest priority first
        let first = queue.pop().await.unwrap();
        assert_eq!(first.priority, 10);

        let second = queue.pop().await.unwrap();
        assert_eq!(second.priority, 5);

        let third = queue.pop().await.unwrap();
        assert_eq!(third.priority, 1);
    }

    #[tokio::test]
    async fn test_max_size() {
        let queue = PriorityQueue::new(Some(2));

        queue.push(PriorityEntry {
            id: Uuid::new_v4(),
            priority: 1,
            source: "test".to_string(),
            data: serde_json::json!({}),
        }).await.unwrap();

        queue.push(PriorityEntry {
            id: Uuid::new_v4(),
            priority: 2,
            source: "test".to_string(),
            data: serde_json::json!({}),
        }).await.unwrap();

        // Should fail when at max
        assert!(queue.push(PriorityEntry {
            id: Uuid::new_v4(),
            priority: 3,
            source: "test".to_string(),
            data: serde_json::json!({}),
        }).await.is_err());
    }

    #[tokio::test]
    async fn test_peek() {
        let queue = PriorityQueue::new(None);

        queue.push(PriorityEntry {
            id: Uuid::new_v4(),
            priority: 10,
            source: "test".to_string(),
            data: serde_json::json!({}),
        }).await.unwrap();

        let peeked = queue.peek().await.unwrap();
        assert_eq!(peeked.priority, 10);

        // Queue should still have the entry
        assert_eq!(queue.len().await, 1);
    }
}

