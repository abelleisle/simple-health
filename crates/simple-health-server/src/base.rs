use crate::auth::{
    authenticate::JWT_SIGNING_KEY,
    cookie::default_cookie,
    jwt::{Claims, generate_jwt, validate_jwt},
};
use crate::core::types::User;
use crate::session::RefreshToken;
use crate::{ServerState, UserContext};
use axum::extract::Request;
use axum::middleware::Next;
use axum::{extract::State, response::IntoResponse};
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
        user: None,
        is_admin: false,
        error: None,
    };

    // JWT takes precedence if present
    if let Some(jwt) = jwt {
        match validate_jwt::<Claims>(JWT_SIGNING_KEY, jwt.value()) {
            Ok(claims) => {
                log::trace!("User ID from JWT {}", claims.user_id);
                let user = User::get(app.db.get_pool(), Some(claims.user_id), None).await;
                match user {
                    Ok(Some(user)) => {
                        context.user = Some(user);
                    }
                    _ => {
                        log::error!("Error fetching user {}, removing cookies", claims.user_id);
                        jar = jar.remove("jwt").remove("refresh");
                    }
                }
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
                jar = jar.add(default_cookie("jwt", jwt, 1));
            }
            // Note: JWT generation errors are intentionally swallowed here
            // to prevent refresh token from being invalidated due to
            // temporary JWT generation issues

            context.user = Some(user);
        }
    }

    // Inject the resolved context into request extensions
    request.extensions_mut().insert(context);

    let response = next.run(request).await;

    // Merge cookie updates with the response
    (jar, response).into_response()
}
