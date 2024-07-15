use crate::api::auth::create_auth_router;
use crate::api::main::create_main_router;
use crate::db::connection::Database;
use crate::state::AppState;
use axum::Router;

pub async fn run_server(db: Database) {
    let state = AppState::new(db);
    let app = Router::new()
        .nest("/", create_main_router())
        .nest("/", create_auth_router())
        .with_state(state);
    let port = "3000";
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect(&format!(
            "Failed to create listener bound to the port {}",
            port
        ));
    axum::serve(listener, app)
        .await
        .expect("Failed to run the server");
}
