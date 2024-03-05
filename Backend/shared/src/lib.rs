use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use vatsim_utils::models::Controller;

pub const DATAFEED_QUEUE_NAME: &str = "vatsim_datafeed";

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub db: u8,
    pub username: Option<String>,
    pub password: Option<String>,
    pub namespace: String,
    pub force_recreate: bool,
}

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    pub connection_string: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub redis: RedisConfig,
    pub postgres: PostgresConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisControllersMsg {
    pub update: DateTime<Utc>,
    pub controllers: Vec<Controller>,
}
