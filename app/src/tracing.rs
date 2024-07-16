use tracing_subscriber::fmt;

pub fn setup_tracing() {
    fmt().compact().init();
}
