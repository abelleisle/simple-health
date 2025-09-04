pub mod schema;

use serde_json::Value;
use sqlx::{Column, Executor, PgPool, Row, TypeInfo};

#[derive(Clone)]
pub struct DatabaseConnection {
    pool: PgPool,
}

impl DatabaseConnection {
    pub async fn connect() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://gym:membership@localhost/health".to_string());

        let pool = PgPool::connect(&database_url).await?;

        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn execute(
        &self,
        query: &str,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.pool.execute(query).await?;
        Ok(result.rows_affected())
    }

    // pub async fn fetch_all_json(
    //     &self,
    //     query: &str,
    // ) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
    //     let rows = sqlx::query(query).fetch_all(&self.pool).await?;
    //     let mut results = Vec::new();
    //
    //     for row in rows {
    //         let mut json_row = serde_json::Map::new();
    //         for (i, col) in row.columns().iter().enumerate() {
    //             let col_name = col.name();
    //             let value: Value = match col.type_info().name() {
    //                 "TEXT" | "VARCHAR" | "CHAR" => row
    //                     .try_get::<Option<String>, _>(i)?
    //                     .map(Value::String)
    //                     .unwrap_or(Value::Null),
    //                 "INT4" | "INTEGER" => row
    //                     .try_get::<Option<i32>, _>(i)?
    //                     .map(|v| Value::Number(v.into()))
    //                     .unwrap_or(Value::Null),
    //                 "INT8" | "BIGINT" => row
    //                     .try_get::<Option<i64>, _>(i)?
    //                     .map(|v| Value::Number(v.into()))
    //                     .unwrap_or(Value::Null),
    //                 "UUID" => row
    //                     .try_get::<Option<uuid::Uuid>, _>(i)?
    //                     .map(|v| Value::String(v.to_string()))
    //                     .unwrap_or(Value::Null),
    //                 "BOOL" => row
    //                     .try_get::<Option<bool>, _>(i)?
    //                     .map(Value::Bool)
    //                     .unwrap_or(Value::Null),
    //                 _ => Value::Null,
    //             };
    //             json_row.insert(col_name.to_string(), value);
    //         }
    //         results.push(Value::Object(json_row));
    //     }
    //
    //     Ok(results)
    // }
    //
    // pub async fn fetch_one_json(
    //     &self,
    //     query: &str,
    // ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>> {
    //     let mut results = self.fetch_all_json(query).await?;
    //     Ok(results.pop())
    // }

    pub async fn list_tables(
        &self,
    ) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let query =
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'";
        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        let tables: Vec<String> = rows
            .iter()
            .map(|row| row.get::<String, _>("table_name"))
            .collect();

        Ok(tables)
    }

    pub fn is_connected(&self) -> bool {
        !self.pool.is_closed()
    }

    pub async fn create_all_tables(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let tables = self.list_tables().await?;

        for table in schema::get_all_table_schemas() {
            if tables.iter().any(|t| t.as_str() == table.name) {
                println!("Table {} already exists", table.name);
            } else {
                println!("Creating table: {}", table.name);
                self.execute(table.sql).await?;
            }
            println!("✅ {} table created/verified", table.name);
        }
        Ok(())
    }
}
