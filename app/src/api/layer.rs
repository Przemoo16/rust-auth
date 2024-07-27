use crate::db::connection::SessionStore;
use crate::libs::auth::Backend;
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use time::Duration;
use tower_sessions::{cookie::Key, service::SignedCookie, Expiry, SessionManagerLayer};

pub fn create_auth_layer(
    store: SessionStore,
    secret_key: &[u8],
    expiration: Duration,
) -> AuthManagerLayer<Backend, SessionStore, SignedCookie> {
    let session_layer = SessionManagerLayer::new(store)
        .with_expiry(Expiry::OnInactivity(expiration))
        .with_signed(Key::from(secret_key));
    let backend = Backend::new();
    AuthManagerLayerBuilder::new(backend, session_layer).build()
}
