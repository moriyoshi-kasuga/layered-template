use tracing::level_filters::LevelFilter;

#[cfg(feature = "prod")]
pub fn init_log() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();
}

#[cfg(not(feature = "prod"))]
pub fn init_log() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let separator = "=".repeat(30);
    let message = format!("{:^30}", "!!Debug logging enabled!!");

    tracing::debug!("{}", separator);
    tracing::debug!("{}", message);
    tracing::debug!("{}", separator);
}
