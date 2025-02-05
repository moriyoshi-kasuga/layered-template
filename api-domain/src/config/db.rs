#[derive(Debug, envman::EnvMan)]
pub struct DbConfig {
    #[envman(default = "sqlite::memory:".to_string())]
    pub db_url: String,
}
