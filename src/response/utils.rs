use serde::{Deserialize, Serialize};

use super::{address::AddressComponents, congressional::{CongressionalDistrict, StateLegislativeDistricts}};


#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fields {
    pub timezone: Option<Timezone>,
    pub zip4: Option<Zip4>,
    pub congressional_district: Option<CongressionalDistrict>,
    pub congressional_districts: Option<Vec<CongressionalDistrict>>,
    pub state_legislative_districts: Option<StateLegislativeDistricts>,
    pub school_districts: Option<SchoolDistricts>,
    pub census: Option<CensusResults>,
    pub acs: Option<CencusAcs>,
}

// TO DO
pub type CensusResults = serde_json::Value;
pub type CencusAcs = serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchoolDistricts {
    pub unified: Option<SchoolDistrict>,
    pub elementary: Option<SchoolDistrict>,
    pub secondary: Option<SchoolDistrict>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchoolDistrict {
    pub name: String,
    pub lea_code: String,
    pub grade_low: String,
    pub grade_high: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timezone {
    pub name: String,
    pub abbreviation: String,
    pub utc_offset: i16,
    pub observes_dst: bool,
    pub source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Zip4 {
    pub record_type: RecordType,
    pub carrier_route: CarrierRoute,
    pub building_or_firm_name: Option<String>,
    pub plus4: Vec<String>,
    pub zip9: Vec<String>,
    pub government_building: Option<String>,
    pub facility_code: FacilityCode,
    pub city_delivery: bool,
    pub valid_delivery_area: bool,
    pub exact_match: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordType {
    pub code: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CarrierRoute {
    pub id: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FacilityCode {
    pub code: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    address_components: AddressComponents,
    formatted_address: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}