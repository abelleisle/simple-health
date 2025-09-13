pub mod authenticate;
pub mod cookie;
pub mod jwt;
pub mod middleware;

use crate::UserContext;

use axum::{
    Extension,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect},
};

// You'll need to implement this function using a password hashing library
pub fn verify_password(password: &str, hash: &str) -> bool {
    // Example with bcrypt (you'll need to add bcrypt to Cargo.toml):
    // bcrypt::verify(password, hash).unwrap_or(false)

    // Placeholder - replace with actual password verification
    password == hash
}

/// middleware that requires the user to be authenticated
pub async fn required_auth(
    Extension(context): Extension<UserContext>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    if context.user.is_none() {
        return Redirect::to("/login").into_response();
    }

    next.run(request).await
}

/// middleware that requires the user to be authenticated
pub async fn required_admin(
    Extension(context): Extension<UserContext>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    if !context.is_admin {
        return Redirect::to("/").into_response();
    }

    next.run(request).await
}
