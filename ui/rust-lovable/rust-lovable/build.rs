use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Create build directories
    let out_dir = env::var("OUT_DIR").unwrap();
    let build_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();
    
    // Create dist directory for web builds
    let dist_dir = build_dir.join("dist");
    fs::create_dir_all(&dist_dir).unwrap();
    
    // Copy assets to dist
    if Path::new("assets").exists() {
        copy_dir_recursive("assets", dist_dir.join("assets")).unwrap();
    }
    
    // Copy templates to dist
    if Path::new("templates").exists() {
        copy_dir_recursive("templates", dist_dir.join("templates")).unwrap();
    }
    
    // Generate build info
    let build_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    
    let rust_version = env::var("RUSTC_WRAPPER")
        .unwrap_or_else(|_| "stable".to_string());
    
    let build_info = format!(
        r#"pub const BUILD_INFO: &str = "Rust Lovable v{} - Built at epoch {}";
pub const BUILD_TIME: &str = "{}";
pub const RUST_VERSION: &str = "{}";
"#,
        env!("CARGO_PKG_VERSION"),
        build_time,
        build_time,
        rust_version
    );
    
    fs::write(dist_dir.join("build_info.rs"), build_info).unwrap();
    
    // Set build flags
    println!("cargo:rerun-if-changed=assets/");
    println!("cargo:rerun-if-changed=templates/");
    println!("cargo:rerun-if-changed=src/");
}

fn copy_dir_recursive(src: &str, dst: std::path::PathBuf) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_recursive(
                src_path.to_str().unwrap(),
                dst_path
            )?;
        } else {
            fs::copy(src_path, dst_path)?;
        }
    }
    
    Ok(())
}