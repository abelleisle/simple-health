use crate::core::types::Meal;
use crate::db;

impl Meal {
    pub async fn insert(self: &Self, pool: &db::DBPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO meals (id, user_id, name, calories, created_at) VALUES ($1, $2, $3, $4, $5)",
        )

            .bind(self.id)
            .bind(self.user_id)
            .bind(&self.name)
            .bind(self.calories)
            .bind(self.created_at)
        .execute(pool)
        .await?;

        Ok(())
    }
}
