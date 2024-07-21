use crate::api::auth::create_auth_router;
use crate::api::main::create_main_router;
use crate::api::middlewares::set_render_options;
use crate::db::connection::Database;
use crate::state::AppState;
use axum::{middleware::from_fn, Router};
use std::net::SocketAddr;

pub async fn run_server(db: Database) {
    let state = AppState::new(db);
    let app = Router::new()
        .nest("/", create_main_router())
        .nest("/", create_auth_router())
        .layer(from_fn(set_render_options))
        .with_state(state);
    let socket_address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&socket_address)
        .await
        .expect(&format!(
            "Failed to create listener bound to the {}",
            &socket_address
        ));
    tracing::info!("Running server on {}", socket_address);
    axum::serve(listener, app)
        .await
        .expect("Failed to run the server");
}
