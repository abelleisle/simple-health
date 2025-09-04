use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use uuid::Uuid;

mod user;

#[derive(FromRow, Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub calorie_goal: i32,
}

impl User {
    pub fn new(email: String, password_hash: String, name: String, calorie_goal: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            email: email,
            password_hash: password_hash,
            name: name,
            calorie_goal: calorie_goal,
        }
    }
}
