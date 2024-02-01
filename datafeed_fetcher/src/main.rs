use flate2::write::{DeflateEncoder, GzEncoder};
use flate2::Compression;
use rsmq_async::{Rsmq, RsmqConnection, RsmqOptions};
use std::io::{Error, Write};
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

    //let _ = rsmq.delete_queue(shared::DATAFEED_QUEUE_NAME).await;

    if let Err(e) = rsmq
        .create_queue(shared::DATAFEED_QUEUE_NAME, None, None, Some(-1))
        .await
    {
        println!("error creating queue");
    }

    if let Err(e) = rsmq
        .set_queue_attributes(shared::DATAFEED_QUEUE_NAME, None, None, Some(-1))
        .await
    {
        println!("could not set max size {}", e); // Note this errors out but seemling sets max size ok
    }

    if let Err(e) = rsmq.get_queue_attributes(shared::DATAFEED_QUEUE_NAME).await {
        println!("err {}", e);
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
        last_datafeed_update = latest_data.general.update.clone();

        let Ok(controllers) = serde_json::to_string(&latest_data.controllers) else {
            // todo println of error
            continue;
        };

        let now = Instant::now();
        let Ok(compressed) = gzip_compress(&controllers) else {
            // todo log error
            continue;
        };
        println!("took {} millis", (Instant::now() - now).as_millis());

        //println!("{:?}", compressed);

        // Send message to Redis with Controllers JSON
        let sent = rsmq
            .send_message::<Vec<u8>>(shared::DATAFEED_QUEUE_NAME, compressed, None)
            .await;
        if let Err(e) = sent {
            println!("{}", controllers);
            panic!("{}", e.to_string());
        }
        println!("Sent to Redis");

        // Sleep for 15 seconds minus the time this loop took
        let sleep_duration = Duration::from_secs(15) - (Instant::now() - start);
        println!("took {} seconds", (Instant::now() - start).as_secs());
        println!("sleeping for {:?}", sleep_duration);
        sleep(sleep_duration).await;
    }
}

pub fn gzip_compress(s: &str) -> Result<Vec<u8>, Error> {
    let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
    e.write_all(s.as_bytes())?;
    e.finish()
}
