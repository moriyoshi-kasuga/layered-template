use tracing::level_filters::LevelFilter;

pub fn init_log() {
    if cfg!(feature = "prod") {
        prod();
    } else {
        dev();
    }
}

fn prod() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();
}

fn dev() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let separator = "=".repeat(30);
    let message = format!("{:^30}", "!!Debug logging enabled!!");

    tracing::debug!("{}", separator);
    tracing::debug!("{}", message);
    tracing::debug!("{}", separator);
}
