use crate::core::types::{Activity, User};
use crate::db;
use chrono::{DateTime, Utc};
use uuid::Uuid;

impl Activity {
    pub async fn insert(self: &Self, pool: &db::DBPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO activities (id, user_id, name, description, calories, duration_s, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
            .bind(self.id)
            .bind(self.user_id)
            .bind(&self.name)
            .bind(&self.description)
            .bind(self.calories)
            .bind(self.duration_s)
            .bind(self.created_at)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn fetch_between_dates(
        user: &User,
        start_date: DateTime<Utc>,
        end_date: Option<DateTime<Utc>>,
        pool: &db::DBPool,
    ) -> Result<Vec<Activity>, sqlx::Error> {
        let end_date = end_date.unwrap_or_else(|| Utc::now());

        sqlx::query_as::<_, Activity>(
            "SELECT id, user_id, name, description, calories, duration_s, created_at FROM activities
             WHERE user_id = $1 AND created_at >= $2 AND created_at <= $3
             ORDER BY created_at DESC"
        )
        .bind(user.id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(pool)
        .await
    }
}
