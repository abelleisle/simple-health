use crate::core::types::User;
use crate::db::DBPool;
use jwt_simple::reexports::rand;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl RefreshToken {
    pub async fn create(pool: &DBPool, user_id: Uuid) -> Result<RefreshToken, sqlx::Error> {
        use rand::RngCore;

        // Generate cryptographically secure random token
        let mut bytes = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut bytes);
        let token = URL_SAFE_NO_PAD.encode(&bytes);

        log::debug!("Creating or updating refresh token for user {}", user_id);
        // Upsert refresh token - handles race conditions by using ON CONFLICT
        // If a token already exists for this user, update it with the new token and expiry
        sqlx::query_as::<_, RefreshToken>(
            "INSERT INTO refresh_keys (user_id, token, expires_at)
             VALUES ($1, $2, NOW() + INTERVAL '30 days')
             ON CONFLICT (user_id)
             DO UPDATE SET token = EXCLUDED.token,
                           expires_at = EXCLUDED.expires_at,
                           updated_at = NOW()
             RETURNING user_id, token, expires_at, created_at",
        )
        .bind(user_id)
        .bind(&token)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_token(
        pool: &DBPool,
        token: &str,
    ) -> Result<Option<RefreshToken>, sqlx::Error> {
        sqlx::query_as::<_, RefreshToken>(
            "SELECT user_id, token, expires_at, created_at
             FROM refresh_keys WHERE token = $1 AND expires_at > NOW()",
        )
        .bind(token)
        .fetch_optional(pool)
        .await
    }

    pub async fn get_by_user_id(
        pool: &DBPool,
        user_id: Uuid,
    ) -> Result<Option<RefreshToken>, sqlx::Error> {
        sqlx::query_as::<_, RefreshToken>(
            "SELECT user_id, token, expires_at, created_at
             FROM refresh_keys WHERE user_id = $1 AND expires_at > NOW()",
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete_by_token(pool: &DBPool, token: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM refresh_keys WHERE token = $1")
            .bind(token)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_by_user_id(pool: &DBPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM refresh_keys WHERE user_id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    pub async fn get_user_from_token(
        pool: &DBPool,
        token: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let refresh_token = Self::get_by_token(pool, token).await?;

        match refresh_token {
            Some(rt) => User::get(pool, Some(rt.user_id), None).await,
            None => Ok(None),
        }
    }
}
