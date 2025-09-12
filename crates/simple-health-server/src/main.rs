mod api;
mod auth;
mod base;
mod core;
mod db;
mod serve;
mod session;
mod utils;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use tower_http::cors::Any;
use tower_http::{cors::CorsLayer, services::ServeDir};
use uuid::Uuid;

use core::types::{Signup, User};

#[derive(Clone)]
pub struct UserContext {
    user_id: Option<Uuid>,
    is_admin: bool,
    error: Option<String>,
}

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

    let signup = Signup {
        email: "user@example.com".to_string(),
        password_hash: "12345".to_string(),
        name: "Test User".to_string(),
    };

    let user = create_test_user(db.get_pool(), &signup).await?;

    log::info!("User uuid: {}", user.id);

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
        .merge(serve::get_routes(state.clone()))
        .nest("/api/v1", api::get_routes())
        .with_state(state.clone())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(middleware::from_fn_with_state(state.clone(), base::base));

    // if utils::dev::is_built_version() {
    //     if let Some(static_dir) = utils::get_static_dir() {
    //         app = app.fallback_service(ServeDir::new(static_dir));
    //     } else {
    //         panic!("Unable to find required static content directory!");
    //     }
    // } else {
    //     app = app.fallback(utils::dev::proxy_to_frontend)
    // }

    app
}

async fn create_test_user(
    pool: &db::DBPool,
    signup: &Signup,
) -> Result<User, Box<dyn std::error::Error + Sync + Send>> {
    let get_user = User::get(pool, None, Some(&signup.email)).await?;
    match get_user {
        Some(u) => {
            log::debug!("Got existing user {}", u.id);
            Ok(u)
        }
        None => {
            log::debug!("Creating new user with email {}", signup.email);
            Ok(User::new(pool, signup).await?)
        }
    }
}
