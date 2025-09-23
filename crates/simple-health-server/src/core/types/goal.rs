use crate::core::types::{Goal, User};
use crate::db;

impl Goal {
    pub async fn new(pool: &db::DBPool, goal: &Goal) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Goal>(
            "INSERT INTO meals (user_id, calories_consumed, calories_burned, active_time_s) VALUES ($1, $2, $3, $4)
            RETURNING user_id, calories_consumed, calories_burned, active_time_s",
        )
        .bind(goal.user_id)
        .bind(goal.consumed)
        .bind(goal.burned)
        .bind(goal.active_time_s)
        .fetch_one(pool)
        .await
    }
}
