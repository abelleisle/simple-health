mod api;
mod utils;

use axum::Router;
use tower_http::{cors::CorsLayer, services::ServeDir};

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

fn create_app() -> Router {
    let mut app = Router::new()
        .nest("/api/v1", api::get_routes())
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
