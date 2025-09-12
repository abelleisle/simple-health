use crate::ServerState;
use crate::auth::authenticate::{login, refresh_token};
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
        .route("/refresh_token", get(refresh_token))
}

async fn health_check(State(state): State<ServerState>) -> Json<serde_json::Value> {
    let db = &state.db;

    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "database": db.is_connected()
    }))
}
