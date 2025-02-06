use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct HealthCheckModel {
    pub server_time: DateTime<Utc>,
}
