use std::sync::Arc;

use api_application::modules::UseCaseModule;
use api_domain::modules::RepositoryModule;
use axum::{routing::get, Extension, Router};

use crate::router::health_check::get_health_check;

pub fn create_router<R: RepositoryModule>(use_case: Arc<UseCaseModule<R>>) -> Router {
    let health_check_router = Router::new().route("/", get(get_health_check::<R>));

    let api_router = Router::new().nest("/hc", health_check_router);

    Router::new()
        .nest("/api", api_router)
        .route_layer(Extension(use_case))
}
