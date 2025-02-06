use api_domain::model::HealthCheckModel;
use chrono::{DateTime, Utc};
use more_convert::Convert;

#[derive(Debug, Convert)]
#[convert(from(HealthCheckModel))]
pub struct HealthCheckDto {
    pub server_time: DateTime<Utc>,
}
