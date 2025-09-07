use crate::core::types::{Goal, UserRef};
use crate::db;

impl Goal {
    pub async fn new(
        pool: &db::DBPool,
        user: UserRef,
        consumed: Option<i32>,
        burned: Option<i32>,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Goal>(
            "INSERT INTO meals (user_id, calories_consumed, calories_burned) VALUES ($1, $2, $3)
            RETURNING user_id, calories_consumed, calories_burned, created_at",
        )
        .bind(user.id())
        .bind(consumed)
        .bind(burned)
        .fetch_one(pool)
        .await
    }
}
