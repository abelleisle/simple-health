use axum::{
    Router, body::Body, extract::Request, response::Json, response::Response, routing::get,
};
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
    } else {
        app = app.fallback(proxy_to_frontend);
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

async fn proxy_to_frontend(req: Request<Body>) -> Response<Body> {
    let uri = req.uri();
    let frontend_url = format!(
        "http://localhost:5173{}",
        uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("/")
    );

    let client = reqwest::Client::new();
    match client.get(&frontend_url).send().await {
        Ok(resp) => {
            let status = resp.status();
            let headers = resp.headers().clone();
            let body = resp.bytes().await.unwrap_or_default();

            let mut response = Response::builder().status(status);
            for (key, value) in headers.iter() {
                response = response.header(key, value);
            }
            response.body(Body::from(body)).unwrap()
        }
        Err(_) => Response::builder()
            .status(502)
            .body(Body::from("Frontend dev server not available"))
            .unwrap(),
    }
}
