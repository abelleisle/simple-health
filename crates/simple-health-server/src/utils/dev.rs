#[allow(dead_code)]
pub fn is_built_version() -> bool {
    std::env::var("CARGO_MANIFEST_DIR").is_err()
}

pub fn is_debug_version() -> bool {
    cfg!(debug_assertions)
}
