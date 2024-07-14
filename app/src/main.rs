use config::Config;
use db::create_db_pool;
use dotenv::dotenv;
use server::run_server;

mod api;
mod config;
mod db;
mod server;
mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::from_env();
    let db_pool = create_db_pool(&config.database_url, config.database_pool_max_connections).await;
    run_server(db_pool).await;
}
