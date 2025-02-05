use crate::repository::health_check::HealthCheckRepository;

#[cfg(feature = "mockall")]
use crate::repository::health_check::MockHealthCheckRepository;

#[cfg_attr(feature = "mockall", mockall::automock(
    type HealthCheckRepository = MockHealthCheckRepository;
))]
pub trait RepositoryModule: Send + Sync + 'static {
    type HealthCheckRepository: HealthCheckRepository;

    fn health_check_repository(&self) -> &Self::HealthCheckRepository;
}
