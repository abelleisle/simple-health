mod api;
mod auth;
mod base;
mod core;
mod crypto;
mod db;
mod serve;
mod session;
mod utils;

use axum::{Router, middleware};
use std::sync::{Arc, RwLock};
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;

use core::types::{User, UserSetting};

#[derive(Clone)]
pub struct UserContext {
    user: Option<User>,
    is_admin: bool,
    error: Option<String>,
    settings: UserSetting,
}

#[derive(Clone)]
pub struct ServerState {
    pub db: db::DatabaseConnection,
    pub config: Arc<RwLock<ServerConfig>>,
}

#[derive(Clone)]
pub struct ServerConfig {
    pub signup_allowed: bool,
    pub signup_disable_after_create: bool,
}

impl ServerState {
    fn is_signup_allowed(&self) -> bool {
        match self.config.read() {
            Ok(config) => config.signup_allowed,
            Err(e) => {
                log::error!("Unable to read config from state: {}", e);
                false
            }
        }
    }

    fn should_disable_signup(&self) -> bool {
        match self.config.read() {
            Ok(config) => config.signup_disable_after_create,
            Err(e) => {
                log::error!("Unable to read config from state: {}", e);
                false
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    std_logger::Config::logfmt().init();

    if utils::dev::is_debug_version() {
        log::warn!("You're running a debug version, performance may be degraded :(");
    }

    log::info!("Starting simple-health server");

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

    let config = ServerConfig {
        signup_allowed: true,
        signup_disable_after_create: true,
    };

    let state = ServerState {
        db,
        config: Arc::new(RwLock::new(config)),
    };

    if state.should_disable_signup() {
        let should_disable = match User::count(&state.db.get_pool()).await {
            Ok(c) => {
                log::debug!("{} users in the database, should disable if > 0", c);
                c > 0
            }
            Err(e) => {
                log::error!(
                    "Unable to fetch user count, disabling signup for safety. Err: {}",
                    e
                );
                true
            }
        };

        state.config.write().unwrap().signup_allowed = !should_disable;
    }

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
    let app = Router::new()
        .merge(serve::get_routes(state.clone()))
        .nest("/api/v1", api::get_routes(state.clone()))
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
