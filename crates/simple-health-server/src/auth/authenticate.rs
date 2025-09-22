use axum::{
    Form,
    extract::{Extension, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;

use crate::auth::{cookie::default_cookie, jwt, jwt::Claims};
use crate::core::types::{Signin, Signup, User};
use crate::session::RefreshToken;
use crate::{ServerState, UserContext};

pub const JWT_SIGNING_KEY: &str = "supersecretsigningkey";

#[axum::debug_handler]
pub async fn login(
    State(app): State<ServerState>,
    jar: CookieJar,
    Extension(_ctx): Extension<UserContext>,
    Form(signin): Form<Signin>,
) -> impl IntoResponse {
    // TODO check if auth-bypass is desired
    // if ctx.user_id.is_some() {
    //     return Redirect::to("/").into_response();
    // }

    let user = match User::validate_and_fetch(app.db.get_pool(), &signin).await {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                log::warn!("User {} failed to login", signin.username);
                return Redirect::to("/login?error=Invalid+credentials").into_response();
            }
        },
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Uh oh :(").into_response(),
    };

    let refresh_token = match RefreshToken::create(app.db.get_pool(), user.id).await {
        Ok(token) => token,
        Err(e) => {
            log::error!("Refresh token creation error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Somethign bad happened, try again later",
            )
                .into_response();
        }
    };

    let claims = Claims::with(&user);
    match jwt::generate_jwt(JWT_SIGNING_KEY, claims) {
        Ok(token) => (
            jar.add(default_cookie("jwt", token, 1)).add(default_cookie(
                "refresh",
                refresh_token.token,
                30 * 24,
            )),
            Redirect::to("/"),
        )
            .into_response(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Uh oh :((").into_response();
        }
    }
}

pub async fn signup(
    State(app): State<ServerState>,
    jar: CookieJar,
    Extension(_ctx): Extension<UserContext>,
    Form(signup): Form<Signup>,
) -> impl IntoResponse {
    // TODO check if auth-bypass is desired
    // if ctx.user_id.is_some() {
    //     return Redirect::to("/").into_response();
    // }

    match User::get(app.db.get_pool(), None, Some(&signup.email)).await {
        Ok(None) => {}
        Ok(Some(_)) => {
            log::warn!("User {} already exists", signup.email);
            return Redirect::to("/signup?error=User+already+exists").into_response();
        }
        Err(e) => {
            log::error!("Error fetching user {}: {}", signup.email, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Uh oh :(").into_response();
        }
    };

    // There is no existing user, let's create one!
    let user = match User::new(app.db.get_pool(), &signup).await {
        Ok(user) => user,
        Err(e) => {
            log::error!("Error creating user {}: {}", signup.email, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Uh oh :(").into_response();
        }
    };

    // Now that we've created the user, log them in

    // Create a refresh token for the user
    let refresh_token = match RefreshToken::create(app.db.get_pool(), user.id).await {
        Ok(token) => token,
        Err(e) => {
            log::error!("Refresh token creation error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Somethign bad happened, try again later",
            )
                .into_response();
        }
    };

    // Create a new JWT token
    let claims = Claims::with(&user);
    let resp = match jwt::generate_jwt(JWT_SIGNING_KEY, claims) {
        Ok(token) => (
            jar.add(default_cookie("jwt", token, 1)).add(default_cookie(
                "refresh",
                refresh_token.token,
                30 * 24,
            )),
            Redirect::to("/"),
        )
            .into_response(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Uh oh :((").into_response();
        }
    };

    if app.should_disable_signup() {
        // If we can't disable signup, we should abort since this could
        // be a security issue
        app.config.write().unwrap().signup_allowed = false;
    }

    return resp;
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenQuery {
    next: Option<String>,
}

pub async fn refresh_token(
    State(app): State<ServerState>,
    jar: CookieJar,
    Query(RefreshTokenQuery { next }): Query<RefreshTokenQuery>,
) -> impl IntoResponse {
    let token = match jar.get("refresh") {
        Some(token) => token,
        None => {
            // if there's no token then the user goes back to /login
            return Redirect::to("/login").into_response();
        }
    };

    // if something goes wrong here we remove the token, otherwise the user could end up
    // in a loop where he's constantly being redirected here and this function fails every time

    let user = match RefreshToken::get_user_from_token(app.db.get_pool(), token.value()).await {
        Ok(Some(user)) => user,
        _ => {
            return (jar.remove(Cookie::from("refresh")), Redirect::to("/login")).into_response();
        }
    };

    // set new jwt
    let claims = Claims::with(&user);
    match jwt::generate_jwt(JWT_SIGNING_KEY, claims) {
        Ok(token) => (
            jar.add(default_cookie("jwt", token, 1)),
            Redirect::to(&next.unwrap_or("/".to_owned())),
        )
            .into_response(),
        Err(_) => (jar.remove(Cookie::from("refresh")), Redirect::to("/login")).into_response(),
    }
}

pub async fn signout(State(app): State<ServerState>, jar: CookieJar) -> impl IntoResponse {
    // Optionally clean up refresh token from database
    if let Some(refresh_token) = jar.get("refresh") {
        if let Err(e) =
            RefreshToken::delete_by_token(app.db.get_pool(), refresh_token.value()).await
        {
            log::warn!("Failed to delete refresh token from database: {}", e);
        }
    }

    // Remove both JWT and refresh cookies
    let jar = jar
        .remove(Cookie::from("jwt"))
        .remove(Cookie::from("refresh"));

    (jar, Redirect::to("/login")).into_response()
}
