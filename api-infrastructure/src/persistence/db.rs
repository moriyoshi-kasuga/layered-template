use api_domain::config::db::DbConfig;
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Clone)]
pub struct DbPool(sqlx::Pool<Sqlite>);

impl DbPool {
    #[allow(clippy::unwrap_used)]
    pub async fn new(config: &DbConfig) -> Self {
        let pool = SqlitePool::connect(&config.db_url).await.unwrap();
        Self(pool)
    }

    pub fn pool(&self) -> Pool<Sqlite> {
        self.0.clone()
    }

    pub async fn begin(&self) -> sqlx::Result<sqlx::Transaction<'static, Sqlite>> {
        self.0.begin().await
    }

    #[cfg(test)]
    pub fn from_pool(pool: sqlx::Pool<Sqlite>) -> Self {
        Self(pool)
    }
}
