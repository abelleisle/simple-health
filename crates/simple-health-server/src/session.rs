use crate::core::types::Session;
use crate::db::DBPool;
use serde::de::DeserializeOwned;
use sqlx::types::Json;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct SessionData {
    user_id: Uuid,
}

impl<D: Serialize + DeserializeOwned + Send + Unpin + 'static> Session<D> {
    pub async fn new(pool: &DBPool, data: D) -> sqlx::Result<Session<D>> {
        Ok(sqlx::query_as::<_, Session<D>>(
            "INSERT INTO sessions (data) VALUES ($1)
            RETURNING id, data, created_at, expires_at",
        )
        .bind(Json(data))
        .fetch_one(pool)
        .await?)
    }

    pub async fn get(pool: &DBPool, id: Uuid) -> sqlx::Result<Option<Session<D>>> {
        sqlx::query_as("select id, data, created_at, expires_at from sessions where id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn expired(&self) -> bool {
        return self.expires_at < chrono::Utc::now();
    }
}
