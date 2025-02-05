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
}
