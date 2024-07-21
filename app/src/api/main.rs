use crate::api::middlewares::RenderOptions;
use crate::state::AppState;
use askama_axum::Template;
use axum::{extract::Extension, routing::get, Router};

pub fn create_main_router() -> Router<AppState> {
    Router::new().route("/", get(home))
}

#[derive(Template)]
#[template(path = "home/index.html")]
struct HomeTemplate {
    options: RenderOptions,
}

async fn home(Extension(options): Extension<RenderOptions>) -> HomeTemplate {
    HomeTemplate { options }
}
