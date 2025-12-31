use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Create build directories
    let out_dir = env::var("OUT_DIR").unwrap();
    let build_dir = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

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
    let rust_version = option_env!("RUSTC_VERSION").unwrap_or("unknown");
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

    let build_info = format!(
        r#"pub const BUILD_INFO: &str = "Rust Lovable v{} - Built on {}";
pub const BUILD_TIME: &str = "{}";
pub const RUST_VERSION: &str = "{}";
"#,
        env!("CARGO_PKG_VERSION"),
        now,
        now,
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
            copy_dir_recursive(src_path.to_str().unwrap(), dst_path)?;
        } else {
            fs::copy(src_path, dst_path)?;
        }
    }

    Ok(())
}
