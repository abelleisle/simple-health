use crate::auth;
use crate::db;

use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::core::types::{Signin, Signup, User};

impl User {
    pub async fn new(pool: &db::DBPool, signup: &Signup) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3)
            RETURNING id, email, name",
        )
        .bind(&signup.email)
        .bind(&signup.password)
        .bind(&signup.name)
        .fetch_one(pool)
        .await;

        return user;
    }

    pub async fn get(
        pool: &db::DBPool,
        id: Option<Uuid>,
        email: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error> {
        match (id, email) {
            (Some(user_id), Some(user_email)) => {
                // Both provided - use OR condition
                sqlx::query_as::<_, User>(
                    "SELECT id, email, name, created_at, updated_at
                   FROM users WHERE id = $1 AND email = $2",
                )
                .bind(user_id)
                .bind(user_email)
                .fetch_optional(pool)
                .await
            }
            (Some(user_id), None) => {
                // ID only
                sqlx::query_as::<_, User>(
                    "SELECT id, email, name, created_at, updated_at
                   FROM users WHERE id = $1",
                )
                .bind(user_id)
                .fetch_optional(pool)
                .await
            }
            (None, Some(user_email)) => {
                // Email only
                sqlx::query_as::<_, User>(
                    "SELECT id, email, name, created_at, updated_at
                   FROM users WHERE email = $1",
                )
                .bind(user_email)
                .fetch_optional(pool)
                .await
            }
            (None, None) => {
                // Neither provided - return error or None
                Ok(None)
            }
        }
    }

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

    // pub async fn create(pool: &PgPool, new_user: &User) -> Result<User, sqlx::Error> {
    //     sqlx::query_as::<_, User>(
    //         "INSERT INTO users (email, name, calorie_goal) VALUES ($1, $2, $3)
    //          RETURNING id, email, name, calorie_goal, created_at, updated_at",
    //     )
    //     .bind(&new_user.email)
    //     .bind(&new_user.name)
    //     // .bind(new_user.calorie_goal)
    //     .fetch_one(pool)
    //     .await
    // }

    // pub async fn update(
    //     pool: &PgPool,
    //     user_id: Uuid,
    //     updates: &User,
    // ) -> Result<Option<User>, sqlx::Error> {
    //     sqlx::query_as::<_, User>(
    //         "UPDATE users SET email = $2, name = $3, calorie_goal = $4, updated_at = NOW()
    //          WHERE id = $1
    //          RETURNING id, email, name, calorie_goal, created_at, updated_at",
    //     )
    //     .bind(user_id)
    //     .bind(&updates.email)
    //     .bind(&updates.name)
    //     // .bind(updates.calorie_goal)
    //     .fetch_optional(pool)
    //     .await
    // }

    pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_with_password(
        pool: &db::DBPool,
        email: &str,
    ) -> Result<Option<(User, String)>, sqlx::Error> {
        let result =
            sqlx::query("SELECT id, email, name, password_hash FROM users WHERE email = $1")
                .bind(email)
                .fetch_optional(pool)
                .await?;

        if let Some(row) = result {
            let user = User {
                id: row.get("id"),
                email: row.get("email"),
                name: row.get("name"),
            };
            let password_hash: String = row.get("password_hash");
            Ok(Some((user, password_hash)))
        } else {
            Ok(None)
        }
    }

    pub async fn validate_and_fetch(
        pool: &db::DBPool,
        signin: &Signin,
    ) -> Result<Option<User>, sqlx::Error> {
        // Fetch user with password hash for validation
        if let Some((user, password_hash)) = User::get_with_password(pool, &signin.username).await?
        {
            // Verify password (you'll need a password hashing crate like bcrypt or argon2)
            if auth::verify_password(&signin.password, &password_hash) {
                return Ok(Some(user));
            }
        }

        Ok(None)
    }
}
