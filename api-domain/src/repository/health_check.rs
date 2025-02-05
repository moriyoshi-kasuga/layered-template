use api_shared::error::ApiResult;

use crate::model::HealthCheckModel;

#[async_trait::async_trait]
#[cfg_attr(feature = "mockall", mockall::automock)]
pub trait HealthCheckRepository {
    async fn health_check(&self) -> ApiResult<HealthCheckModel>;
}
