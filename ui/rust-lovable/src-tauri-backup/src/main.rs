#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;

mod api;
mod sandbox;
mod updater;
mod system_info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    let app_state = Arc::new(Mutex::new(AppState::new()));
    
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // System info commands
            system_info::get_system_info,
            system_info::get_platform_info,
            system_info::check_hardware_requirements,
            
            // Core API commands
            api::create_project,
            api::process_message,
            api::create_sandbox,
            api::execute_code,
            api::get_sandbox_status,
            api::create_component,
            api::health_check,
            api::get_metrics,
            
            // Streaming commands
            api::streaming::generate_ai_code_stream,
            api::streaming::apply_ai_code_stream,
            api::streaming::monitor_vite_logs,
            
            // Package management
            api::packages::detect_and_install_packages,
            api::packages::install_packages_v2,
            api::packages::extract_brand_styles,
            
            // Vite integration
            api::vite::check_vite_errors,
            api::vite::report_vite_error,
            api::vite::clear_vite_errors_cache,
            api::vite::restart_vite,
            api::vite::get_conversation_state,
            api::vite::analyze_edit_intent,
            
            // File system
            api::files::get_sandbox_files,
            api::files::read_file,
            api::files::write_file,
            api::files::delete_file,
            api::files::create_directory,
            api::files::search_files,
            api::files::get_file_stats,
            
            // Export and deployment
            api::export::create_zip,
            api::export::export_to_github,
            api::export::deploy_to_vercel,
            api::export::get_project_stats,
            
            // Updater commands
            updater::check_for_updates,
            updater::install_update,
            
            // Sandbox management
            sandbox::create_isolated_environment,
            sandbox::execute_in_sandbox,
            sandbox::get_sandbox_logs,
            sandbox::monitor_sandbox,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    Ok(())
}

pub struct AppState {
    pub sandboxes: std::collections::HashMap<String, sandbox::SandboxInstance>,
    pub config: Config,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            sandboxes: std::collections::HashMap::new(),
            config: Config::default(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub ai_provider: String,
    pub api_keys: std::collections::HashMap<String, String>,
    pub platform_target: String,
    pub auto_update: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ai_provider: "openai".to_string(),
            api_keys: std::collections::HashMap::new(),
            platform_target: "universal".to_string(),
            auto_update: true,
        }
    }
}