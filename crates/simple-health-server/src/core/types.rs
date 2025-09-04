use crate::db::schema::TableRequired;
use crate::register_table;
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

impl TableRequired for User {
    const CREATE_TABLE_SQL: &'static str = "CREATE TABLE IF NOT EXISTS users (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        email VARCHAR(255) NOT NULL UNIQUE,
        password_hash VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        calorie_goal INTEGER NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
    )";

    const TABLE_NAME: &'static str = "users";
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

register_table!(User);
