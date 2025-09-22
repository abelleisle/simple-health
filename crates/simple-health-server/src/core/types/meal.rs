use crate::core::types::Meal;
use crate::db;
use chrono::{DateTime, Utc};
use uuid::Uuid;

impl Meal {
    pub async fn insert(self: &Self, pool: &db::DBPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO meals (id, user_id, name, description, calories, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
        )
            .bind(self.id)
            .bind(self.user_id)
            .bind(&self.name)
            .bind(&self.description)
            .bind(self.calories)
            .bind(self.created_at)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn fetch_between_dates(
        user_id: &Uuid,
        start_date: DateTime<Utc>,
        end_date: Option<DateTime<Utc>>,
        pool: &db::DBPool,
    ) -> Result<Vec<Meal>, sqlx::Error> {
        let end_date = end_date.unwrap_or_else(|| Utc::now());

        sqlx::query_as::<_, Meal>(
            "SELECT id, user_id, name, description, calories, created_at FROM meals 
             WHERE user_id = $1 AND created_at >= $2 AND created_at <= $3 
             ORDER BY created_at DESC",
        )
        .bind(user_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(pool)
        .await
    }
}
