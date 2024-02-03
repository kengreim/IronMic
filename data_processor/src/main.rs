use crate::database::models::{Artcc, ControllerSession, PositionSession};
use crate::database::queries::{
    db_get_active_controller_sessions, db_get_active_position_sessions, db_get_all_artccs,
    db_get_latest_fetch_record, db_insert_vnas_fetch_record, db_update_controller_session,
    db_update_position_session, db_update_vnas_artcc, db_update_vnas_facility,
    db_update_vnas_position,
};
use crate::matchers::{all_matches, single_or_no_match};
use crate::session_trackers::{
    ActiveSessionsMap, ControllerSessionTracker, PositionSessionTracker,
};
use crate::vnas::api::{VnasApi, VnasApiError};
use crate::vnas::api_dtos::ArtccRoot;
use crate::vnas::extended_models::{AllPositions, Callsign, PositionExt};
use chrono::{DateTime, Utc};
use flate2::read::DeflateDecoder;
use futures::future::join_all;
use rsmq_async::{Rsmq, RsmqConnection, RsmqError, RsmqOptions};
use sqlx::migrate::MigrateError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::io::{Error, Read};
use std::time::Duration;
use tokio::time::sleep;
use tracing::subscriber::SetGlobalDefaultError;
use tracing::{error, info, instrument, trace, warn};
use uuid::Uuid;
use vatsim_utils::models::Controller;

mod database;
mod matchers;
mod session_trackers;
mod vnas;

