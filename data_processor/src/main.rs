use crate::database::{
    db_get_active_controller_sessions, db_get_active_position_sessions,
    db_update_active_controller_session, db_update_active_position_session,
    db_update_ended_controller_session, db_update_ended_position_session, db_update_vnas_artcc,
    db_update_vnas_facility, db_update_vnas_position,
};
use crate::db_models::{ControllerSession, PositionSession};
use crate::matchers::{all_matches, PositionMatcher};
use crate::stats_models::{ControllerSessionTracker, PositionSessionTracker};
use crate::vnas_api_models::ArtccRoot;
use chrono::{DateTime, Utc};
use db_models::{Artcc, VnasFetchRecord};
use flate2::read::{DeflateDecoder, GzDecoder};
use futures::future::join_all;
use matchers::single_or_no_match;
use rsmq_async::{Rsmq, RsmqConnection, RsmqError, RsmqOptions};
use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::io::{Error, Read};
use uuid::Uuid;
use vatsim_utils::models::Controller;
use vnas_aggregate_models::{AllPositions, Callsign};
use vnas_api::VnasApi;

mod database;
mod db_models;
mod matchers;
mod stats_models;
mod vnas_aggregate_models;
mod vnas_api;
mod vnas_api_models;

#[derive(Debug, thiserror::Error)]
enum DbInitError {
    #[error("error with database")]
    DbError(#[from] sqlx::Error),

    #[error("error updated vNAS data")]
    VnasDataUpdateError(#[from] VnasDataUpdateError),

    #[error("could not apply migrations")]
    MigrationError(#[from] MigrateError),
}

#[derive(Debug, thiserror::Error)]
enum VnasDataUpdateError {
    #[error("error with database")]
    DbError(#[from] sqlx::Error),

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

    match filtered.next() {
        Some(existing_artcc) => new_fetched_artcc.last_updated_at > existing_artcc.last_updated,
        None => true,
    }
}

async fn update_artcc_in_db(pool: &Pool<Postgres>, artcc: &ArtccRoot) -> Result<(), sqlx::Error> {
    // Insert or update Artcc root
    db_update_vnas_artcc(pool, artcc).await?;

    // Insert or update all Facilities in Artcc
    for f in artcc.all_facilities_with_info() {
        db_update_vnas_facility(pool, &f).await?;
    }

    // Insert or update all Positions in Artcc
    for p in artcc.all_positions_with_parents() {
        db_update_vnas_position(pool, &p, artcc).await?;
    }

    Ok(())
}

async fn update_all_artccs_in_db(
    pool: &Pool<Postgres>,
    force_update: bool,
) -> Result<Option<Vec<PositionMatcher>>, VnasDataUpdateError> {
    // Get record of latest vNAS data fetch. Update if none or stale data (>24 hours old)
    let latest_record = sqlx::query_as::<_, VnasFetchRecord>(
        "select id, update_time, success from vnas_fetch_records where success = true order by update_time desc;",
    )
        .fetch_optional(pool)
        .await?;

    // Update if we've never initialized DB or haven't done it in 24 hours, or we want to force update
    if latest_record.is_none()
        || (Utc::now() - latest_record.unwrap().update_time)
            > chrono::Duration::seconds(60 * 60 * 24)
        || force_update
    {
        let vnas = VnasApi::new().unwrap();
        let fetched_artccs = vnas.get_all_artccs_data().await?;

        let db_artccs = sqlx::query_as::<_, Artcc>("select * from artccs;")
            .fetch_all(pool)
            .await?;

        let needs_update = fetched_artccs
            .iter()
            .filter(|a| should_update_artcc(a, &db_artccs));

        // Apply update to all Artccs that need update and await joined result
        let results = join_all(needs_update.map(|artcc| update_artcc_in_db(pool, artcc))).await;

        // Store record of vNAS data check. If any errors, log as unsuccessful
        sqlx::query("insert into vnas_fetch_records (update_time, success) values ($1, $2);")
            .bind(Utc::now())
            .bind(!results.iter().any(|r| r.is_err()))
            .execute(pool)
            .await?;

        let position_matchers: Vec<PositionMatcher> = fetched_artccs
            .iter()
            .flat_map(|f| f.all_positions_with_parents())
            .map(PositionMatcher::from)
            .collect();

        return Ok(Some(position_matchers));
    }

    Ok(None)
}

async fn initialize_db(connection_string: &str) -> Result<Pool<Postgres>, DbInitError> {
    // Create Db connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string)
        .await?;

    // Run any new migrations
    let migrate = sqlx::migrate!("./migrations").run(&pool).await;

    if let Err(e) = migrate {
        println!("Error {}", e)
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

    // TODO -- need to capture and return error
    let Ok(db_pool) = initialize_db("postgres://postgres:pw@localhost/ironmic").await else {
        panic!("Could not initialize DB connection pool")
    };

    // Initialized position matchers
    let Ok(Some(mut position_matchers)) = update_all_artccs_in_db(&db_pool, true).await else {
        panic!("Could not initialize DB position matchers")
    };

    // TODO -- need to capture and return error
    let Ok(mut rsmq) = initialize_rsmq(shared::DATAFEED_QUEUE_NAME).await else {
        panic!("Could not initialize Redis connection")
    };

    while let msg = rsmq
        .receive_message::<Vec<u8>>(shared::DATAFEED_QUEUE_NAME, None)
        .await
    {
        if let Err(e) = &msg {
            println!("here");
            println!("Error receiving message")
        }

        if let Some(message) = msg.unwrap() {
            let decompressed = gzip_decompress(&message.message).unwrap(); // todo strengthen parsing safety

            let controllers: Vec<Controller> = serde_json::from_str(&decompressed).unwrap(); // TODO -- strengthen parsing safety
            let vnas_controllers: Vec<&Controller> = controllers
                .iter()
                .filter(|c| is_active_vnas_controller(c))
                .collect();

            if vnas_controllers.is_empty() {
                if let Ok(Some(new_pms)) = update_all_artccs_in_db(&db_pool, false).await {
                    position_matchers = new_pms
                }
            } else {
                let x = process_datafeed(vnas_controllers, &position_matchers, &db_pool).await;
                if let Err(e) = x {
                    println!("{}", e)
                }
            }

            _ = rsmq
                .delete_message(shared::DATAFEED_QUEUE_NAME, &message.id)
                .await;
        }
    }
}

// async fn process_datafeed(
//     datafeed_controllers: Vec<&Controller>,
//     position_matchers: &Vec<PositionMatcher>,
//     pool: &Pool<Postgres>,
// ) {
//     let active_controller_sessions = get_active_controller_sessions_from_db(pool).await?;
//     let active_position_sessions = get_active_position_sessions_from_db(pool).await?;
//
//     let position_session_id_hashmap
//
// }

async fn process_datafeed(
    datafeed_controllers: Vec<&Controller>,
    position_matchers: &[PositionMatcher],
    pool: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    //let existing_active_controller_sessions = get

    // Get all existing active controllers in DB as vector. Convert to Hashmap
    // Get all existing position sessions in DB as vector. Convert to Hashmap
    // Get all active controllers from datafeed as vector
    // For each controller in datafeed
    //      - If controller already exists in Hashmap (i.e., already marked active), update last_updated time for controller session and associated positon session
    //              - Mark controller as still active in existing controllers Hashmap
    //              - Mark position session as still active in existing position sessions Hashmap
    //      - If controller does not exist
    //              - Check to see if position session exists. If no, create new and tag as still active. If yes, tag as still active
    //              - Create new controller session (with PostionMatcher) and tag as still active and associate with position session and add to Hashmap
    // For each position session in Hashmap
    //      - If not tagged active, mark ended
    // For each controller session in Hashmap
    //      - If not tagged active, mark ended
    // Write all positions and controller sessions to DB (including active / not active state)

    let mut active_controller_sessions: HashMap<_, _> = db_get_active_controller_sessions(pool)
        .await?
        .into_iter()
        .map(|c| {
            (
                format!("{} {}", c.cid, c.start_time.timestamp()),
                ControllerSessionTracker::new(c),
            )
        })
        .collect();

    let mut active_position_sessions: HashMap<_, _> = db_get_active_position_sessions(pool)
        .await?
        .into_iter()
        .map(|p| {
            (
                p.position_simple_callsign.clone(),
                PositionSessionTracker::new(p.clone()),
            )
        })
        .collect();

    // TODO -- need to load all of the existing controlller sessions into position sessions

    for datafeed_controller in datafeed_controllers {
        let parsed_time = DateTime::parse_from_rfc3339(&datafeed_controller.logon_time)
            .unwrap_or(DateTime::default())
            .to_utc();
        let controller_key = format!("{} {}", datafeed_controller.cid, parsed_time.timestamp());
        println!("looking: {}", controller_key);

        // for k in active_controller_sessions.keys() {
        //     let x = k.clone();
        //     println!("key {}", x)
        // }

        if let Some(controller_session_tracker) =
            active_controller_sessions.get_mut(&controller_key)
        {
            println!(
                "Found existing controller {} on {}",
                datafeed_controller.cid, datafeed_controller.callsign
            );

            controller_session_tracker.mark_active();
            controller_session_tracker.update_from(datafeed_controller);

            // Find Position based on "simple callsign" (no infix) and mark as active
            if let Some(position_session_tracker) =
                active_position_sessions.get_mut(&datafeed_controller.simple_callsign())
            {
                position_session_tracker.mark_active();
                position_session_tracker.update_from(datafeed_controller);
            }
        } else {
            // TODO -- see if there is position linked to this simple callsign. If not, create one that we can attach new controller session to. Add to position hashmap and mark active

            println!(
                "Found new controller {} on {}",
                datafeed_controller.cid, datafeed_controller.callsign
            );

            if let Some(position_session_tracker) =
                active_position_sessions.get_mut(&datafeed_controller.simple_callsign())
            {
                // There is an existing position, so create new controller session attached to it

                if let Some(new_controller_session_tracker) = create_new_controller_session_tracker(
                    datafeed_controller,
                    position_matchers,
                    &position_session_tracker.position_session,
                ) {
                    active_controller_sessions.insert(
                        format!(
                            "{} {}",
                            new_controller_session_tracker.controller_session.cid,
                            new_controller_session_tracker
                                .controller_session
                                .start_time
                                .timestamp()
                        ),
                        new_controller_session_tracker,
                    );

                    position_session_tracker.mark_active();
                    position_session_tracker.update_from(datafeed_controller);
                }
            } else {
                // TODO -- create position session and controller session and add
                if let Some(new_position_session_tracker) =
                    create_new_position_session_tracker(datafeed_controller, position_matchers)
                {
                    println!("here");
                    if let Some(new_controller_session_tracker) =
                        create_new_controller_session_tracker(
                            datafeed_controller,
                            position_matchers,
                            &new_position_session_tracker.position_session,
                        )
                    {
                        println!("here2");

                        active_position_sessions.insert(
                            new_position_session_tracker
                                .position_session
                                .position_simple_callsign
                                .to_owned(),
                            new_position_session_tracker,
                        );

                        active_controller_sessions.insert(
                            format!(
                                "{} {}",
                                new_controller_session_tracker.controller_session.cid,
                                new_controller_session_tracker
                                    .controller_session
                                    .start_time
                                    .timestamp()
                            ),
                            new_controller_session_tracker,
                        );
                    }
                }
            }
        }
    }

    // for c in active_controller_sessions.keys() {
    //     println!("{}", c);
    //     let v = active_controller_sessions.get(c).unwrap();
    //     println!("{}", v.controller_session.connected_callsign)
    // }

    for mut p in active_position_sessions.into_values() {
        if !p.marked_active {
            let _ = p.try_end_session(None);
            db_update_ended_position_session(pool, &p).await?;
        } else {
            db_update_active_position_session(pool, &p).await?;
        }
    }

    // TODO -- iterate through hashmap and save values
    for mut c in active_controller_sessions.into_values() {
        if !c.marked_active {
            let _ = c.try_end_session(None);
            db_update_ended_controller_session(pool, &c).await?;
        } else {
            db_update_active_controller_session(pool, &c).await?;
        }
    }

    Ok(())
}

fn create_new_controller_session_tracker(
    datafeed_controller: &Controller,
    position_matchers: &[PositionMatcher],
    assoc_position: &PositionSession,
) -> Option<ControllerSessionTracker> {
    let position_id =
        single_or_no_match(position_matchers, datafeed_controller).map(|pm| pm.position.id.clone());

    if let (Ok(start_time), Ok(last_updated)) = (
        DateTime::parse_from_rfc3339(&datafeed_controller.logon_time),
        DateTime::parse_from_rfc3339(&datafeed_controller.last_updated),
    ) {
        let new_controller_session = ControllerSession {
            id: Uuid::now_v7(),
            start_time: start_time.to_utc(),
            end_time: None,
            last_updated: last_updated.to_utc(),
            is_active: true,
            cid: datafeed_controller.cid as i32,
            position_id,
            position_simple_callsign: assoc_position.position_simple_callsign.to_owned(),
            connected_callsign: datafeed_controller.callsign.to_owned(),
            position_session_id: assoc_position.id,
            position_session_is_active: assoc_position.is_active,
        };

        Some(ControllerSessionTracker {
            controller_session: new_controller_session.clone(),
            marked_active: true,
        })
    } else {
        None
    }
}

fn create_new_position_session_tracker(
    datafeed_controller: &Controller,
    position_matchers: &[PositionMatcher],
) -> Option<PositionSessionTracker> {
    // First check if we can match on at least one position
    if let Some(possible_positions) = all_matches(position_matchers, datafeed_controller) {
        // Create hashmap of possible matches. If all matches are from same facility, continue
        let facility_hashmap: HashMap<_, _> = possible_positions
            .into_iter()
            .map(|p| (&p.parent_facility.id, p))
            .collect();
        if facility_hashmap.len() == 1 {
            let facility_hashmap_key = facility_hashmap.keys().next().unwrap();

            return if let (Ok(start_time), Ok(last_updated)) = (
                DateTime::parse_from_rfc3339(&datafeed_controller.logon_time),
                DateTime::parse_from_rfc3339(&datafeed_controller.last_updated),
            ) {
                let new_position_session = PositionSession {
                    id: Uuid::now_v7(),
                    start_time: start_time.to_utc(),
                    end_time: None,
                    last_updated: last_updated.to_utc(),
                    is_active: true,
                    facility_id: facility_hashmap
                        .get(facility_hashmap_key)
                        .unwrap()
                        .parent_facility
                        .id
                        .to_owned(),
                    facility_name: facility_hashmap
                        .get(facility_hashmap_key)
                        .unwrap()
                        .parent_facility
                        .name
                        .to_owned(),
                    position_simple_callsign: datafeed_controller.simple_callsign().to_owned(),
                };

                Some(PositionSessionTracker {
                    position_session: new_position_session,
                    marked_active: true,
                })
            } else {
                None
            };
        }
        return None;
    }

    return None;
}

pub fn is_active_vnas_controller(c: &Controller) -> bool {
    c.server == "VIRTUALNAS" && c.facility > 0 && c.frequency != "199.998"
}

pub fn gzip_decompress(b: &[u8]) -> Result<String, Error> {
    let mut d = DeflateDecoder::new(b);
    let mut s = String::new();
    //println!("{:?}", b);
    d.read_to_string(&mut s)?;
    Ok(s)
}
