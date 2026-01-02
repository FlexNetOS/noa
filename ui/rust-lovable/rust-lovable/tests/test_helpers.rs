// Test helper functions for Rust Lovable tests

use regex::Regex;
use std::collections::HashMap;

// JavaScript import extraction (moved from packages.rs for testing)
pub fn extract_js_imports(code: &str, packages: &mut HashMap<String, (u32, Vec<String>)>) {
    // Extract ES6 imports
    let import_regex =
        Regex::new(r#"import\s+(?:\{[^}]+\}|\*\s+as\s+\w+|\w+)\s+from\s+['"]([^'"]+)['"];?"#)
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

    // Extract CommonJS requires
    let require_regex = Regex::new(r#"require\s*\(\s*['"]([^'"]+)['"]\s*\);?"#).unwrap();
    for cap in require_regex.captures_iter(code) {
        if let Some(require_path) = cap.get(1) {
            let path = require_path.as_str();
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

// Rust import extraction (moved from packages.rs for testing)
pub fn extract_rust_imports(code: &str, packages: &mut HashMap<String, (u32, Vec<String>)>) {
    // Extract Rust use statements
    let use_regex = Regex::new(r#"use\s+([a-zA-Z_][a-zA-Z0-9_]*)::"#).unwrap();
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

// Vite error analysis (moved from vite.rs for testing)
pub fn analyze_error(message: &str) -> Vec<String> {
    let mut suggestions = vec![];

    if message.contains("Module not found") {
        suggestions.push("Try running 'npm install' to install missing dependencies".to_string());
        suggestions.push("Check if the package name is spelled correctly".to_string());
        suggestions.push("Verify the package is in your package.json".to_string());
    }

    if message.contains("Cannot find module") {
        suggestions.push("Install the missing package: npm install <package-name>".to_string());
        suggestions.push(
            "Clear node_modules and reinstall: rm -rf node_modules && npm install".to_string(),
        );
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

// Auto-fix detection (moved from vite.rs for testing)
pub fn check_auto_fix(message: &str) -> bool {
    message.contains("Module not found")
        || message.contains("Cannot find module")
        || message.contains("missing dependency")
}

// Language detection (moved from files.rs for testing)
pub fn detect_language(file_path: &str) -> String {
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

// Basic hardware detection for testing
pub fn basic_hardware_detection() -> String {
    let cpu_cores = 4; // Mock value
    let memory_gb = 8; // Mock value
    let gpu_available = false; // Mock value
    let disk_usage = 45; // Mock value

    format!(
        r#"{{
          "cpu": {{"cores": {}}},
          "memory": {{"gb": {}}},
          "gpu": {{"available": {}}},
          "storage": {{"usage_percent": {}}}
        }}"#,
        cpu_cores, memory_gb, gpu_available, disk_usage
    )
}

// Intent classification for testing
pub fn classify_intent(message: &str) -> (String, f32) {
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

// Generate suggested actions for testing
pub fn generate_suggested_actions(intent: &str) -> Vec<String> {
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
        _ => vec![
            "Analyze requirements".to_string(),
            "Apply changes".to_string(),
        ],
    }
}
