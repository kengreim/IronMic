use crate::database::models::{ControllerSession, PositionSession};
use chrono::{DateTime, Utc};
use std::cmp::{max, min};
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

    pub fn mark_active_from(&mut self, c: &Controller) {
        self.marked_active = true;

        if let Ok(d) = DateTime::parse_from_rfc3339(c.last_updated.as_str()) {
            self.position_session.last_updated =
                max(d.to_utc(), self.position_session.last_updated);
        }

        if let Ok(d) = DateTime::parse_from_rfc3339(c.logon_time.as_str()) {
            self.position_session.start_time = min(d.to_utc(), self.position_session.start_time);
        }
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

    pub fn mark_active_from(&mut self, c: &Controller) {
        self.marked_active = true;
        if let Ok(d) = DateTime::parse_from_rfc3339(c.last_updated.as_str()) {
            self.controller_session.last_updated = d.to_utc()
        }
    }

    pub fn try_end_session(&mut self, end_time: Option<DateTime<Utc>>) -> bool {
        self.controller_session.try_end_session(end_time)
    }
}
