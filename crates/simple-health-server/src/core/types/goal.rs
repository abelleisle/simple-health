use crate::core::types::{Goal, User};
use crate::db;

use chrono::{DateTime, Utc};

impl Goal {
    const DEFAULT_CONSUMED: i32 = 2000;
    const DEFAULT_BURNED: i32 = 300;
    const DEFAULT_ACTIVE_TIME_S: i32 = 30 * 60;

    pub fn default(user: &User) -> Self {
        Goal {
            user_id: user.id,
            calories_consumed: Some(Self::DEFAULT_CONSUMED),
            calories_burned: Some(Self::DEFAULT_BURNED),
            active_time_s: Some(Self::DEFAULT_ACTIVE_TIME_S),
        }
    }

    pub async fn new(pool: &db::DBPool, goal: &Goal) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Goal>(
            "INSERT INTO goals (user_id, calories_consumed, calories_burned, active_time_s) VALUES ($1, $2, $3, $4)
            RETURNING user_id, calories_consumed, calories_burned, active_time_s",
        )
        .bind(goal.user_id)
        .bind(goal.calories_consumed)
        .bind(goal.calories_burned)
        .bind(goal.active_time_s)
        .fetch_one(pool)
        .await
    }

    pub async fn latest(
        pool: &db::DBPool,
        user: &User,
        time: DateTime<Utc>,
    ) -> Result<Goal, sqlx::Error> {
        let result = sqlx::query_as::<_, Goal>(
            "WITH latest_values AS (
                SELECT
                    user_id,
                    FIRST_VALUE(calories_consumed) OVER (
                        PARTITION BY user_id
                        ORDER BY CASE WHEN calories_consumed IS NOT NULL THEN created_at END DESC NULLS LAST
                    ) as calories_consumed,
                    FIRST_VALUE(calories_burned) OVER (
                        PARTITION BY user_id
                        ORDER BY CASE WHEN calories_burned IS NOT NULL THEN created_at END DESC NULLS LAST
                    ) as calories_burned,
                    FIRST_VALUE(active_time_s) OVER (
                        PARTITION BY user_id
                        ORDER BY CASE WHEN active_time_s IS NOT NULL THEN created_at END DESC NULLS LAST
                    ) as active_time_s,
                    ROW_NUMBER() OVER (PARTITION BY user_id ORDER BY created_at DESC) as rn
                FROM goals
                WHERE user_id = $1 AND created_at <= $2
            )
            SELECT user_id, calories_consumed, calories_burned, active_time_s
            FROM latest_values
            WHERE rn = 1"
        )
        .bind(user.id)
        .bind(time)
        .fetch_optional(pool)
        .await?;

        match result {
            Some(goal) => Ok(goal),
            None => Ok(Self::default(user)),
        }
    }

    pub fn get_consumed(&self) -> i32 {
        self.calories_consumed.unwrap_or(Self::DEFAULT_CONSUMED)
    }

    pub fn get_burned(&self) -> i32 {
        self.calories_burned.unwrap_or(Self::DEFAULT_BURNED)
    }

    pub fn get_active_time(&self) -> i32 {
        self.active_time_s.unwrap_or(Self::DEFAULT_ACTIVE_TIME_S)
    }
}
