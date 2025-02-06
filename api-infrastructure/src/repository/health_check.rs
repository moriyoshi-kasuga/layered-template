use api_domain::{model::HealthCheckModel, repository::health_check::HealthCheckRepository};
use api_shared::error::ApiResult;
use chrono::{DateTime, Utc};

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
        let server_time =
            sqlx::query_scalar::<_, DateTime<Utc>>("SELECT created_at FROM server_info LIMIT 1")
                .fetch_one(&self.db.pool())
                .await?;

        Ok(HealthCheckModel { server_time })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use api_domain::repository::health_check::HealthCheckRepository;
    use chrono::{DateTime, Utc};

    use crate::{
        persistence::db::DbPool,
        repository::{health_check::HealthCheckRepositoryImpl, MIGRATOR},
    };

    #[sqlx::test(migrator = "MIGRATOR", fixtures("health_check"))]
    fn health_check(pool: sqlx::SqlitePool) {
        let repo = HealthCheckRepositoryImpl::new(DbPool::from_pool(pool));
        let result = repo.health_check().await.unwrap();
        assert_eq!(
            result.server_time,
            DateTime::<Utc>::from_str("2025-02-05T22:49:22+09:00").unwrap(),
        )
    }
}
