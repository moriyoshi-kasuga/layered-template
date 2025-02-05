use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct HealthCheckModel {
    pub server_time: DateTime<Local>,
}
