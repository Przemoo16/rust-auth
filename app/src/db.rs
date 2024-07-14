use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn create_db_pool(url: &str, max_connections: u32) -> PgPool {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(url)
        .await
        .expect("Failed to create database connection pool")
}
