use crate::crypto;
use crate::db;

use sqlx::Row;
use uuid::Uuid;

use crate::core::types::{Goal, Signin, Signup, User, UserSetting};

impl User {
    pub async fn create(pool: &db::DBPool, signup: &Signup) -> Result<User, sqlx::Error> {
        let user = Self::new(pool, signup).await?;

        let goal = signup.goals.clone().unwrap_or(Goal::default(&user));
        let _ = Goal::new(pool, &goal).await?;

        let settings = signup.settings.clone().unwrap_or(UserSetting::default());
        let _ = UserSetting::new(pool, &user, &settings).await?;

        return Ok(user);
    }

    async fn new(pool: &db::DBPool, signup: &Signup) -> Result<User, sqlx::Error> {
        let password_hash = crypto::password::hash(&signup.password).map_err(|e| {
            log::error!("Unable to hash user {} password: {}", signup.email, e);
            return sqlx::Error::BeginFailed;
        })?;

        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3)
            RETURNING id, email, name",
        )
        .bind(&signup.email)
        .bind(password_hash)
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

    pub async fn count(pool: &db::DBPool) -> Result<i64, sqlx::Error> {
        let result = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    // pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
    //     let result = sqlx::query("DELETE FROM users WHERE id = $1")
    //         .bind(user_id)
    //         .execute(pool)
    //         .await?;
    //
    //     Ok(result.rows_affected() > 0)
    // }

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
            return match crypto::password::verify(&signin.password, &password_hash) {
                Ok(true) => Ok(Some(user)),
                Ok(false) => {
                    log::warn!("Invalid credentials for user {}", user.email);
                    Ok(None)
                }
                Err(e) => {
                    log::error!("Unable to verify user {} password: {}", user.email, e);
                    Err(sqlx::Error::InvalidArgument(
                        "Unable to parse hashed password".to_string(),
                    ))
                }
            };
        }

        Ok(None)
    }
}
