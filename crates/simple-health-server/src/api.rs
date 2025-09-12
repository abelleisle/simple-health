use crate::ServerState;
use crate::auth::authenticate::login;
use axum::{
    Router,
    extract::State,
    response::Json,
    routing::{get, post},
};

pub fn get_routes() -> Router<ServerState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/login", post(login))
}

async fn health_check(State(state): State<ServerState>) -> Json<serde_json::Value> {
    let db = &state.db;

    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "database": db.is_connected()
    }))
}
