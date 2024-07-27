use app::config::Config;
use app::db::connection::{setup_db_pool, setup_session_store};
use app::server::{run_server, ServerConfig};
use app::tracing::setup_tracing;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app_config = Config::from_env();
    setup_tracing();
    let db = setup_db_pool(&app_config.db.url, app_config.db.pool_max_connections).await;
    let session_store = setup_session_store(db.clone()).await;
    run_server(ServerConfig {
        db,
        session_store,
        app_config,
    })
    .await;
}
