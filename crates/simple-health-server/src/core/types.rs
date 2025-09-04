use diesel::prelude::*;

use ts_rs::TS;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::users)]
#[derive(TS)]
#[ts(export)]
struct User {
    id: Uuid,
    email: String,
    name: String,
    calorieGoal: u32,
}
