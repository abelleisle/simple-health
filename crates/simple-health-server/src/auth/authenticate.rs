use axum::{
    Form,
    extract::{Query, State},
    http::{
        StatusCode,
        header::{HeaderMap, SET_COOKIE},
    },
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use serde::Deserialize;

use crate::ServerState;
use crate::auth::{cookie::default_cookie, jwt, jwt::Claims};
use crate::core::types::{Signin, User};
use crate::session::RefreshToken;

pub const JWT_SIGNING_KEY: &str = "supersecretsigningkey";

#[axum::debug_handler]
pub async fn login(
    State(app): State<ServerState>,
    jar: CookieJar,
    Form(signin): Form<Signin>,
) -> impl IntoResponse {
    // dummy function to get a user
    // let user = match db::user::get(&app.pg_pool, &username, &password).await {
    //     None => return Redirect::to("/signup").into_response()
    //     Some(user) => user
    // };
    let user = match User::validate_and_fetch(app.db.get_pool(), &signin).await {
        Ok(user) => match user {
            Some(user) => user,
            None => return Redirect::to("/signup").into_response(),
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
