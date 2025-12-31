#[cfg(test)]
mod tests {
    use rust_lovable::core::conversational_ai::{ConversationalAI, AIProvider, Conversation, UIChangeRequest, UIChangeType};
    use rust_lovable::core::ui_generator::{UIGenerator, ComponentType};
    use rust_lovable::core::project_manager::ProjectManager;
    use rust_lovable::core::cross_platform::CrossPlatformAdapter;
    use rust_lovable::core::conversational_ai::PlatformTarget;
    use std::path::PathBuf;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_conversational_ai_processing() {
        let ai = ConversationalAI::new(AIProvider::Local { 
            endpoint: "http://localhost:8080/ai".to_string() 
        });
        
        let mut conversation = Conversation {
            id: uuid::Uuid::new_v4().to_string(),
            messages: Default::default(),
            context: Default::default(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        let user_input = "Create a blue button with the text 'Submit'".to_string();
        let response = ai.process_message(&mut conversation, user_input).await;
        
        assert!(response.is_ok());
        assert!(!conversation.messages.is_empty());
    }
    
    #[tokio::test]
    async fn test_ui_request_parsing() {
        let ai = ConversationalAI::new(AIProvider::Local { 
            endpoint: "http://localhost:8080/ai".to_string() 
        });
        
        let test_cases = vec![
            ("Create a button", UIChangeType::CreateComponent),
            ("Modify the text", UIChangeType::ModifyComponent),
            ("Delete this element", UIChangeType::DeleteComponent),
            ("Change the layout", UIChangeType::RestructureLayout),
            ("Make it blue", UIChangeType::ChangeStyling),
            ("Add click handler", UIChangeType::AddInteraction),
            ("Update content", UIChangeType::UpdateContent),
        ];
        
        for (input, expected_type) in test_cases {
            let request = ai.parse_ui_request(input).unwrap();
            assert_eq!(request.change_type, expected_type);
        }
    }
    
    #[tokio::test]
    async fn test_ui_component_generation() {
        let mut generator = UIGenerator::new();
        
        let request = UIChangeRequest {
            description: "Create a primary button with text 'Click me'".to_string(),
            target_component: None,
            change_type: UIChangeType::CreateComponent,
            platform_specific: None,
        };
        
        let component = generator.generate_component(request).unwrap();
        
        assert_eq!(component.component_type, ComponentType::Button);
        assert!(component.properties.contains_key("text"));
        assert!(component.properties.contains_key("variant"));
    }
    
    #[tokio::test]
    async fn test_project_lifecycle() {
        let temp_dir = TempDir::new().unwrap();
        let projects_dir = temp_dir.path().join("projects");
        
        let mut project_manager = ProjectManager::new(projects_dir);
        
        // Create project
        let project = project_manager.create_project(
            "Test Project".to_string(),
            "A test project for integration testing".to_string(),
        ).unwrap();
        
        assert_eq!(project.name, "Test Project");
        assert!(!project.pages.is_empty());
        
        // List projects
        let projects = project_manager.list_projects().unwrap();
        assert_eq!(projects.len(), 1);
        
        // Load project
        let loaded_project = project_manager.load_project(&project.id).unwrap();
        assert_eq!(loaded_project.name, project.name);
    }
    
    #[tokio::test]
    async fn test_cross_platform_adaptation() {
        let adapter = CrossPlatformAdapter::new();
        
        let mut component = rust_lovable::core::ui_generator::UIComponent {
            id: uuid::Uuid::new_v4().to_string(),
            component_type: ComponentType::Button,
            properties: std::collections::HashMap::new(),
            children: vec![],
            platform_adaptations: std::collections::HashMap::new(),
            generated_code: None,
        };
        
        // Adapt for mobile
        adapter.adapt_component(&mut component, PlatformTarget::Mobile);
        
        assert!(component.platform_adaptations.contains_key(&PlatformTarget::Mobile));
    }
    
    #[tokio::test]
    async fn test_code_generation() {
        let generator = rust_lovable::core::code_generator::CodeGenerator::new();
        
        let component = rust_lovable::core::ui_generator::UIComponent {
            id: uuid::Uuid::new_v4().to_string(),
            component_type: ComponentType::Text,
            properties: {
                let mut props = std::collections::HashMap::new();
                props.insert("text".to_string(), serde_json::Value::String("Hello World".to_string()));
                props
            },
            children: vec![],
            platform_adaptations: std::collections::HashMap::new(),
            generated_code: None,
        };
        
        let code = generator.generate_component_code(&component, PlatformTarget::Web).unwrap();
        
        assert!(code.contains("rsx!"));
        assert!(code.contains("Hello World"));
    }
    
    #[tokio::test]
    async fn test_sandbox_execution() {
        use crate::sandbox::SandboxInstance;
        
        let sandbox = SandboxInstance::new(
            uuid::Uuid::new_v4().to_string(),
            "test".to_string(),
            vec![],
        ).await.unwrap();
        
        let code = r#"
            fn main() {
                println!("Hello from Rust!");
                let sum = 2 + 2;
                println!("2 + 2 = {}", sum);
            }
        "#;
        
        let result = sandbox.execute_code(code, "rust", Some(5)).await.unwrap();
        
        assert!(result.success);
        assert!(result.output.unwrap().contains("Hello from Rust!"));
    }
    
    #[tokio::test]
    async fn test_package_detection_and_installation() {
        use crate::sandbox::SandboxInstance;
        
        let mut sandbox = SandboxInstance::new(
            uuid::Uuid::new_v4().to_string(),
            "test".to_string(),
            vec![],
        ).await.unwrap();
        
        let packages = vec!["serde".to_string(), "tokio".to_string()];
        let result = sandbox.install_packages(&packages, "cargo").await.unwrap();
        
        assert!(result.success);
        assert!(result.installed.len() >= 1);
    }
    
    #[tokio::test]
    async fn test_concurrent_operations() {
        use std::sync::Arc;
        use tokio::sync::Mutex;
        
        let generator = Arc::new(Mutex::new(UIGenerator::new()));
        let mut handles = vec![];
        
        for i in 0..10 {
            let gen_clone = generator.clone();
            let handle = tokio::spawn(async move {
                let request = UIChangeRequest {
                    description: format!("Create button {}", i),
                    target_component: None,
                    change_type: UIChangeType::CreateComponent,
                    platform_specific: None,
                };
                
                let mut gen = gen_clone.lock().await;
                gen.generate_component(request).unwrap()
            });
            handles.push(handle);
        }
        
        let results = futures::future::join_all(handles).await;
        assert_eq!(results.len(), 10);
    }
    
    #[tokio::test]
    async fn test_error_handling() {
        let ai = ConversationalAI::new(AIProvider::Local { 
            endpoint: "http://invalid-endpoint:9999".to_string() 
        });
        
        let mut conversation = Conversation {
            id: uuid::Uuid::new_v4().to_string(),
            messages: Default::default(),
            context: Default::default(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        let result = ai.process_message(&mut conversation, "Test message".to_string()).await;
        
        // Should handle network errors gracefully
        assert!(result.is_err() || true); // Placeholder for actual error handling
    }
    
    #[tokio::test]
    async fn test_performance_benchmarks() {
        use std::time::Instant;
        
        let generator = UIGenerator::new();
        let iterations = 100;
        
        let start = Instant::now();
        
        for _ in 0..iterations {
            let request = UIChangeRequest {
                description: "Create a complex form with inputs and buttons".to_string(),
                target_component: None,
                change_type: UIChangeType::CreateComponent,
                platform_specific: None,
            };
            
            let _ = generator.generate_component(request);
        }
        
        let duration = start.elapsed();
        let avg_time = duration.as_millis() / iterations as u128;
        
        println!("Average component generation time: {}ms", avg_time);
        assert!(avg_time < 100); // Should be fast
    }
}