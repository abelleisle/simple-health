use crate::db::DatabaseConnection;

use async_trait::async_trait;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls, Row};

#[derive(Clone)]
pub struct PostgresQL {
    pub client: Arc<Client>,
}

#[async_trait]
impl DatabaseConnection for PostgresQL {
    async fn connect()
    -> Result<Box<dyn DatabaseConnection>, Box<dyn std::error::Error + Send + Sync>> {
        let (client, connection) = tokio_postgres::connect(
            "host=localhost dbname=health user=gym password=membership",
            NoTls,
        )
        .await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Database connection error: {}", e);
            }
        });

        Ok(Box::new(PostgresQL {
            client: Arc::new(client),
        }))
    }

    async fn execute(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.client.execute(query, params).await?)
    }

    async fn query(
        &self,
        query: &str,
        params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<Row>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.client.query(query, params).await?)
    }

    async fn list_tables(&self) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let rows = self
            .client
            .query(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'",
                &[],
            )
            .await?;

        let tables: Vec<String> = rows.iter().map(|row| row.get::<_, String>(0)).collect();

        Ok(tables)
    }

    async fn create_tables_if_not_exists(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Create users table based on User struct from types.rs
        let create_users_table = "
            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                email VARCHAR(255) NOT NULL UNIQUE,
                name VARCHAR(255) NOT NULL,
                calorie_goal INTEGER NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
        ";

        self.client.execute(create_users_table, &[]).await?;
        println!("✅ Users table created/verified");

        Ok(())
    }

    fn is_closed(&self) -> bool {
        self.client.is_closed()
    }

    fn clone_box(&self) -> Box<dyn DatabaseConnection> {
        Box::new(self.clone())
    }
}

// impl PostgresQL {
//     pub async fn connect() -> Result<Self, Error> {
//         let (client, connection) = tokio_postgres::connect(
//             "host=localhost dbname=health user=gym password=membership",
//             NoTls,
//         )
//         .await?;
//
//         tokio::spawn(async move {
//             if let Err(e) = connection.await {
//                 eprintln!("Database connection error: {}", e);
//             }
//         });
//
//         Ok(PostgresQL {
//             client: Arc::new(client),
//         })
//     }
// }
