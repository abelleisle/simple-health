use axum_extra::extract::cookie::Cookie;
use time::Duration;

pub(crate) fn default_cookie<'a>(key: &str, token: String, duration_hrs: u64) -> Cookie<'a> {
    Cookie::build((key.to_string(), token))
        .path("/")
        .http_only(true)
        .max_age(Duration::seconds((duration_hrs * 3600) as i64))
        .secure(!crate::utils::dev::is_debug_version())
        .build()
}

pub mod settings {
    use crate::core::types::UserSetting;
    use axum_extra::extract::cookie::{Cookie, CookieJar};

    pub fn set<'a>(settings: UserSetting) -> Result<Cookie<'a>, serde_json::Error> {
        let settings_json = serde_json::to_string(&settings)?;
        let cookie = Cookie::build(("settings", settings_json))
            .path("/")
            .secure(!crate::utils::dev::is_debug_version())
            .http_only(true)
            .build();

        Ok(cookie)
    }

    // Get settings from cookie
    pub fn get(jar: &CookieJar) -> Option<UserSetting> {
        let cookie = jar.get("settings")?;
        serde_json::from_str::<UserSetting>(cookie.value()).ok()
    }
}
