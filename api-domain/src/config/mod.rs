use std::net::SocketAddr;

use db::DbConfig;
use envman::EnvMan;

pub mod db;

#[derive(Debug, envman::EnvMan)]
pub struct ApiConfig {
    #[envman(default = "127.0.0.1:3000", alltime_parse)]
    pub server_addr: SocketAddr,

    #[envman(nest)]
    pub db: DbConfig,
}

impl ApiConfig {
    #[allow(clippy::unwrap_used)]
    pub fn init() -> Self {
        ApiConfig::load().unwrap()
    }
}
