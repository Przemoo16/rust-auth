use std::str::FromStr;
use tracing::Level;
use tracing_subscriber::fmt;

pub fn setup_tracing(log_level: &str) {
    fmt()
        .with_max_level(
            Level::from_str(&log_level).expect(&format!("Invalid log level {}", log_level)),
        )
        .compact()
        .init();
}
