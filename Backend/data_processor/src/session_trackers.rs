use crate::database::models::{ControllerSession, PositionSession};
use crate::{interval_from, make_controller_key};
use chrono::{DateTime, Utc};
use std::cmp::{max, min};
use std::collections::HashMap;
use vatsim_utils::models::Controller;

pub struct PositionSessionTracker {
    pub position_session: PositionSession,
    pub marked_active: bool,
}

impl PositionSessionTracker {
    pub fn new(position_session: PositionSession) -> PositionSessionTracker {
        PositionSessionTracker {
            position_session,
            marked_active: false,
        }
    }

    pub fn mark_active_from(&mut self, c: &Controller, datafeed_update: DateTime<Utc>) {
        self.marked_active = true;

        if let Ok(d) = DateTime::parse_from_rfc3339(c.last_updated.as_str()) {
            self.position_session.last_updated =
                max(d.to_utc(), self.position_session.last_updated);
        }

        if let Ok(d) = DateTime::parse_from_rfc3339(c.logon_time.as_str()) {
            self.position_session.start_time = min(d.to_utc(), self.position_session.start_time);
        }

        self.position_session.datafeed_last = datafeed_update;
        self.position_session.duration = interval_from(
            self.position_session.start_time,
            self.position_session.last_updated,
        )
    }

    pub fn try_end_session(&mut self, end_time: Option<DateTime<Utc>>) -> bool {
        self.position_session.try_end_session(end_time)
    }
}

pub struct ControllerSessionTracker {
    pub controller_session: ControllerSession,
    pub marked_active: bool,
}

impl ControllerSessionTracker {
    pub fn new(controller_session: ControllerSession) -> ControllerSessionTracker {
        ControllerSessionTracker {
            controller_session,
            marked_active: false,
        }
    }

    pub fn mark_active_from(&mut self, c: &Controller, datafeed_update: DateTime<Utc>) {
        self.marked_active = true;
        if let Ok(d) = DateTime::parse_from_rfc3339(c.last_updated.as_str()) {
            self.controller_session.last_updated = d.to_utc()
        }

        self.controller_session.datafeed_last = datafeed_update;
        self.controller_session.duration = interval_from(
            self.controller_session.start_time,
            self.controller_session.last_updated,
        )
    }

    pub fn try_end_session(&mut self, end_time: Option<DateTime<Utc>>) -> bool {
        self.controller_session.try_end_session(end_time)
    }
}

pub struct ActiveSessionsMap {
    pub controllers: HashMap<String, ControllerSessionTracker>,
    pub positions: HashMap<String, PositionSessionTracker>,
}

impl ActiveSessionsMap {
    pub fn insert_new_controller(&mut self, c: ControllerSessionTracker) {
        self.controllers.insert(
            make_controller_key(
                &c.controller_session.cid.to_string(),
                c.controller_session.start_time,
            ),
            c,
        );
    }

    pub fn insert_new_position(&mut self, p: PositionSessionTracker) {
        self.positions
            .insert(p.position_session.position_simple_callsign.to_owned(), p);
    }

    pub fn controller_exists(&self, key: &str) -> bool {
        self.controllers.contains_key(key)
    }

    pub fn position_exists(&self, key: &str) -> bool {
        self.positions.contains_key(key)
    }

    pub fn mark_controller_active_from(
        &mut self,
        key: &str,
        controller: &Controller,
        update: DateTime<Utc>,
    ) {
        if let Some(c) = self.controllers.get_mut(key) {
            c.mark_active_from(controller, update);
        }
    }

    pub fn mark_position_active_from(
        &mut self,
        key: &str,
        controller: &Controller,
        update: DateTime<Utc>,
    ) {
        if let Some(p) = self.positions.get_mut(key) {
            p.mark_active_from(controller, update);
        }
    }

    pub fn get_position(&self, key: &str) -> Option<&PositionSessionTracker> {
        self.positions.get(key)
    }
}
