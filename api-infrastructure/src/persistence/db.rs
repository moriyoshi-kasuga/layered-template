use api_domain::config::db::DbConfig;
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Clone)]
pub struct DbPool(sqlx::Pool<Sqlite>);

impl DbPool {
    pub async fn new(config: &DbConfig) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(&config.db_url).await?;
        Ok(Self(pool))
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
