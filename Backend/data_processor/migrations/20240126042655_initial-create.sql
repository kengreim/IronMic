create table if not exists artccs (
    id text primary key,
    last_updated timestamptz not null
);

create table if not exists facilities (
    id text primary key,
    name text not null,
    type text not null,
    last_updated timestamptz not null,
    parent_facility_id text references facilities (id),
    parent_artcc_id text references artccs (id)
);

create table if not exists positions (
    id text not null,
    name text not null,
    radio_name text not null,
    callsign text not null,
    callsign_prefix text not null,
    callsign_infix text,
    callsign_suffix text not null,
    callsign_without_infix text not null,
    frequency integer not null,
    starred bool not null,
    parent_facility_id text references facilities (id),
    last_updated timestamptz not null,
    primary key (id, parent_facility_id)
);

create table if not exists position_sessions (
    id uuid not null,
    start_time timestamptz not null,
    end_time timestamptz,
    last_updated timestamptz not null,
    duration interval not null,
    datafeed_first timestamptz not null,
    datafeed_last timestamptz not null,
    is_active boolean not null,
    assoc_vnas_facilities jsonb,
    position_simple_callsign text not null,
    primary key (id, is_active),
    constraint if_completed_then_endtime_is_not_null check(is_active or (end_time is not null))
) partition by list (is_active);

create table if not exists active_position_sessions partition of position_sessions for values in (true);
create table if not exists completed_position_sessions partition of position_sessions for values in (false);

create table if not exists controller_sessions (
    id uuid not null,
    start_time timestamptz not null,
    end_time timestamptz,
    last_updated timestamptz not null,
    duration interval not null,
    datafeed_first timestamptz not null,
    datafeed_last timestamptz not null,
    is_active boolean not null,
    cid integer not null,
    assoc_vnas_positions jsonb,
    position_simple_callsign text not null,
    connected_callsign text not null,
    connected_frequency text not null,
    position_session_id uuid not null,
    position_session_is_active boolean not null,
    primary key (id, is_active),
    foreign key (position_session_id, position_session_is_active) references position_sessions on update cascade,
    constraint if_completed_then_endtime_is_not_null check(is_active or (end_time is not null))
) partition by list (is_active);

create table if not exists active_controller_sessions partition of controller_sessions for values in (true);
create table if not exists completed_controller_sessions partition of controller_sessions for values in (false);

create table if not exists vnas_fetch_records (
    id integer generated always as identity primary key,
    update_time timestamptz not null,
    success boolean not null
);

create table if not exists datafeed_records (
    id integer generated always as identity primary key,
    update timestamptz not null,
    num_tracked_controller_sessions int not null,
    num_tracked_position_sessions int not null
);
