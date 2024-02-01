use chrono::{DateTime, Utc};
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

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct ControllerSession {
    pub id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub last_updated: DateTime<Utc>,
    pub is_active: bool,
    pub cid: i32,
    //pub facility_id: String,
    //pub facility_name: String,
    pub position_id: Option<String>,
    pub position_simple_callsign: String,
    pub connected_callsign: String,
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

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct PositionSession {
    pub id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub last_updated: DateTime<Utc>,
    pub is_active: bool,
    //pub cid: i32,
    pub facility_id: String,
    pub facility_name: String,
    //pub position_id: Option<String>,
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

// impl TryFrom<(&PositionMatcher, &Controller)> for ControllerSession {
//     type Error = &'static str;
//
//     fn try_from(
//         (matcher, controller): (&PositionMatcher, &Controller),
//     ) -> Result<Self, Self::Error> {
//         if let (Ok(start), Ok(updated)) = (
//             DateTime::parse_from_rfc3339(&controller.logon_time),
//             DateTime::parse_from_rfc3339(&controller.last_updated),
//         ) {
//             Ok(ControllerSession {
//                 start_time: start.to_utc(),
//                 end_time: None,
//                 last_updated: updated.to_utc(),
//                 is_active: true,
//                 cid: controller.cid as i32,
//                 //facility_id: matcher.parent_facility.id.to_owned(),
//                 //facility_name: matcher.parent_facility.name.to_owned(),
//                 position_id: Some(matcher.position.id.to_owned()),
//                 position_simple_callsign: matcher.position.simple_callsign().to_owned(),
//                 connected_callsign: controller.callsign.to_owned(),
//                 position_session_id:
//             })
//         } else {
//             Err("Could not parse logon time")
//         }
//     }
// }
