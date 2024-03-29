use super::api_dtos::{ArtccRoot, Facility, Position};
use regex::{Error, Regex};
use std::num::ParseFloatError;
use vatsim_utils::models::Controller;

pub trait AllFacilities {
    fn all_facilities(&self) -> Vec<Facility>;

    fn all_facilities_with_info(
        &self,
        artcc: &ArtccRoot,
        parent: Option<&Facility>,
    ) -> Vec<FacilityWithTreeInfo>;
}

pub trait AllPositions {
    fn all_positions(&self) -> Vec<Position>;
    fn all_positions_with_parents(&self) -> Vec<PositionExt>;
}

pub struct FacilityWithTreeInfo {
    pub facility: Facility,
    pub parent_facility: Option<Facility>,
    pub artcc_root: ArtccRoot,
}

impl ArtccRoot {
    pub fn all_facilities_with_info(&self) -> Vec<FacilityWithTreeInfo> {
        self.facility.all_facilities_with_info(self, None)
    }
}

impl AllFacilities for Facility {
    fn all_facilities(&self) -> Vec<Facility> {
        if self.child_facilities.is_empty() {
            vec![self.to_owned()]
        } else {
            let mut vec = vec![self.to_owned()];
            self.child_facilities
                .iter()
                .for_each(|f| vec.extend(f.all_facilities()));
            vec
        }
    }

    fn all_facilities_with_info(
        &self,
        artcc: &ArtccRoot,
        parent: Option<&Facility>,
    ) -> Vec<FacilityWithTreeInfo> {
        let node = FacilityWithTreeInfo {
            facility: self.to_owned(),
            artcc_root: artcc.to_owned(),
            parent_facility: parent.map(|p| p.to_owned()),
        };
        if self.child_facilities.is_empty() {
            vec![node]
        } else {
            let mut vec = vec![node];
            self.child_facilities
                .iter()
                .for_each(|f| vec.extend(f.all_facilities_with_info(artcc, Some(self))));
            vec
        }
    }
}

impl AllPositions for ArtccRoot {
    fn all_positions(&self) -> Vec<Position> {
        self.facility.all_positions()
    }
    fn all_positions_with_parents(&self) -> Vec<PositionExt> {
        self.facility.all_positions_with_parents()
    }
}

impl AllPositions for Facility {
    fn all_positions(&self) -> Vec<Position> {
        if self.child_facilities.is_empty() {
            self.positions.to_owned()
        } else {
            let mut vec = self.positions.to_owned();
            self.child_facilities
                .iter()
                .for_each(|f| vec.extend(f.all_positions()));
            vec
        }
    }

    fn all_positions_with_parents(&self) -> Vec<PositionExt> {
        if self.child_facilities.is_empty() {
            map_positions_with_parent(self)
        } else {
            let mut vec = map_positions_with_parent(self);
            self.child_facilities
                .iter()
                .for_each(|f| vec.extend(f.all_positions_with_parents()));
            vec
        }
    }
}

fn map_positions_with_parent(facility: &Facility) -> Vec<PositionExt> {
    facility
        .positions
        .iter()
        .map(|p| PositionExt {
            parent_facility: facility.clone(),
            position: p.clone(),
            regex: p.build_match_regex().unwrap(),
        })
        .collect()
}

pub struct PositionExt {
    pub parent_facility: Facility,
    pub position: Position,
    pub regex: Regex,
}

impl PositionExt {
    pub fn is_match(&self, controller: &Controller) -> bool {
        self.regex.is_match(&controller.callsign)
            && if let Ok(b) = self.is_freq_match(&controller.frequency) {
                b
            } else {
                dbg!("Error parsing VATSIM freq {}", &controller.frequency);
                false
            }
    }

    fn is_freq_match(&self, vatsim_freq_str: &str) -> Result<bool, ParseFloatError> {
        let vatsim_freq_f = vatsim_freq_str.parse::<f64>();
        if let Ok(f) = vatsim_freq_f {
            let vatsim_freq_i64 = (f * 1e6).round() as i64;
            Ok(self.position.frequency == vatsim_freq_i64)
        } else {
            Err(vatsim_freq_f.unwrap_err())
        }
    }
}

pub trait Callsign {
    fn callsign_prefix(&self) -> &str;
    fn callsign_infix(&self) -> Option<&str>;
    fn callsign_suffix(&self) -> &str;
    fn simple_callsign(&self) -> String;
    fn is_match_for(&self, callsign: &str) -> bool;
    fn build_match_regex(&self) -> Result<Regex, Error>;
}

impl Callsign for Position {
    fn callsign_prefix(&self) -> &str {
        self.callsign.split('_').next().unwrap()
    }

    fn callsign_infix(&self) -> Option<&str> {
        let splits = self.callsign.split('_').collect::<Vec<&str>>();
        if splits.len() >= 3 {
            Some(splits.get(1).unwrap())
        } else {
            None
        }
    }

    fn callsign_suffix(&self) -> &str {
        self.callsign
            .split('_')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
    }

    fn simple_callsign(&self) -> String {
        format!("{}_{}", self.callsign_prefix(), self.callsign_suffix())
    }

    fn is_match_for(&self, callsign: &str) -> bool {
        self.build_match_regex().unwrap().is_match(callsign)
    }

    fn build_match_regex(&self) -> Result<Regex, Error> {
        let prefix_str = self.callsign_prefix();
        let infix_re = match self.callsign_infix() {
            Some(infix) => format!(r"{infix}[1-9]?_"),
            None => r"([1-9]_)?".to_owned(),
        };
        let suffix_str = self.callsign_suffix();
        Regex::new(format!("{prefix_str}_{infix_re}{suffix_str}").as_str())
    }
}

impl Callsign for Controller {
    fn callsign_prefix(&self) -> &str {
        self.callsign.split('_').next().unwrap()
    }

    fn callsign_infix(&self) -> Option<&str> {
        let splits = self.callsign.split('_').collect::<Vec<&str>>();
        if splits.len() >= 3 {
            Some(splits.get(1).unwrap())
        } else {
            None
        }
    }

    fn callsign_suffix(&self) -> &str {
        self.callsign
            .split('_')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
    }

    fn simple_callsign(&self) -> String {
        format!("{}_{}", self.callsign_prefix(), self.callsign_suffix())
    }

    fn is_match_for(&self, callsign: &str) -> bool {
        self.build_match_regex().unwrap().is_match(callsign)
    }

    fn build_match_regex(&self) -> Result<Regex, Error> {
        let prefix_str = self.callsign_prefix();
        let infix_re = match self.callsign_infix() {
            Some(infix) => format!(r"{infix}[1-9]?_"),
            None => r"([1-9]_)?".to_owned(),
        };
        let suffix_str = self.callsign_suffix();
        Regex::new(format!("{prefix_str}_{infix_re}{suffix_str}").as_str())
    }
}
