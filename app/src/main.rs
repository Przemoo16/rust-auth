use config::Config;
use db::create_db_pool;
use dotenv::dotenv;
use server::run_server;

mod config;
mod db;
mod server;
mod state;

// TODO:
// 1. Do I need the tls-native-tls feature in the sqlx

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::from_env();
    let db_pool = create_db_pool(&config.database_url, config.database_pool_max_connections).await;
    run_server(db_pool).await;
}
