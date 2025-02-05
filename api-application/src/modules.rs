use std::sync::Arc;

use api_domain::modules::RepositoryModule;

use crate::usecase::health_check::HealthCheckUseCase;

pub struct UseCaseModule<R: RepositoryModule> {
    pub health_check_use_case: HealthCheckUseCase<R>,
}

impl<R: RepositoryModule> UseCaseModule<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self {
            health_check_use_case: HealthCheckUseCase::new(Arc::clone(&repository)),
        }
    }
}
