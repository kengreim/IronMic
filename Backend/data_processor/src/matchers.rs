use crate::vnas::extended_models::PositionExt;
use vatsim_utils::models::Controller;

#[allow(dead_code)]
pub fn single_or_no_match<'a>(
    matchers: &'a [PositionExt],
    controller: &Controller,
) -> Option<&'a PositionExt> {
    let mut matched: Vec<&PositionExt> = vec![];
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
    matchers: &'a [PositionExt],
    controller: &Controller,
) -> Option<Vec<&'a PositionExt>> {
    let positions: Vec<&PositionExt> = matchers.iter().filter(|m| m.is_match(controller)).collect();
    if positions.is_empty() {
        None
    } else {
        Some(positions)
    }
}
