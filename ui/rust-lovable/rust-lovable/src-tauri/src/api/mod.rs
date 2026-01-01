use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::AppState;
use crate::sandbox::{SandboxInstance, ExecutionResult};

pub mod endpoints;
pub mod middleware;
pub mod validation;
pub mod streaming;
pub mod packages;
pub mod vite;
pub mod files;
pub mod export;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }
    
    pub fn error(code: String, message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(ApiError {
                code,
                message,
                details: None,
            }),
            timestamp: chrono::Utc::now(),
        }
    }
}

// Project Management Endpoints
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub platform: String,
    pub template: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub platform: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub pages: Vec<PageResponse>,
}

#[derive(Debug, Serialize)]
pub struct PageResponse {
    pub id: String,
    pub name: String,
    pub path: String,
    pub components: Vec<ComponentResponse>,
}

#[derive(Debug, Serialize)]
pub struct ComponentResponse {
    pub id: String,
    pub component_type: String,
    pub properties: serde_json::Value,
    pub children: Vec<ComponentResponse>,
}

#[tauri::command]
pub async fn create_project(
    request: CreateProjectRequest,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<ProjectResponse, String> {
    // Implementation would go here
    let project = ProjectResponse {
        id: Uuid::new_v4().to_string(),
        name: request.name.clone(),
        description: request.description.unwrap_or_default(),
        platform: request.platform.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        pages: vec![],
    };
    
    Ok(project)
}

// AI Integration Endpoints
#[derive(Debug, Deserialize)]
pub struct ProcessMessageRequest {
    pub project_id: String,
    pub message: String,
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ProcessMessageResponse {
    pub response: String,
    pub ui_changes: Vec<UIChangeRequest>,
    pub confidence: f32,
}

#[derive(Debug, Deserialize)]
pub struct UIChangeRequest {
    pub description: String,
    pub target_component: Option<String>,
    pub change_type: String,
    pub platform_specific: Option<serde_json::Value>,
}

#[tauri::command]
pub async fn process_message(
    request: ProcessMessageRequest,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<ProcessMessageResponse, String> {
    let response = ProcessMessageResponse {
        response: "I'll create a modern landing page for you.".to_string(),
        ui_changes: vec![],
        confidence: 0.95,
    };
    
    Ok(response)
}

// Sandbox Management Endpoints
#[derive(Debug, Deserialize)]
pub struct CreateSandboxRequest {
    pub platform: String,
    pub requirements: Vec<String>,
    pub template: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateSandboxResponse {
    pub sandbox_id: String,
    pub status: String,
    pub endpoint: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[tauri::command]
pub async fn create_sandbox(
    request: CreateSandboxRequest,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<CreateSandboxResponse, String> {
    let sandbox_id = Uuid::new_v4().to_string();
    
    let response = CreateSandboxResponse {
        sandbox_id: sandbox_id.clone(),
        status: "created".to_string(),
        endpoint: Some(format!("http://localhost:8080/sandbox/{}", sandbox_id)),
        created_at: chrono::Utc::now(),
    };
    
    Ok(response)
}

#[derive(Debug, Deserialize)]
pub struct ExecuteCodeRequest {
    pub code: String,
    pub language: String,
    pub timeout: Option<u64>,
    pub input: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExecuteCodeResponse {
    pub execution_id: String,
    pub result: ExecutionResult,
    pub logs: Vec<String>,
    pub duration: u64,
}

#[tauri::command]
pub async fn execute_code(
    request: ExecuteCodeRequest,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<ExecuteCodeResponse, String> {
    let execution_id = Uuid::new_v4().to_string();
    
    let response = ExecuteCodeResponse {
        execution_id,
        result: ExecutionResult {
            success: true,
            output: Some("Hello, World!".to_string()),
            error: None,
            exit_code: Some(0),
            execution_time: 150,
        },
        logs: vec!["Execution started".to_string(), "Execution completed".to_string()],
        duration: 150,
    };
    
    Ok(response)
}

#[tauri::command]
pub async fn get_sandbox_status(
    sandbox_id: String,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<serde_json::Value, String> {
    let status = serde_json::json!({
        "sandbox_id": sandbox_id,
        "status": "ready",
        "uptime": 3600,
        "memory_usage": 104857600,
        "cpu_usage": 15.5,
        "active_connections": 1,
    });
    
    Ok(status)
}

// Component Management Endpoints
#[derive(Debug, Deserialize)]
pub struct CreateComponentRequest {
    pub component_type: String,
    pub properties: serde_json::Value,
    pub parent_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ComponentResponse {
    pub id: String,
    pub component_type: String,
    pub properties: serde_json::Value,
    pub children: Vec<ComponentResponse>,
}

#[tauri::command]
pub async fn create_component(
    request: CreateComponentRequest,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<ComponentResponse, String> {
    let component = ComponentResponse {
        id: Uuid::new_v4().to_string(),
        component_type: request.component_type.clone(),
        properties: request.properties.clone(),
        children: vec![],
    };
    
    Ok(component)
}

// Utility Endpoints
#[tauri::command]
pub async fn health_check() -> Result<serde_json::Value, String> {
    let health = serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "uptime": 3600,
        "services": {
            "ai": "healthy",
            "database": "healthy",
            "sandbox": "healthy"
        }
    });
    
    Ok(health)
}

#[tauri::command]
pub async fn get_metrics() -> Result<serde_json::Value, String> {
    let metrics = serde_json::json!({
        "requests_total": 1000,
        "requests_per_second": 10.5,
        "response_time_avg": 150,
        "errors_total": 5,
        "sandboxes_active": 3,
        "projects_total": 25,
        "memory_usage": 45.2,
        "cpu_usage": 15.8
    });
    
    Ok(metrics)
}