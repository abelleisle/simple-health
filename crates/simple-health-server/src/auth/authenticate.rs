use axum::{
    Form,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

use crate::ServerState;
use crate::auth::{cookie::default_cookie, jwt, jwt::Claims};
use crate::core::types::{Signin, User};

use axum_extra::extract::CookieJar;

const JWT_SIGNING_KEY: &str = "supersecretsigningkey";

pub async fn login(
    State(app): State<ServerState>,
    jar: CookieJar, // CookieJar is available in axum_extras
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

    // TODO
    // get/create a refresh token for the user
    // let refresh_token = match db::refresh_tokens::create(user.id).await {
    //     Ok(token) => token,
    //     Err(_) => {
    //         return (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             "Somethign bad happened, try again later"
    //         ).into_response();
    //     }
    // };
    let refresh_token: String = "refresh_token_test".to_string();

    let claims = Claims::with(&user);
    match jwt::generate_jwt(JWT_SIGNING_KEY, claims) {
        Ok(token) => (
            [("hx-redirect", "/")],
            jar.add(default_cookie("jwt", token, 1)).add(default_cookie(
                "refresh",
                refresh_token,
                30 * 24,
            )),
        )
            .into_response(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Uh oh :((").into_response();
        }
    }
}
