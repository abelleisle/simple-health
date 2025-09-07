mod api;
mod core;
mod db;
mod utils;

use axum::Router;
use tower_http::{cors::CorsLayer, services::ServeDir};

#[derive(Clone)]
pub struct ServerState {
    pub db: db::DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    std_logger::Config::logfmt().init();

    let db = db::DatabaseConnection::connect().await.map_err(|e| {
        log::error!("Failed to connect to database: {}", e);
        e
    })?;

    db.run_migrations().await.map_err(|e| {
        log::error!("Database migrations failed :( Reason: {e}");
        e
    })?;

    db.show_debug_stats().await.map_err(|e| {
        log::warn!("Failed to print database stats. Reason: {e}");
        e
    })?;

    let state = ServerState { db };
    let app = create_app(state);

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
        log::warn!("Failed to bind backend address: {e}");
        e
    })?;

    log::info!("Server running on {addr}");
    axum::serve(listener, app).await.map_err(|e| {
        log::warn!("Failed to start axum backend server: {e}");
        e
    })?;

    Ok(())
}

fn create_app(state: ServerState) -> Router {
    let mut app = Router::new()
        .nest("/api/v1", api::get_routes())
        .with_state(state)
        .layer(CorsLayer::permissive());

    if utils::dev::is_built_version() {
        if let Some(static_dir) = utils::get_static_dir() {
            app = app.fallback_service(ServeDir::new(static_dir));
        } else {
            panic!("Unable to find required static content directory!");
        }
    } else {
        app = app.fallback(utils::dev::proxy_to_frontend)
    }

    app
}
