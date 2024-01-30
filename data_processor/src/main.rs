use crate::vnas_api_models::ArtccRoot;
use chrono::Utc;
use db_models::{Artcc, VnasFetchRecord};
use futures::future::join_all;
use regex::Regex;
use rsmq_async::{Rsmq, RsmqConnection, RsmqError, RsmqOptions};
use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::num::ParseFloatError;
use vatsim_utils::models::Controller;
use vnas_aggregate_models::{AllFacilities, AllPositions, Callsign, PositionWithParentFacility};
use vnas_api::VnasApi;
use vnas_api_models::{Facility, Position};

mod db_models;
mod stats_models;
mod vnas_aggregate_models;
mod vnas_api;
mod vnas_api_models;

#[derive(Debug, thiserror::Error)]
enum DbInitError {
    #[error("could not connect to database")]
    DbError(#[from] sqlx::Error),

    #[error("could not apply migrations")]
    MigrationError(#[from] MigrateError),

    #[error("could not fetch data with reqwest")]
    ReqwestError(#[from] reqwest::Error),
}

async fn initialize_rsmq(queue_name: &str) -> Result<Rsmq, RsmqError> {
    let connection_options = RsmqOptions {
        host: "localhost".to_string(),
        port: 6379,
        db: 0,
        realtime: false,
        username: None,
        password: None,
        ns: "rsmq".to_string(),
    };

    let mut rsmq = Rsmq::new(connection_options).await?;
    let queues = rsmq.list_queues().await?;
    if !queues.contains(&queue_name.to_string()) {
        rsmq.create_queue(queue_name, None, None, None).await?
    }

    Ok(rsmq)
}

fn should_update_artcc(new_fetched_artcc: &ArtccRoot, existing_db_artccs: &[Artcc]) -> bool {
    let mut filtered = existing_db_artccs
        .iter()
        .filter(|a| a.id == new_fetched_artcc.id);

    match filtered.nth(0) {
        Some(existing_artcc) => &new_fetched_artcc.last_updated_at > &existing_artcc.last_updated,
        None => true,
    }
}

async fn update_artcc_in_db(pool: &Pool<Postgres>, artcc: &ArtccRoot) -> Result<(), sqlx::Error> {
    // Insert or update Artcc root
    sqlx::query(
        r"
        insert into artccs (id, last_updated)
        values ($1, $2)
        on conflict (id) do update set
            last_updated = excluded.last_updated;
        ",
    )
    .bind(&artcc.id)
    .bind(&artcc.last_updated_at)
    .execute(pool)
    .await?;

    // Insert or update all Facilities in Artcc
    for f in artcc.all_facilities_with_info() {
        sqlx::query(
            r"
            insert into facilities (id, name, type, last_updated, parent_facility_id, parent_artcc_id)
            values ($1, $2, $3, $4, $5, $6)
            on conflict (id) do update set
                name = excluded.name,
                type = excluded.type,
                last_updated = excluded.last_updated,
                parent_facility_id = excluded.parent_facility_id,
                parent_artcc_id = excluded.parent_artcc_id;
            ")
            .bind(&f.facility.id)
            .bind(&f.facility.name)
            .bind(&f.facility.type_field.to_string())
            .bind(&f.artcc_root.last_updated_at)
            .bind(&f.parent_facility.map(|p| p.id))
            .bind(&f.artcc_root.id)
            .execute(pool)
            .await?;
    }

    // Insert or update all Positions in Artcc
    for p in artcc.all_positions_with_parents() {
        sqlx::query(
            r"
            insert into positions (id, name, radio_name, callsign, callsign_prefix, callsign_infix, callsign_suffix, callsign_without_infix, frequency, parent_facility_id, last_updated)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            on conflict (id) do update set
                name = excluded.name,
                radio_name = excluded.radio_name,
                callsign = excluded.callsign,
                callsign_prefix = excluded.callsign_prefix,
                callsign_infix = excluded.callsign_infix,
                callsign_suffix = excluded.callsign_suffix,
                callsign_without_infix = excluded.callsign_without_infix,
                frequency = excluded.frequency,
                parent_facility_id = excluded.parent_facility_id,
                last_updated = excluded.last_updated;
            ")
            .bind(&p.position.id)
            .bind(&p.position.name)
            .bind(&p.position.radio_name)
            .bind(&p.position.callsign)
            .bind(&p.position.callsign_prefix())
            .bind(&p.position.callsign_infix())
            .bind(&p.position.callsign_suffix())
            .bind(format!("{}_{}", &p.position.callsign_prefix(), &p.position.callsign_suffix()))
            .bind(&p.position.frequency)
            .bind(&p.parent_facility.id)
            .bind(&artcc.last_updated_at)
            .execute(pool)
            .await?;
    }

    println!("Finished {}", artcc.id);

    Ok(())
}

async fn initialize_db(connection_string: &str) -> Result<Pool<Postgres>, DbInitError> {
    // Create Db connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string)
        .await?;

    // Run any new migrations
    let migrate = sqlx::migrate!("./migrations").run(&pool).await?;

    // Get record of latest vNAS data fetch. Update if none or stale data (>24 hours old)
    let latest_record = sqlx::query_as::<_, VnasFetchRecord>(
        "select id, update_time, success from vnas_fetch_records where success = true order by update_time desc;",
    )
    .fetch_optional(&pool)
    .await?;

    // Update if we've never initialized DB or haven't done it in 24 hours
    if latest_record.is_none()
        || (Utc::now() - latest_record.unwrap().update_time)
            > chrono::Duration::seconds(60 * 60 * 24)
    {
        let vnas = VnasApi::new().unwrap();
        let fetched_artccs = vnas.get_all_artccs_data().await?;

        let db_artccs = sqlx::query_as::<_, Artcc>("select * from artccs;")
            .fetch_all(&pool)
            .await?;

        let needs_update = fetched_artccs
            .iter()
            .filter(|a| should_update_artcc(a, &db_artccs));

        // Apply update to all Artccs that need update and await joined result
        let results = join_all(needs_update.map(|artcc| update_artcc_in_db(&pool, artcc))).await;

        // Store record of vNAS data check. If any errors, log as unsuccessful
        sqlx::query("insert into vnas_fetch_records (update_time, success) values ($1, $2);")
            .bind(Utc::now())
            .bind(!results.iter().any(|r| r.is_err()))
            .execute(&pool)
            .await?;
    }

    Ok(pool)
}

#[tokio::main]
async fn main() {
    // Overall flow
    // - Initialize DB if needed, and do initial fetch if no vNAS data fetches have been done
    // - Initialize Redis connection
    // - Start Redis message receive loop
    // - With message:
    // -    If no USA controllers online, check if last data fetch was more than 24 hours ago. If yes, fetch new data and update any ARTCCS that need updating
    // -    If USA controllers online, process existing active sessions (keep open or close) and add new sessions if needed
    // -    Aggregate stats

    // let mut rsmq = initialize_rsmq(shared::DATAFEED_QUEUE_NAME).await.unwrap();
    //
    // let mut num = 0;
    //
    // while let msg = rsmq
    //     .receive_message::<String>(shared::DATAFEED_QUEUE_NAME, None)
    //     .await
    // {
    //     if let Err(e) = &msg {
    //         println!("here");
    //         println!("Error receiving message")
    //     }
    //
    //     if let Some(message) = msg.unwrap() {
    //         num += 1;
    //         println!("got message {}", &num);
    //         let x: Vec<Controller> = serde_json::from_str(&message.message).unwrap();
    //
    //         println!("{}", x.len());
    //         _ = rsmq
    //             .delete_message(shared::DATAFEED_QUEUE_NAME, &message.id)
    //             .await;
    //     }
    // }

    let x = initialize_db("postgres://postgres:pw@localhost/ironmic").await;
    if let Err(e) = x {
        println!("{}", e)
    }
}

// async fn main() -> Result<(), Error> {
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect("postgres://postgres:pw@localhost/ironmic")
//         .await
//         .expect("Error creating db pool");
//
//     let migrate = sqlx::migrate!("./migrations")
//         .run(&pool)
//         .await
//         .expect("Error with migrations");
//
//     let x = VnasApi::new().unwrap();
//     let all_artccs = x.get_all_artccs_data().await?;
//
//     let all_facilities_info: Vec<FacilityWithTreeInfo> = all_artccs
//         .iter()
//         .flat_map(|f| f.all_facilities_with_info())
//         .collect();
//
//     let start = Instant::now();
//     for artcc in all_artccs.iter() {
//         let d = DateTime::parse_from_rfc3339(&artcc.last_updated_at).unwrap();
//
//         let row = sqlx::query("insert into artccs (id, last_updated) values ($1, $2);")
//             .bind(&artcc.id)
//             .bind(d)
//             .execute(&pool)
//             .await
//             .expect("Error inserting");
//     }
//
//     for f in all_facilities_info.iter() {
//         let d = DateTime::parse_from_rfc3339(&f.artcc_root.last_updated_at).unwrap();
//         let parent_facility_id_str = if let Some(s) = &f.parent_facility {
//             Some(&s.id)
//         } else {
//             None
//         };
//
//         let row = sqlx::query("insert into facilities (id, name, type, last_updated, parent_facility_id, parent_artcc_id) values ($1, $2, $3, $4, $5, $6);")
//             .bind(&f.facility.id)
//             .bind(&f.facility.name)
//             .bind(&f.facility.type_field.to_string())
//             .bind(d)
//             .bind(parent_facility_id_str)
//             .bind(&f.artcc_root.id)
//             .execute(&pool)
//             .await
//             .expect("Error inserting");
//     }
//
//     for p in all_artccs
//         .iter()
//         .flat_map(|f| f.all_positions_with_parents())
//     {
//         let row = sqlx::query("insert into positions (id, name, radio_name, callsign, callsign_prefix, callsign_infix, callsign_suffix, callsign_without_infix, frequency, parent_facility_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10);")
//             .bind(&p.position.id)
//             .bind(&p.position.name)
//             .bind(&p.position.radio_name)
//             .bind(&p.position.callsign)
//             .bind(&p.position.callsign_prefix())
//             .bind(&p.position.callsign_infix())
//             .bind(&p.position.callsign_suffix())
//             .bind(format!("{}_{}", &p.position.callsign_prefix(), &p.position.callsign_suffix()))
//             .bind(&p.position.frequency)
//             .bind(&p.parent_facility.id)
//             .execute(&pool)
//             .await
//             .expect("Error inserting");
//     }
//
//     // sqlx::query("insert into positions (id, name, radio_name, callsign, callsign_prefix, callsign_infix, callsign_suffix, callsign_without_infix, frequency, parent_facility_id, parent_artcc_id) select * from unnest
//     println!("DB time in micros {}", start.elapsed().as_micros());
//     println!("Length {}", all_facilities_info.len());
//
//     let api = Vatsim::new().await.unwrap();
//     let latest_data_result = api.get_v3_data().await.unwrap();
//
//     let start = Instant::now();
//
//     let position_matchers: Vec<PositionMatcher> = all_artccs
//         .iter()
//         .flat_map(|f| f.all_positions_with_parents())
//         .map(|p| PositionMatcher::from(p))
//         .collect();
//
//     println!("{}", start.elapsed().as_micros());
//     println!("here");
//
//     println!("{}", position_matchers.len());
//     println!("{}", latest_data_result.controllers.len());
//
//     for controller in latest_data_result
//         .controllers
//         .into_iter()
//         .filter(|c| is_active_vnas_controller(c))
//     {
//         println!("Trying {} with CID {}", controller.callsign, controller.cid);
//         let time = Instant::now();
//
//         let x = single_or_no_match(&position_matchers, &controller);
//
//         if let Some(pm) = x {
//             let y = ControllerSession::try_from((pm, &controller));
//             println!(
//                 "Found match for {} - {} parent {} with {}",
//                 pm.position.name,
//                 pm.position.callsign,
//                 pm.parent_facility.name,
//                 controller.callsign
//             )
//         } else {
//             println!("No single match found for {}", controller.callsign)
//         }
//
//         //
//         // for pm in position_matchers.as_slice() {
//         //     if pm.is_match(&controller) {
//         //         println!(
//         //             "Found match for {} - {} parent {} with {}",
//         //             pm.position.name,
//         //             pm.position.callsign,
//         //             pm.parent_facility.name,
//         //             controller.callsign
//         //         );
//         //         // let z: ControllerSession = ControllerSessionBuilder::default()
//         //         //     .start_time(
//         //         //         DateTime::parse_from_rfc3339(&controller.logon_time)
//         //         //             .unwrap()
//         //         //             .to_utc(),
//         //         //     )
//         //         //     .build();
//         //         // let q: ControllerSession = ControllerSession::builder()
//         //         //     .start_time(
//         //         //         DateTime::parse_from_rfc3339(&controller.logon_time)
//         //         //             .unwrap()
//         //         //             .to_utc(),
//         //         //     )
//         //         //     .cid(controller.cid)
//         //         //     .build();
//         //         let q = ControllerSession::try_from((pm, &controller));
//         //     }
//         // }
//         println!("{}", time.elapsed().as_millis());
//     }
//     // let online_positions: Vec<&Controller> = latest_data_result
//     //     .controllers
//     //     .iter()
//     //     .filter(|c| flat_positions.iter().any(|p| p.is_match_for(&c.callsign)))
//     //     .collect();
//
//     // let mut positions: Vec<Position> = vec![];
//     // if let Ok(z) = y {
//     //     z.iter().for_each(|a| positions.extend(a.all_positions()))
//     // }
//     //
//     // println!("{}", positions.len());
//     // println!("{}", start.elapsed().as_micros());
//
//     Ok(())
// }

pub fn is_active_vnas_controller(c: &Controller) -> bool {
    c.server == "VIRTUALNAS" && c.facility > 0 && c.frequency != "199.998"
}

pub struct PositionMatcher {
    pub parent_facility: Facility,
    pub position: Position,
    pub regex: Regex,
}

impl PositionMatcher {
    pub fn is_match(&self, controller: &Controller) -> bool {
        self.regex.is_match(&controller.callsign)
            && if let Ok(b) = self.is_freq_match(&controller.frequency) {
                b
            } else {
                dbg!("Error parsing VATSIM freq {}", &controller.frequency);
                false
            }
    }

    pub fn is_starred_match(&self, controller: &Controller) -> bool {
        self.is_match(controller) && self.position.starred
    }

    fn is_freq_match(&self, vatsim_freq_str: &str) -> Result<bool, ParseFloatError> {
        let vatsim_freq_f = vatsim_freq_str.parse::<f64>();
        if let Ok(f) = vatsim_freq_f {
            let vatsim_freq_i64 = (f * 1e6).round() as i64;
            Ok(self.position.frequency == vatsim_freq_i64)
        } else {
            Err(vatsim_freq_f.unwrap_err())
        }
    }
}

fn single_or_no_match<'a>(
    matchers: &'a [PositionMatcher],
    controller: &Controller,
) -> Option<&'a PositionMatcher> {
    let mut matched: Vec<&PositionMatcher> = vec![];
    for matcher in matchers {
        if matcher.is_match(&controller) {
            matched.push(matcher);
        }
    }
    if matched.len() == 1 {
        Some(matched[0])
    } else {
        matched.retain(|p| p.position.starred);
        if matched.len() == 1 {
            Some(matched[0])
        } else {
            None
        }
    }
}

fn all_matches<'a>(
    matchers: &'a [PositionMatcher],
    controller: &Controller,
) -> Option<Vec<&'a PositionMatcher>> {
    let x: Vec<&PositionMatcher> = matchers.iter().filter(|m| m.is_match(controller)).collect();
    if x.is_empty() {
        None
    } else {
        Some(x)
    }
}

impl From<PositionWithParentFacility> for PositionMatcher {
    fn from(value: PositionWithParentFacility) -> Self {
        PositionMatcher {
            parent_facility: value.parent_facility,
            position: value.position.clone(),
            regex: value.position.build_match_regex().unwrap(),
        }
    }
}
