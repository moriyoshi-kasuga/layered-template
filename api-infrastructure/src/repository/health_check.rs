use api_domain::{model::HealthCheckModel, repository::health_check::HealthCheckRepository};
use api_shared::error::ApiResult;
use chrono::Local;

use crate::persistence::db::DbPool;

pub struct HealthCheckRepositoryImpl {
    db: DbPool,
}

impl HealthCheckRepositoryImpl {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn health_check(&self) -> ApiResult<HealthCheckModel> {
        let _ = self.db.pool().acquire().await?;

        Ok(HealthCheckModel {
            server_time: Local::now(),
        })
    }
}
