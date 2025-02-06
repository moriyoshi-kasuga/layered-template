pub mod health_check;

#[cfg(test)]
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations");
