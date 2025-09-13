use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use uuid::Uuid;

mod goal;
mod meal;
mod user;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Signup {
    pub email: String,
    pub password_hash: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Signin {
    pub username: String,
    pub password: String,
}

pub enum UserRef {
    User(User),
    ID(Uuid),
}

impl UserRef {
    pub fn id(self: Self) -> Uuid {
        match self {
            UserRef::User(u) => u.id,
            UserRef::ID(i) => i,
        }
    }
}

#[derive(FromRow, Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}

#[derive(FromRow, Clone, Debug, Deserialize, TS)]
#[ts(export)]
pub struct Goal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub consumed: i32,
    pub burned: i32,
    pub created_at: DateTime<Utc>,
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
    Breakfast,
    Lunch,
    Dinner,
    Snack,
    Coffee,
}

#[derive(FromRow, Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Activity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub calories: i32,
    pub created_at: DateTime<Utc>,
}
