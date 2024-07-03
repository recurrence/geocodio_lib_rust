use serde::{Deserialize, Serialize};

use crate::json::{address::{Address, AddressComponents}, utils::{Input, Location}};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchResult {
    pub query: Option<String>,
    pub response: Option<Response>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseResult {
    pub address_components: Option<AddressComponents>,
    pub formatted_address: Option<String>,
    pub location: Option<Location>,
    pub accuracy: Option<f64>,
    pub accuracy_type: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub input: Option<Input>,
    pub results: Option<Vec<ResponseResult>>,
}

// Single Response
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeocodeResponse {
    pub input: Input,
    pub results: Vec<Address>,
    pub debug: Option<Debug>,
}

// Batch Response
#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeocodeBatchResponse {
    // pub input: Input,
    pub results: Option<Vec<BatchResult>>,
    pub debug: Option<Debug>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Debug {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeocodeReverseResponse {
    pub results: Option<Vec<Address>>,
    pub debug: Option<Debug>,
}