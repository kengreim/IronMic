use crate::database::models::{
    ControllerSession, PositionSession, VnasFacilityInfo, VnasPositionInfo,
};
use crate::make_controller_key;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use vatsim_utils::models::Controller;

#[derive(PartialEq, Clone)]
pub enum ActiveSessionTrackerSource {
    FromDatabase,
    NewlyCreated,
}

#[derive(Clone)]
pub struct PositionSessionTracker {
    pub position_session: PositionSession,
    pub marked_active: bool,
    pub assoc_vnas_facilities: Option<Vec<VnasFacilityInfo>>,
    pub source: ActiveSessionTrackerSource,
}

impl PositionSessionTracker {
    pub fn new(
        position_session: PositionSession,
        source: ActiveSessionTrackerSource,
    ) -> PositionSessionTracker {
        PositionSessionTracker {
            position_session,
            marked_active: false,
            assoc_vnas_facilities: None,
            source,
        }
    }

    pub fn mark_active_from(&mut self, c: &Controller, datafeed_update: DateTime<Utc>) {
        self.marked_active = true;
        self.position_session.mark_active_from(c, datafeed_update);
    }

    pub fn end_session(
        &mut self,
        end_time: Option<DateTime<Utc>>,
        datafeed_update: Option<DateTime<Utc>>,
    ) {
        self.position_session.end_session(end_time, datafeed_update)
    }
}

#[derive(Clone)]
pub struct ControllerSessionTracker {
    pub controller_session: ControllerSession,
    pub marked_active: bool,
    pub assoc_vnas_positions: Option<Vec<VnasPositionInfo>>,
    pub source: ActiveSessionTrackerSource,
}

impl ControllerSessionTracker {
    pub fn new(
        controller_session: ControllerSession,
        source: ActiveSessionTrackerSource,
    ) -> ControllerSessionTracker {
        ControllerSessionTracker {
            controller_session,
            marked_active: false,
            assoc_vnas_positions: None,
            source,
        }
    }

    pub fn mark_active_from(&mut self, c: &Controller, datafeed_update: DateTime<Utc>) {
        self.marked_active = true;
        self.controller_session.mark_active_from(c, datafeed_update);
    }

    pub fn end_session(
        &mut self,
        end_time: Option<DateTime<Utc>>,
        datafeed_update: Option<DateTime<Utc>>,
    ) {
        self.controller_session
            .end_session(end_time, datafeed_update)
    }
}

pub struct ActiveSessionsMap {
    pub controllers: HashMap<String, ControllerSessionTracker>,
    pub positions: HashMap<String, PositionSessionTracker>,
    pub cooldown_controllers: HashMap<String, ControllerSessionTracker>,
    pub cooldown_positions: HashMap<String, PositionSessionTracker>,
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

    pub fn cooldown_controller_exists(&self, key: &str) -> bool {
        self.cooldown_controllers.contains_key(key)
    }

    pub fn position_exists(&self, key: &str) -> bool {
        self.positions.contains_key(key)
    }

    pub fn cooldown_position_exists(&self, key: &str) -> bool {
        self.cooldown_positions.contains_key(key)
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

    pub fn resurrect_controller_from(
        &mut self,
        key: &str,
        controller: &Controller,
        update: DateTime<Utc>,
    ) {
        if let Some(c) = self.cooldown_controllers.get_mut(key) {
            c.mark_active_from(controller, update);
            let new_c = c.clone();
            self.insert_new_controller(new_c);
            self.cooldown_controllers.remove(key);
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

    pub fn resurrect_position_from(
        &mut self,
        key: &str,
        controller: &Controller,
        update: DateTime<Utc>,
    ) {
        if let Some(p) = self.cooldown_positions.get_mut(key) {
            p.mark_active_from(controller, update);
            let new_p = p.clone();
            self.insert_new_position(new_p); // TODO -- some kind of check if position already exists?
            self.cooldown_positions.remove(key);
        }
    }

    pub fn get_position(&self, key: &str) -> Option<&PositionSessionTracker> {
        self.positions.get(key)
    }

    #[allow(dead_code)]
    pub fn get_controller(&self, key: &str) -> Option<&ControllerSessionTracker> {
        self.controllers.get(key)
    }
}
