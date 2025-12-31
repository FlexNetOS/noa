use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::AppState;

#[derive(Debug, Serialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    pub permissions: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetSandboxFilesResponse {
    pub sandbox_id: String,
    pub current_path: String,
    pub files: Vec<FileEntry>,
    pub total_size: u64,
    pub file_count: u32,
}

pub async fn get_sandbox_files(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
) -> Result<Json<GetSandboxFilesResponse>, StatusCode> {
    // Mock implementation - in production, read from actual sandbox directory
    let files = vec![
        FileEntry {
            name: "src".to_string(),
            path: "/src".to_string(),
            is_directory: true,
            size: None,
            modified_at: Some(chrono::Utc::now()),
            permissions: Some("755".to_string()),
        },
        FileEntry {
            name: "package.json".to_string(),
            path: "/package.json".to_string(),
            is_directory: false,
            size: Some(2048),
            modified_at: Some(chrono::Utc::now()),
            permissions: Some("644".to_string()),
        },
        FileEntry {
            name: "README.md".to_string(),
            path: "/README.md".to_string(),
            is_directory: false,
            size: Some(1024),
            modified_at: Some(chrono::Utc::now()),
            permissions: Some("644".to_string()),
        },
        FileEntry {
            name: "tailwind.config.js".to_string(),
            path: "/tailwind.config.js".to_string(),
            is_directory: false,
            size: Some(512),
            modified_at: Some(chrono::Utc::now()),
            permissions: Some("644".to_string()),
        },
    ];
    
    Ok(Json(GetSandboxFilesResponse {
        sandbox_id: sandbox_id.clone(),
        current_path: "/".to_string(),
        files,
        total_size: 3584,
        file_count: 3,
    }))
}

#[derive(Debug, Deserialize)]
pub struct ReadFileRequest {
    pub file_path: String,
}

#[derive(Debug, Serialize)]
pub struct ReadFileResponse {
    pub file_path: String,
    pub content: String,
    pub size: u64,
    pub language: String,
    pub line_count: u32,
}

pub async fn read_file(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
    Json(request): Json<ReadFileRequest>,
) -> Result<Json<ReadFileResponse>, StatusCode> {
    // Mock implementation - in production, read from sandbox filesystem
    let content = match request.file_path.as_str() {
        "/package.json" => r#"{
  "name": "rust-lovable-project",
  "version": "1.0.0",
  "dependencies": {
    "dioxus": "^0.6.0",
    "tailwind": "^3.0.0"
  }
}"#,
        "/README.md" => "# Rust Lovable Project\n\nGenerated with Rust Lovable",
        "/tailwind.config.js" => "module.exports = {\n  content: ['./src/**/*.{html,js,ts,jsx,tsx}'],\n  theme: {\n    extend: {}\n  }\n}",
        _ => "// File content would be read here",
    };
    
    let size = content.len() as u64;
    let line_count = content.lines().count() as u32;
    let language = detect_language(&request.file_path);
    
    Ok(Json(ReadFileResponse {
        file_path: request.file_path.clone(),
        content: content.to_string(),
        size,
        language,
        line_count,
    }))
}

#[derive(Debug, Deserialize)]
pub struct WriteFileRequest {
    pub file_path: String,
    pub content: String,
    pub create_if_not_exists: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct WriteFileResponse {
    pub success: bool,
    pub file_path: String,
    pub size: u64,
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

pub async fn write_file(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
    Json(request): Json<WriteFileRequest>,
) -> Result<Json<WriteFileResponse>, StatusCode> {
    // Mock implementation - in production, write to sandbox filesystem
    let size = request.content.len() as u64;
    
    Ok(Json(WriteFileResponse {
        success: true,
        file_path: request.file_path.clone(),
        size,
        modified_at: chrono::Utc::now(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct DeleteFileRequest {
    pub file_path: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteFileResponse {
    pub success: bool,
    pub file_path: String,
    pub message: String,
}

pub async fn delete_file(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
    Json(request): Json<DeleteFileRequest>,
) -> Result<Json<DeleteFileResponse>, StatusCode> {
    Ok(Json(DeleteFileResponse {
        success: true,
        file_path: request.file_path.clone(),
        message: "File deleted successfully".to_string(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct CreateDirectoryRequest {
    pub directory_path: String,
    pub recursive: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CreateDirectoryResponse {
    pub success: bool,
    pub directory_path: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_directory(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
    Json(request): Json<CreateDirectoryRequest>,
) -> Result<Json<CreateDirectoryResponse>, StatusCode> {
    Ok(Json(CreateDirectoryResponse {
        success: true,
        directory_path: request.directory_path.clone(),
        created_at: chrono::Utc::now(),
    }))
}

fn detect_language(file_path: &str) -> String {
    let ext = Path::new(file_path)
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

#[derive(Debug, Serialize)]
pub struct FileSearchResponse {
    pub query: String,
    pub results: Vec<FileSearchResult>,
    pub total_results: u32,
    pub search_time_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct FileSearchResult {
    pub file_path: String,
    pub line_number: u32,
    pub column_number: u32,
    pub context: String,
    pub preview: String,
}

pub async fn search_files(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<FileSearchResponse>, StatusCode> {
    let query = request["query"].as_str().unwrap_or("");
    
    // Mock search results
    let results = vec![
        FileSearchResult {
            file_path: "/src/components/Button.tsx".to_string(),
            line_number: 15,
            column_number: 8,
            context: "className=\"btn-primary\"".to_string(),
            preview: "... className=\"btn-primary\" ...".to_string(),
        },
        FileSearchResult {
            file_path: "/src/styles/components.css".to_string(),
            line_number: 42,
            column_number: 3,
            context: ".btn-primary {".to_string(),
            preview: ".btn-primary {\n  background-color: #3B82F6;".to_string(),
        },
    ];
    
    Ok(Json(FileSearchResponse {
        query: query.to_string(),
        results,
        total_results: results.len() as u32,
        search_time_ms: 150,
    }))
}

#[derive(Debug, Serialize)]
pub struct FileStatsResponse {
    pub total_files: u32,
    pub total_size: u64,
    pub file_type_breakdown: HashMap<String, u32>,
    pub largest_files: Vec<FileSizeInfo>,
    pub recently_modified: Vec<FileEntry>,
}

#[derive(Debug, Serialize)]
pub struct FileSizeInfo {
    pub file_path: String,
    pub size: u64,
    pub percentage_of_total: f32,
}

pub async fn get_file_stats(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
) -> Result<Json<FileStatsResponse>, StatusCode> {
    let mut file_types = HashMap::new();
    file_types.insert("tsx".to_string(), 12);
    file_types.insert("ts".to_string(), 8);
    file_types.insert("css".to_string(), 5);
    file_types.insert("json".to_string(), 3);
    
    let largest_files = vec![
        FileSizeInfo {
            file_path: "/node_modules/lodash/lodash.js".to_string(),
            size: 531416,
            percentage_of_total: 45.2,
        },
        FileSizeInfo {
            file_path: "/dist/assets/index.js".to_string(),
            size: 245760,
            percentage_of_total: 20.9,
        },
    ];
    
    Ok(Json(FileStatsResponse {
        total_files: 156,
        total_size: 1175552,
        file_type_breakdown: file_types,
        largest_files,
        recently_modified: vec![],
    }))
}