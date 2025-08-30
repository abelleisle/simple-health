use axum::{Router, response::Json, routing::get};
use std::path::PathBuf;
use tower_http::{cors::CorsLayer, services::ServeDir};

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

fn create_app() -> Router {
    let api_routes = Router::new().route("/health", get(health_check));

    let mut app = Router::new()
        .nest("/api/v1", api_routes)
        .layer(CorsLayer::permissive());

    if let Some(static_dir) = get_static_dir() {
        app = app.fallback_service(ServeDir::new(static_dir));
    }

    app
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

fn is_built_version() -> bool {
    std::env::var("CARGO_MANIFEST_DIR").is_err()
}

fn get_static_dir() -> Option<PathBuf> {
    if is_built_version() {
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
