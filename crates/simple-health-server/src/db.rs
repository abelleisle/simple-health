mod postgresql;
pub use postgresql::PostgresQL;

use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait DatabaseConnection: Send + Sync {
    async fn connect()
    -> Result<Box<dyn DatabaseConnection>, Box<dyn std::error::Error + Send + Sync>>
    where
        Self: Sized;

    async fn execute(&self, query: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>>;

    async fn fetch_all_json(
        &self,
        query: &str,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>>;

    async fn fetch_one_json(
        &self,
        query: &str,
    ) -> Result<Option<Value>, Box<dyn std::error::Error + Send + Sync>>;

    async fn list_tables(&self) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>>;

    async fn create_tables_if_not_exists(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    fn is_connected(&self) -> bool;

    fn clone_box(&self) -> Box<dyn DatabaseConnection>;
}

impl Clone for Box<dyn DatabaseConnection> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
