use super::models::{Artcc, ControllerSession, PositionSession, VnasFetchRecord};
use crate::session_trackers::{ControllerSessionTracker, PositionSessionTracker};
use crate::vnas::api_dtos::ArtccRoot;
use crate::vnas::extended_models::{Callsign, FacilityWithTreeInfo, PositionExt};
use chrono::{DateTime, Utc};
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, Pool, Postgres};

pub async fn db_update_position_session(
    pool: &Pool<Postgres>,
    p: &PositionSessionTracker,
) -> Result<PgQueryResult, Error> {
    if p.marked_active {
        sqlx::query(
        r"
            insert into position_sessions (id, start_time, end_time, last_updated, duration, datafeed_first, datafeed_last, is_active, assoc_vnas_facilities, position_simple_callsign)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            on conflict (id, is_active) do update set
                end_time = excluded.end_time,
                last_updated = excluded.last_updated,
                duration = excluded.duration,
                datafeed_last = excluded.datafeed_last;"
        )
            .bind(p.position_session.id)
            .bind(p.position_session.start_time)
            .bind(p.position_session.end_time)
            .bind(p.position_session.last_updated)
            .bind(&p.position_session.duration)
            .bind(p.position_session.datafeed_first)
            .bind(p.position_session.datafeed_last)
            .bind(p.position_session.is_active)
            .bind(&p.position_session.assoc_vnas_facilities)
            .bind(&p.position_session.position_simple_callsign)
            .execute(pool)
            .await
    } else {
        sqlx::query(
            r"
            update position_sessions set
                is_active = $2,
                end_time = $3,
                last_updated = $4,
                duration = $5,
                datafeed_last = $6
            where id = $1;",
        )
        .bind(p.position_session.id)
        .bind(p.position_session.is_active)
        .bind(p.position_session.end_time)
        .bind(p.position_session.last_updated)
        .bind(&p.position_session.duration)
        .bind(p.position_session.datafeed_last)
        .execute(pool)
        .await
    }
}

