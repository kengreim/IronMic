use crate::vnas_aggregate_models::{Callsign, PositionWithParentFacility};
use crate::vnas_api_models::{Facility, Position};
use regex::Regex;
use std::num::ParseFloatError;
use vatsim_utils::models::Controller;

pub struct PositionMatcher {
    pub parent_facility: Facility,
    pub position: Position,
    pub regex: Regex,
}

impl PositionMatcher {
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

pub fn single_or_no_match<'a>(
    matchers: &'a [PositionMatcher],
    controller: &Controller,
) -> Option<&'a PositionMatcher> {
    let mut matched: Vec<&PositionMatcher> = vec![];
    for matcher in matchers {
        if matcher.is_match(controller) {
            matched.push(matcher);
        }
    }
    if matched.len() == 1 {
        Some(matched[0])
    } else {
        matched.retain(|p| p.position.starred);
        if matched.len() == 1 {
            Some(matched[0])
        } else {
            None
        }
    }
}

pub fn all_matches<'a>(
    matchers: &'a [PositionMatcher],
    controller: &Controller,
) -> Option<Vec<&'a PositionMatcher>> {
    let x: Vec<&PositionMatcher> = matchers.iter().filter(|m| m.is_match(controller)).collect();
    if x.is_empty() {
        None
    } else {
        Some(x)
    }
}

impl From<PositionWithParentFacility> for PositionMatcher {
    fn from(value: PositionWithParentFacility) -> Self {
        PositionMatcher {
            parent_facility: value.parent_facility,
            position: value.position.clone(),
            regex: value.position.build_match_regex().unwrap(),
        }
    }
}
