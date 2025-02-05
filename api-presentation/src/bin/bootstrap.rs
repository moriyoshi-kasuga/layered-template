use std::sync::Arc;

use api_application::modules::UseCaseModule;
use api_domain::config::ApiConfig;
use api_infrastructure::{modules::RepositoryModuleImpl, persistence::db::DbPool};
use api_presentation::startup::{create_server, log::init_log, router::create_router};

#[tokio::main]
pub async fn main() {
    let config = ApiConfig::init();
    let db = DbPool::new(&config.db).await;

    let repository = RepositoryModuleImpl::new(db);
    let repository = Arc::new(repository);
    let use_case = UseCaseModule::new(repository);
    let use_case = Arc::new(use_case);

    init_log();

    let router = create_router(use_case);
    create_server(&config, router).await;
}
