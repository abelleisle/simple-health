use crate::auth::{
    authenticate::JWT_SIGNING_KEY,
    cookie::default_cookie,
    jwt::{Claims, generate_jwt, validate_jwt},
};
use crate::session::RefreshToken;
use crate::{ServerState, UserContext};
use axum::middleware::Next;
use axum::{
    Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use axum::{body::Body, extract::Request, response::Response};
use axum_extra::extract::cookie::CookieJar;

pub async fn base(
    State(app): State<ServerState>,
    mut request: Request,
    next: Next,
) -> impl IntoResponse {
    let mut jar = CookieJar::from_headers(request.headers());
    let jwt = jar.get("jwt");
    let refresh = jar.get("refresh");

    // Default context for unauthenticated requests
    let mut context = UserContext {
        user_id: None,
        is_admin: false,
    };

    // JWT takes precedence if present
    if let Some(jwt) = jwt {
        match validate_jwt::<Claims>(JWT_SIGNING_KEY, jwt.value()) {
            Ok(claims) => {
                context.user_id = Some(claims.user_id);
            }
            Err(_) => {
                // Clear potentially compromised cookies
                jar = jar.remove("jwt").remove("refresh");
            }
        }
    }
    // Fall back to refresh token if JWT is absent/invalid
    else if let Some(refresh) = refresh {
        if let Ok(Some(user)) =
            RefreshToken::get_user_from_token(app.db.get_pool(), refresh.value()).await
        {
            // if let Ok(Some(user)) = db::refresh_tokens::get_user(&app.pg_pool, refresh.value()).await {
            let claims = Claims::with(&user);
            if let Ok(jwt) = generate_jwt(JWT_SIGNING_KEY, claims) {
                context.user_id = Some(user.id);
                jar = jar.add(default_cookie("jwt", jwt, 1));
            }
            // Note: JWT generation errors are intentionally swallowed here
            // to prevent refresh token from being invalidated due to
            // temporary JWT generation issues
        }
    }

    // Inject the resolved context into request extensions
    request.extensions_mut().insert(context);

    let response = next.run(request).await;

    // Merge cookie updates with the response
    (jar, response).into_response()
}
