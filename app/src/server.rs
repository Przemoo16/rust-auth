use crate::api::auth::create_auth_router;
use crate::api::constant::SIGNIN_ROUTE;
use crate::api::layer::create_auth_layer;
use crate::api::main::create_main_router;
use crate::api::middleware::set_render_options;
use crate::config::Config;
use crate::db::connection::{Database, SessionStore};
use crate::libs::auth::Backend;
use crate::libs::signal::shutdown_signal;
use crate::state::AppState;
use axum::{
    middleware::from_fn,
    routing::{get, get_service},
    Router,
};
use axum_login::login_required;
use std::net::SocketAddr;
use time::Duration;
use tokio::{task::spawn, time::Duration as TaskDuration};
use tower_http::services::ServeDir;
use tower_sessions::ExpiredDeletion;
use tracing::info;

pub struct ServerConfig {
    pub db: Database,
    pub session_store: SessionStore,
    pub app_config: Config,
}

pub async fn run_server(config: ServerConfig) {
    let auth_layer = create_auth_layer(
        config.session_store.clone(),
        config.db.clone(),
        &config.app_config.auth.secret_key,
        Duration::minutes(config.app_config.auth.session_expiration_minutes),
    );
    let state = AppState::new(config.db);
    let app = Router::new()
        .route(
            "/protected",
            get(|| async { "Gotta be logged in to see me!" }),
        )
        .route_layer(login_required!(Backend, login_url = SIGNIN_ROUTE))
        .nest("/", create_main_router())
        .nest("/", create_auth_router())
        .layer(from_fn(set_render_options))
        .layer(auth_layer)
        .with_state(state)
        .nest_service("/styles", get_service(ServeDir::new("dist/styles")))
        .nest_service("/scripts", get_service(ServeDir::new("scripts")));
    let socket_address = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&socket_address)
        .await
        .expect(&format!(
            "Failed to create listener bound to the {}",
            &socket_address
        ));

    let deletion_task = spawn(
        config
            .session_store
            .continuously_delete_expired(TaskDuration::from_secs(
                config
                    .app_config
                    .auth
                    .delete_expired_sessions_interval_seconds,
            )),
    );

    info!("Running server on {}", socket_address);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await
        .expect("Failed to run the server");
}
