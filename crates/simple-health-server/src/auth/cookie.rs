use axum_extra::extract::cookie::Cookie;
use time::Duration;

pub(crate) fn default_cookie<'a>(key: &str, token: String, duration_hrs: u64) -> Cookie<'a> {
    Cookie::build((key.to_string(), token))
        .path("/")
        .http_only(true)
        .max_age(Duration::seconds((duration_hrs * 3600) as i64))
        .secure(!crate::utils::IS_DEBUG)
        .build()
}
