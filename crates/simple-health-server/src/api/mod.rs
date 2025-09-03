use crate::db;

use crate::ServerState;
use axum::{Router, extract::State, response::Json, routing::get};

pub fn get_routes() -> Router<ServerState> {
    Router::new().route("/health", get(health_check))
}

async fn health_check(State(state): State<ServerState>) -> Json<serde_json::Value> {
    let db: db::Database = state.db;

    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "database": !db.client.is_closed()
    }))
}
