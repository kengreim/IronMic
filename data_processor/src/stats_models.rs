use crate::db_models::{ControllerSession, PositionSession};
use crate::{db_models, PositionMatcher};
use chrono::{DateTime, Utc};
use std::marker::PhantomData;
use vatsim_utils::models::Controller;
//
// pub struct Active;
// pub struct Completed;
//
// pub struct ControllerSession<State = Active> {
//     state: PhantomData<State>,
//     pub start_time: DateTime<Utc>,
//     pub end_time: Option<DateTime<Utc>>,
//     pub last_updated: DateTime<Utc>,
//     pub cid: i32,
//     pub facility_id: String,
//     pub facility_name: String,
//     pub position_id: Option<String>,
//     pub position_callsign: String,
//     pub connected_callsign: String,
// }
//
// impl Default for ControllerSession<Active> {
//     fn default() -> Self {
//         Self {
//             state: PhantomData::<Active>,
//             start_time: Utc::now(),
//             end_time: None,
//             last_updated: Utc::now(),
//             cid: Default::default(),
//             facility_id: Default::default(),
//             facility_name: Default::default(),
//             position_id: None,
//             position_callsign: Default::default(),
//             connected_callsign: Default::default(),
//         }
//     }
// }
//
// impl ControllerSession<Active> {
//     // pub fn builder() -> ControllerSessionBuilder {
//     //     ControllerSessionBuilder::new()
//     // }
//
//     pub fn end_session(self, end_time: Option<DateTime<Utc>>) -> ControllerSession<Completed> {
//         ControllerSession {
//             state: PhantomData::<Completed>,
//             start_time: self.start_time,
//             end_time: if end_time.is_some() {
//                 end_time
//             } else {
//                 Some(self.last_updated)
//             },
//             last_updated: self.last_updated,
//             cid: self.cid,
//             facility_id: self.facility_id,
//             facility_name: self.facility_name,
//             position_id: self.position_id,
//             position_callsign: self.position_callsign,
//             connected_callsign: self.connected_callsign,
//         }
//     }
// }
//
// impl TryFrom<(&PositionMatcher, &Controller)> for ControllerSession<Active> {
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
//                 state: PhantomData::<Active>,
//                 start_time: start.to_utc(),
//                 end_time: None,
//                 last_updated: updated.to_utc(),
//                 cid: controller.cid as i32,
//                 facility_id: matcher.parent_facility.id.to_owned(),
//                 facility_name: matcher.parent_facility.name.to_owned(),
//                 position_id: Some(matcher.position.id.to_owned()),
//                 position_callsign: matcher.position.callsign.to_owned(),
//                 connected_callsign: controller.callsign.to_owned(),
//             })
//         } else {
//             Err("Could not parse logon time")
//         }
//     }
// }
//
// pub struct ControllerSessionBuilder {
//     pub start_time: Option<DateTime<Utc>>,
//     pub cid: Option<u64>,
// }

// impl From<db_models::ControllerSession> for ControllerSession {
//     fn from(value: db_models::ControllerSession) -> Self {
//         Self {
//             state: if value.is_active {
//                 PhantomData::<Active>
//             } else {
//                 PhantomData::<Completed>
//             },
//             start_time: value.start_time,
//             end_time: value.end_time,
//             last_updated: value.last_updated,
//             cid: value.cid,
//             facility_id: value.facility_id,
//             facility_name: value.facility_name,
//             position_id: value.position_id,
//             position_callsign: value.position_callsign,
//             connected_callsign: value.connected_callsign,
//         }
//     }
// }

// impl ControllerSessionBuilder {
//     pub fn new() -> Self {
//         ControllerSessionBuilder {
//             start_time: None,
//             cid: Default::default(),
//         }
//     }
//
//     pub fn start_time(mut self, start_time: DateTime<Utc>) -> ControllerSessionBuilder {
//         self.start_time = Some(start_time);
//         self
//     }
//
//     pub fn cid(mut self, cid: u64) -> ControllerSessionBuilder {
//         self.cid = Some(cid);
//         self
//     }
//
//     pub fn build(self) -> ControllerSession {
//         ControllerSession {
//             state: PhantomData::<Active>,
//             start_time: if let Some(t) = self.start_time {
//                 t
//             } else {
//                 Utc::now()
//             },
//             end_time: None,
//             cid: self.cid.unwrap(),
//         }
//     }
// }

// impl Default for ControllerSessionBuilder {
//     fn default() -> Self {
//         Self::new()
//     }
// }
//
// pub struct PositionSession<State = Active> {
//     state: PhantomData<State>,
//     pub callsign_prefix: String,
//     pub callsign_suffix: String,
//     pub controller_sessions: Vec<ControllerSession>,
//     pub start_time: DateTime<Utc>,
//     pub end_time: Option<DateTime<Utc>>,
// }
//
// impl Default for PositionSession<Active> {
//     fn default() -> Self {
//         Self {
//             state: PhantomData::<Active>,
//             callsign_prefix: String::new(),
//             callsign_suffix: String::new(),
//             start_time: Utc::now(),
//             end_time: None,
//             controller_sessions: vec![],
//         }
//     }
// }
//
// impl PositionSession<Active> {
//     pub fn end_session(self, end_time: DateTime<Utc>) -> PositionSession<Completed> {
//         PositionSession {
//             state: PhantomData::<Completed>,
//             callsign: self.callsign,
//             controller_sessions: self.controller_sessions,
//             start_time: self.start_time,
//             end_time: Some(end_time),
//         }
//     }
// }

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

    // pub fn add_controller_session(&mut self, c: ControllerSessionTracker) {
    //     self.controller_sessions.push(c);
    // }

    pub fn mark_active(&mut self) {
        self.marked_active = true
    }
    //
    // pub fn find_controller_session(
    //     &self,
    //     controller: &Controller,
    // ) -> Option<&ControllerSessionTracker> {
    //     let filtered: Vec<_> = self
    //         .controller_sessions
    //         .iter()
    //         .filter(|c| {
    //             c.controller_session.cid == controller.cid as i32
    //                 && c.controller_session.start_time
    //                     == DateTime::parse_from_rfc3339(&controller.logon_time)
    //                         .unwrap_or(DateTime::default())
    //                         .to_utc()
    //         })
    //         .collect();
    //
    //     if filtered.is_empty() {
    //         None
    //     } else {
    //         Some(filtered[0])
    //     }
    // }

    pub fn update_from(&mut self, c: &Controller) {
        if let Ok(d) = DateTime::parse_from_rfc3339(c.last_updated.as_str()) {
            if d.to_utc() > self.position_session.last_updated {
                self.position_session.last_updated = d.to_utc()
            }
        }

        if let Ok(d) = DateTime::parse_from_rfc3339(c.logon_time.as_str()) {
            if d.to_utc() < self.position_session.start_time {
                self.position_session.start_time = d.to_utc()
            }
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

    pub fn mark_active(&mut self) {
        self.marked_active = true
    }

    pub fn update_from(&mut self, c: &Controller) {
        if let Ok(d) = DateTime::parse_from_rfc3339(c.last_updated.as_str()) {
            self.controller_session.last_updated = d.to_utc()
        }
    }

    pub fn try_end_session(&mut self, end_time: Option<DateTime<Utc>>) -> bool {
        self.controller_session.try_end_session(end_time)
    }
}
