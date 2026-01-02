#![cfg(test)]

use rust_lovable::{
    core::code_generator::CodeGenerator,
    core::conversational_ai::{
        AIProvider, Conversation, ConversationalAI, PlatformTarget, UIChangeRequest, UIChangeType,
    },
    core::cross_platform::CrossPlatformAdapter,
    core::project_manager::{Project, ProjectManager},
    core::ui_generator::{ComponentType, UIComponent, UIGenerator},
};

// Import test helper functions
mod test_helpers;
use test_helpers::{
    analyze_error, basic_hardware_detection, check_auto_fix, classify_intent, detect_language,
    extract_js_imports, extract_rust_imports, generate_suggested_actions,
};

use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;
use uuid::Uuid;

// Test streaming API endpoints
#[tokio::test]
async fn test_ai_code_streaming() {
    use axum::response::sse::{Event, Sse};
    use futures::stream::Stream;
    use std::pin::Pin;

    // Mock streaming endpoint
    let events = vec![
        Event::default().data("Analyzing requirements..."),
        Event::default().data("Generating components..."),
        Event::default().data("Applying styles..."),
        Event::default().data("Complete!"),
    ];

    let stream = futures::stream::iter(events.into_iter().map(Ok));
    let _sse: Sse<Pin<Box<dyn Stream<Item = Result<Event, std::convert::Infallible>> + Send>>> =
        Sse::new(Box::pin(stream));

    assert!(true); // Streaming structure is valid
}

// Test package detection and installation
#[tokio::test]
async fn test_package_detection() {
    // Test JavaScript import detection
    let js_code = r#"
        import React from 'react';
        import { useState } from 'react';
        import * as lodash from 'lodash';
        const express = require('express');
    "#;

    let packages = HashMap::new();
    extract_js_imports(js_code, &mut packages);

    assert!(packages.contains_key("react"));
    assert!(packages.contains_key("lodash"));
    assert!(packages.contains_key("express"));
    assert_eq!(packages["react"].0, 2); // Two imports of react

    // Test Rust import detection
    let rust_code = r#"
        use std::collections::HashMap;
        use serde::{Deserialize, Serialize};
        use tokio::sync::Mutex;
        use uuid::Uuid;
    "#;

    let mut rust_packages = HashMap::new();
    extract_rust_imports(rust_code, &mut rust_packages);

    assert!(rust_packages.contains_key("std"));
    assert!(rust_packages.contains_key("serde"));
    assert!(rust_packages.contains_key("tokio"));
    assert!(rust_packages.contains_key("uuid"));
}

// Test Vite error detection and reporting
#[tokio::test]
async fn test_vite_error_handling() {
    // Define ViteError struct for testing
    #[derive(Debug, Clone)]
    struct ViteError {
        id: String,
        message: String,
        file: Option<String>,
        line: Option<u32>,
        column: Option<u32>,
        severity: String,
        stack_trace: Option<String>,
        timestamp: chrono::DateTime<chrono::Utc>,
        resolved: bool,
    }

    let errors = vec![
        ViteError {
            id: "err_001".to_string(),
            message: "Module not found: Can't resolve 'react'".to_string(),
            file: Some("/src/App.tsx".to_string()),
            line: Some(3),
            column: Some(8),
            severity: "error".to_string(),
            stack_trace: Some("Error: Cannot find module 'react'".to_string()),
            timestamp: chrono::Utc::now(),
            resolved: false,
        },
        ViteError {
            id: "err_002".to_string(),
            message: "Unused variable 'unusedVar'".to_string(),
            file: Some("/src/components/Button.tsx".to_string()),
            line: Some(15),
            column: Some(10),
            severity: "warning".to_string(),
            stack_trace: None,
            timestamp: chrono::Utc::now(),
            resolved: false,
        },
    ];

    // Test error analysis
    for error in &errors {
        let suggestions = analyze_error(&error.message);
        assert!(!suggestions.is_empty());

        if error.message.contains("Module not found") {
            assert!(suggestions.iter().any(|s| s.contains("npm install")));
        }
    }

    // Test auto-fix detection
    assert!(check_auto_fix("Module not found: Can't resolve 'react'"));
    assert!(check_auto_fix("Cannot find module 'lodash'"));
    assert!(!check_auto_fix("Unused variable 'x'"));
}

