use errors::Error;
use json::{address::AddressParams, utils::Coordinates};
use request::fetch::{batch_fetch, proxy_new};
use response::{GeocodeBatchResponse, GeocodeResponse, GeocodeReverseResponse};

pub mod json;
pub mod errors;
pub mod request;
pub mod response;
pub(crate) mod utils;

pub struct GeocodioProxy {
    pub client: reqwest::Client,
    pub base_url: reqwest::Url,
    pub api_key: String,
}

impl GeocodioProxy {
    /// Instantiate new GeocodioProxy API client from .env GEOCODIO_API_KEY variable
    pub fn new() -> Result<Self, Error> {
        dotenv::dotenv().ok();
        let api_key = std::env::var("GEOCODIO_API_KEY")?;
        proxy_new(api_key)
    }

    /// Instantiate new GeocodioProxy API client by passing api key
    pub fn new_from_key(api_key: String) -> Result<Self, Error> {
        proxy_new(api_key)
    }

    /// Geocode a single address
    pub async fn geocode(&self, address: AddressParams, fields: Option<&[&str]>) -> Result<GeocodeResponse, Error> {
        let mut params = match address {
            AddressParams::String(address) => address.to_string(),
            AddressParams::AddressInput(address) => address.fmt_string(),
        };
        if let Some(fields) = fields {
            params.push_str(format!("&fields={}", fields.join(",")).as_str());
        }
        let endpoint = "geocode";
        geo_fetch!(self, endpoint, params, GeocodeResponse)
    }

    /// Batch Geocode
    pub async fn geocode_batch(&self, addresses: Vec<AddressParams>) -> Result<GeocodeBatchResponse, Error> {
        let mut params: Vec<String> = Vec::new();
        addresses.iter().for_each(|address| {
            match address {
                AddressParams::String(address) => params.push(address.to_string()),
                AddressParams::AddressInput(address) => params.push(address.to_string()),
            };
        });
        let endpoint = format!("geocode?api_key={}", &self.api_key);
        batch_fetch(self, endpoint, params).await
    }

    /// Reverse geocode a tuple of (lat,lng)
    pub async fn reverse_geocode(&self, coordinates: Coordinates) -> Result<GeocodeReverseResponse, Error> {
        let params = format!("q={},{}", coordinates.latitude, coordinates.longitude);
        let endpoint = "reverse";
        geo_fetch!(self, endpoint, params, GeocodeReverseResponse)
    }

    // TODO: reverse geocode batch
    pub async fn reverse_geocode_batch(&self, coordinates: Vec<Coordinates>) -> Result<GeocodeBatchResponse, Error> {
        let params = coordinates.iter().map(|coords| {
                format!("{},{}", coords.latitude, coords.longitude)
            }).collect::<Vec<String>>();
        let endpoint = format!("reverse?api_key={}", &self.api_key);
        batch_fetch(self, endpoint, params).await
    }
}