pub async fn db_update_controller_session(
    pool: &Pool<Postgres>,
    c: &ControllerSessionTracker,
) -> Result<PgQueryResult, Error> {
    if c.marked_active {
        sqlx::query(
            r"
        insert into controller_sessions (id, start_time, end_time, last_updated, duration, datafeed_first, datafeed_last, is_active, cid, assoc_vnas_positions, position_simple_callsign, connected_callsign, connected_frequency, position_session_id, position_session_is_active)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        on conflict (id, is_active) do update set
            end_time = excluded.end_time,
            last_updated = excluded.last_updated,
            duration = excluded.duration,
            datafeed_last = excluded.datafeed_last;
        ")
            .bind(c.controller_session.id)
            .bind(c.controller_session.start_time)
            .bind(c.controller_session.end_time)
            .bind(c.controller_session.last_updated)
            .bind(&c.controller_session.duration)
            .bind(c.controller_session.datafeed_first)
            .bind(c.controller_session.datafeed_last)
            .bind(c.controller_session.is_active)
            .bind(c.controller_session.cid)
            .bind(&c.controller_session.assoc_vnas_positions)
            .bind(&c.controller_session.position_simple_callsign)
            .bind(&c.controller_session.connected_callsign)
            .bind(&c.controller_session.connected_frequency)
            .bind(c.controller_session.position_session_id)
            .bind(c.controller_session.position_session_is_active)
            .execute(pool)
            .await
    } else {
        sqlx::query(
            r"
        update controller_sessions set
            is_active = $2,
            end_time = $3,
            last_updated = $4,
            duration = $5,
            datafeed_last = $6
        where id = $1;",
        )
        .bind(c.controller_session.id)
        .bind(c.controller_session.is_active)
        .bind(c.controller_session.end_time)
        .bind(c.controller_session.last_updated)
        .bind(&c.controller_session.duration)
        .bind(c.controller_session.datafeed_last)
        .execute(pool)
        .await
    }
}

pub async fn db_update_vnas_position(
    pool: &Pool<Postgres>,
    p: &PositionExt,
    artcc: &ArtccRoot,
) -> Result<PgQueryResult, Error> {
    sqlx::query(
    r"
        insert into positions (id, name, radio_name, callsign, callsign_prefix, callsign_infix, callsign_suffix, callsign_without_infix, frequency, starred, parent_facility_id, last_updated)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        on conflict (id, parent_facility_id) do update set
            name = excluded.name,
            radio_name = excluded.radio_name,
            callsign = excluded.callsign,
            callsign_prefix = excluded.callsign_prefix,
            callsign_infix = excluded.callsign_infix,
            callsign_suffix = excluded.callsign_suffix,
            callsign_without_infix = excluded.callsign_without_infix,
            frequency = excluded.frequency,
            starred = excluded.starred,
            last_updated = excluded.last_updated;
        ")
        .bind(&p.position.id)
        .bind(&p.position.name)
        .bind(&p.position.radio_name)
        .bind(&p.position.callsign)
        .bind(p.position.callsign_prefix())
        .bind(p.position.callsign_infix())
        .bind(p.position.callsign_suffix())
        .bind(format!("{}_{}", &p.position.callsign_prefix(), &p.position.callsign_suffix()))
        .bind(p.position.frequency)
        .bind(p.position.starred)
        .bind(&p.parent_facility.id)
        .bind(artcc.last_updated_at)
        .execute(pool)
        .await
}

pub async fn db_update_vnas_facility(
    pool: &Pool<Postgres>,
    f: &FacilityWithTreeInfo,
) -> Result<PgQueryResult, Error> {
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
        ",
    )
    .bind(&f.facility.id)
    .bind(&f.facility.name)
    .bind(&f.facility.type_field.to_string())
    .bind(f.artcc_root.last_updated_at)
    .bind(&f.parent_facility.as_ref().map(|p| p.id.clone()))
    .bind(&f.artcc_root.id)
    .execute(pool)
    .await
}

pub async fn db_update_vnas_artcc(
    pool: &Pool<Postgres>,
    artcc: &ArtccRoot,
) -> Result<PgQueryResult, Error> {
    sqlx::query(
        r"
        insert into artccs (id, last_updated)
        values ($1, $2)
        on conflict (id) do update set
            last_updated = excluded.last_updated;
        ",
    )
    .bind(&artcc.id)
    .bind(artcc.last_updated_at)
    .execute(pool)
    .await
}

pub async fn db_get_active_controller_sessions(
    pool: &Pool<Postgres>,
) -> Result<Vec<ControllerSession>, Error> {
    sqlx::query_as::<_, ControllerSession>("select * from active_controller_sessions;")
        .fetch_all(pool)
        .await
}

pub async fn db_get_active_position_sessions(
    pool: &Pool<Postgres>,
) -> Result<Vec<PositionSession>, Error> {
    sqlx::query_as::<_, PositionSession>("select * from active_position_sessions;")
        .fetch_all(pool)
        .await
}

pub async fn db_insert_vnas_fetch_record(
    pool: &Pool<Postgres>,
    success: bool,
) -> Result<PgQueryResult, Error> {
    sqlx::query("insert into vnas_fetch_records (update_time, success) values ($1, $2);")
        .bind(Utc::now())
        .bind(success)
        .execute(pool)
        .await
}

pub async fn db_get_latest_fetch_record(
    pool: &Pool<Postgres>,
) -> Result<Option<VnasFetchRecord>, Error> {
    sqlx::query_as::<_, VnasFetchRecord>("select id, update_time, success from vnas_fetch_records where success = true order by update_time desc;")
        .fetch_optional(pool)
        .await
}

pub async fn db_get_all_artccs(pool: &Pool<Postgres>) -> Result<Vec<Artcc>, Error> {
    sqlx::query_as::<_, Artcc>("select * from artccs;")
        .fetch_all(pool)
        .await
}

pub async fn db_insert_datafeed_record(
    pool: &Pool<Postgres>,
    update: DateTime<Utc>,
    num_tracked_controller_sessions: i32,
    num_tracked_position_sessions: i32,
) -> Result<PgQueryResult, Error> {
    sqlx::query("insert into datafeed_records (update, num_tracked_controller_sessions, num_tracked_position_sessions) values ($1, $2, $3);")
        .bind(update)
        .bind(num_tracked_controller_sessions)
        .bind(num_tracked_position_sessions)
        .execute(pool)
        .await
}
