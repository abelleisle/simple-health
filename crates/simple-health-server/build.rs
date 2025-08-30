use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let frontend_path = "../../frontend/web";
    let dist_path = format!("{}/dist", frontend_path);

    if Path::new(frontend_path).exists() {
        println!("cargo:rerun-if-changed=../../frontend/web/src");
        println!("cargo:rerun-if-changed=../../frontend/web/package.json");
        println!("cargo:rerun-if-changed=../../frontend/web/vite.config.ts");

        println!("Building frontend...");
        let output = Command::new("bun")
            .args(&["run", "build"])
            .current_dir(frontend_path)
            .output()
            .expect("Failed to execute bun build");

        if !output.status.success() {
            panic!(
                "Frontend build failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        if Path::new(&dist_path).exists() {
            let target_dir =
                std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "../../target".to_string());
            let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
            let static_dir = format!("{}/{}/static", target_dir, profile);

            if Path::new(&static_dir).exists() {
                fs::remove_dir_all(&static_dir).ok();
            }

            copy_dir(&dist_path, &static_dir).expect("Failed to copy dist to static");
            println!("Frontend assets copied to {}", static_dir);
        }
    }
}

fn copy_dir(src: &str, dst: &str) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = Path::new(dst).join(entry.file_name());

        if path.is_dir() {
            copy_dir(&path.to_string_lossy(), &dest_path.to_string_lossy())?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}
