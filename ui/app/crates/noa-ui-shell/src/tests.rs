//! Tests for the noa-ui-shell components.

#[cfg(test)]
mod settings_tests {
    #[test]
    fn test_theme_names() {
        let themes = ["light", "dark", "cupcake", "bumblebee", "emerald", "corporate", "synthwave", "retro"];
        
        assert_eq!(themes.len(), 8);
        assert!(themes.contains(&"dark"));
        assert!(themes.contains(&"light"));
    }

    #[test]
    fn test_font_size_range() {
        let min: u32 = 12;
        let max: u32 = 24;
        let default: u32 = 14;
        
        assert!(default >= min);
        assert!(default <= max);
    }
}

#[cfg(test)]
mod log_level_tests {
    #[test]
    fn test_log_levels() {
        let levels = ["error", "warn", "info", "debug"];
        
        assert_eq!(levels.len(), 4);
        assert!(levels.contains(&"error"));
        assert!(levels.contains(&"debug"));
    }

    #[test]
    fn test_log_sources() {
        let sources = ["api", "llama", "p2p", "agent", "ui"];
        
        assert_eq!(sources.len(), 5);
    }
}

#[cfg(test)]
mod resource_tests {
    #[test]
    fn test_resource_thresholds() {
        fn get_status(value: f32) -> &'static str {
            if value > 90.0 { "error" } 
            else if value > 70.0 { "warning" } 
            else { "normal" }
        }
        
        assert_eq!(get_status(95.0), "error");
        assert_eq!(get_status(75.0), "warning");
        assert_eq!(get_status(50.0), "normal");
    }

    #[test]
    fn test_cache_hit_rate_bounds() {
        fn validate_rate(rate: f32) -> bool {
            rate >= 0.0 && rate <= 1.0
        }
        
        assert!(validate_rate(0.0));
        assert!(validate_rate(0.5));
        assert!(validate_rate(1.0));
        assert!(!validate_rate(1.5));
        assert!(!validate_rate(-0.1));
    }
}

#[cfg(test)]
mod api_client_integration_tests {
    use noa_api_client::{ChatMessage, Provider, Model};

    #[test]
    fn test_chat_message_creation() {
        let msg = ChatMessage {
            role: "user".to_string(),
            content: "Hello".to_string(),
        };
        
        assert_eq!(msg.role, "user");
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_chat_message_roles() {
        let user_msg = ChatMessage {
            role: "user".to_string(),
            content: "test".to_string(),
        };
        
        let assistant_msg = ChatMessage {
            role: "assistant".to_string(),
            content: "test".to_string(),
        };
        
        assert!(user_msg.role == "user");
        assert!(assistant_msg.role == "assistant");
    }

    #[test]
    fn test_provider_creation() {
        let provider = Provider {
            id: "ollama".to_string(),
            name: "Ollama".to_string(),
            provider_type: "local".to_string(),
            status: "online".to_string(),
            priority: Some(1),
        };
        
        assert_eq!(provider.id, "ollama");
        assert_eq!(provider.status, "online");
    }

    #[test]
    fn test_model_creation() {
        let model = Model {
            id: "qwen2.5-coder:3b".to_string(),
            name: "Qwen 2.5 Coder 3B".to_string(),
            size: Some("2.5GB".to_string()),
            context_length: Some(32768),
        };
        
        assert_eq!(model.id, "qwen2.5-coder:3b");
        assert!(model.size.is_some());
    }
}
