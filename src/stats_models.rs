use crate::vnas_models::PositionWithParentFacility;
use crate::PositionMatcher;
use chrono::{DateTime, Utc};
use std::error::Error;
use std::marker::PhantomData;
use vatsim_utils::models::Controller;

pub struct Active;
pub struct Completed;

pub struct ControllerSession<State = Active> {
    state: PhantomData<State>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub cid: u64,
    pub facility_id: String,
    pub facility_name: String,
    pub position_callsign: String,
    pub connected_callsign: String,
}

impl Default for ControllerSession<Active> {
    fn default() -> Self {
        Self {
            state: PhantomData::<Active>,
            start_time: Utc::now(),
            end_time: None,
            cid: Default::default(),
            facility_id: Default::default(),
            facility_name: Default::default(),
            position_callsign: Default::default(),
            connected_callsign: Default::default(),
        }
    }
}

impl ControllerSession<Active> {
    // pub fn builder() -> ControllerSessionBuilder {
    //     ControllerSessionBuilder::new()
    // }

    pub fn end_session(self, end_time: Option<DateTime<Utc>>) -> ControllerSession<Completed> {
        ControllerSession {
            state: PhantomData::<Completed>,
            start_time: self.start_time,
            end_time: if end_time.is_some() {
                end_time
            } else {
                Some(Utc::now())
            },
            cid: self.cid,
            facility_id: self.facility_id,
            facility_name: self.facility_name,
            position_callsign: self.position_callsign,
            connected_callsign: self.connected_callsign,
        }
    }
}

impl TryFrom<(&PositionMatcher, &Controller)> for ControllerSession<Active> {
    type Error = &'static str;

    fn try_from(
        (matcher, controller): (&PositionMatcher, &Controller),
    ) -> Result<Self, Self::Error> {
        if let Ok(t) = DateTime::parse_from_rfc3339(&controller.logon_time) {
            Ok(ControllerSession {
                state: PhantomData::<Active>,
                start_time: t.to_utc(),
                end_time: None,
                cid: controller.cid,
                facility_id: matcher.parent_facility.id.to_owned(),
                facility_name: matcher.parent_facility.name.to_owned(),
                position_callsign: matcher.position.callsign.to_owned(),
                connected_callsign: controller.callsign.to_owned(),
            })
        } else {
            Err("Could not parse logon time")
        }
    }
}

pub struct ControllerSessionBuilder {
    pub start_time: Option<DateTime<Utc>>,
    pub cid: Option<u64>,
}

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

pub struct PositionSession<State = Active> {
    state: PhantomData<State>,
    pub callsign: String,
    pub controller_sessions: Vec<ControllerSession>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

impl Default for PositionSession<Active> {
    fn default() -> Self {
        Self {
            state: PhantomData::<Active>,
            callsign: String::new(),
            start_time: Utc::now(),
            end_time: None,
            controller_sessions: vec![],
        }
    }
}

impl PositionSession<Active> {
    pub fn end_session(self, end_time: DateTime<Utc>) -> PositionSession<Completed> {
        PositionSession {
            state: PhantomData::<Completed>,
            callsign: self.callsign,
            controller_sessions: self.controller_sessions,
            start_time: self.start_time,
            end_time: Some(end_time),
        }
    }
}
