use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use vatsim_utils::models::Controller;

pub const DATAFEED_QUEUE_NAME: &str = "vatsim_datafeed";

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Redis {
    host: String,
    port: u16,
    db: u8,
    realtime: bool,
    username: Option<String>,
    password: Option<String>,
    ns: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisControllersMsg {
    pub update: DateTime<Utc>,
    pub controllers: Vec<Controller>,
}