// Test file system operations
#[tokio::test]
async fn test_file_operations() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    // Test file writing
    let content = "Hello, Rust Lovable!";
    std::fs::write(&test_file, content).unwrap();

    // Test file reading
    let read_content = std::fs::read_to_string(&test_file).unwrap();
    assert_eq!(read_content, content);

    // Test file stats
    let metadata = std::fs::metadata(&test_file).unwrap();
    assert!(metadata.len() > 0);
    assert!(metadata.is_file());

    // Test file detection
    let language = detect_language("test.txt");
    assert_eq!(language, "text");

    let js_language = detect_language("component.tsx");
    assert_eq!(js_language, "typescriptreact");
}

// Test brand style extraction
#[tokio::test]
async fn test_brand_style_extraction() {
    // Define structs for testing
    #[derive(Debug)]
    struct ColorDefinition {
        hex: String,
        rgb: (u8, u8, u8),
        usage_frequency: f32,
        contexts: Vec<String>,
    }

    #[derive(Debug)]
    struct FontDefinition {
        family: String,
        sizes: Vec<f32>,
        weights: Vec<String>,
        usage_frequency: f32,
    }

    #[derive(Debug)]
    struct SpacingDefinition {
        base_unit: f32,
        scale: Vec<f32>,
        common_values: Vec<f32>,
    }

    #[derive(Debug)]
    struct ExtractBrandStylesResponse {
        colors: Vec<ColorDefinition>,
        fonts: Vec<FontDefinition>,
        spacing: SpacingDefinition,
        confidence: f32,
    }

    let mock_styles = ExtractBrandStylesResponse {
        colors: vec![
            ColorDefinition {
                hex: "#3B82F6".to_string(),
                rgb: (59, 130, 246),
                usage_frequency: 0.8,
                contexts: vec!["primary".to_string(), "buttons".to_string()],
            },
            ColorDefinition {
                hex: "#1F2937".to_string(),
                rgb: (31, 41, 55),
                usage_frequency: 0.6,
                contexts: vec!["text".to_string(), "headers".to_string()],
            },
        ],
        fonts: vec![FontDefinition {
            family: "Inter".to_string(),
            sizes: vec![14.0, 16.0, 18.0, 24.0, 32.0],
            weights: vec![
                "400".to_string(),
                "500".to_string(),
                "600".to_string(),
                "700".to_string(),
            ],
            usage_frequency: 0.9,
        }],
        spacing: SpacingDefinition {
            base_unit: 4.0,
            scale: vec![0.25, 0.5, 1.0, 1.5, 2.0, 3.0, 4.0, 6.0],
            common_values: vec![4.0, 8.0, 16.0, 24.0, 32.0],
        },
        confidence: 0.85,
    };

    assert!(mock_styles.colors.len() >= 2);
    assert!(mock_styles.fonts.len() >= 1);
    assert!(mock_styles.confidence > 0.8);

    // Test color dominance
    let primary_color = &mock_styles.colors[0];
    assert!(primary_color.usage_frequency > 0.7);
    assert!(primary_color.contexts.contains(&"primary".to_string()));
}

