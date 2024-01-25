use chrono::{DateTime, Utc};
use std::marker::PhantomData;

pub struct NotStated;
pub struct Active;
pub struct Completed;

pub struct ControllerSession<State = Active> {
    state: PhantomData<State>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

impl Default for ControllerSession<Active> {
    fn default() -> Self {
        Self {
            state: PhantomData::<Active>,
            start_time: Utc::now(),
            end_time: None,
        }
    }
}

impl ControllerSession<Active> {
    pub fn end_session(self, end_time: DateTime<Utc>) -> ControllerSession<Completed> {
        ControllerSession {
            state: PhantomData::<Completed>,
            start_time: self.start_time,
            end_time: Some(end_time),
        }
    }
}

pub struct ControllerSessionBuilder<State> {
    state: PhantomData<State>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

impl ControllerSessionBuilder<Active> {
    pub fn new() -> Self {
        ControllerSessionBuilder {
            state: PhantomData::<Active>,
            start_time: None,
            end_time: None,
        }
    }

    pub fn start_time(mut self, start_time: DateTime<Utc>) -> ControllerSessionBuilder<Active> {
        self.start_time = Some(start_time);
        self
    }

    pub fn build(self) -> ControllerSession<Active> {
        ControllerSession {
            state: PhantomData::<Active>,
            start_time: if let Some(t) = self.start_time {
                t
            } else {
                Utc::now()
            },
            end_time: None,
        }
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
