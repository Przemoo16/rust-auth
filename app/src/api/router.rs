use crate::{
    api::app::{
        asset::create_assets_router,
        auth::create_auth_router,
        main::{create_main_router, handler_404},
        protected::create_protected_router,
    },
    state::AppState,
};
use axum::Router;

pub fn create_api_router() -> Router<AppState> {
    Router::new()
        .merge(create_main_router())
        .merge(create_auth_router())
        .merge(create_protected_router())
        .merge(create_assets_router())
        .fallback(handler_404)
}
