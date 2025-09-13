-- #[derive(sqlx::Type, Clone, Debug, Serialize, Deserialize, TS)]
-- pub enum FoodType {
--     Meal,
--     Snack,
--     Drink,
-- }
--
-- #[derive(FromRow, Clone, Debug, Serialize, Deserialize, TS)]
-- #[sqlx(type_name = "user_role", rename_all = "lowercase")]
-- pub struct FoodEntry {
--     pub id: Uuid,
--     pub name: String,
--     pub calories: i32,
--     pub entry_type: FoodType,
--     pub time: DateTime<Utc>,
-- }

CREATE TABLE meals (
    id uuid PRIMARY KEY UNIQUE DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255),
    description VARCHAR(255),
    calories INTEGER NOT NULL,
    created_at timestamptz NOT NULL DEFAULT NOW(),
    updated_at timestamptz
);

select trigger_updated_at('meals');
