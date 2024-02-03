use crate::vnas::extended_models::PositionWithParentFacility;
use vatsim_utils::models::Controller;

pub fn single_or_no_match<'a>(
    matchers: &'a [PositionWithParentFacility],
    controller: &Controller,
) -> Option<&'a PositionWithParentFacility> {
    let mut matched: Vec<&PositionWithParentFacility> = vec![];
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
    matchers: &'a [PositionWithParentFacility],
    controller: &Controller,
) -> Option<Vec<&'a PositionWithParentFacility>> {
    let x: Vec<&PositionWithParentFacility> =
        matchers.iter().filter(|m| m.is_match(controller)).collect();
    if x.is_empty() {
        None
    } else {
        Some(x)
    }
}
