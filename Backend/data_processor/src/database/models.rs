use crate::interval_from;
use crate::vnas::extended_models::PositionExt;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use tracing::info;
use uuid::Uuid;
use vatsim_utils::models::Controller;

#[derive(Debug, sqlx::FromRow)]
pub struct VnasFetchRecord {
    pub id: i32,
    pub update_time: DateTime<Utc>,
    pub success: bool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Artcc {
    pub id: String,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VnasPositionInfo {
    pub id: String,
    pub name: String,
    pub radio_name: String,
    pub callsign: String,
    pub frequency: i32,
    pub starred: bool,
    pub parent_facility_id: String,
}

impl From<&PositionExt> for VnasPositionInfo {
    fn from(value: &PositionExt) -> Self {
        let p = &value.position;
        VnasPositionInfo {
            id: p.id.to_owned(),
            name: p.name.to_owned(),
            radio_name: p.radio_name.to_owned(),
            callsign: p.callsign.to_owned(),
            frequency: p.frequency as i32,
            starred: p.starred,
            parent_facility_id: value.parent_facility.id.to_owned(),
        }
    }
}

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct ControllerSession {
    pub id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub last_updated: DateTime<Utc>,
    pub duration: sqlx::postgres::types::PgInterval,
    pub datafeed_first: DateTime<Utc>,
    pub datafeed_last: DateTime<Utc>,
    pub is_active: bool,
    pub cid: i32,
    pub position_simple_callsign: String,
    pub connected_callsign: String,
    pub connected_frequency: String,
    pub position_session_id: Uuid,
    pub position_session_is_active: bool,
    pub is_cooling_down: bool,
}

impl ControllerSession {
    pub fn end_session(&mut self, end_time: Option<DateTime<Utc>>) {
        if self.is_active {
            if !self.is_cooling_down {
                self.end_time = end_time.or(Some(self.last_updated));
            }

            let cooldown_end = self.end_time.expect("None time") + Duration::minutes(5);
            if Utc::now() < cooldown_end {
                self.is_active = true;
                self.is_cooling_down = true
            } else {
                self.is_active = false;
                self.is_cooling_down = false;
            }

            self.duration = interval_from(self.start_time, self.end_time.expect("None time"));
        }
    }

    pub fn mark_active_from(&mut self, c: &Controller, datafeed_update: DateTime<Utc>) {
        self.is_active = true;

        // Needed because we could have a controller that we have marked as inactive due to dropping from
        // a datafeed, but we keep them in "cooldown active" state in the DB. If they reappear, delete the
        // end time
        if self.end_time.is_some() {
            self.end_time = None;

            self.is_cooling_down = false;
            info!(?self, ?c, "Resurrecting controller session")
        }

        if let Ok(d) = DateTime::parse_from_rfc3339(c.last_updated.as_str()) {
            self.last_updated = d.to_utc()
        }

        self.datafeed_last = datafeed_update;
        self.duration = interval_from(self.start_time, self.last_updated)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VnasFacilityInfo {
    pub id: String,
    pub name: String,
}

impl From<&PositionExt> for VnasFacilityInfo {
    fn from(value: &PositionExt) -> Self {
        VnasFacilityInfo {
            id: value.parent_facility.id.to_owned(),
            name: value.parent_facility.name.to_owned(),
        }
    }
}

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct PositionSession {
    pub id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub last_updated: DateTime<Utc>,
    pub duration: sqlx::postgres::types::PgInterval,
    pub datafeed_first: DateTime<Utc>,
    pub datafeed_last: DateTime<Utc>,
    pub is_active: bool,
    pub position_simple_callsign: String,
    pub is_cooling_down: bool,
}

impl PositionSession {
    pub fn end_session(&mut self, end_time: Option<DateTime<Utc>>) {
        if self.is_active {
            if !self.is_cooling_down {
                self.end_time = end_time.or(Some(self.last_updated));
            }

            let cooldown_end = self.end_time.expect("None time") + Duration::minutes(5);
            if Utc::now() < cooldown_end {
                self.is_active = true;
                self.is_cooling_down = true
            } else {
                self.is_active = false;
                self.is_cooling_down = false;
            }

            self.duration = interval_from(self.start_time, self.end_time.expect("None time"));
        }
    }

    pub fn mark_active_from(&mut self, c: &Controller, datafeed_update: DateTime<Utc>) {
        self.is_active = true;

        // Needed because we could have a controller that we have marked as inactive due to dropping from
        // a datafeed, but we keep them in "cooldown active" state in the DB. If they reappear, delete the
        // end time
        if self.end_time.is_some() {
            self.end_time = None;
            self.is_cooling_down = false;
            info!(?self, ?c, "Resurrecting position session")
        }

        if let Ok(d) = DateTime::parse_from_rfc3339(c.last_updated.as_str()) {
            self.last_updated = max(d.to_utc(), self.last_updated);
        }

        if let Ok(d) = DateTime::parse_from_rfc3339(c.logon_time.as_str()) {
            self.start_time = min(d.to_utc(), self.start_time);
        }

        self.datafeed_last = datafeed_update;
        self.duration = interval_from(self.start_time, self.last_updated)
    }
}
