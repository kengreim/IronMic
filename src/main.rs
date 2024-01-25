use crate::stats_models::{ControllerSession, ControllerSessionBuilder};
use crate::vnas_api::VnasApi;
use crate::vnas_models::{
    AllFacilities, AllPositions, Facility, Position, PositionWithParentFacility,
};
use chrono::{DateTime, Utc};
use regex::Regex;
use reqwest::Error;
use std::time::Instant;
use vatsim_utils::live_api::Vatsim;
use vatsim_utils::models::Controller;

mod stats_models;
mod vnas_api;
mod vnas_models;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // let artccs = vec!["ZAB", "ZMA", "ZOA", "ZHU", "ZBW", "ZDV", "ZID", "ZKC", "ZLA", "ZME", "ZMP", "ZFW", "ZAN", "ZUA", "ZJX", "ZSE", "ZNY", "ZAU", "ZDC", "ZLC", "ZOB", "ZTL", "ZSU", "ZHN"];
    //
    // for artcc in artccs {
    //     let url = format!("https://data-api.vnas.vatsim.net/api/artccs/{artcc}");
    //     println!("Trying {}", url);
    //     let body = get(url)
    //         .await?
    //         .json::<ArtccRoot>()
    //         .await?;
    //
    //     println!("{}", body.facility.name)
    // }

    let x = VnasApi::new().unwrap();
    let all_artccs = x.get_all_artccs_data().await?;
    // if let Ok(z) = y {
    //     println!("success")
    // }
    //
    // let y = x.get_artcc_data("ZOA").await?;
    // for z in y.all_positions() {
    //     println!("{}", z.callsign )
    // }

    let api = Vatsim::new().await.unwrap();
    let latest_data_result = api.get_v3_data().await.unwrap();

    let start = Instant::now();

    let position_matchers: Vec<PositionMatcher> = all_artccs
        .iter()
        .flat_map(|f| f.all_positions_with_parents())
        .map(|p| PositionMatcher::from(p))
        .collect();

    println!("{}", start.elapsed().as_micros());
    println!("here");

    println!("{}", position_matchers.len());
    println!("{}", latest_data_result.controllers.len());

    for controller in latest_data_result
        .controllers
        .into_iter()
        .filter(|c| is_active_vnas_controller(c))
    {
        println!("Trying {} with CID {}", controller.callsign, controller.cid);
        let time = Instant::now();
        for pm in position_matchers.as_slice() {
            if pm.is_match(&controller.callsign) {
                println!(
                    "Found match for {} - {} parent {} with {}",
                    pm.position.name,
                    pm.position.callsign,
                    pm.parent_facility.name,
                    controller.callsign
                );
                // let z: ControllerSession = ControllerSessionBuilder::default()
                //     .start_time(
                //         DateTime::parse_from_rfc3339(&controller.logon_time)
                //             .unwrap()
                //             .to_utc(),
                //     )
                //     .build();
                let q: ControllerSession = ControllerSession::builder()
                    .start_time(
                        DateTime::parse_from_rfc3339(&controller.logon_time)
                            .unwrap()
                            .to_utc(),
                    )
                    .cid(controller.cid)
                    .build();
                let d = Utc::now() - q.start_time;
            }
        }
        println!("{}", time.elapsed().as_millis());
    }
    // let online_positions: Vec<&Controller> = latest_data_result
    //     .controllers
    //     .iter()
    //     .filter(|c| flat_positions.iter().any(|p| p.is_match_for(&c.callsign)))
    //     .collect();

    // let mut positions: Vec<Position> = vec![];
    // if let Ok(z) = y {
    //     z.iter().for_each(|a| positions.extend(a.all_positions()))
    // }
    //
    // println!("{}", positions.len());
    // println!("{}", start.elapsed().as_micros());

    Ok(())
}

pub fn is_active_vnas_controller(c: &Controller) -> bool {
    c.server == "VIRTUALNAS" && c.facility > 0 && c.frequency != "199.998"
}

pub struct PositionMatcher {
    pub parent_facility: Facility,
    pub position: Position,
    pub regex: Regex,
}

impl PositionMatcher {
    pub fn is_match(&self, haystack: &str) -> bool {
        self.regex.is_match(haystack)
    }
}

impl From<PositionWithParentFacility> for PositionMatcher {
    fn from(value: PositionWithParentFacility) -> Self {
        PositionMatcher {
            parent_facility: value.parent_facility,
            position: value.position.clone(),
            regex: value.position.match_regex().unwrap(),
        }
    }
}
