use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use uuid::Uuid;

mod entry;
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

#[derive(sqlx::Type, Clone, Debug, Serialize, Deserialize, TS)]
pub enum FoodType {
    Meal,
    Snack,
    Drink,
}

#[derive(FromRow, Clone, Debug, Serialize, Deserialize, TS)]
pub struct FoodEntry {
    pub id: Uuid,
    pub name: String,
    pub calories: i32,
    pub entry_type: FoodType,
    pub time: DateTime<Utc>,
}

impl FoodEntry {
    pub fn new(
        name: String,
        calories: i32,
        entry_type: FoodType,
        time: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name,
            calories: calories,
            entry_type: entry_type,
            time: if let Some(tt) = time { tt } else { Utc::now() },
        }
    }
}
