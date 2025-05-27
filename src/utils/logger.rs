use tracing_subscriber::{fmt, EnvFilter};

pub fn init() {
    let filter = EnvFilter::from_default_env();
    fmt()
        .with_env_filter(filter)
        .init();
}
