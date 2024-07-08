use serde::{Deserialize, Serialize};

use crate::response::{address::{Address, AddressComponents}, utils::{Input, Location}};

/// Address information formatted to match the json response
pub mod address;
/// Congressional district information
pub mod congressional;
/// Extra response information
pub mod utils;

/// The result type for [`GeocodioProxy::geocode`]. 
/// 
/// Contains a parsed input and a vector of addresses. The reason there are multiple
/// addresses in 'results' is because if the input address isn't properly formatted
/// or is missing some piece of the address, the API will send multiple addresses that 
/// could be the match, with an accuracy score attached.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeocodeResponse {
    pub input: Input,
    pub results: Vec<Address>,
}

/// The result type for [`GeocodioProxy::geocode_batch`] and [`GeocodioProxy::reverse_geocode_batch`].
#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeocodeBatchResponse {
    pub results: Option<Vec<BatchResult>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeocodeReverseResponse {
    pub results: Option<Vec<Address>>,
}

/// Individual results from each [`GeocodeBatchResponse`].
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchResult {
    pub query: Option<String>,
    pub response: Option<Response>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Response {
    pub input: Option<Input>,
    pub results: Option<Vec<ResponseResult>>,
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