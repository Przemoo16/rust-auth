use crate::{
    api::{
        constant::SIGNIN_ROUTE,
        middleware::{set_default_response_headers_for_protected, RenderOptions},
    },
    libs::auth::Backend,
    state::AppState,
};
use askama_axum::Template;
use axum::{extract::Extension, middleware::map_response, routing::get, Router};
use axum_login::login_required;
use tower::ServiceBuilder;

pub fn create_protected_router() -> Router<AppState> {
    Router::new().route("/protected", get(protected)).layer(
        ServiceBuilder::new()
            .layer(login_required!(Backend, login_url = SIGNIN_ROUTE))
            .layer(map_response(set_default_response_headers_for_protected)),
    )
}

#[derive(Template)]
#[template(path = "pages/protected/index.html")]
struct ProtectedTemplate {
    options: RenderOptions,
}

async fn protected(Extension(options): Extension<RenderOptions>) -> ProtectedTemplate {
    ProtectedTemplate { options }
}
