use crate::db::connection::SessionStore;
use crate::libs::auth::Backend;
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use time::Duration;
use tower_sessions::{cookie::Key, service::SignedCookie, Expiry, SessionManagerLayer};

pub fn create_auth_layer(
    store: SessionStore,
    secret_key: &str,
    expiration: Duration,
) -> AuthManagerLayer<Backend, SessionStore, SignedCookie> {
    let decoded_key = STANDARD.decode(secret_key).expect("Invalid base64 key");
    let session_layer = SessionManagerLayer::new(store)
        .with_expiry(Expiry::OnInactivity(expiration))
        .with_signed(Key::from(&decoded_key));
    let backend = Backend::new();
    AuthManagerLayerBuilder::new(backend, session_layer).build()
}
