use chrono::{DateTime, Utc};
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use flate2::write::DeflateEncoder;
use flate2::Compression;
use rsmq_async::{Rsmq, RsmqConnection, RsmqError, RsmqOptions};
use shared::{Config, RedisControllersMsg};
use std::cmp::min;
use std::io::{Error, Write};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::{debug, error, warn};
use vatsim_utils::live_api::Vatsim;
use vatsim_utils::models::Controller;

#[tokio::main]
async fn main() -> Result<(), SetGlobalDefaultError> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .json()
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // Set up config
    let config = match Figment::new()
        .merge(Toml::file("Settings.toml"))
        .merge(Env::prefixed("STATUSA_").split("_"))
        .extract::<Config>()
    {
        Ok(config) => config,
        Err(e) => {
            error!(error = ?e, "Configuration could not be initialized");
            panic!("Configuration could not be initialized")
        }
    };

    // Set up VATSIM Datafeed
    let mut last_datafeed_update = String::new();
    let api = Vatsim::new()
        .await
        .expect("Could not initialize VATSIM API");

    // Set up Redis Queue based on configuration
    let connection_options = RsmqOptions {
        host: config.redis.host,
        port: config.redis.port,
        db: config.redis.db,
        realtime: false,
        username: config.redis.username,
        password: config.redis.password,
        ns: config.redis.namespace,
    };

    let mut rsmq = match initialize_rsmq(connection_options, config.redis.force_recreate).await {
        Ok(rsmq) => rsmq,
        Err(e) => {
            error!(error = ?e, "RSMQ could not be initialized");
            panic!("RSMQ could not be initialized")
        }
    };

    // Datafetcher infinite loop
    loop {
        let start = Instant::now();

        // Get data and check that there was no error
        let latest_data_result = api.get_v3_data().await;
        if let Err(e) = latest_data_result {
            warn!(error = ?e, "Could not fetch VATSIM data");
            sleep(Duration::from_secs(1)).await;
            continue;
        };

        // Unwrap and check if duplicate from last fetch
        // Safe to unwrap because checked Err case above already
        let latest_data = latest_data_result.expect("Error getting VATSIM API data");

        if latest_data.general.update == last_datafeed_update {
            debug!(time = %latest_data.general.update, "Found duplicate");
            sleep(Duration::from_secs(3)).await;
            continue;
        }

        // Update timestamp of latest data and process datafeed
        last_datafeed_update = latest_data.general.update.clone();

        let update_timestamp = if let Ok(update_timestamp) =
            DateTime::parse_from_rfc3339(&latest_data.general.update_timestamp)
        {
            update_timestamp.to_utc()
        } else {
            warn!(
                timestamp = latest_data.general.update_timestamp,
                "Could not parse timestamp"
            );
            continue;
        };

        let Ok(compressed) = compress(update_timestamp, latest_data.controllers) else {
            warn!("Could not compress");
            continue;
        };

        // Send message to Redis with Controllers JSON
        let sent = rsmq
            .send_message::<Vec<u8>>(shared::DATAFEED_QUEUE_NAME, compressed, None)
            .await;
        if let Err(e) = sent {
            warn!(error = ?e, "Could not send message to Redis");
            // No continue here because at this point we want to sleep for 5 seconds
        }

        // Sleep for 5 seconds minus the time this loop took, with some protections to make sure we
        // don't have a negative duration
        let loop_time = Instant::now() - start;
        if loop_time > Duration::from_secs(4) {
            warn!(?loop_time, "Long loop");
        }
        let sleep_duration = Duration::from_secs(5) - min(Duration::from_secs(4), loop_time);
        debug!(?sleep_duration, "Sleeping");
        sleep(sleep_duration).await;
    }
}

async fn initialize_rsmq(
    connection_options: RsmqOptions,
    force_recreate: bool,
) -> Result<Rsmq, RsmqError> {
    let mut rsmq = Rsmq::new(connection_options).await?;
    let queues = rsmq.list_queues().await?;

    let queue_exists = queues.contains(&shared::DATAFEED_QUEUE_NAME.to_string());
    if queue_exists && force_recreate {
        rsmq.delete_queue(shared::DATAFEED_QUEUE_NAME).await?;
        rsmq.create_queue(shared::DATAFEED_QUEUE_NAME, None, None, Some(-1))
            .await?;
    } else if queue_exists {
        rsmq.set_queue_attributes(shared::DATAFEED_QUEUE_NAME, None, None, Some(-1))
            .await?;
    } else {
        rsmq.create_queue(shared::DATAFEED_QUEUE_NAME, None, None, Some(-1))
            .await?
    }

    Ok(rsmq)
}

fn compress(update: DateTime<Utc>, controllers: Vec<Controller>) -> Result<Vec<u8>, Error> {
    let msg = RedisControllersMsg {
        update,
        controllers,
    };

    let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
    let s = serde_json::to_string(&msg)?;
    e.write_all(s.as_bytes())?;
    e.finish()
}
