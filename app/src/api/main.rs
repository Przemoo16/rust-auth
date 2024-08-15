use crate::state::AppState;
use crate::{api::middleware::RenderOptions, libs::auth::AuthSession};
use askama_axum::Template;
use axum::{extract::Extension, routing::get, Router};

pub fn create_main_router() -> Router<AppState> {
    Router::new().route("/", get(home))
}

#[derive(Template)]
#[template(path = "pages/home/index.html")]
struct HomeTemplate {
    options: RenderOptions,
    is_authenticated: bool,
}

async fn home(
    Extension(options): Extension<RenderOptions>,
    auth_session: AuthSession,
) -> HomeTemplate {
    let is_authenticated = auth_session.user.is_some();
    HomeTemplate {
        options,
        is_authenticated,
    }
}

#[derive(Template)]
#[template(path = "pages/404.html")]
pub struct NotFoundTemplate {
    options: RenderOptions,
}

pub async fn handler_404(Extension(options): Extension<RenderOptions>) -> NotFoundTemplate {
    NotFoundTemplate { options }
}
