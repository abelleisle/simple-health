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

/*
/// middleware that requires the user to be an admin
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
*/
