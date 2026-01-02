use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::AppState;
use crate::sandbox::{SandboxInstance, ExecutionResult};

// API Response Types
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
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

pub async fn create_project(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<Json<ApiResponse<ProjectResponse>>, StatusCode> {
    let mut app_state = state.lock().await;
    
    // Implementation would go here
    // For now, return a mock response
    let project = ProjectResponse {
        id: Uuid::new_v4().to_string(),
        name: request.name.clone(),
        description: request.description.unwrap_or_default(),
        platform: request.platform.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        pages: vec![],
    };
    
    Ok(Json(ApiResponse::success(project)))
}

pub async fn list_projects(
    State(state): State<Arc<Mutex<AppState>>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<ProjectResponse>>>, StatusCode> {
    // Implementation would go here
    Ok(Json(ApiResponse::success(vec![])))
}

pub async fn get_project(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(project_id): Path<String>,
) -> Result<Json<ApiResponse<ProjectResponse>>, StatusCode> {
    // Implementation would go here
    Err(StatusCode::NOT_FOUND)
}

pub async fn update_project(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(project_id): Path<String>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<ProjectResponse>>, StatusCode> {
    // Implementation would go here
    Err(StatusCode::NOT_FOUND)
}

pub async fn delete_project(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(project_id): Path<String>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Implementation would go here
    Ok(Json(ApiResponse::success(())))
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

pub async fn process_message(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<ProcessMessageRequest>,
) -> Result<Json<ApiResponse<ProcessMessageResponse>>, StatusCode> {
    // Implementation would go here
    let response = ProcessMessageResponse {
        response: "I'll create a modern landing page for you.".to_string(),
        ui_changes: vec![],
        confidence: 0.95,
    };
    
    Ok(Json(ApiResponse::success(response)))
}

#[derive(Debug, Deserialize)]
pub struct GenerateCodeRequest {
    pub project_id: String,
    pub component_id: String,
    pub platform: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateCodeResponse {
    pub code: String,
    pub language: String,
    pub platform: String,
    pub dependencies: Vec<String>,
}

pub async fn generate_code(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<GenerateCodeRequest>,
) -> Result<Json<ApiResponse<GenerateCodeResponse>>, StatusCode> {
    let response = GenerateCodeResponse {
        code: "// Generated code would go here".to_string(),
        language: "rust".to_string(),
        platform: request.platform.clone(),
        dependencies: vec!["dioxus".to_string()],
    };
    
    Ok(Json(ApiResponse::success(response)))
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

pub async fn create_sandbox(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<CreateSandboxRequest>,
) -> Result<Json<ApiResponse<CreateSandboxResponse>>, StatusCode> {
    let mut app_state = state.lock().await;
    let sandbox_id = Uuid::new_v4().to_string();
    
    // Create sandbox instance (simplified)
    let response = CreateSandboxResponse {
        sandbox_id: sandbox_id.clone(),
        status: "created".to_string(),
        endpoint: Some(format!("http://localhost:8080/sandbox/{}", sandbox_id)),
        created_at: chrono::Utc::now(),
    };
    
    Ok(Json(ApiResponse::success(response)))
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

pub async fn execute_code(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
    Json(request): Json<ExecuteCodeRequest>,
) -> Result<Json<ApiResponse<ExecuteCodeResponse>>, StatusCode> {
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
    
    Ok(Json(ApiResponse::success(response)))
}

pub async fn get_sandbox_status(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let status = serde_json::json!({
        "sandbox_id": sandbox_id,
        "status": "ready",
        "uptime": 3600,
        "memory_usage": 104857600,
        "cpu_usage": 15.5,
        "active_connections": 1,
    });
    
    Ok(Json(ApiResponse::success(status)))
}

pub async fn get_sandbox_logs(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<String>>>, StatusCode> {
    let logs = vec![
        "Sandbox created".to_string(),
        "Requirements installed".to_string(),
        "Ready for execution".to_string(),
    ];
    
    Ok(Json(ApiResponse::success(logs)))
}

pub async fn kill_sandbox(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Implementation would go here
    Ok(Json(ApiResponse::success(())))
}

// Component Management Endpoints
#[derive(Debug, Deserialize)]
pub struct CreateComponentRequest {
    pub component_type: String,
    pub properties: serde_json::Value,
    pub parent_id: Option<String>,
}

pub async fn create_component(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(project_id): Path<String>,
    Json(request): Json<CreateComponentRequest>,
) -> Result<Json<ApiResponse<ComponentResponse>>, StatusCode> {
    let component = ComponentResponse {
        id: Uuid::new_v4().to_string(),
        component_type: request.component_type.clone(),
        properties: request.properties.clone(),
        children: vec![],
    };
    
    Ok(Json(ApiResponse::success(component)))
}

pub async fn list_components(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(project_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<ComponentResponse>>>, StatusCode> {
    Ok(Json(ApiResponse::success(vec![])))
}

pub async fn update_component(
    State(state): State<Arc<Mutex<AppState>>>,
    Path((project_id, component_id)): Path<(String, String)>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<ComponentResponse>>, StatusCode> {
    Err(StatusCode::NOT_FOUND)
}

pub async fn delete_component(
    State(state): State<Arc<Mutex<AppState>>>,
    Path((project_id, component_id)): Path<(String, String)>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    Ok(Json(ApiResponse::success(())))
}

// Utility Endpoints
pub async fn health_check() -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
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
    
    Ok(Json(ApiResponse::success(health)))
}

pub async fn get_metrics() -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
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
    
    Ok(Json(ApiResponse::success(metrics)))
}

// Error handlers
pub async fn handle_404() -> Json<ApiError> {
    Json(ApiError {
        code: "NOT_FOUND".to_string(),
        message: "Resource not found".to_string(),
        details: None,
    })
}

pub async fn handle_500() -> Json<ApiError> {
    Json(ApiError {
        code: "INTERNAL_ERROR".to_string(),
        message: "Internal server error".to_string(),
        details: None,
    })
}