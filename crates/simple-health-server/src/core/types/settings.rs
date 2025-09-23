use crate::core::types::{User, UserSetting};
use crate::db;

impl UserSetting {
    pub fn default() -> Self {
        UserSetting {
            timezone: "UTC".to_string(),
            darkmode: false,
        }
    }

    pub async fn new(
        pool: &db::DBPool,
        user: &User,
        settings: &UserSetting,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, UserSetting>(
            "INSERT INTO user_settings (user_id, timezone, darkmode) VALUES ($1, $2, $3)
            RETURNING timezone, darkmode",
        )
        .bind(user.id)
        .bind(&settings.timezone)
        .bind(&settings.darkmode)
        .fetch_one(pool)
        .await
    }

    pub async fn get_or_default(
        pool: &db::DBPool,
        user: &User,
    ) -> Result<UserSetting, sqlx::Error> {
        let db_settings = sqlx::query_as::<_, UserSetting>(
            "SELECT timezone, darkmode FROM user_settings WHERE user_id = $1",
        )
        .bind(user.id)
        .fetch_optional(pool)
        .await?;

        match db_settings {
            Some(settings) => Ok(settings),
            None => Ok(Self::default()),
        }
    }

    pub async fn update(
        pool: &db::DBPool,
        user: &User,
        settings: &UserSetting,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO user_settings (user_id, timezone, darkmode) VALUES ($1, $2, $3)
            ON CONFLICT (user_id) DO UPDATE SET
                timezone = EXCLUDED.timezone,
                darkmode = EXCLUDED.darkmode",
        )
        .bind(user.id)
        .bind(&settings.timezone)
        .bind(&settings.darkmode)
        .execute(pool)
        .await?;

        Ok(())
    }
}
