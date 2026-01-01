use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::process::Command;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct DetectPackagesRequest {
    pub code: String,
    pub file_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DetectPackagesResponse {
    pub detected_packages: Vec<DetectedPackage>,
    pub package_manager: String,
    pub confidence: f32,
}

#[derive(Debug, Serialize)]
pub struct DetectedPackage {
    pub name: String,
    pub package_type: String, // dependency or devDependency
    pub usage_count: u32,
    pub import_examples: Vec<String>,
    pub confidence: f32,
}

pub async fn detect_and_install_packages(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
    Json(request): Json<DetectPackagesRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Detect packages from code
    let detected = detect_packages_from_code(&request.code, request.file_path.as_deref())?;
    
    // Install detected packages
    let mut installed = vec![];
    let mut failed = vec![];
    
    for package in &detected.detected_packages {
        match install_package(&sandbox_id, &package.name, &detected.package_manager).await {
            Ok(_) => installed.push(package.name.clone()),
            Err(e) => failed.push((package.name.clone(), e.to_string())),
        }
    }
    
    Ok(Json(serde_json::json!({
        "success": failed.is_empty(),
        "detected": detected.detected_packages.len(),
        "installed": installed.len(),
        "failed": failed.len(),
        "installed_packages": installed,
        "failed_packages": failed,
        "package_manager": detected.package_manager
    })))
}

fn detect_packages_from_code(code: &str, file_path: Option<&str>) -> Result<DetectPackagesResponse, String> {
    let mut packages = HashMap::new();
    let file_ext = file_path
        .and_then(|p| p.split('.').last())
        .unwrap_or("js");
    
    // Extract imports/requires from JavaScript/TypeScript
    if file_ext == "js" || file_ext == "jsx" || file_ext == "ts" || file_ext == "tsx" {
        extract_js_imports(code, &mut packages);
    }
    
    // Extract imports from Rust
    if file_ext == "rs" {
        extract_rust_imports(code, &mut packages);
    }
    
    // Convert to response format
    let detected_packages: Vec<DetectedPackage> = packages
        .into_iter()
        .map(|(name, (usage_count, import_examples))| {
            DetectedPackage {
                name,
                package_type: "dependency".to_string(),
                usage_count,
                import_examples,
                confidence: 0.9, // Simplified confidence calculation
            }
        })
        .collect();
    
    Ok(DetectPackagesResponse {
        detected_packages,
        package_manager: detect_package_manager(file_path),
        confidence: 0.85,
    })
}

fn extract_js_imports(code: &str, packages: &mut HashMap<String, (u32, Vec<String>)>) {
    // Extract ES6 imports
    let import_regex = regex::Regex::new(r#"import\s+(?:\{[^}]+\}|\*\s+as\s+\w+|\w+)\s+from\s+['"]([^'"]+)['"];?"#).unwrap();
    for cap in import_regex.captures_iter(code) {
        if let Some(import_path) = cap.get(1) {
            let path = import_path.as_str();
            if !path.starts_with('.') && !path.starts_with('/') {
                let package_name = path.split('/').next().unwrap_or(path);
                let entry = packages.entry(package_name.to_string()).or_insert((0, vec![]));
                entry.0 += 1;
                entry.1.push(path.to_string());
            }
        }
    }
    
    // Extract CommonJS requires
    let require_regex = regex::Regex::new(r#"require\s*\(\s*['"]([^'"]+)['"]\s*\);?"#).unwrap();
    for cap in require_regex.captures_iter(code) {
        if let Some(require_path) = cap.get(1) {
            let path = require_path.as_str();
            if !path.starts_with('.') && !path.starts_with('/') {
                let package_name = path.split('/').next().unwrap_or(path);
                let entry = packages.entry(package_name.to_string()).or_insert((0, vec![]));
                entry.0 += 1;
                entry.1.push(path.to_string());
            }
        }
    }
}

fn extract_rust_imports(code: &str, packages: &mut HashMap<String, (u32, Vec<String>)>) {
    // Extract Rust use statements
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

fn detect_package_manager(file_path: Option<&str>) -> String {
    // Simple detection - in production, check for lock files
    if let Some(path) = file_path {
        let path = Path::new(path);
        let parent = path.parent().unwrap_or(Path::new("."));
        
        if parent.join("package-lock.json").exists() {
            return "npm".to_string();
        }
        if parent.join("yarn.lock").exists() {
            return "yarn".to_string();
        }
        if parent.join("pnpm-lock.yaml").exists() {
            return "pnpm".to_string();
        }
        if parent.join("bun.lock").exists() {
            return "bun".to_string();
        }
        if parent.join("Cargo.lock").exists() {
            return "cargo".to_string();
        }
    }
    
    "npm".to_string() // Default fallback
}

async fn install_package(
    sandbox_id: &str,
    package_name: &str,
    package_manager: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let (cmd, args) = match package_manager {
        "npm" => ("npm", vec!["install", package_name]),
        "yarn" => ("yarn", vec!["add", package_name]),
        "pnpm" => ("pnpm", vec!["add", package_name]),
        "bun" => ("bun", vec!["add", package_name]),
        "cargo" => ("cargo", vec!["add", package_name]),
        _ => return Err(format!("Unsupported package manager: {}", package_manager).into()),
    };
    
    let output = Command::new(cmd)
        .args(&args)
        .output()
        .await?;
    
    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Package installation failed: {}", String::from_utf8_lossy(&output.stderr)).into())
    }
}

#[derive(Debug, Deserialize)]
pub struct InstallPackagesRequest {
    pub packages: Vec<String>,
    pub package_manager: String,
    pub install_dev: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct InstallPackagesResponse {
    pub success: bool,
    pub installed: Vec<String>,
    pub failed: Vec<(String, String)>,
    pub logs: Vec<String>,
}

pub async fn install_packages_v2(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
    Json(request): Json<InstallPackagesRequest>,
) -> Result<Json<InstallPackagesResponse>, StatusCode> {
    let mut installed = vec![];
    let mut failed = vec![];
    let mut logs = vec![];
    
    for package in &request.packages {
        match install_package(&sandbox_id, package, &request.package_manager).await {
            Ok(_) => {
                installed.push(package.clone());
                logs.push(format!("✓ Installed {}", package));
            }
            Err(e) => {
                failed.push((package.clone(), e.to_string()));
                logs.push(format!("✗ Failed to install {}: {}", package, e));
            }
        }
    }
    
    Ok(Json(InstallPackagesResponse {
        success: failed.is_empty(),
        installed,
        failed,
        logs,
    }))
}

#[derive(Debug, Deserialize)]
pub struct ExtractBrandStylesRequest {
    pub url: String,
    pub include_colors: Option<bool>,
    pub include_fonts: Option<bool>,
    pub include_spacing: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ExtractBrandStylesResponse {
    pub colors: Vec<ColorDefinition>,
    pub fonts: Vec<FontDefinition>,
    pub spacing: SpacingDefinition,
    pub confidence: f32,
}

#[derive(Debug, Serialize)]
pub struct ColorDefinition {
    pub hex: String,
    pub rgb: (u8, u8, u8),
    pub usage_frequency: f32,
    pub contexts: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FontDefinition {
    pub family: String,
    pub sizes: Vec<f32>,
    pub weights: Vec<String>,
    pub usage_frequency: f32,
}

#[derive(Debug, Serialize)]
pub struct SpacingDefinition {
    pub base_unit: f32,
    pub scale: Vec<f32>,
    pub common_values: Vec<f32>,
}

pub async fn extract_brand_styles(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<ExtractBrandStylesRequest>,
) -> Result<Json<ExtractBrandStylesResponse>, StatusCode> {
    // Mock implementation - in production, scrape and analyze the website
    let response = ExtractBrandStylesResponse {
        colors: vec![
            ColorDefinition {
                hex: "#3B82F6".to_string(),
                rgb: (59, 130, 246),
                usage_frequency: 0.8,
                contexts: vec!["primary", "buttons", "links"],
            },
            ColorDefinition {
                hex: "#1F2937".to_string(),
                rgb: (31, 41, 55),
                usage_frequency: 0.6,
                contexts: vec!["text", "headers"],
            },
        ],
        fonts: vec![
            FontDefinition {
                family: "Inter".to_string(),
                sizes: vec![14.0, 16.0, 18.0, 24.0, 32.0],
                weights: vec!["400", "500", "600", "700"],
                usage_frequency: 0.9,
            },
        ],
        spacing: SpacingDefinition {
            base_unit: 4.0,
            scale: vec![0.25, 0.5, 1.0, 1.5, 2.0, 3.0, 4.0, 6.0],
            common_values: vec![4.0, 8.0, 16.0, 24.0, 32.0],
        },
        confidence: 0.85,
    };
    
    Ok(Json(response))
}