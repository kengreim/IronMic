use chrono::{DateTime, Utc};
use std::marker::PhantomData;

pub struct Active;
pub struct Completed;

pub struct ControllerSession<State = Active> {
    state: PhantomData<State>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub cid: u64,
}

impl Default for ControllerSession<Active> {
    fn default() -> Self {
        Self {
            state: PhantomData::<Active>,
            start_time: Utc::now(),
            end_time: None,
            cid: Default::default(),
        }
    }
}

impl ControllerSession<Active> {
    pub fn builder() -> ControllerSessionBuilder {
        ControllerSessionBuilder::new()
    }

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
        }
    }
}

pub struct ControllerSessionBuilder {
    pub start_time: Option<DateTime<Utc>>,
    pub cid: Option<u64>,
}

impl ControllerSessionBuilder {
    pub fn new() -> Self {
        ControllerSessionBuilder {
            start_time: None,
            cid: Default::default(),
        }
    }

    pub fn start_time(mut self, start_time: DateTime<Utc>) -> ControllerSessionBuilder {
        self.start_time = Some(start_time);
        self
    }

    pub fn cid(mut self, cid: u64) -> ControllerSessionBuilder {
        self.cid = Some(cid);
        self
    }

    pub fn build(self) -> ControllerSession {
        ControllerSession {
            state: PhantomData::<Active>,
            start_time: if let Some(t) = self.start_time {
                t
            } else {
                Utc::now()
            },
            end_time: None,
            cid: self.cid.unwrap(),
        }
    }
}

impl Default for ControllerSessionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

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
