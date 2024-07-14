use crate::state::AppState;
use axum::{extract::State, routing::get, Router};
use sqlx::{postgres::PgPool, query};

pub async fn run_server(db: PgPool) {
    let state = AppState::new(db);
    let app = Router::new().route("/", get(home)).with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to create TCP listener");
    axum::serve(listener, app)
        .await
        .expect("Failed to run the server");
}

async fn home(State(state): State<AppState>) -> String {
    let res = query!("SELECT 1 as result").fetch_one(&state.db).await;
    match res {
        Ok(_) => "Healthy DB".to_string(),
        Err(_) => "Unhealthy DB".to_string(),
    }
}
