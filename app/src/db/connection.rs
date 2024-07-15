use sqlx::{
    migrate,
    postgres::{PgPool, PgPoolOptions},
};

pub type Database = PgPool;

pub async fn create_db_pool(url: &str, max_connections: u32) -> Database {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(url)
        .await
        .expect("Failed to create database connection pool")
}

pub async fn run_migrations(db: &Database) {
    migrate!().run(db).await.expect("Failed to run migrations");
}
