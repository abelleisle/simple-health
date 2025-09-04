use sqlx::PgPool;
use uuid::Uuid;

use crate::core::types::User;

impl User {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, name, calorie_goal, created_at, updated_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, name, calorie_goal, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn create(pool: &PgPool, new_user: &User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (email, name, calorie_goal) VALUES ($1, $2, $3) 
             RETURNING id, email, name, calorie_goal, created_at, updated_at",
        )
        .bind(&new_user.email)
        .bind(&new_user.name)
        .bind(new_user.calorie_goal)
        .fetch_one(pool)
        .await
    }

    pub async fn update(
        pool: &PgPool,
        user_id: Uuid,
        updates: &User,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "UPDATE users SET email = $2, name = $3, calorie_goal = $4, updated_at = NOW() 
             WHERE id = $1 
             RETURNING id, email, name, calorie_goal, created_at, updated_at",
        )
        .bind(user_id)
        .bind(&updates.email)
        .bind(&updates.name)
        .bind(updates.calorie_goal)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
