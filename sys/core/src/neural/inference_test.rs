//! Unit tests for inference engine
//!
//! Tests for inference functionality with streaming

#[cfg(test)]
mod tests {
    use crate::neural::inference::InferenceRequest;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_inference_request_creation() {
        // Test that inference request can be created
        let request = InferenceRequest {
            model_id: "test-model".to_string(),
            prompt: "Hello, world!".to_string(),
            context_id: None,
            temperature: Some(0.7),
            top_p: Some(0.9),
            top_k: Some(40),
            max_tokens: Some(100),
            stream: false,
        };

        assert_eq!(request.model_id, "test-model");
        assert_eq!(request.prompt, "Hello, world!");
        assert_eq!(request.stream, false);
    }

    #[tokio::test]
    async fn test_inference_request_with_context() {
        // Test inference request with context ID
        let context_id = Some(Uuid::new_v4());
        let request = InferenceRequest {
            model_id: "test-model".to_string(),
            prompt: "Continue conversation".to_string(),
            context_id,
            temperature: None,
            top_p: None,
            top_k: None,
            max_tokens: None,
            stream: true,
        };

        assert!(request.context_id.is_some());
        assert_eq!(request.stream, true);
    }

    #[test]
    fn test_inference_request_defaults() {
        // Test inference request with default values
        let request = InferenceRequest {
            model_id: "test-model".to_string(),
            prompt: "Test prompt".to_string(),
            context_id: None,
            temperature: None,
            top_p: None,
            top_k: None,
            max_tokens: None,
            stream: false,
        };

        assert_eq!(request.model_id, "test-model");
        assert_eq!(request.temperature, None);
    }
}
