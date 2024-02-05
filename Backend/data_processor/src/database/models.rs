use crate::vnas::extended_models::PositionExt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
        }
    }
}

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct ControllerSession {
    pub id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub last_updated: DateTime<Utc>,
    pub is_active: bool,
    pub cid: i32,
    pub assoc_vnas_positions: Option<sqlx::types::Json<Vec<VnasPositionInfo>>>,
    pub position_simple_callsign: String,
    pub connected_callsign: String,
    pub connected_frequency: String,
    pub position_session_id: Uuid,
    pub position_session_is_active: bool,
}

impl ControllerSession {
    pub fn try_end_session(&mut self, end_time: Option<DateTime<Utc>>) -> bool {
        if !self.is_active {
            false
        } else {
            self.end_time = end_time.or(Some(self.last_updated));
            self.is_active = false;
            true
        }
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
    pub is_active: bool,
    pub assoc_vnas_facilities: Option<sqlx::types::Json<Vec<VnasFacilityInfo>>>,
    pub position_simple_callsign: String,
}

impl PositionSession {
    pub fn try_end_session(&mut self, end_time: Option<DateTime<Utc>>) -> bool {
        if !self.is_active {
            false
        } else {
            self.end_time = end_time.or(Some(self.last_updated));
            self.is_active = false;
            true
        }
    }
}
