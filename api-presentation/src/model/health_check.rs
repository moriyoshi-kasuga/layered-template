use api_application::model::health_check::HealthCheckDto;
use chrono::{DateTime, Local};
use more_convert::Convert;
use serde::Serialize;

#[derive(Debug, Serialize, Convert)]
#[serde(rename_all = "camelCase")]
#[convert(from(HealthCheckDto))]
pub struct HealthCheckResponse {
    pub server_time: DateTime<Local>,
}
