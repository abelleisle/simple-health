pub mod dev;

use std::path::PathBuf;

pub const IS_DEBUG: bool = cfg!(debug_assertions);

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
