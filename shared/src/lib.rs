use serde::Deserialize;

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
