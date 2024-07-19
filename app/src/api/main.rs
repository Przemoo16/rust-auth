use crate::state::AppState;
use askama_axum::Template;
use axum::{routing::get, Router};

pub fn create_main_router() -> Router<AppState> {
    Router::new().route("/", get(home))
}

#[derive(Template)]
#[template(path = "pages/home/index.html")]
struct HomeTemplate {}

async fn home() -> HomeTemplate {
    HomeTemplate {}
}
