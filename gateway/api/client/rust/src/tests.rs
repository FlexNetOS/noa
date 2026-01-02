//! Tests for the NOA API client.

#[cfg(test)]
mod tests {
    use crate::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_mock_server() -> MockServer {
        MockServer::start().await
    }

    #[tokio::test]
    async fn test_client_creation() {
        let client = Client::new("http://localhost:3001");
        assert_eq!(client.base_url, "http://localhost:3001");
    }

    #[tokio::test]
    async fn test_client_default() {
        let client = Client::default();
        assert_eq!(client.base_url, crate::DEFAULT_ENDPOINT);
    }

    #[tokio::test]
    async fn test_trailing_slash_removed() {
        let client = Client::new("http://localhost:3001/");
        assert_eq!(client.base_url, "http://localhost:3001");
    }

    #[tokio::test]
    async fn test_health_check_success() {
        let server = setup_mock_server().await;
        
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok",
                "version": "0.1.0"
            })))
            .mount(&server)
            .await;

        let client = Client::new(&server.uri());
        let result = client.health().await;
        
        assert!(result.is_ok());
        let health = result.unwrap();
        assert_eq!(health.status, "ok");
    }

    #[tokio::test]
    async fn test_health_check_failure() {
        let server = setup_mock_server().await;
        
        Mock::given(method("GET"))
            .and(path("/health"))
            .respond_with(ResponseTemplate::new(503).set_body_string("Service unavailable"))
            .mount(&server)
            .await;

        let client = Client::new(&server.uri());
        let result = client.health().await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_status_endpoint() {
        let server = setup_mock_server().await;
        
        Mock::given(method("GET"))
            .and(path("/api/v1/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "running",
                "components": {
                    "api": true,
                    "database": true,
                    "embedder": false,
                    "agents": true,
                    "p2p": false
                }
            })))
            .mount(&server)
            .await;

        let client = Client::new(&server.uri());
        let result = client.status().await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_chat_request() {
        let server = setup_mock_server().await;
        
        Mock::given(method("POST"))
            .and(path("/api/v1/chat"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "chat_123",
                "content": "Hello! How can I help you?",
                "provider": "ollama",
                "model": "qwen2.5-coder",
                "usage": {
                    "prompt_tokens": 10,
                    "completion_tokens": 15,
                    "total_tokens": 25
                }
            })))
            .mount(&server)
            .await;

        let client = Client::new(&server.uri());
        let result = client.chat(ChatRequest {
            message: "Hello".to_string(),
            provider: Some("ollama".to_string()),
            model: None,
            history: None,
            stream: false,
        }).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.content, "Hello! How can I help you?");
        assert_eq!(response.provider, "ollama");
    }

    #[tokio::test]
    async fn test_chat_error_response() {
        let server = setup_mock_server().await;
        
        Mock::given(method("POST"))
            .and(path("/api/v1/chat"))
            .respond_with(ResponseTemplate::new(429).set_body_json(serde_json::json!({
                "error": "Rate limit exceeded",
                "code": "RATE_LIMITED"
            })))
            .mount(&server)
            .await;

        let client = Client::new(&server.uri());
        let result = client.chat(ChatRequest {
            message: "Hello".to_string(),
            provider: None,
            model: None,
            history: None,
            stream: false,
        }).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Api { status, message } => {
                assert_eq!(status, 429);
                assert!(message.contains("Rate limit"));
            }
            _ => panic!("Expected API error"),
        }
    }

    #[tokio::test]
    async fn test_list_providers() {
        let server = setup_mock_server().await;
        
        Mock::given(method("GET"))
            .and(path("/api/v1/providers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "providers": [
                    {"id": "ollama", "name": "Ollama", "type": "local", "status": "online"},
                    {"id": "openai", "name": "OpenAI", "type": "cloud", "status": "offline"}
                ]
            })))
            .mount(&server)
            .await;

        let client = Client::new(&server.uri());
        let result = client.list_providers().await;
        
        assert!(result.is_ok());
        let providers = result.unwrap();
        assert_eq!(providers.providers.len(), 2);
    }

    #[tokio::test]
    async fn test_list_models() {
        let server = setup_mock_server().await;
        
        Mock::given(method("GET"))
            .and(path("/api/v1/providers/ollama/models"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "models": [
                    {"id": "qwen2.5-coder:3b", "name": "Qwen2.5 Coder 3B", "size": "2.5GB"},
                    {"id": "llama3.2:3b", "name": "Llama 3.2 3B", "size": "2.0GB"}
                ]
            })))
            .mount(&server)
            .await;

        let client = Client::new(&server.uri());
        let result = client.list_models("ollama").await;
        
        assert!(result.is_ok());
        let models = result.unwrap();
        assert_eq!(models.models.len(), 2);
    }

    #[tokio::test]
    async fn test_connection_error() {
        let client = Client::new("http://localhost:99999");
        let result = client.health().await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Http(_)));
    }
}
