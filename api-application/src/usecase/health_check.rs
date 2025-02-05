use std::sync::Arc;

use api_domain::{modules::RepositoryModule, repository::health_check::HealthCheckRepository};
use api_shared::error::ApiResult;

use crate::model::health_check::HealthCheckDto;

pub struct HealthCheckUseCase<R: RepositoryModule> {
    repository: Arc<R>,
}

impl<R: RepositoryModule> HealthCheckUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn health_check(&self) -> ApiResult<HealthCheckDto> {
        self.repository
            .health_check_repository()
            .health_check()
            .await
            .map(Into::into)
    }
}
