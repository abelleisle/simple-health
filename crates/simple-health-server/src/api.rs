use crate::auth::{
    authenticate::{login, refresh_token, signup},
    required_auth,
};
use crate::core::types::{Activity, Meal};
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
        .route("/meal", post(meal))
        .route("/activity", post(activity))
        .layer(middleware::from_fn_with_state(state, required_auth_api))
        .route("/health", get(health_check))
        .route("/login", post(login))
        .route("/signup", post(signup))
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
    if context.user.is_none() {
        log::warn!("Unauthorized access to api!");
        return StatusCode::UNAUTHORIZED.into_response();
    }

    next.run(request).await
}

pub async fn meal(
    State(app): State<ServerState>,
    Extension(ctx): Extension<UserContext>,
    Json(mut meal): Json<Meal>,
) -> impl IntoResponse {
    if ctx.user.is_none() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    meal.user_id = ctx.user.unwrap().id;
    log::info!("Meal: {:?}", meal);

    let _ = meal.insert(app.db.get_pool()).await.map_err(|e| {
        log::error!("Unable to insert meal {:?} into db: {}", meal, e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    });

    StatusCode::OK
}

pub async fn activity(
    State(app): State<ServerState>,
    Extension(ctx): Extension<UserContext>,
    Json(mut activity): Json<Activity>,
) -> impl IntoResponse {
    if ctx.user.is_none() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    activity.user_id = ctx.user.unwrap().id;
    log::info!("Activity: {:?}", activity);

    let _ = activity.insert(app.db.get_pool()).await.map_err(|e| {
        log::error!("Unable to insert activity {:?} into db: {}", activity, e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    });

    StatusCode::OK
}