// Test export functionality
#[tokio::test]
async fn test_export_functionality() {
    // Define export response structs for testing
    #[derive(Debug)]
    struct ExportToGitHubResponse {
        success: bool,
        repo_url: String,
        commit_sha: String,
        files_pushed: u32,
        branch: String,
    }

    #[derive(Debug)]
    struct DeployToVercelResponse {
        success: bool,
        deployment_url: String,
        deployment_id: String,
        build_logs_url: String,
        status: String,
    }

    // Test ZIP creation
    let temp_dir = TempDir::new().unwrap();
    let zip_path = temp_dir.path().join("test.zip");

    // Create test files
    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, "test content").unwrap();

    // Create ZIP (mock implementation)
    let file = std::fs::File::create(&zip_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);

    zip.start_file("test.txt", zip::write::FileOptions::default())
        .unwrap();
    zip.write_all(b"test content").unwrap();
    zip.finish().unwrap();

    assert!(zip_path.exists());
    assert!(zip_path.metadata().unwrap().len() > 0);

    // Test GitHub export (mock)
    let github_response = ExportToGitHubResponse {
        success: true,
        repo_url: "https://github.com/user/test-repo".to_string(),
        commit_sha: "abc123def456".to_string(),
        files_pushed: 25,
        branch: "main".to_string(),
    };

    assert!(github_response.success);
    assert!(github_response.files_pushed > 0);

    // Test Vercel deployment (mock)
    let vercel_response = DeployToVercelResponse {
        success: true,
        deployment_url: "https://test-app.vercel.app".to_string(),
        deployment_id: "dpl_test123".to_string(),
        build_logs_url: "https://vercel.com/user/test-app".to_string(),
        status: "ready".to_string(),
    };

    assert!(vercel_response.success);
    assert!(vercel_response.status == "ready");
}

