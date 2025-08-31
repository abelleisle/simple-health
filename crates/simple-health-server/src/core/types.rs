use ts_rs::TS;
use uuid::Uuid;

#[derive(TS)]
#[ts(export)]
struct User {
    id: Uuid,
    email: String,
    name: String,
    calorieGoal: u32,
}
