use crate::repository::health_check::HealthCheckRepository;

pub trait RepositoryModule: Send + Sync + 'static {
    type HealthCheckRepository: HealthCheckRepository;

    fn health_check_repository(&self) -> &Self::HealthCheckRepository;
}
