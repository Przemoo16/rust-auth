use sqlx::{
    migrate,
    postgres::{PgPool, PgPoolOptions},
};
use tower_sessions_sqlx_store::PostgresStore;

pub type Database = PgPool;

pub async fn setup_db_pool(url: &str, max_connections: u32) -> Database {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(url)
        .await
        .expect("Failed to create database connection pool");
    migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    pool
}

pub type SessionStore = PostgresStore;

pub async fn setup_session_store(db: Database) -> SessionStore {
    let store = PostgresStore::new(db);
    store.migrate().await.expect("Failed to run migrations");
    store
}
