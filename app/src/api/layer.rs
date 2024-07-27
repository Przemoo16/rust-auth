use crate::db::connection::{Database, SessionStore};
use crate::libs::auth::Backend;
use axum_login::{AuthManagerLayer, AuthManagerLayerBuilder};
use time::Duration;
use tower_sessions::{cookie::Key, service::SignedCookie, Expiry, SessionManagerLayer};

pub fn create_auth_layer(
    session_store: SessionStore,
    db: Database,
    secret_key: &[u8],
    expiration: Duration,
) -> AuthManagerLayer<Backend, SessionStore, SignedCookie> {
    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(expiration))
        .with_signed(Key::from(secret_key));
    let backend = Backend::new(db);
    AuthManagerLayerBuilder::new(backend, session_layer).build()
}
