//! Additional unit tests for inference context management
//!
//! These tests complement the tests in context.rs

#[cfg(test)]
mod tests {
    use crate::neural::context::{InferenceContext, ContextManager, MessageRole};

    #[tokio::test]
    async fn test_context_manager_creation() {
        // Test that context manager can be created
        let manager = ContextManager::new();
        // Manager is created successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_context_manager_create_context() {
        // Test creating context through manager
        let manager = ContextManager::new();
        let context_id = manager.create_context("test-model".to_string(), 2048).await;
        
        let context = manager.get_context(&context_id).await;
        assert!(context.is_some());
        assert_eq!(context.unwrap().model_id, "test-model");
    }

    #[tokio::test]
    async fn test_context_manager_delete_context() {
        // Test deleting context
        let manager = ContextManager::new();
        let context_id = manager.create_context("test-model".to_string(), 2048).await;
        
        manager.delete_context(&context_id).await;
        
        let context = manager.get_context(&context_id).await;
        assert!(context.is_none());
    }

    #[test]
    fn test_context_format_prompt() {
        // Test prompt formatting
        let mut context = InferenceContext::new("test-model".to_string(), 2048);
        
        context.add_message(MessageRole::System, "You are a helpful assistant.".to_string(), 10).unwrap();
        context.add_message(MessageRole::User, "Hello".to_string(), 5).unwrap();
        context.add_message(MessageRole::Assistant, "Hi there!".to_string(), 8).unwrap();
        
        let formatted = context.format_prompt("How are you?");
        assert!(formatted.contains("System:"));
        assert!(formatted.contains("User:"));
        assert!(formatted.contains("Assistant:"));
        assert!(formatted.contains("How are you?"));
    }

    #[test]
    fn test_context_token_count() {
        // Test token counting
        let mut context = InferenceContext::new("test-model".to_string(), 2048);
        
        assert_eq!(context.token_count(), 0);
        
        context.add_message(MessageRole::User, "Hello".to_string(), 10).unwrap();
        assert_eq!(context.token_count(), 10);
        
        context.add_message(MessageRole::Assistant, "Hi".to_string(), 5).unwrap();
        assert_eq!(context.token_count(), 15);
    }

    #[test]
    fn test_context_is_full() {
        // Test is_full check
        let mut context = InferenceContext::new("test-model".to_string(), 100);
        
        assert!(!context.is_full());
        
        context.add_message(MessageRole::User, "Message".to_string(), 100).unwrap();
        assert!(context.is_full());
    }
}

