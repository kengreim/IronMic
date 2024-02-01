use crate::db_models;
use crate::db_models::{ControllerSession, PositionSession};
use crate::stats_models::{ControllerSessionTracker, PositionSessionTracker};
use crate::vnas_aggregate_models::{Callsign, FacilityWithTreeInfo, PositionWithParentFacility};
use crate::vnas_api_models::ArtccRoot;
use sqlx::postgres::PgQueryResult;
use sqlx::{Error, Pool, Postgres};

pub async fn db_update_ended_position_session(
    pool: &Pool<Postgres>,
    p: &PositionSessionTracker,
) -> Result<PgQueryResult, Error> {
    sqlx::query(
        r"
        update position_sessions set
            is_active = $2,
            end_time = $3,
            last_updated = $4
        where id = $1;
    ",
    )
    .bind(&p.position_session.id)
    .bind(&p.position_session.is_active)
    .bind(&p.position_session.end_time)
    .bind(&p.position_session.last_updated)
    .execute(pool)
    .await
}

pub async fn db_update_active_position_session(
    pool: &Pool<Postgres>,
    p: &PositionSessionTracker,
) -> Result<PgQueryResult, Error> {
    sqlx::query(
    r"
        insert into position_sessions (id, start_time, end_time, last_updated, is_active, facility_id, facility_name, position_simple_callsign)
        values ($1, $2, $3, $4, $5, $6, $7, $8)
        on conflict (id, is_active) do update set
            end_time = excluded.end_time,
            last_updated = excluded.last_updated,
            is_active = excluded.is_active;"
    )
        .bind(&p.position_session.id)
        .bind(&p.position_session.start_time)
        .bind(&p.position_session.end_time)
        .bind(&p.position_session.last_updated)
        .bind(&p.position_session.is_active)
        .bind(&p.position_session.facility_id)
        .bind(&p.position_session.facility_name)
        .bind(&p.position_session.position_simple_callsign)
        .execute(pool)
        .await
}

pub async fn db_update_ended_controller_session(
    pool: &Pool<Postgres>,
    c: &ControllerSessionTracker,
) -> Result<PgQueryResult, Error> {
    sqlx::query(
        r"
        update controller_sessions set
            is_active = $2,
            end_time = $3,
            last_updated = $4
        where id = $1;",
    )
    .bind(&c.controller_session.id)
    .bind(&c.controller_session.is_active)
    .bind(&c.controller_session.end_time)
    .bind(&c.controller_session.last_updated)
    .execute(pool)
    .await
}

pub async fn db_update_active_controller_session(
    pool: &Pool<Postgres>,
    c: &ControllerSessionTracker,
) -> Result<PgQueryResult, Error> {
    sqlx::query(
    r"
        insert into controller_sessions (id, start_time, end_time, last_updated, is_active, cid, position_id, position_simple_callsign, connected_callsign, position_session_id, position_session_is_active)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        on conflict (id, is_active) do update set
            end_time = excluded.end_time,
            last_updated = excluded.last_updated,
            is_active = excluded.is_active;
        ")
        .bind(&c.controller_session.id)
        .bind(&c.controller_session.start_time)
        .bind(&c.controller_session.end_time)
        .bind(&c.controller_session.last_updated)
        .bind(&c.controller_session.is_active)
        .bind(&c.controller_session.cid)
        .bind(&c.controller_session.position_id)
        .bind(&c.controller_session.position_simple_callsign)
        .bind(&c.controller_session.connected_callsign)
        .bind(&c.controller_session.position_session_id)
        .bind(&c.controller_session.position_session_is_active)
        .execute(pool)
        .await
}

pub async fn db_update_vnas_position(
    pool: &Pool<Postgres>,
    p: &PositionWithParentFacility,
    artcc: &ArtccRoot,
) -> Result<PgQueryResult, Error> {
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
    .bind(&f.artcc_root.last_updated_at)
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
    .bind(&artcc.last_updated_at)
    .execute(pool)
    .await
}

pub async fn db_get_active_controller_sessions(
    pool: &Pool<Postgres>,
) -> Result<Vec<ControllerSession>, sqlx::Error> {
    let db_sessions = sqlx::query_as::<_, db_models::ControllerSession>(
        "select * from active_controller_sessions;",
    )
    .fetch_all(pool)
    .await?;

    Ok(db_sessions)
}

pub async fn db_get_active_position_sessions(
    pool: &Pool<Postgres>,
) -> Result<Vec<PositionSession>, sqlx::Error> {
    let db_sessions =
        sqlx::query_as::<_, db_models::PositionSession>("select * from active_position_sessions;")
            .fetch_all(pool)
            .await?;

    Ok(db_sessions)
}
