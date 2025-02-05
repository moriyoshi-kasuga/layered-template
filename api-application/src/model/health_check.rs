use api_domain::model::HealthCheckModel;
use chrono::{DateTime, Local};
use more_convert::Convert;

#[derive(Debug, Convert)]
#[convert(from(HealthCheckModel))]
pub struct HealthCheckDto {
    pub server_time: DateTime<Local>,
}
