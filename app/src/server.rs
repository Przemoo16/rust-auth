use crate::{
    api::{
        layer::create_auth_layer,
        main::handler_404,
        middleware::{set_default_response_headers, set_request_render_options},
        router::create_api_router,
    },
    config::Config,
    db::connection::{setup_db_pool, setup_session_store, Database, SessionStore},
    libs::signal::shutdown_signal,
    state::AppState,
    tracing::setup_tracing,
};
use axum::{
    middleware::{map_request, map_response},
    Router,
};
use std::net::SocketAddr;
use time::Duration;
use tokio::{task::spawn, time::Duration as TaskDuration};
use tower::ServiceBuilder;
use tower_sessions::ExpiredDeletion;
use tracing::info;

const PORT: u16 = 3000;

pub async fn run_server() {
    setup_tracing();

    let config = Config::from_env();

    let db = setup_db_pool(&config.db.url, config.db.pool_max_connections).await;
    let session_store = setup_session_store(db.clone()).await;

    let router = create_router(&config, db, session_store.clone());

    let deletion_task = spawn(
        session_store.continuously_delete_expired(TaskDuration::from_secs(
            config.auth.delete_expired_sessions_interval_seconds,
        )),
    );

    let socket_address = SocketAddr::from(([0, 0, 0, 0], PORT));
    let listener = tokio::net::TcpListener::bind(&socket_address)
        .await
        .expect(&format!(
            "Failed to create listener bound to the {}",
            &socket_address
        ));
    info!("Running server on {}", socket_address);
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await
        .expect("Failed to run the server");
}

fn create_router(config: &Config, db: Database, session_store: SessionStore) -> Router {
    let auth_layer = create_auth_layer(
        session_store,
        db.clone(),
        &config.auth.secret_key,
        Duration::minutes(config.auth.session_expiration_minutes),
    );
    let state = AppState::new(db);
    Router::new()
        .merge(create_api_router())
        .fallback(handler_404)
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(auth_layer)
                .layer(map_request(set_request_render_options))
                .layer(map_response(set_default_response_headers)),
        )
}
