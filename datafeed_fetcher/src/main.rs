use rsmq_async::{Rsmq, RsmqConnection, RsmqOptions};
use shared;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use vatsim_utils::live_api::Vatsim;

#[tokio::main]
async fn main() {
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

    let mut rsmq = Rsmq::new(connection_options).await.expect("error");

    if let Err(e) = rsmq
        .create_queue(shared::DATAFEED_QUEUE_NAME, None, None, None)
        .await
    {
        todo!()
    }

    // Datafetcher infinite loop
    loop {
        let start = Instant::now();

        // Get data and check that there was no error
        let latest_data_result = api.get_v3_data().await;
        if let Err(e) = latest_data_result {
            dbg!(e);
            sleep(Duration::from_secs(1)).await;
            continue;
        };

        // Unwrap and check if duplicate from last fetch
        let latest_data = latest_data_result.unwrap();
        if latest_data.general.update == last_datafeed_update {
            dbg!("Found duplicate");
            sleep(Duration::from_secs(1)).await;
            continue;
        }

        // Update timestamp of latest data and process datafeed
        last_datafeed_update = latest_data.general.update.to_owned();

        // Send message to Redis with Controllers JSON
        let sent = rsmq
            .send_message(
                shared::DATAFEED_QUEUE_NAME,
                serde_json::to_string(&latest_data.controllers).unwrap(),
                None,
            )
            .await;
        if let Err(e) = sent {
            todo!()
        }

        // Sleep for 15 seconds minus the time this loop took
        let sleep_duration = Duration::from_secs(15) - (Instant::now() - start);
        sleep(sleep_duration).await;
    }
}
