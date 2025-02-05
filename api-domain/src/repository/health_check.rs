use api_shared::error::ApiResult;

use crate::model::HealthCheckModel;

#[async_trait::async_trait]
pub trait HealthCheckRepository {
    async fn health_check(&self) -> ApiResult<HealthCheckModel>;
}