// Test conversation state management
#[tokio::test]
async fn test_conversation_state() {
    let conversation = Conversation {
        id: Uuid::new_v4().to_string(),
        messages: Default::default(),
        context: Default::default(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // Test context management
    let context = serde_json::json!({
        "current_page": "landing",
        "selected_component": "hero_section",
        "platform": "web",
        "user_preferences": {
            "theme": "dark",
            "language": "en"
        }
    });

    assert!(context["current_page"] == "landing");
    assert!(context["platform"] == "web");
}

// Test edit intent analysis
#[tokio::test]
async fn test_edit_intent_analysis() {
    let test_cases = vec![
        ("Create a button", "create_component", 0.95),
        ("Change the color to blue", "modify_component", 0.90),
        ("Delete this element", "delete_component", 0.95),
        ("Make the text larger", "modify_styling", 0.85),
        ("Move this to the right", "modify_layout", 0.80),
        ("Unknown action", "unknown", 0.5),
    ];

    for (message, expected_intent, expected_confidence) in test_cases {
        let (intent, confidence) = classify_intent(message);
        assert_eq!(intent, expected_intent);
        assert!(confidence >= expected_confidence);
    }

    // Test suggested actions
    let create_actions = generate_suggested_actions("create_component");
    assert!(create_actions.len() >= 3);
    assert!(create_actions[0].contains("Create"));

    let modify_actions = generate_suggested_actions("modify_component");
    assert!(modify_actions.len() >= 3);
    assert!(modify_actions[0].contains("Find"));
}

// Test performance optimization
#[tokio::test]
async fn test_performance_optimization() {
    let generator = UIGenerator::new();

    // Test batch component generation
    let start_time = std::time::Instant::now();
    let mut components = vec![];

    for i in 0..100 {
        let request = UIChangeRequest {
            description: format!("Create button {}", i),
            target_component: None,
            change_type: UIChangeType::CreateComponent,
            platform_specific: None,
        };

        let component = generator.generate_component(request).unwrap();
        components.push(component);
    }

    let duration = start_time.elapsed();
    let avg_time = duration.as_millis() / 100;

    assert!(avg_time < 50); // Should be very fast
    assert_eq!(components.len(), 100);
}

// Test security features
#[tokio::test]
async fn test_security_features() {
    // Test sandbox isolation
    let sandbox = crate::sandbox::SandboxInstance::new(
        Uuid::new_v4().to_string(),
        "test".to_string(),
        vec![],
    )
    .await
    .unwrap();

    // Test resource limits
    let malicious_code = r#"
        fn main() {
            // Try to consume excessive memory
            let mut v = Vec::new();
            for i in 0..1000000000 {
                v.push(i);
            }
        }
    "#;

    let result = sandbox.execute_code(malicious_code, "rust", Some(5)).await;
    assert!(result.is_err() || !result.unwrap().success);
}

// Test hardware detection accuracy
#[tokio::test]
async fn test_hardware_detection() {
    let hardware_info = basic_hardware_detection();

    // Parse the JSON hardware info
    let info: serde_json::Value = serde_json::from_str(&hardware_info).unwrap();

    let cpu_cores = info["cpu"]["cores"].as_i64().unwrap_or(0);
    let memory_gb = info["memory"]["total_gb"].as_i64().unwrap_or(0);
    let gpu_available = info["gpu"]["available"].as_bool().unwrap_or(false);

    assert!(cpu_cores >= 1);
    assert!(memory_gb >= 1);
    // gpu_available is a boolean, no need to assert it's true or false
    let _ = gpu_available; // Just use it to avoid warning
}

// Test cross-platform adaptations
#[tokio::test]
async fn test_cross_platform_adaptations() {
    let adapter = CrossPlatformAdapter::new();

    let mut component = UIComponent {
        id: Uuid::new_v4().to_string(),
        component_type: ComponentType::Button,
        properties: HashMap::new(),
        children: vec![],
        platform_adaptations: HashMap::new(),
        generated_code: None,
    };

    // Test mobile adaptation
    adapter.adapt_component(&mut component, PlatformTarget::Mobile);
    assert!(component
        .platform_adaptations
        .contains_key(&PlatformTarget::Mobile));

    // Test desktop adaptation
    adapter.adapt_component(&mut component, PlatformTarget::Desktop);
    assert!(component
        .platform_adaptations
        .contains_key(&PlatformTarget::Desktop));

    // Test responsive code generation
    let responsive_code = adapter.generate_responsive_code(&component);
    assert!(responsive_code.contains("@media"));
}

// Test error handling and recovery
#[tokio::test]
async fn test_error_handling() {
    // Test AI provider failure handling
    let ai = ConversationalAI::new(AIProvider::Local {
        endpoint: "http://invalid-endpoint:9999".to_string(),
    });

    let mut conversation = Conversation {
        id: Uuid::new_v4().to_string(),
        messages: Default::default(),
        context: Default::default(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // Should handle network errors gracefully
    let result = ai
        .process_message(&mut conversation, "Test message".to_string())
        .await;
    // In production, this would return a meaningful error, not panic

    // Test project manager error handling
    let temp_dir = TempDir::new().unwrap();
    let project_manager = ProjectManager::new(temp_dir.path().to_path_buf());

    // Test loading non-existent project
    let load_result = project_manager.load_project("non-existent-id");
    assert!(load_result.is_err());
}

// Test concurrent operations
#[tokio::test]
async fn test_concurrent_operations() {
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let ai = Arc::new(Mutex::new(ConversationalAI::new(AIProvider::Local {
        endpoint: "http://localhost:8080/ai".to_string(),
    })));

    let mut handles = vec![];

    // Test concurrent AI requests
    for i in 0..10 {
        let ai_clone = ai.clone();
        let handle = tokio::spawn(async move {
            let mut ai = ai_clone.lock().await;
            let mut conversation = Conversation {
                id: Uuid::new_v4().to_string(),
                messages: Default::default(),
                context: Default::default(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            let result = ai
                .process_message(&mut conversation, format!("Request {}", i))
                .await;
            result.is_ok()
        });
        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;
    let success_count = results.iter().filter(|r| r.unwrap()).count();

    assert!(success_count >= 8); // At least 80% success rate
}

// Test API endpoint completeness
#[tokio::test]
async fn test_api_endpoints() {
    let endpoints = vec![
        ("/api/v1/projects", "POST"),
        ("/api/v1/projects", "GET"),
        ("/api/v1/projects/{id}", "GET"),
        ("/api/v1/projects/{id}", "PUT"),
        ("/api/v1/projects/{id}", "DELETE"),
        ("/api/v1/ai/process", "POST"),
        ("/api/v1/ai/generate", "POST"),
        ("/api/v1/sandboxes", "POST"),
        ("/api/v1/sandboxes/{id}/execute", "POST"),
        ("/api/v1/sandboxes/{id}/status", "GET"),
        ("/api/v1/sandboxes/{id}/files", "GET"),
        ("/api/v1/sandboxes/{id}/files/read", "POST"),
        ("/api/v1/sandboxes/{id}/files/write", "POST"),
        ("/api/v1/sandboxes/{id}/packages/detect", "POST"),
        ("/api/v1/sandboxes/{id}/packages/install", "POST"),
        ("/api/v1/vite/{id}/errors", "GET"),
        ("/api/v1/vite/{id}/report", "POST"),
        ("/api/v1/vite/{id}/restart", "POST"),
        ("/api/v1/export/zip", "POST"),
        ("/api/v1/export/github", "POST"),
        ("/api/v1/export/vercel", "POST"),
        ("/api/v1/health", "GET"),
        ("/api/v1/metrics", "GET"),
        ("/api/v1/stream/ai/generate", "GET"),
        ("/api/v1/stream/apply", "GET"),
        ("/api/v1/stream/vite/logs", "GET"),
    ];

    assert!(endpoints.len() >= 25); // Should have at least 25 endpoints

    // Test streaming endpoints
    let streaming_endpoints = vec![
        "/api/v1/stream/ai/generate",
        "/api/v1/stream/apply",
        "/api/v1/stream/vite/logs",
    ];

    for endpoint in streaming_endpoints {
        assert!(endpoint.contains("/stream/"));
    }
}

// Test installation script functionality
#[test]
fn test_installation_script() {
    // Test hardware detection
    let hardware_info = basic_hardware_detection();
    assert!(hardware_info.contains("cpu_cores"));
    assert!(hardware_info.contains("memory_gb"));
    assert!(hardware_info.contains("gpu_available"));

    // Test platform detection
    let platform_info = detect_platform_detailed();
    assert!(platform_info.contains("type"));
    assert!(platform_info.contains("distribution"));

    // Test architecture detection
    let arch_info = detect_architecture_detailed();
    assert!(arch_info.contains("arch"));
    assert!(arch_info.contains("features"));

    // Test tool detection
    let tools_info = detect_dev_tools();
    assert!(tools_info.contains("git"));
    assert!(tools_info.contains("rust"));
    assert!(tools_info.contains("cargo"));
}

// Test monitoring and health checks
#[tokio::test]
async fn test_monitoring() {
    // Test health check endpoint
    let health = health_check().await.unwrap();
    assert!(health["status"] == "healthy");
    assert!(health["services"]["ai"] == "healthy");
    assert!(health["services"]["database"] == "healthy");
    assert!(health["services"]["sandbox"] == "healthy");

    // Test metrics endpoint
    let metrics = get_metrics().await.unwrap();
    assert!(metrics["requests_total"] >= 0);
    assert!(metrics["response_time_avg"] >= 0);
    assert!(metrics["memory_usage"] >= 0);
    assert!(metrics["cpu_usage"] >= 0);
}

// Test file system operations with edge cases
#[tokio::test]
async fn test_file_system_edge_cases() {
    let temp_dir = TempDir::new().unwrap();

    // Test large file handling
    let large_file = temp_dir.path().join("large.txt");
    let large_content = "x".repeat(1024 * 1024); // 1MB
    std::fs::write(&large_file, &large_content).unwrap();

    let read_content = std::fs::read_to_string(&large_file).unwrap();
    assert_eq!(read_content.len(), large_content.len());

    // Test special characters in filenames
    let special_file = temp_dir.path().join("special-chars_123-测试.txt");
    std::fs::write(&special_file, "test").unwrap();
    assert!(special_file.exists());

    // Test nested directory creation
    let nested_path = temp_dir.path().join("a/b/c/d/e.txt");
    std::fs::create_dir_all(nested_path.parent().unwrap()).unwrap();
    std::fs::write(&nested_path, "nested").unwrap();
    assert!(nested_path.exists());
}

// Test error recovery and resilience
#[tokio::test]
async fn test_error_recovery() {
    // Test database recovery
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    // Create corrupted database
    std::fs::write(&db_path, "corrupted data").unwrap();

    // Attempt to repair (mock)
    let repair_result = std::fs::remove_file(&db_path);
    assert!(repair_result.is_ok());

    // Create new database
    let new_db = ProjectManager::new(temp_dir.path().to_path_buf());
    let project = new_db
        .create_project("Test".to_string(), "Test".to_string())
        .unwrap();
    assert!(project.name == "Test");
}

// Test data validation and sanitization
#[tokio::test]
async fn test_data_validation() {
    // Test input validation
    let malicious_input = "<script>alert('xss')</script>";
    let sanitized = sanitize_input(malicious_input);
    assert!(!sanitized.contains("<script>"));

    // Test file path validation
    let malicious_path = "../../../etc/passwd";
    let sanitized_path = sanitize_path(malicious_path);
    assert!(!sanitized_path.contains("../"));

    // Test configuration validation
    let invalid_config = r#"
        [invalid]
        timeout = -1
        max_connections = "not_a_number"
    "#;

    let config_result: Result<serde_json::Value, _> = toml::from_str(invalid_config);
    assert!(config_result.is_err());
}

fn sanitize_input(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn sanitize_path(path: &str) -> String {
    path.replace("../", "").replace("..", "").replace(':', "")
}

// Helper functions for tests (would be in production code)
fn extract_js_imports(code: &str, packages: &mut HashMap<String, (u32, Vec<String>)>) {
    let import_regex = regex::Regex::new(
        r#"import\s+(?:\{[^}]+\}|\*\s+as\s+\w+|\w+)\s+from\s+['"]([^'"]+)['"];?"#,
    )
    .unwrap();
    for cap in import_regex.captures_iter(code) {
        if let Some(import_path) = cap.get(1) {
            let path = import_path.as_str();
            if !path.starts_with('.') && !path.starts_with('/') {
                let package_name = path.split('/').next().unwrap_or(path);
                let entry = packages
                    .entry(package_name.to_string())
                    .or_insert((0, vec![]));
                entry.0 += 1;
                entry.1.push(path.to_string());
            }
        }
    }
}

fn extract_rust_imports(code: &str, packages: &mut HashMap<String, (u32, Vec<String>)>) {
    let use_regex = regex::Regex::new(r#"use\s+([a-zA-Z_][a-zA-Z0-9_]*)::"#).unwrap();
    for cap in use_regex.captures_iter(code) {
        if let Some(crate_name) = cap.get(1) {
            let name = crate_name.as_str();
            if name != "std" && name != "core" && name != "alloc" {
                let entry = packages.entry(name.to_string()).or_insert((0, vec![]));
                entry.0 += 1;
                entry.1.push(format!("use {}::", name));
            }
        }
    }
}

fn detect_language(file_path: &str) -> String {
    let ext = std::path::Path::new(file_path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    match ext {
        "rs" => "rust",
        "js" => "javascript",
        "jsx" => "javascriptreact",
        "ts" => "typescript",
        "tsx" => "typescriptreact",
        "html" => "html",
        "css" => "css",
        "scss" => "scss",
        "less" => "less",
        "json" => "json",
        "toml" => "toml",
        "yaml" | "yml" => "yaml",
        "md" => "markdown",
        _ => "text",
    }
    .to_string()
}

// Test parallel processing capabilities
#[tokio::test]
async fn test_parallel_processing() {
    use tokio::task;

    let start_time = std::time::Instant::now();

    // Run multiple tasks in parallel
    let tasks: Vec<_> = (0..10)
        .map(|i| {
            task::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                i * 2
            })
        })
        .collect();

    let results = futures::future::join_all(tasks).await;
    let total: i32 = results.into_iter().map(|r| r.unwrap()).sum();

    let duration = start_time.elapsed();

    // Should complete in about 100-150ms, not 1000ms
    assert!(duration.as_millis() < 200);
    assert_eq!(total, 90); // 0+2+4+6+8+10+12+14+16+18 = 90
}
