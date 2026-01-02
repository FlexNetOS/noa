use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::process::Command;

use crate::AppState;

// Import missing types from other modules
use crate::api::endpoints::{AnalyzeEditIntentResponse, EditIntentRequest};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ViteError {
    pub id: String,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub severity: String, // error, warning
    pub stack_trace: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub resolved: bool,
}

#[derive(Debug, Serialize)]
pub struct CheckViteErrorsResponse {
    pub has_errors: bool,
    pub error_count: u32,
    pub errors: Vec<ViteError>,
    pub last_check: chrono::DateTime<chrono::Utc>,
}

pub async fn check_vite_errors(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
) -> Result<Json<CheckViteErrorsResponse>, StatusCode> {
    // Mock implementation - in production, check Vite build output
    let errors = vec![
        ViteError {
            id: "err_001".to_string(),
            message: "Module not found: Can't resolve 'react'".to_string(),
            file: Some("/src/App.tsx".to_string()),
            line: Some(3),
            column: Some(8),
            severity: "error".to_string(),
            stack_trace: Some("Error: Cannot find module 'react'\n    at ...".to_string()),
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
    
    Ok(Json(CheckViteErrorsResponse {
        has_errors: true,
        error_count: errors.len() as u32,
        errors,
        last_check: chrono::Utc::now(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct ReportViteErrorRequest {
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub severity: String,
    pub stack_trace: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReportViteErrorResponse {
    pub error_id: String,
    pub status: String,
    pub suggestions: Vec<String>,
    pub auto_fix_available: bool,
}

pub async fn report_vite_error(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
    Json(request): Json<ReportViteErrorRequest>,
) -> Result<Json<ReportViteErrorResponse>, StatusCode> {
    let error_id = uuid::Uuid::new_v4().to_string();
    
    // Analyze error and provide suggestions
    let suggestions = analyze_error(&request.message);
    let auto_fix_available = check_auto_fix(&request.message);
    
    Ok(Json(ReportViteErrorResponse {
        error_id: error_id.clone(),
        status: "reported".to_string(),
        suggestions,
        auto_fix_available,
    }))
}

fn analyze_error(message: &str) -> Vec<String> {
    let mut suggestions = vec![];
    
    if message.contains("Module not found") {
        suggestions.push("Try running 'npm install' to install missing dependencies".to_string());
        suggestions.push("Check if the package name is spelled correctly".to_string());
        suggestions.push("Verify the package is in your package.json".to_string());
    }
    
    if message.contains("Cannot find module") {
        suggestions.push("Install the missing package: npm install <package-name>".to_string());
        suggestions.push("Clear node_modules and reinstall: rm -rf node_modules && npm install".to_string());
    }
    
    if message.contains("SyntaxError") {
        suggestions.push("Check for missing semicolons or brackets".to_string());
        suggestions.push("Verify your TypeScript/JavaScript syntax".to_string());
    }
    
    if message.contains("TypeError") {
        suggestions.push("Check if you're calling a function that doesn't exist".to_string());
        suggestions.push("Verify the variable you're using is defined".to_string());
    }
    
    suggestions
}

fn check_auto_fix(message: &str) -> bool {
    message.contains("Module not found") || 
    message.contains("Cannot find module") ||
    message.contains("missing dependency")
}

#[derive(Debug, Serialize)]
pub struct ClearViteErrorsCacheResponse {
    pub cleared_count: u32,
    pub active_errors: u32,
    pub cache_size_before: u32,
    pub cache_size_after: u32,
}

pub async fn clear_vite_errors_cache(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
) -> Result<Json<ClearViteErrorsCacheResponse>, StatusCode> {
    // Mock implementation - clear error cache
    Ok(Json(ClearViteErrorsCacheResponse {
        cleared_count: 5,
        active_errors: 0,
        cache_size_before: 1024,
        cache_size_after: 0,
    }))
}

pub async fn restart_vite(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // In production, restart the Vite development server
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Vite development server restarted",
        "pid": 12345,
        "port": 5173,
        "status": "running"
    })))
}

#[derive(Debug, Serialize)]
pub struct ConversationStateResponse {
    pub conversation_id: String,
    pub project_id: String,
    pub messages: Vec<Message>,
    pub context: serde_json::Value,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub id: String,
    pub role: String, // user, assistant, system
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: Option<serde_json::Value>,
}

pub async fn get_conversation_state(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(conversation_id): Path<String>,
) -> Result<Json<ConversationStateResponse>, StatusCode> {
    Ok(Json(ConversationStateResponse {
        conversation_id: conversation_id.clone(),
        project_id: "project_123".to_string(),
        messages: vec![
            Message {
                id: "msg_1".to_string(),
                role: "user".to_string(),
                content: "Create a modern landing page".to_string(),
                timestamp: chrono::Utc::now(),
                metadata: None,
            },
            Message {
                id: "msg_2".to_string(),
                role: "assistant".to_string(),
                content: "I'll create a modern landing page with a hero section and feature cards.".to_string(),
                timestamp: chrono::Utc::now(),
                metadata: None,
            },
        ],
        context: serde_json::json!({
            "current_page": "landing",
            "selected_component": "hero_section",
            "platform": "web"
        }),
        last_activity: chrono::Utc::now(),
    }))
}

#[derive(Debug, Serialize)]
pub struct AnalyzeEditIntentResponse {
    pub intent: String,
    pub confidence: f32,
    pub suggested_actions: Vec<String>,
    pub target_component: Option<String>,
    pub requires_backend_change: bool,
}

pub async fn analyze_edit_intent(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(project_id): Path<String>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<AnalyzeEditIntentResponse>, StatusCode> {
    let intent_request: EditIntentRequest = serde_json::from_value(request)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Analyze the user's edit intent
    let (intent, confidence) = classify_intent(&intent_request.message);
    let suggested_actions = generate_suggested_actions(&intent);
    let target_component = extract_target_component(&intent_request.message);
    let requires_backend_change = check_backend_requirement(&intent);
    
    Ok(Json(AnalyzeEditIntentResponse {
        intent,
        confidence,
        suggested_actions,
        target_component,
        requires_backend_change,
    }))
}

#[derive(Debug, Deserialize)]
struct EditIntentRequest {
    message: String,
    context: Option<serde_json::Value>,
}

fn classify_intent(message: &str) -> (String, f32) {
    let lower_msg = message.to_lowercase();
    
    if lower_msg.contains("create") || lower_msg.contains("add") {
        ("create_component".to_string(), 0.95)
    } else if lower_msg.contains("change") || lower_msg.contains("modify") {
        ("modify_component".to_string(), 0.90)
    } else if lower_msg.contains("delete") || lower_msg.contains("remove") {
        ("delete_component".to_string(), 0.95)
    } else if lower_msg.contains("style") || lower_msg.contains("color") {
        ("modify_styling".to_string(), 0.85)
    } else if lower_msg.contains("layout") || lower_msg.contains("position") {
        ("modify_layout".to_string(), 0.80)
    } else {
        ("unknown".to_string(), 0.5)
    }
}

fn generate_suggested_actions(intent: &str) -> Vec<String> {
    match intent {
        "create_component" => vec![
            "Create the new component".to_string(),
            "Add it to the current page".to_string(),
            "Apply default styling".to_string(),
        ],
        "modify_component" => vec![
            "Find the target component".to_string(),
            "Apply the requested changes".to_string(),
            "Update related components if needed".to_string(),
        ],
        "modify_styling" => vec![
            "Update component CSS properties".to_string(),
            "Apply responsive adaptations".to_string(),
            "Preview changes immediately".to_string(),
        ],
        _ => vec!["Analyze requirements".to_string(), "Apply changes".to_string()],
    }
}

fn extract_target_component(message: &str) -> Option<String> {
    // Simple extraction - look for "the button", "this form", etc.
    if message.contains("button") {
        Some("button".to_string())
    } else if message.contains("form") {
        Some("form".to_string())
    } else if message.contains("header") {
        Some("header".to_string())
    } else {
        None
    }
}

fn check_backend_requirement(intent: &str) -> bool {
    matches!(intent, "create_component" | "delete_component" | "modify_layout")
}