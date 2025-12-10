//! Context Management for Inference
//!
//! T108: Implement context management for inference
//! §3.2: Local-First & Offline-Capable
//! US2: Context management for neural runtime

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Inference context for managing conversation history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceContext {
    pub id: Uuid,
    pub model_id: String,
    pub messages: VecDeque<Message>,
    pub max_context_length: usize,
    pub current_tokens: usize,
}

/// Message in the context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub tokens: usize,
}

/// Message role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

impl InferenceContext {
    /// Create a new inference context
    pub fn new(model_id: String, max_context_length: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            model_id,
            messages: VecDeque::new(),
            max_context_length,
            current_tokens: 0,
        }
    }

    /// Add a message to the context
    pub fn add_message(&mut self, role: MessageRole, content: String, tokens: usize) -> Result<()> {
        // Check if adding this message would exceed context length
        if self.current_tokens + tokens > self.max_context_length {
            // Remove oldest messages until we have room
            while self.current_tokens + tokens > self.max_context_length
                && !self.messages.is_empty()
            {
                if let Some(oldest) = self.messages.pop_front() {
                    self.current_tokens -= oldest.tokens;
                }
            }
        }

        let message = Message {
            role,
            content,
            tokens,
        };

        self.current_tokens += tokens;
        self.messages.push_back(message);

        Ok(())
    }

    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear();
        self.current_tokens = 0;
    }

    /// Get formatted prompt from context
    pub fn format_prompt(&self, new_prompt: &str) -> String {
        let mut prompt = String::new();

        // Add system message if present
        for msg in &self.messages {
            if msg.role == MessageRole::System {
                prompt.push_str(&format!("System: {}\n", msg.content));
            }
        }

        // Add conversation history
        for msg in &self.messages {
            match msg.role {
                MessageRole::User => {
                    prompt.push_str(&format!("User: {}\n", msg.content));
                }
                MessageRole::Assistant => {
                    prompt.push_str(&format!("Assistant: {}\n", msg.content));
                }
                MessageRole::System => {
                    // Already added
                }
            }
        }

        // Add new prompt
        prompt.push_str(&format!("User: {}\n", new_prompt));
        prompt.push_str("Assistant: ");

        prompt
    }

    /// Get current token count
    pub fn token_count(&self) -> usize {
        self.current_tokens
    }

    /// Check if context is full
    pub fn is_full(&self) -> bool {
        self.current_tokens >= self.max_context_length
    }
}

/// Context manager for multiple inference contexts
pub struct ContextManager {
    contexts: Arc<RwLock<std::collections::HashMap<Uuid, InferenceContext>>>,
}

impl ContextManager {
    /// Create a new context manager
    pub fn new() -> Self {
        Self {
            contexts: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Create a new context
    pub async fn create_context(&self, model_id: String, max_context_length: usize) -> Uuid {
        let context = InferenceContext::new(model_id, max_context_length);
        let id = context.id;

        let mut contexts = self.contexts.write().await;
        contexts.insert(id, context);

        id
    }

    /// Get a context
    pub async fn get_context(&self, id: &Uuid) -> Option<InferenceContext> {
        let contexts = self.contexts.read().await;
        contexts.get(id).cloned()
    }

    /// Update a context
    pub async fn update_context(&self, context: InferenceContext) -> Result<()> {
        let mut contexts = self.contexts.write().await;
        contexts.insert(context.id, context);
        Ok(())
    }

    /// Delete a context
    pub async fn delete_context(&self, id: &Uuid) {
        let mut contexts = self.contexts.write().await;
        contexts.remove(id);
    }

    /// Clear all contexts
    pub async fn clear_all(&self) {
        let mut contexts = self.contexts.write().await;
        contexts.clear();
    }
}

impl Default for ContextManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let context = InferenceContext::new("model1".to_string(), 2048);
        assert_eq!(context.model_id, "model1");
        assert_eq!(context.max_context_length, 2048);
        assert_eq!(context.current_tokens, 0);
    }

    #[test]
    fn test_add_message() {
        let mut context = InferenceContext::new("model1".to_string(), 2048);
        context.add_message(MessageRole::User, "Hello".to_string(), 10).unwrap();
        assert_eq!(context.messages.len(), 1);
        assert_eq!(context.current_tokens, 10);
    }

    #[test]
    fn test_context_overflow() {
        let mut context = InferenceContext::new("model1".to_string(), 100);
        context.add_message(MessageRole::User, "Message 1".to_string(), 30).unwrap();
        context.add_message(MessageRole::User, "Message 2".to_string(), 30).unwrap();
        context.add_message(MessageRole::User, "Message 3".to_string(), 30).unwrap();
        context.add_message(MessageRole::User, "Message 4".to_string(), 30).unwrap();

        // Should have removed oldest messages
        assert!(context.current_tokens <= 100);
    }
}
