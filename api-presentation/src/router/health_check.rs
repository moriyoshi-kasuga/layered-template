use api_domain::modules::RepositoryModule;

use crate::{helper::api_response::ApiResponse, model::health_check::HealthCheckResponse};

use super::UseCaseExtension;

pub async fn get_health_check<R: RepositoryModule>(
    usecase: UseCaseExtension<R>,
) -> ApiResponse<HealthCheckResponse> {
    let result = usecase
        .health_check_use_case
        .health_check()
        .await
        .map(Into::into);

    ApiResponse::message("Health Check is passed", result)
}
