use crate::api::{middleware::RenderOptions, response::create_redirect_for_authenticated};
use crate::libs::auth::is_anonymous;
use crate::state::AppState;
use askama_axum::Template;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, routing::get, Router};
use axum_login::predicate_required;

pub fn create_main_router() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .layer(predicate_required!(
            is_anonymous,
            create_redirect_for_authenticated()
        ))
}

#[derive(Template)]
#[template(path = "pages/home/index.html")]
struct HomeTemplate {
    options: RenderOptions,
}

async fn home(Extension(options): Extension<RenderOptions>) -> HomeTemplate {
    HomeTemplate { options }
}

#[derive(Template)]
#[template(path = "pages/404.html")]
pub struct NotFoundTemplate {
    options: RenderOptions,
}

pub async fn handler_404(Extension(options): Extension<RenderOptions>) -> impl IntoResponse {
    let template = NotFoundTemplate { options };
    (StatusCode::NOT_FOUND, template).into_response()
}
