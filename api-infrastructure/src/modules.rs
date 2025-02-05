use api_domain::modules::RepositoryModule;

use crate::{persistence::db::DbPool, repository::health_check::HealthCheckRepositoryImpl};

pub struct RepositoryModuleImpl {
    health_check_repository: HealthCheckRepositoryImpl,
}

impl RepositoryModuleImpl {
    pub fn new(db: DbPool) -> Self {
        Self {
            health_check_repository: HealthCheckRepositoryImpl::new(db),
        }
    }
}

impl RepositoryModule for RepositoryModuleImpl {
    type HealthCheckRepository = HealthCheckRepositoryImpl;

    fn health_check_repository(&self) -> &Self::HealthCheckRepository {
        &self.health_check_repository
    }
}
