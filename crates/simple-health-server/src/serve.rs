use axum::{
    Router,
    extract::{Extension, State},
    http::StatusCode,
    response::{Html, Response},
    routing::get,
};
use chrono::Local;
use std::collections::HashMap;
use tera::{Context, Tera};
use tower_http::services::ServeDir;

use crate::ServerState;

pub fn get_routes() -> Router<ServerState> {
    let mut tera = Tera::new("frontend/web/templates/**/*").expect("Failed to initialize Tera");

    tera.autoescape_on(vec!["html", "htm"]);

    Router::new()
        .route("/", get(dashboard))
        .route("/login", get(login))
        .nest_service("/static/css", ServeDir::new("frontend/web/static/css"))
        .nest_service("/static/js", ServeDir::new("frontend/web/static/js"))
        .nest_service(
            "/static/assets",
            ServeDir::new("frontend/web/static/assets"),
        )
        .layer(Extension(tera))
}

async fn dashboard(
    State(state): State<ServerState>,
    Extension(tera): Extension<Tera>,
) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();

    // Add dummy user data
    let mut user = HashMap::new();
    user.insert("name", "Test User");
    user.insert("calorie_goal", "2000");
    context.insert("user", &user);

    // Add dummy stats using serde_json for proper serialization
    context.insert(
        "stats",
        &serde_json::json!({
            "total_calories": 1247,
            "remaining_calories": 753,
            "progress_percentage": 62,
            "meal_breakdown": {
                "breakfast": 350,
                "lunch": 480,
                "dinner": 320,
                "snack": 97
            },
            "entries": [
                {
                    "name": "Oatmeal with banana",
                    "type": "breakfast",
                    "time": "8:30 AM",
                    "calories": "350"
                },
                {
                    "name": "Chicken salad",
                    "type": "lunch",
                    "time": "12:45 PM",
                    "calories": "480"
                }
            ]
        }),
    );

    // Add current date
    let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    context.insert("selected_date", &current_date);

    // Add health status with database connection check
    let db_connected = state.db.is_connected();
    context.insert(
        "health",
        &serde_json::json!({
            "backend_healthy": true,
            "database_connected": db_connected,
            "message": if db_connected { "Backend Healthy" } else { "Backend Issues" }
        }),
    );

    let rendered = tera.render("dashboard.html.tera", &context).map_err(|e| {
        log::error!("Error rendering dashboard: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(rendered))
}

async fn login(
    State(_state): State<ServerState>,
    Extension(tera): Extension<Tera>,
) -> Result<Html<String>, StatusCode> {
    let context = Context::new();

    // You can add error handling and username persistence here
    // context.insert("error", "Invalid credentials");
    // context.insert("username", "user@example.com");

    let rendered = tera.render("login.html.tera", &context).map_err(|e| {
        log::error!("Error rendering login: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(rendered))
}
