use flate2::write::DeflateEncoder;
use flate2::Compression;
use rsmq_async::{Rsmq, RsmqConnection, RsmqError, RsmqOptions};
use std::io::{Error, Write};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::{debug, error, warn};
use vatsim_utils::live_api::Vatsim;

#[tokio::main]
async fn main() -> Result<(), SetGlobalDefaultError> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // Set up VATSIM Datafeed
    let mut last_datafeed_update = String::new();
    let api = Vatsim::new().await.unwrap();

    // Set up Redis Queue
    let connection_options = RsmqOptions {
        host: "localhost".to_string(),
        port: 6379,
        db: 0,
        realtime: false,
        username: None,
        password: None,
        ns: "rsmq".to_string(),
    };

    let mut rsmq = match initialize_rsmq(connection_options, false).await {
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
        let latest_data = latest_data_result.unwrap();
        if latest_data.general.update == last_datafeed_update {
            debug!(time = %latest_data.general.update, "Found duplicate");
            sleep(Duration::from_secs(1)).await;
            continue;
        }

        // Update timestamp of latest data and process datafeed
        last_datafeed_update = latest_data.general.update.clone();

        let Ok(controllers) = serde_json::to_string(&latest_data.controllers) else {
            warn!("Could not deserialize JSON from vatsim_utils");
            continue;
        };

        let Ok(compressed) = compress(&controllers) else {
            warn!("Could not compress");
            continue;
        };

        // Send message to Redis with Controllers JSON
        let sent = rsmq
            .send_message::<Vec<u8>>(shared::DATAFEED_QUEUE_NAME, compressed, None)
            .await;
        if let Err(e) = sent {
            warn!(error = ?e, "Could not send message to Redis");
            // No continue here because at this point we want to sleep for 15 seconds
        }

        // Sleep for 15 seconds minus the time this loop took
        let sleep_duration = Duration::from_secs(15) - (Instant::now() - start);
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

fn compress(s: &str) -> Result<Vec<u8>, Error> {
    let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
    e.write_all(s.as_bytes())?;
    e.finish()
}
