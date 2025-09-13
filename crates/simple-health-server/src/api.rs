use crate::auth::{
    authenticate::{login, refresh_token},
    required_auth,
};
use crate::{ServerState, UserContext};
use axum::{
    Extension, Router,
    extract::{Request, State},
    http::StatusCode,
    middleware,
    middleware::Next,
    response::{IntoResponse, Json, Redirect},
    routing::{get, post},
};

pub fn get_routes(state: ServerState) -> Router<ServerState> {
    Router::new()
        .route("/health", get(health_check))
        .layer(middleware::from_fn_with_state(state, required_auth_api))
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

/// middleware that requires the user to be authenticated
pub async fn required_auth_api(
    Extension(context): Extension<UserContext>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    if context.user_id.is_none() {
        log::warn!("Unauthorized access to api!");
        return StatusCode::UNAUTHORIZED.into_response();
    }

    next.run(request).await
}
