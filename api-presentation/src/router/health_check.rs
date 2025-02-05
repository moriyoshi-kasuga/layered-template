use api_domain::modules::RepositoryModule;
use api_shared::error::ApiResult;

use crate::{model::health_check::HealthCheckResponse, UseCaseExtension};

pub async fn get_health_check<R: RepositoryModule>(
    usecase: UseCaseExtension<R>,
) -> ApiResult<HealthCheckResponse> {
    usecase
        .health_check_use_case
        .health_check()
        .await
        .map(Into::into)
}
