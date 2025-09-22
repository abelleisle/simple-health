use axum::{
    extract::{FromRef, OptionalFromRequestParts},
    http::request::Parts,
    response::Redirect,
};
use axum_extra::extract::cookie::CookieJar;
use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use uuid::Uuid;

use crate::ServerState;
use crate::core::types::User;

const JWT_SIGNING_KEY: &str = "supersecretsigningkey";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iat: usize,
    pub exp: usize,
    pub user_id: Uuid,
    pub user_email: String,
}

impl Claims {
    pub fn with(user: &User) -> Self {
        let now = Utc::now();
        let expire: TimeDelta = Duration::hours(24);
        let exp: usize = (now + expire).timestamp() as usize;
        let iat: usize = now.timestamp() as usize;
        Self {
            iat,
            exp,
            user_id: user.id,
            user_email: user.email.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CookieJwt<T: DeserializeOwned>(pub T);

// since axum 0.8 you can implement extractors meant to be Option<T>
// this is very useful, expecially for scenarios where endpoint can be accessed
// both by authed users and non-authed users
impl<S, T> OptionalFromRequestParts<S> for CookieJwt<T>
where
    ServerState: FromRef<S>,
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = Redirect;

    async fn from_request_parts(
        req: &mut Parts,
        _state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let jar = CookieJar::from_headers(&req.headers);
        if let Some(jwt) = jar.get("jwt").map(|c| c.value()) {
            return match validate_jwt::<T>(JWT_SIGNING_KEY, jwt) {
                Ok(data) => return Ok(Some(CookieJwt(data))),
                // user tampered with cookie here, we want to delete that cookie
                // returning None here would have been okay too if you're okay with
                // manufactured cookies :)
                Err(_) => Err(Redirect::to("/logout")),
            };
        }

        // if refresh token is present, try and get a new jwt
        // by redirecting user to /refresh_token endpoint
        if jar.get("refresh").is_some() {
            return Err(Redirect::to(
                format!("/api/v1/refresh_token?next={}", req.uri).as_str(),
            ));
        }

        // at this point, user has no jwt and no refresh token
        Ok(None)
    }
}

pub fn validate_jwt<T: DeserializeOwned>(
    secret: &str,
    token: &str,
) -> Result<T, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<T>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;

    Ok(token_data.claims)
}

pub fn generate_jwt<T: Serialize>(
    secret: &str,
    claim: T,
) -> Result<String, jsonwebtoken::errors::Error> {
    let token_data = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token_data)
}
