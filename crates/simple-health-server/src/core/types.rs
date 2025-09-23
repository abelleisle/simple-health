use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use uuid::Uuid;

mod activity;
mod goal;
mod meal;
mod settings;
mod user;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Signup {
    pub name: String,
    pub email: String,
    pub password: String,
    pub goals: Option<Goal>,
    pub settings: Option<UserSetting>,
}

#[derive(FromRow, Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UserSetting {
    pub timezone: String,
    pub darkmode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Signin {
    pub username: String,
    pub password: String,
}

#[derive(FromRow, Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}

#[derive(FromRow, Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Goal {
    pub user_id: Uuid,
    pub consumed: Option<i32>,
    pub burned: Option<i32>,
    pub active_time_s: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
    Coffee,
}

#[derive(FromRow, Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Meal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub calories: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ActivityType {
    Walk,
    Run,
    Hike,
    Bike,
    Ski,
}

#[derive(FromRow, Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Activity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub calories: i32,
    pub duration_s: Option<i32>,
    pub created_at: DateTime<Utc>,
}
