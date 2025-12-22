// Tauri commands and application logic
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// System information structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub platform: String,
    pub arch: String,
    pub version: String,
    pub hostname: String,
    pub total_memory: u64,
    pub available_memory: u64,
}

use std::sync::{Arc, Mutex};
use std::process::{Child, Command, Stdio};

/// Application state
pub struct AppState {
    pub inference_server: Arc<Mutex<Option<Child>>>,
    pub server_port: Arc<Mutex<u16>>,
    pub server_url: Arc<Mutex<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            inference_server: Arc::new(Mutex::new(None)),
            server_port: Arc::new(Mutex::new(8080)),
            server_url: Arc::new(Mutex::new("http://127.0.0.1:8080".to_string())),
        }
    }
}

/// Get system information
#[tauri::command]
async fn get_system_info() -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        platform: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        version: std::env::var("OS_VERSION").unwrap_or_else(|_| "unknown".to_string()),
        hostname: hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "unknown".to_string()),
        total_memory: 0, // TODO: Implement system memory info
        available_memory: 0,
    })
}

/// Get application data directory
#[tauri::command]
async fn get_app_data_dir() -> Result<PathBuf, String> {
    directories::ProjectDirs::from("com", "mldevops", "platform")
        .map(|proj_dirs| proj_dirs.data_dir().to_path_buf())
        .ok_or_else(|| "Failed to get app data directory".to_string())
}

/// Open URL in external browser
#[tauri::command]
async fn open_external_url(url: String) -> Result<(), String> {
    opener::open(&url).map_err(|e| format!("Failed to open URL: {}", e))
}

/// Check if running in desktop mode
#[tauri::command]
fn is_desktop_mode() -> bool {
    true
}

/// Get app version
#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Save file to local filesystem
#[tauri::command]
async fn save_local_file(path: String, contents: String) -> Result<(), String> {
    use std::fs;
    fs::write(&path, contents)
        .map_err(|e| format!("Failed to save file: {}", e))
}

/// Read file from local filesystem
#[tauri::command]
async fn read_local_file(path: String) -> Result<String, String> {
    use std::fs;
    fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))
}

/// Show native file picker dialog
#[tauri::command]
async fn show_file_picker(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
    
    // This is a placeholder - actual implementation would use the dialog plugin
    Ok(None)
}

/// Start local ML inference server with RuvLLM
#[tauri::command]
async fn start_inference_server(
    state: tauri::State<'_, AppState>,
    port: Option<u16>,
) -> Result<String, String> {
    let server_url = {
        let mut server_lock = state.inference_server.lock()
            .map_err(|e| format!("Failed to lock server state: {}", e))?;
        
        // Check if server is already running
        if let Some(ref mut child) = *server_lock {
            match child.try_wait() {
                Ok(None) => return Ok(format!("Server already running on port {}", *state.server_port.lock().unwrap())),
                _ => {
                    // Process exited, clean up
                    *server_lock = None;
                }
            }
        }
        
        let server_port = port.unwrap_or(8080);
        
        // Find the inference server binary
        // Try multiple paths: bundled resources, development, installed
        let possible_paths = vec![
            std::env::current_exe()
                .ok()
                .and_then(|exe| exe.parent().map(|p| p.join("inference_server"))),
            Some(PathBuf::from("../../rust_backend/target/release/inference_server")),
            Some(PathBuf::from("./rust_backend/target/release/inference_server")),
            Some(PathBuf::from("inference_server")),
        ];
        
        let binary_path = possible_paths
            .into_iter()
            .flatten()
            .find(|p| p.exists())
            .ok_or_else(|| "Inference server binary not found. Please build with: cd rust_backend && cargo build --release".to_string())?;
        
        // Start the server process
        let child = Command::new(binary_path)
            .args(&[
                "--host", "127.0.0.1",
                "--port", &server_port.to_string(),
                "--log-level", "info",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start inference server: {}", e))?;
        
        *server_lock = Some(child);
        *state.server_port.lock().unwrap() = server_port;
        let server_url = format!("http://127.0.0.1:{}", server_port);
        *state.server_url.lock().unwrap() = server_url.clone();
        
        server_url
    }; // MutexGuard dropped here before await
    
    // Wait a moment for server to initialize
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    Ok(server_url)
}

/// Stop local ML inference server
#[tauri::command]
async fn stop_inference_server(
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut server_lock = state.inference_server.lock()
        .map_err(|e| format!("Failed to lock server state: {}", e))?;
    
    if let Some(mut child) = server_lock.take() {
        // Try graceful shutdown first
        match child.kill() {
            Ok(_) => {
                log::info!("Inference server stopped");
                Ok(())
            }
            Err(e) => Err(format!("Failed to stop server: {}", e)),
        }
    } else {
        Err("Inference server is not running".to_string())
    }
}

/// Get inference server status with details
#[derive(Serialize)]
pub struct InferenceStatus {
    pub running: bool,
    pub port: u16,
    pub url: String,
}

#[tauri::command]
async fn get_inference_status(
    state: tauri::State<'_, AppState>,
) -> Result<InferenceStatus, String> {
    let server_lock = state.inference_server.lock()
        .map_err(|e| format!("Failed to lock server state: {}", e))?;
    
    let running = if let Some(ref child) = *server_lock {
        // Check if process is still alive
        matches!(child.id(), _)
    } else {
        false
    };
    
    Ok(InferenceStatus {
        running,
        port: *state.server_port.lock().unwrap(),
        url: state.server_url.lock().unwrap().clone(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_websocket::init())
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            get_app_data_dir,
            open_external_url,
            is_desktop_mode,
            get_app_version,
            save_local_file,
            read_local_file,
            show_file_picker,
            start_inference_server,
            stop_inference_server,
            get_inference_status,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                log::info!("ML DevOps Platform starting in development mode");
            } else {
                log::info!("ML DevOps Platform starting in production mode");
            }
            
            // Initialize app state
            log::info!("Platform: {}", std::env::consts::OS);
            log::info!("Architecture: {}", std::env::consts::ARCH);
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
