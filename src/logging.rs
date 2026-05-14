use tracing_subscriber::{fmt, EnvFilter};

pub fn init_logging(default_filter: &str) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));

    let _ = fmt()
        .with_env_filter(filter)
        .json()
        .with_target(true)
        .try_init();
}
