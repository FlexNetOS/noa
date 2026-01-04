//! End-to-end tests for the unified Dioxus UI.
//!
//! These tests verify the complete UI stack works together.

use std::time::Duration;

/// Test configsuration.
struct Testconfigs {
    api_url: String,
    timeout: Duration,
}

impl Default for Testconfigs {
    fn default() -> Self {
        Self {
            api_url: std::env::var("NOA_API_URL")
                .unwrap_or_else(|_| "http://localhost:3001".to_string()),
            timeout: Duration::from_secs(30),
        }
    }
}

#[cfg(test)]
mod api_integration {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires running API server"]
    async fn test_api_health() {
        let configs = Testconfigs::default();
        let client = reqwest::Client::new();
        
        let response = client
            .get(format!("{}/health", configs.api_url))
            .timeout(configs.timeout)
            .send()
            .await;
        
        assert!(response.is_ok(), "API should be reachable");
        assert!(response.unwrap().status().is_success());
    }

    #[tokio::test]
    #[ignore = "Requires running API server"]
    async fn test_api_status() {
        let configs = Testconfigs::default();
        let client = reqwest::Client::new();
        
        let response = client
            .get(format!("{}/api/v1/status", configs.api_url))
            .timeout(configs.timeout)
            .send()
            .await;
        
        assert!(response.is_ok());
        let body: serde_json::Value = response.unwrap().json().await.unwrap();
        assert!(body.get("status").is_some());
    }

    #[tokio::test]
    #[ignore = "Requires running API server"]
    async fn test_providers_endpoint() {
        let configs = Testconfigs::default();
        let client = reqwest::Client::new();
        
        let response = client
            .get(format!("{}/api/v1/providers", configs.api_url))
            .timeout(configs.timeout)
            .send()
            .await;
        
        assert!(response.is_ok());
        let body: serde_json::Value = response.unwrap().json().await.unwrap();
        assert!(body.get("providers").is_some());
    }
}

#[cfg(test)]
mod daemon_integration {
    use super::*;

    #[tokio::test]
    #[ignore = "Requires running daemon"]
    async fn test_daemon_health() {
        let client = reqwest::Client::new();
        
        let response = client
            .get("http://localhost:9090/health")
            .timeout(Duration::from_secs(5))
            .send()
            .await;
        
        assert!(response.is_ok());
        let body: serde_json::Value = response.unwrap().json().await.unwrap();
        assert_eq!(body["status"], "ok");
    }

    #[tokio::test]
    #[ignore = "Requires running daemon"]
    async fn test_daemon_status() {
        let client = reqwest::Client::new();
        
        let response = client
            .get("http://localhost:9090/status")
            .timeout(Duration::from_secs(5))
            .send()
            .await;
        
        assert!(response.is_ok());
    }

    #[tokio::test]
    #[ignore = "Requires running daemon"]
    async fn test_daemon_peers() {
        let client = reqwest::Client::new();
        
        let response = client
            .get("http://localhost:9090/peers")
            .timeout(Duration::from_secs(5))
            .send()
            .await;
        
        assert!(response.is_ok());
        let body: serde_json::Value = response.unwrap().json().await.unwrap();
        assert!(body["peers"].is_array());
    }

    #[tokio::test]
    #[ignore = "Requires running daemon"]
    async fn test_daemon_agents() {
        let client = reqwest::Client::new();
        
        let response = client
            .get("http://localhost:9090/agents")
            .timeout(Duration::from_secs(5))
            .send()
            .await;
        
        assert!(response.is_ok());
        let body: serde_json::Value = response.unwrap().json().await.unwrap();
        assert!(body["agents"].is_array());
    }
}

#[cfg(test)]
mod ui_component_tests {
    /// Verify module exports are correct.
    #[test]
    fn test_shell_exports() {
        // This is a compile-time test - if it compiles, exports are correct
        use noa_ui_shell::{
            App,
            ChatPage, ChatInput, ChatMessages, ProviderSelector,
            InferencePage, ServerControl, ModelSelector, CompletionPanel,
            SettingsPage, ProviderSettings, AppearanceSettings, AboutPage,
            LogsPage, LogViewer, LogFilters,
            MetricsPage, StatusCard, MetricChart,
        };
        
        // Type assertions
        fn assert_component<T>() {}
        
        // All exports should be valid types
        let _ = || {
            assert_component::<fn() -> dioxus::prelude::Element>();
        };
    }
}

#[cfg(test)]
mod configs_validation {
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_capsule_schema_valid() {
        let schema_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .join("configs/base/schemas/capsule.schema.json");
        
        let content = fs::read_to_string(&schema_path).expect("Schema file should exist");
        let schema: serde_json::Value = serde_json::from_str(&content).expect("Schema should be valid JSON");
        
        assert_eq!(schema["$schema"], "https://json-schema.org/draft/2020-12/schema");
        assert!(schema.get("properties").is_some());
    }

    #[test]
    fn test_all_capsules_have_required_fields() {
        let capsules_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .join("sandbox/agents/capsules");
        
        for entry in fs::read_dir(&capsules_dir).expect("Capsules dir should exist") {
            let entry = entry.unwrap();
            let path = entry.path();
            
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                let content = fs::read_to_string(&path).expect("Capsule should be readable");
                let capsule: serde_json::Value = serde_json::from_str(&content)
                    .expect(&format!("Capsule {:?} should be valid JSON", path));
                
                // Required fields
                assert!(capsule.get("name").is_some(), "{:?} missing name", path);
                assert!(capsule.get("version").is_some(), "{:?} missing version", path);
                assert!(capsule.get("resources").is_some(), "{:?} missing resources", path);
                assert!(capsule.get("capabilities").is_some(), "{:?} missing capabilities", path);
            }
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use std::time::Instant;

    #[test]
    fn test_configs_load_performance() {
        let start = Instant::now();
        
        for _ in 0..100 {
            let _ = std::fs::read_to_string("../configs/base/ai-providers.json");
        }
        
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 1000, "configs loading should be fast");
    }

    #[test]
    fn test_json_parse_performance() {
        let json_str = r#"{
            "providers": [
                {"id": "ollama", "name": "Ollama", "status": "online"},
                {"id": "openai", "name": "OpenAI", "status": "offline"}
            ]
        }"#;
        
        let start = Instant::now();
        
        for _ in 0..1000 {
            let _: serde_json::Value = serde_json::from_str(json_str).unwrap();
        }
        
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 500, "JSON parsing should be fast");
    }
}
