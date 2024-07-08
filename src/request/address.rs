use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    #[serde(rename = "lat")]
    pub latitude: f64,
    #[serde(rename = "lng")]
    pub longitude: f64,
}

pub enum AddressParams {
    String(String),
    AddressInput(AddressInput),
}

#[derive(Serialize, Deserialize)]
pub struct AddressInput {
    pub line_1: Option<String>,
    pub line_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
}