use api_domain::config::ApiConfig;
use axum::Router;
use tokio::{net::TcpListener, signal};

pub mod log;
pub mod router;

pub async fn create_server(config: &ApiConfig, router: Router) {
    let listener = TcpListener::bind(config.server_addr).await.unwrap();

    tracing::info!("API Server listening on {:?}", listener);

    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|_| panic!("API Server cannot launch"))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("API Server shutdown signal received");
        },
        _ = terminate => {
            tracing::info!("API Server terminate signal received");
        },
    }
}
