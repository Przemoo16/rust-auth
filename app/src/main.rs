use app::server::run_server;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    run_server().await;
}
