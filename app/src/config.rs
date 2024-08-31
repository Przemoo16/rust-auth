use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::env::var;

pub struct Config {
    pub auth: AuthConfig,
    pub db: DatabaseConfig,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            auth: AuthConfig::from_env(),
            db: DatabaseConfig::from_env(),
        }
    }
}

pub struct AuthConfig {
    pub secret_key: Vec<u8>,
    pub session_expiration_minutes: i64,
    pub delete_expired_sessions_interval_seconds: u64,
}

impl AuthConfig {
    pub fn from_env() -> Self {
        Self {
            secret_key: decode_base64(&read_env("AUTH_SECRET_KEY")),
            session_expiration_minutes: read_env("AUTH_SESSION_EXPIRATION_MINUTES")
                .parse()
                .expect("AUTH_SESSION_EXPIRATION_MINUTES must be a number"),
            delete_expired_sessions_interval_seconds: read_env(
                "AUTH_DELETE_EXPIRED_SESSIONS_INTERVAL_SECONDS",
            )
            .parse()
            .expect("AUTH_DELETE_EXPIRED_SESSIONS_INTERVAL_SECONDS must be a number"),
        }
    }
}

pub struct DatabaseConfig {
    pub url: String,
    pub pool_max_connections: u32,
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        Self {
            url: read_env("DATABASE_URL"),
            pool_max_connections: read_env("DATABASE_POOL_MAX_CONNECTIONS")
                .parse()
                .expect("DATABASE_POOL_MAX_CONNECTIONS must be a number"),
        }
    }
}

fn read_env(key: &str) -> String {
    var(key).expect(&format!("Failed to read the {} env variable", key))
}

fn decode_base64(string: &str) -> Vec<u8> {
    STANDARD.decode(string).expect("Invalid base64 string")
}
