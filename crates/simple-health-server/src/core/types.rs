use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;
use uuid::Uuid;

// mod entry;
mod meal;
mod user;

use crate::db;

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

// #[derive(sqlx::Type, Clone, Debug, Serialize, Deserialize, TS)]
// #[sqlx(type_name = "user_role", rename_all = "lowercase")]
// pub enum FoodType {
//     Meal,
//     Snack,
//     Drink,
// }

#[derive(FromRow, Clone, Debug, Serialize, Deserialize, TS)]
pub struct Meal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub calories: i32,
    pub created_at: DateTime<Utc>,
}
