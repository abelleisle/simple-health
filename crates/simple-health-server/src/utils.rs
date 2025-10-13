pub mod dev;

use std::path::PathBuf;

#[allow(dead_code)]
pub fn get_static_dir() -> Option<PathBuf> {
    if dev::is_built_version() {
        let exe_dir = std::env::current_exe().ok()?.parent()?.to_path_buf();
        let static_dir = exe_dir.join("static");
        if static_dir.exists() {
            Some(static_dir)
        } else {
            None
        }
    } else {
        None
    }
}

/// Get the templates directory path
/// In development: returns "frontend/web/templates"
/// In production: returns "../share/simple-health/templates" relative to the executable
pub fn get_templates_dir() -> PathBuf {
    if dev::is_built_version() {
        // Production: binary is at $out/bin/simple-health-server
        // Templates are at $out/share/simple-health/templates
        let exe_path = std::env::current_exe().expect("Failed to get executable path");
        let exe_dir = exe_path
            .parent()
            .expect("Failed to get executable directory");
        exe_dir.join("../share/simple-health/templates")
    } else {
        // Development: use relative path from project root
        PathBuf::from("frontend/web/templates")
    }
}

/// Get the static files directory path
/// In development: returns "frontend/web/static"
/// In production: returns "../share/simple-health/static" relative to the executable
pub fn get_static_files_dir() -> PathBuf {
    if dev::is_built_version() {
        // Production: binary is at $out/bin/simple-health-server
        // Static files are at $out/share/simple-health/static
        let exe_path = std::env::current_exe().expect("Failed to get executable path");
        let exe_dir = exe_path
            .parent()
            .expect("Failed to get executable directory");
        exe_dir.join("../share/simple-health/static")
    } else {
        // Development: use relative path from project root
        PathBuf::from("frontend/web/static")
    }
}
