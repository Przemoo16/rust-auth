use app::config::Config;
use app::db::connection::{create_db_pool, run_migrations};
use app::server::run_server;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::from_env();
    let db_pool = create_db_pool(&config.database_url, config.database_pool_max_connections).await;
    run_migrations(&db_pool).await;
    run_server(db_pool).await;
}
