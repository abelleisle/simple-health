use axum::{
    Router,
    extract::{Extension, State},
    http::StatusCode,
    response::{Html, Response},
    routing::get,
};
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
    State(_state): State<ServerState>,
    Extension(tera): Extension<Tera>,
) -> Result<Html<String>, StatusCode> {
    let context = Context::new();

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
