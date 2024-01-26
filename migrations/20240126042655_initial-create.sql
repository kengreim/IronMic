CREATE TABLE IF NOT EXISTS contoller_sessions (
    id SERIAL NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL,
    cid INTEGER NOT NULL,
    facility_id TEXT NOT NULL,
    facility_name TEXT NOT NULL,
    position_id TEXT,
    position_callsign TEXT NOT NULL,
    connected_callsign TEXT NOT NULL,
    position_session_id SERIAL REFERENCES positions_sessions (id)
    PRIMARY KEY (id, is_active),
    CONSTRAINT if_completed_then_endtime_is_not_null CHECK(is_active OR (end_time IS NOT NULL))

) PARTITION BY LIST (is_active);

CREATE TABLE active_controller_sessions PARTITION OF controller_sessions FOR VALUES IN (TRUE);
CREATE TABLE completed_controller_sessions PARTITION OF controller_sessions FOR VALUES IN (FALSE);

CREATE TABLE IF NOT EXISTS position_sessions (
    id SERIAL NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL,
    cid INTEGER NOT NULL,
    facility_id TEXT NOT NULL,
    facility_name TEXT NOT NULL,
    position_id TEXT,
    position_callsign TEXT NOT NULL,
    connected_callsign TEXT NOT NULL,
    PRIMARY KEY (id, is_active),
    CONSTRAINT if_completed_then_endtime_is_not_null CHECK(is_active OR (end_time IS NOT NULL))
) PARTITION BY LIST (is_active);

CREATE TABLE active_position_sessions PARTITION OF position_sessions FOR VALUES IN (TRUE);
CREATE TABLE completed_position_sessions PARTITION OF position_sessions FOR VALUES IN (FALSE);