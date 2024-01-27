use crate::stats_models::ControllerSession;
use crate::vnas_aggregate_models::{
    AllFacilities, AllPositions, Callsign, FacilityWithTreeInfo, PositionWithParentFacility,
};
use crate::vnas_api::VnasApi;
use crate::vnas_api_models::{Facility, Position};
use chrono::DateTime;
use regex::Regex;
use reqwest::Error;
use sqlx::postgres::PgPoolOptions;
use std::num::ParseFloatError;
use std::time::Instant;
use vatsim_utils::live_api::Vatsim;
use vatsim_utils::models::Controller;

mod stats_models;
mod vnas_aggregate_models;
mod vnas_api;
mod vnas_api_models;

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
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:pw@localhost/ironmic")
        .await
        .expect("Error creating db pool");

    let migrate = sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Error with migrations");

    let x = VnasApi::new().unwrap();
    let all_artccs = x.get_all_artccs_data().await?;

    let all_facilities_info: Vec<FacilityWithTreeInfo> = all_artccs
        .iter()
        .flat_map(|f| f.all_facilities_with_info())
        .collect();

    let start = Instant::now();
    for artcc in all_artccs.iter() {
        let d = DateTime::parse_from_rfc3339(&artcc.last_updated_at).unwrap();

        let row = sqlx::query("insert into artccs (id, last_updated) values ($1, $2);")
            .bind(&artcc.id)
            .bind(d)
            .execute(&pool)
            .await
            .expect("Error inserting");
    }

    for f in all_facilities_info.iter() {
        let d = DateTime::parse_from_rfc3339(&f.artcc_root.last_updated_at).unwrap();
        let parent_facility_id_str = if let Some(s) = &f.parent_facility {
            Some(&s.id)
        } else {
            None
        };

        let row = sqlx::query("insert into facilities (id, name, type, last_updated, parent_facility_id, parent_artcc_id) values ($1, $2, $3, $4, $5, $6);")
            .bind(&f.facility.id)
            .bind(&f.facility.name)
            .bind(&f.facility.type_field.to_string())
            .bind(d)
            .bind(parent_facility_id_str)
            .bind(&f.artcc_root.id)
            .execute(&pool)
            .await
            .expect("Error inserting");
    }

    for p in all_artccs
        .iter()
        .flat_map(|f| f.all_positions_with_parents())
    {
        let row = sqlx::query("insert into positions (id, name, radio_name, callsign, callsign_prefix, callsign_infix, callsign_suffix, callsign_without_infix, frequency, parent_facility_id) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10);")
            .bind(&p.position.id)
            .bind(&p.position.name)
            .bind(&p.position.radio_name)
            .bind(&p.position.callsign)
            .bind(&p.position.callsign_prefix())
            .bind(&p.position.callsign_infix())
            .bind(&p.position.callsign_suffix())
            .bind(format!("{}_{}", &p.position.callsign_prefix(), &p.position.callsign_suffix()))
            .bind(&p.position.frequency)
            .bind(&p.parent_facility.id)
            .execute(&pool)
            .await
            .expect("Error inserting");
    }

    // sqlx::query("insert into positions (id, name, radio_name, callsign, callsign_prefix, callsign_infix, callsign_suffix, callsign_without_infix, frequency, parent_facility_id, parent_artcc_id) select * from unnest
    println!("DB time in micros {}", start.elapsed().as_micros());
    println!("Length {}", all_facilities_info.len());

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

        let x = single_or_no_match(&position_matchers, &controller);

        if let Some(pm) = x {
            let y = ControllerSession::try_from((pm, &controller));
            println!(
                "Found match for {} - {} parent {} with {}",
                pm.position.name,
                pm.position.callsign,
                pm.parent_facility.name,
                controller.callsign
            )
        } else {
            println!("No single match found for {}", controller.callsign)
        }

        //
        // for pm in position_matchers.as_slice() {
        //     if pm.is_match(&controller) {
        //         println!(
        //             "Found match for {} - {} parent {} with {}",
        //             pm.position.name,
        //             pm.position.callsign,
        //             pm.parent_facility.name,
        //             controller.callsign
        //         );
        //         // let z: ControllerSession = ControllerSessionBuilder::default()
        //         //     .start_time(
        //         //         DateTime::parse_from_rfc3339(&controller.logon_time)
        //         //             .unwrap()
        //         //             .to_utc(),
        //         //     )
        //         //     .build();
        //         // let q: ControllerSession = ControllerSession::builder()
        //         //     .start_time(
        //         //         DateTime::parse_from_rfc3339(&controller.logon_time)
        //         //             .unwrap()
        //         //             .to_utc(),
        //         //     )
        //         //     .cid(controller.cid)
        //         //     .build();
        //         let q = ControllerSession::try_from((pm, &controller));
        //     }
        // }
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
    pub fn is_match(&self, controller: &Controller) -> bool {
        self.regex.is_match(&controller.callsign)
            && if let Ok(b) = self.is_freq_match(&controller.frequency) {
                b
            } else {
                dbg!("Error parsing VATSIM freq {}", &controller.frequency);
                false
            }
    }

    pub fn is_starred_match(&self, controller: &Controller) -> bool {
        self.is_match(controller) && self.position.starred
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

fn single_or_no_match<'a>(
    matchers: &'a [PositionMatcher],
    controller: &Controller,
) -> Option<&'a PositionMatcher> {
    let mut matched: Vec<&PositionMatcher> = vec![];
    for matcher in matchers {
        if matcher.is_match(&controller) {
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

fn all_matches<'a>(
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