#[derive(Debug, thiserror::Error)]
enum InitError {
    #[error("error with database")]
    Database(#[from] sqlx::Error),

    #[error("error updated vNAS data")]
    VnasDataUpdate(#[from] VnasDataUpdateError),

    #[error("could not apply migrations")]
    Migration(#[from] MigrateError),
}

#[derive(Debug, thiserror::Error)]
enum VnasDataUpdateError {
    #[error("error with database")]
    DbError(#[from] sqlx::Error),

    #[error("could not fetch datat")]
    ApiError(#[from] VnasApiError),
}

#[tokio::main]
async fn main() -> Result<(), SetGlobalDefaultError> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .json()
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // Overall flow
    // - Initialize DB if needed, and do initial fetch if no vNAS data fetches have been done
    // - Initialize Redis connection
    // - Start Redis message receive loop
    // - With message:
    // -    If no USA controllers online, check if last data fetch was more than 24 hours ago. If yes, fetch new data and update any ARTCCS that need updating
    // -    If USA controllers online, process existing active sessions (keep open or close) and add new sessions if needed
    // -    Aggregate stats

    let db_pool = match initialize_db("postgres://postgres:pw@localhost/ironmic").await {
        Ok(db_pool) => db_pool,
        Err(e) => {
            error!(error = ?e, "Could not initialize DB connection pool");
            panic!("Could not initialize DB connection pool")
        }
    };

    let mut vnas_positions = match update_all_artccs_in_db(&db_pool, true).await {
        Ok(Some(vnas_positions)) => vnas_positions,
        Ok(None) => {
            error!("Could not initialize DB position matchers, returned None");
            panic!("Could not initialize DB position matchers")
        }
        Err(e) => {
            error!(error = ?e, "Could not initialize DB position matchers");
            panic!("Could not initialize DB position matchers")
        }
    };

    let mut rsmq = match initialize_rsmq(shared::DATAFEED_QUEUE_NAME).await {
        Ok(rsmq) => rsmq,
        Err(e) => {
            error!(error = ?e, "Could not initialize Redis connection");
            panic!("Could not initialize Redis connection");
        }
    };

    // Start of infinite loop
    loop {
        let msg = rsmq
            .receive_message::<Vec<u8>>(shared::DATAFEED_QUEUE_NAME, None)
            .await;

        if let Err(e) = &msg {
            warn!(error = ?e, "Error receiving message from Redis");
            continue;
        }

        if let Some(message) = msg.unwrap() {
            let decompressed = match decompress(&message.message) {
                Ok(s) => s,
                Err(e) => {
                    warn!(error = ?e, "Error decompressing message from Redis");
                    continue;
                }
            };

            let controllers: Vec<Controller> = match serde_json::from_str(&decompressed) {
                Ok(c) => c,
                Err(e) => {
                    warn!(error = ?e, "Error deserializing JSON from Redis");
                    continue;
                }
            };

            let vnas_controllers: Vec<&Controller> = controllers
                .iter()
                .filter(|c| is_active_vnas_controller(c))
                .collect();

            if vnas_controllers.is_empty() {
                if let Ok(Some(new_pms)) = update_all_artccs_in_db(&db_pool, false).await {
                    vnas_positions = new_pms
                }
            } else if let Err(e) =
                process_datafeed(vnas_controllers, &vnas_positions, &db_pool).await
            {
                warn!(error = ?e, "Error processing datafeed")
            }

            if let Err(e) = rsmq
                .delete_message(shared::DATAFEED_QUEUE_NAME, &message.id)
                .await
            {
                warn!(error = ?e, "Error deleting message in Redis");
            }
        } else {
            trace!("No message received from queue, sleeping");
            sleep(Duration::from_secs(1)).await
        }
    }
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
        rsmq.create_queue(queue_name, None, None, Some(-1)).await?
    }

    Ok(rsmq)
}

async fn initialize_db(connection_string: &str) -> Result<Pool<Postgres>, InitError> {
    // Create Db connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string)
        .await?;

    // Run any new migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
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

#[instrument(skip(pool))]
async fn update_all_artccs_in_db(
    pool: &Pool<Postgres>,
    force_update: bool,
) -> Result<Option<Vec<PositionExt>>, VnasDataUpdateError> {
    // Get record of latest vNAS data fetch. Update if none or stale data (>24 hours old)
    let latest_record = db_get_latest_fetch_record(pool).await?;

    // Update if we've never initialized DB or haven't done it in 24 hours, or we want to force update
    if latest_record.is_none()
        || (Utc::now() - latest_record.unwrap().update_time)
            > chrono::Duration::seconds(60 * 60 * 24)
        || force_update
    {
        let fetched_artccs = VnasApi::new().unwrap().get_all_artccs_data().await?;
        let db_artccs = db_get_all_artccs(pool).await?;

        let needs_update = fetched_artccs
            .iter()
            .filter(|a| should_update_artcc(a, &db_artccs));

        // Apply update to all Artccs that need update and await joined result
        let results = join_all(needs_update.map(|artcc| update_artcc_in_db(pool, artcc))).await;

        // Store record of vNAS data check. If any errors, log as unsuccessful
        db_insert_vnas_fetch_record(pool, !results.iter().any(|r| r.is_err())).await?;

        let position_matchers: Vec<PositionExt> = fetched_artccs
            .iter()
            .flat_map(|f| f.all_positions_with_parents())
            .collect();

        return Ok(Some(position_matchers));
    }

    Ok(None)
}

async fn process_datafeed(
    datafeed_controllers: Vec<&Controller>,
    vnas_positions: &[PositionExt],
    pool: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
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

    let mut active = load_active_sessions(pool).await?;

    for datafeed_controller in datafeed_controllers {
        let parsed_time = DateTime::parse_from_rfc3339(&datafeed_controller.logon_time)
            .unwrap_or(DateTime::default())
            .to_utc();
        let controller_key = make_controller_key(&datafeed_controller.cid.to_string(), parsed_time);
        let position_key = &datafeed_controller.simple_callsign();

        // If we have already tracked the controller, mark controller and position as active
        if active.controller_exists(&controller_key) {
            active.mark_controller_active_from(&controller_key, datafeed_controller);

            // Find Position based on "simple callsign" (no infix) and mark as active
            if active.position_exists(position_key) {
                active.mark_position_active_from(position_key, datafeed_controller)
            }
        // We are currently tracking this position, so create new controller tracker and attach to position
        } else if active.position_exists(position_key) {
            let position_tracker = active.get_position(position_key).unwrap();

            // There is an existing position, so create new controller session attached to it
            if let Some(new_controller_session_tracker) = create_new_controller_session_tracker(
                datafeed_controller,
                vnas_positions,
                &position_tracker.position_session,
            ) {
                active.insert_new_controller(new_controller_session_tracker);
                active.mark_position_active_from(position_key, datafeed_controller);
            }
        // We aren't currently tracking this position or controller, so create both
        } else if let Some(new_position_session_tracker) =
            create_new_position_session_tracker(datafeed_controller, vnas_positions)
        {
            if let Some(new_controller_session_tracker) = create_new_controller_session_tracker(
                datafeed_controller,
                vnas_positions,
                &new_position_session_tracker.position_session,
            ) {
                active.insert_new_position(new_position_session_tracker);
                active.insert_new_controller(new_controller_session_tracker);
            }
        }
    }

    save_actives(pool, active).await?;

    Ok(())
}

async fn load_active_sessions(pool: &Pool<Postgres>) -> Result<ActiveSessionsMap, sqlx::Error> {
    let controllers: HashMap<_, _> = db_get_active_controller_sessions(pool)
        .await?
        .into_iter()
        .map(|c| {
            (
                make_controller_key(&c.cid.to_string(), c.start_time),
                ControllerSessionTracker::new(c),
            )
        })
        .collect();

    let positions: HashMap<_, _> = db_get_active_position_sessions(pool)
        .await?
        .into_iter()
        .map(|p| {
            (
                p.position_simple_callsign.clone(),
                PositionSessionTracker::new(p.clone()),
            )
        })
        .collect();

    Ok(ActiveSessionsMap {
        controllers,
        positions,
    })
}

async fn save_actives(pool: &Pool<Postgres>, active: ActiveSessionsMap) -> Result<(), sqlx::Error> {
    for mut p in active.positions.into_values() {
        if !p.marked_active {
            let _ = p.try_end_session(None);
        }
        db_update_position_session(pool, &p).await?;
    }

    for mut c in active.controllers.into_values() {
        if !c.marked_active {
            let _ = c.try_end_session(None);
        }
        db_update_controller_session(pool, &c).await?;
    }

    Ok(())
}

fn create_new_controller_session_tracker(
    datafeed_controller: &Controller,
    vnas_positions: &[PositionExt],
    assoc_position: &PositionSession,
) -> Option<ControllerSessionTracker> {
    let position_id =
        single_or_no_match(vnas_positions, datafeed_controller).map(|p| p.position.id.clone());

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
            connected_frequency: datafeed_controller.frequency.to_owned(),
            position_session_id: assoc_position.id,
            position_session_is_active: assoc_position.is_active,
        };

        Some(ControllerSessionTracker {
            controller_session: new_controller_session.clone(),
            marked_active: true,
        })
    } else {
        warn!(
            start_time = datafeed_controller.logon_time,
            last_updated = datafeed_controller.last_updated,
            "Could not parse time from strings"
        );
        None
    }
}

fn create_new_position_session_tracker(
    datafeed_controller: &Controller,
    vnas_positions: &[PositionExt],
) -> Option<PositionSessionTracker> {
    // First check if we can match on at least one position
    if let Some(possible_positions) = all_matches(vnas_positions, datafeed_controller) {
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
                warn!(
                    start_time = datafeed_controller.logon_time,
                    last_updated = datafeed_controller.last_updated,
                    "Could not parse time from strings"
                );
                None
            };
        }
        info!(
            callsign = datafeed_controller.callsign,
            frequency = datafeed_controller.frequency,
            "More than 1 facility found matching this connection"
        );
        None
    } else {
        info!(
            callsign = datafeed_controller.callsign,
            frequency = datafeed_controller.frequency,
            "No positions found matching this connection"
        );
        None
    }
}

pub fn is_active_vnas_controller(c: &Controller) -> bool {
    c.server == "VIRTUALNAS" && c.facility > 0 && c.frequency != "199.998"
}

fn decompress(b: &[u8]) -> Result<String, Error> {
    let mut d = DeflateDecoder::new(b);
    let mut s = String::new();
    d.read_to_string(&mut s)?;
    Ok(s)
}

fn make_controller_key(cid: &str, time: DateTime<Utc>) -> String {
    format!("{} {}", cid, time.timestamp())
}
