use regex::{Error, Regex};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtccRoot {
    pub id: String,
    pub last_updated_at: String,
    pub facility: Facility,
    pub visibility_centers: Vec<Point>,
    pub aliases_last_updated_at: String,
    pub video_maps: Vec<VideoMap>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FacilityType {
    Artcc,
    Tracon,
    AtctTracon,
    AtctRapcon,
    Atct,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StarsColorSet {
    Tcw,
    Tdw,
    Dod,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BeaconCodeBankType {
    Vfr,
    Ifr,
    Any,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BeaconCodeBankCategory {
    Internal,
    External,
    Military,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BeaconCodeBankPriority {
    Primary,
    Secondary,
    Tertiary,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Facility {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: FacilityType,
    pub name: String,
    pub child_facilities: Vec<Facility>,
    pub eram_configuration: Option<FacilityEramConfiguration>,
    pub stars_configuration: Option<FacilityStarsConfiguration>,
    pub tower_cab_configuration: Option<TowerCabConfiguration>,
    pub asdex_configuration: Option<AsdexConfiguration>,
    pub tdls_configuration: Option<TdlsConfiguration>,
    pub flight_strips_configuration: Option<FlightStripsConfiguration2>,
    pub positions: Vec<Position>,
    pub neighboring_facility_ids: Vec<String>,
    pub non_nas_facility_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TowerCabConfiguration {
    pub video_map_id: String,
    pub default_rotation: i16,
    pub default_zoom_range: i16,
    pub aircraft_visibility_ceiling: i32,
    pub tower_location: Option<Point>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoldShortRunwayPair {
    pub id: String,
    pub arrival_runway_id: String,
    pub hold_short_id: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsdexConfiguration {
    pub video_map_id: String,
    pub default_rotation: i16,
    pub default_zoom_range: i16,
    pub target_visibility_range: i32,
    pub target_visibility_ceiling: i32,
    pub fix_rules: Vec<FixRule>,
    pub use_destination_id_as_fix: bool,
    pub runway_configurations: Vec<RunwayConfiguration>,
    pub positions: Vec<AsdexPosition>,
    pub default_position_id: String,
    pub tower_location: Point,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixRule {
    pub id: String,
    pub search_pattern: String,
    pub fix_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunwayConfiguration {
    pub id: String,
    pub name: String,
    pub arrival_runway_ids: Vec<String>,
    pub departure_runway_ids: Vec<String>,
    pub hold_short_runway_pairs: Vec<HoldShortRunwayPair>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsdexPosition {
    pub id: String,
    pub name: String,
    pub runway_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub id: String,
    pub name: String,
    pub starred: bool,
    pub radio_name: String,
    pub callsign: String,
    pub frequency: i64,
    pub stars_configuration: Option<PositionStarsConfiguration>,
    pub eram_configuration: Option<PositionEramConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TdlsConfiguration {
    pub mandatory_sid: bool,
    pub mandatory_climbout: bool,
    pub mandatory_climbvia: bool,
    pub mandatory_initial_alt: bool,
    pub mandatory_dep_freq: bool,
    pub mandatory_expect: bool,
    pub mandatory_contact_info: bool,
    pub mandatory_local_info: bool,
    pub sids: Vec<Sid>,
    pub climbouts: Vec<IdValPair<String, String>>,
    pub climbvias: Vec<IdValPair<String, String>>,
    pub initial_alts: Vec<IdValPair<String, String>>,
    pub dep_freqs: Vec<IdValPair<String, String>>,
    pub expects: Vec<IdValPair<String, String>>,
    pub contact_infos: Vec<IdValPair<String, String>>,
    pub local_infos: Vec<IdValPair<String, String>>,
    pub default_sid_id: Option<String>,
    pub default_transition_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sid {
    pub name: String,
    pub id: String,
    pub transitions: Vec<Transition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transition {
    pub name: String,
    pub id: String,
    pub first_route_point: Option<String>,
    pub default_expect: Option<String>,
    pub default_climbvia: Option<String>,
    pub default_dep_freq: Option<String>,
    pub default_initial_alt: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdValPair<A, B> {
    pub id: A,
    pub value: B,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlightStripsConfiguration {
    pub strip_bays: Vec<StripBay>,
    pub external_bays: Vec<ExternalBay>,
    pub display_destination_airport_ids: bool,
    pub display_barcodes: bool,
    pub enable_arrival_strips: bool,
    pub enable_separate_arr_dep_printers: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StripBay {
    pub id: String,
    pub name: String,
    pub number_of_racks: i16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalBay {
    pub facility_id: String,
    pub bay_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionStarsConfiguration {
    pub subset: i16,
    pub sector_id: String,
    pub area_id: String,
    pub color_set: StarsColorSet,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FacilityStarsConfiguration {
    pub areas: Vec<Area>,
    pub internal_airports: Vec<String>,
    pub beacon_code_banks: Vec<StarsBeaconCodeBank>,
    pub rpcs: Vec<Rpc>,
    pub primary_scratchpad_rules: Vec<ScratchpadRule>,
    pub secondary_scratchpad_rules: Vec<ScratchpadRule>,
    pub rnav_patterns: Vec<Value>,
    #[serde(rename = "allow4CharacterScratchpad")]
    pub allow4character_scratchpad: bool,
    pub stars_handoff_ids: Vec<StarsHandoffId>,
    pub video_map_ids: Vec<String>,
    pub map_groups: Vec<MapGroup>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    pub id: String,
    pub name: String,
    pub visibility_center: Option<Point>,
    pub surveillance_range: i16,
    pub ssa_airports: Vec<String>,
    pub tower_list_configurations: Vec<TowerListConfiguration>,
    pub ldb_beacon_codes_inhibited: bool,
    pub pdb_ground_speed_inhibited: bool,
    pub display_requested_alt_in_fdb: bool,
    pub use_vfr_position_symbol: bool,
    pub show_destination_departures: bool,
    pub show_destination_satellite_arrivals: bool,
    pub show_destination_primary_arrivals: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TowerListConfiguration {
    pub id: String,
    pub airport_id: String,
    pub range: i16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StarsBeaconCodeBank {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: BeaconCodeBankType,
    pub start: i16,
    pub end: i16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rpc {
    pub id: String,
    pub index: i16,
    pub airport_id: String,
    pub position_symbol_tie: String,
    pub position_symbol_stagger: String,
    pub master_runway: Runway,
    pub slave_runway: Runway,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runway {
    pub runway_id: String,
    pub heading_tolerance: i16,
    pub near_side_half_width: f64,
    pub far_side_half_width: f64,
    pub near_side_distance: f64,
    pub region_length: f64,
    pub target_reference_point: Point,
    pub target_reference_line_heading: f64,
    pub target_reference_line_length: f64,
    pub target_reference_point_altitude: i32,
    pub image_reference_point: Point,
    pub image_reference_line_heading: f64,
    pub image_reference_line_length: f64,
    pub tie_mode_offset: f64,
    pub descent_point_distance: f64,
    pub descent_point_altitude: i64,
    pub above_path_tolerance: i16,
    pub below_path_tolerance: i16,
    pub default_leader_direction: String,
    pub scratchpad_patterns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScratchpadRule {
    pub id: String,
    pub airport_ids: Vec<String>,
    pub search_pattern: String,
    pub min_altitude: Option<i32>,
    pub max_altitude: Option<i32>,
    pub template: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StarsHandoffId {
    pub id: String,
    pub facility_id: String,
    pub handoff_number: i16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapGroup {
    pub id: String,
    pub map_ids: Vec<Option<i32>>,
    pub tcps: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlightStripsConfiguration2 {
    pub strip_bays: Vec<StripBay>,
    pub external_bays: Vec<Value>,
    pub display_destination_airport_ids: bool,
    pub display_barcodes: bool,
    pub enable_arrival_strips: bool,
    pub enable_separate_arr_dep_printers: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StarsConfiguration3 {
    pub subset: i64,
    pub sector_id: String,
    pub area_id: String,
    pub color_set: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FacilityEramConfiguration {
    pub nas_id: String,
    pub geo_maps: Vec<GeoMap>,
    pub emergency_checklist: Vec<String>,
    pub position_relief_checklist: Vec<String>,
    pub internal_airports: Vec<Value>,
    pub beacon_code_banks: Vec<EramBeaconCodeBank>,
    pub neighboring_stars_configurations: Vec<NeighboringStarsConfiguration>,
    pub neighboring_caats_configurations: Vec<Value>,
    pub coordination_fixes: Vec<Value>,
    pub reference_fixes: Vec<String>,
    pub asr_sites: Vec<AsrSite>,
    pub conflict_alert_floor: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsrSite {
    pub id: String,
    pub asr_id: String,
    pub location: Point,
    pub range: i16,
    pub ceiling: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoMap {
    pub id: String,
    pub name: String,
    pub label_line1: String,
    pub label_line2: String,
    pub filter_menu: Vec<FilterMenu>,
    pub bcg_menu: Vec<String>,
    pub video_map_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterMenu {
    pub id: String,
    pub label_line1: String,
    pub label_line2: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EramBeaconCodeBank {
    pub id: String,
    pub category: BeaconCodeBankCategory,
    pub priority: BeaconCodeBankPriority,
    pub subset: i16,
    pub start: i16,
    pub end: i16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeighboringStarsConfiguration {
    pub id: String,
    pub facility_id: String,
    pub stars_id: String,
    pub single_character_stars_id: Option<String>,
    #[serde(rename = "fieldEFormat")]
    pub field_eformat: String,
    #[serde(rename = "fieldELetter")]
    pub field_eletter: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionEramConfiguration {
    pub sector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoMap {
    pub id: String,
    pub name: String,
    pub tags: Vec<String>,
    pub short_name: Option<String>,
    pub source_file_name: String,
    pub last_updated_at: String,
    pub stars_brightness_category: String,
    pub stars_id: Option<i16>,
    pub stars_always_visible: bool,
    pub tdm_only: bool,
}
