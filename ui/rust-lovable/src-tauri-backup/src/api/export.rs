use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::process::Command;
use uuid::Uuid;
use zip::write::FileOptions;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateZipRequest {
    pub project_id: String,
    pub include_node_modules: Option<bool>,
    pub include_build: Option<bool>,
    pub format: String, // zip, tar, tar.gz
}

#[derive(Debug, Serialize)]
pub struct CreateZipResponse {
    pub download_id: String,
    pub download_url: String,
    pub file_size: u64,
    pub file_count: u32,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_zip(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<CreateZipRequest>,
) -> Result<Json<CreateZipResponse>, StatusCode> {
    let download_id = Uuid::new_v4().to_string();
    let download_path = format!("/tmp/rust-lovable-downloads/{}.zip", download_id);
    let project_path = format!("/tmp/rust-lovable-projects/{}", request.project_id);
    
    // Create download directory
    fs::create_dir_all(Path::new(&download_path).parent().unwrap())
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Create ZIP file
    let file = fs::File::create(&download_path)
        .map_err(|e| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);
    
    // Add project files to ZIP
    let mut file_count = 0;
    let mut total_size = 0;
    
    if let Ok(entries) = fs::read_dir(&project_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let relative_path = path.strip_prefix(&project_path)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            
            // Skip node_modules and build directories if requested
            if !request.include_node_modules.unwrap_or(false) && relative_path.contains("node_modules") {
                continue;
            }
            if !request.include_build.unwrap_or(false) && relative_path.contains("dist") {
                continue;
            }
            
            if path.is_file() {
                let metadata = entry.metadata()
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                zip.start_file(relative_path, options)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                let content = fs::read(&path)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                zip.write_all(&content)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                
                file_count += 1;
                total_size += metadata.len();
            }
        }
    }
    
    zip.finish()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(CreateZipResponse {
        download_id: download_id.clone(),
        download_url: format!("/api/v1/download/{}", download_id),
        file_size: total_size,
        file_count,
        expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
    }))
}

pub async fn download_zip(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(download_id): Path<String>,
) -> Result<axum::response::Response, StatusCode> {
    let download_path = format!("/tmp/rust-lovable-downloads/{}.zip", download_id);
    
    if !Path::new(&download_path).exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    let content = fs::read(&download_path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let response = axum::response::Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/zip")
        .header("Content-Disposition", format!("attachment; filename=\"project-{}.zip\"", download_id))
        .body(axum::body::Body::from(content))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Clean up file after download
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
        let _ = fs::remove_file(&download_path);
    });
    
    Ok(response)
}

#[derive(Debug, Deserialize)]
pub struct ExportToGitHubRequest {
    pub project_id: String,
    pub repo_name: String,
    pub github_token: String,
    pub is_private: Option<bool>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExportToGitHubResponse {
    pub success: bool,
    pub repo_url: String,
    commit_sha: String,
    pub files_pushed: u32,
    pub branch: String,
}

pub async fn export_to_github(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<ExportToGitHubRequest>,
) -> Result<Json<ExportToGitHubResponse>, StatusCode> {
    // Mock implementation - in production, use GitHub API
    Ok(Json(ExportToGitHubResponse {
        success: true,
        repo_url: format!("https://github.com/user/{}", request.repo_name),
        commit_sha: "abc123def456".to_string(),
        files_pushed: 25,
        branch: "main".to_string(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct DeployToVercelRequest {
    pub project_id: String,
    pub vercel_token: String,
    pub project_name: Option<String>,
    pub framework: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DeployToVercelResponse {
    pub success: bool,
    pub deployment_url: String,
    pub deployment_id: String,
    pub build_logs_url: String,
    pub status: String,
}

pub async fn deploy_to_vercel(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<DeployToVercelRequest>,
) -> Result<Json<DeployToVercelResponse>, StatusCode> {
    // Mock implementation - in production, use Vercel API
    Ok(Json(DeployToVercelResponse {
        success: true,
        deployment_url: "https://rust-lovable-project-abc123.vercel.app".to_string(),
        deployment_id: "dpl_abc123def456".to_string(),
        build_logs_url: "https://vercel.com/user/project/abc123".to_string(),
        status: "ready".to_string(),
    }))
}

#[derive(Debug, Serialize)]
pub struct ProjectStatsResponse {
    pub project_id: String,
    pub total_components: u32,
    pub total_files: u32,
    pub total_size: u64,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub build_status: String,
    pub deployments: Vec<DeploymentInfo>,
}

#[derive(Debug, Serialize)]
pub struct DeploymentInfo {
    pub id: String,
    pub platform: String,
    pub url: String,
    pub status: String,
    pub deployed_at: chrono::DateTime<chrono::Utc>,
}

pub async fn get_project_stats(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(project_id): Path<String>,
) -> Result<Json<ProjectStatsResponse>, StatusCode> {
    Ok(Json(ProjectStatsResponse {
        project_id: project_id.clone(),
        total_components: 12,
        total_files: 25,
        total_size: 524288,
        last_modified: chrono::Utc::now(),
        build_status: "success".to_string(),
        deployments: vec![
            DeploymentInfo {
                id: "dpl_1".to_string(),
                platform: "vercel".to_string(),
                url: "https://rust-lovable-project.vercel.app".to_string(),
                status: "ready".to_string(),
                deployed_at: chrono::Utc::now(),
            },
        ],
    }))
}