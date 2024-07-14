use crate::state::AppState;
use axum::{extract::State, routing::get, Router};
use sqlx::query;

pub fn create_main_router() -> Router<AppState> {
    Router::new().route("/", get(home))
}

async fn home(State(state): State<AppState>) -> String {
    let res = query!("SELECT 1 as result").fetch_one(&state.db).await;
    match res {
        Ok(_) => "Healthy DB".to_string(),
        Err(_) => "Unhealthy DB".to_string(),
    }
}
