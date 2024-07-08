#![doc = include_str!("../README.md")]

use errors::Error;
use response::{GeocodeBatchResponse, GeocodeResponse, GeocodeReverseResponse};
use request::{address::{AddressParams, Coordinates}, fetch::{batch_fetch, proxy_new}};

/// Response structs formatted from the json in the API docs
pub mod response;
/// Errors that can occur in the Crate
pub mod errors;
/// Request structs and functions 
pub mod request;

/// A struct used to interface with the [Geocodio API](https://www.geocod.io/docs/#introduction).
/// 
/// The client and URL are already provided, all that's needed from you is your Geocodio
/// API key. There's an option if you have it in a .env file with the name 'GEOCODIO_API_KEY' 
/// ([`GeocodioProxy::new`]), or if you're using another method to obtain your key and 
/// assigning it to a variable ([`GeocodioProxy::new_from_key()`]).
/// ```rust
/// let geocodio = GeocodioProxy::new().unwrap();
/// // or
/// let geocodio = GeocodioProxy::new_from_key(my_api_key).unwrap();
/// ```
/// 
/// Once you instantiate the struct, you can either 
/// geocode or reverse geocode a single address or 
/// a batch of addresses [(up to 10,000 lookups)](https://www.geocod.io/docs/#batch-geocoding).
pub struct GeocodioProxy {
    pub client: reqwest::Client,
    pub base_url: reqwest::Url,
    pub api_key: String,
}

// ========== instantiate GeocodeProxy ==========
impl GeocodioProxy {
    /// Create a new instance of [`GeocodioProxy`] via the variable 'GEOCODIO_API_KEY' you define in a .env file.
    pub fn new() -> Result<Self, Error> {
        dotenv::dotenv().ok();
        let api_key = std::env::var("GEOCODIO_API_KEY")?;
        proxy_new(api_key)
    }

    /// Create a new instance of [`GeocodioProxy`] via a variable you pass into the method.
    pub fn new_from_key(api_key: String) -> Result<Self, Error> {
        proxy_new(api_key)
    }
}

// ========== geocode address(es) ==========
impl GeocodioProxy {
    /// Geocode a single address.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use geocodio_lib_rust::{request::address::{AddressInput, AddressParams}, GeocodioProxy};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let geocodio = GeocodioProxy::new().unwrap();
    ///    let response = geocodio
    ///        .geocode(
    ///            AddressParams::AddressInput(AddressInput {
    ///                 line_1: Some("1500 Sugar Bowl Dr".to_string()),
    ///                 line_2: None,
    ///                 city: Some("New Orleans".to_string()),
    ///                 state: Some("LA".to_string()),
    ///                 country: Some("US".to_string()),
    ///                 postal_code: Some("70112".to_string()),
    ///            }),
    ///            None,
    ///        )
    ///        .await
    ///        .unwrap();
    ///    println!(
    ///        "The coordinates for the Superdome are: {}, {}", 
    ///        response.results[0].location.latitude, 
    ///        response.results[0].location.longitude
    ///    )
    ///}
    /// ```
    pub async fn geocode(&self, address: AddressParams, fields: Option<&[&str]>) -> Result<GeocodeResponse, Error> {
        let mut params = match address {
            AddressParams::String(address) => address.to_string(),
            AddressParams::AddressInput(address) => address.fmt_string(),
        };
        if let Some(fields) = fields {
            params.push_str(format!("&fields={}", fields.join(",")).as_str());
        }
        let endpoint = "geocode";
        single_fetch!(self, endpoint, params, GeocodeResponse)
    }

    /// Batch Geocode up to [10,000 addresses](https://www.geocod.io/docs/#batch-geocoding).
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use geocodio_lib_rust::{request::address::AddressParams, GeocodioProxy};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let addresses = vec![
    ///        AddressParams::String("1500 Sugar Bowl Dr, New Orleans, LA 70112".to_string()),
    ///        AddressParams::String("1 MetLife Stadium Dr, East Rutherford, NJ 07073".to_string()),
    ///        AddressParams::String("1 AT&T Way, Arlington, TX 76011".to_string())
    ///    ];
    ///
    ///    let geocodio = GeocodioProxy::new().unwrap();
    ///    let response = geocodio
    ///        .geocode_batch(addresses)
    ///        .await
    ///        .unwrap();
    ///
    ///    response.results.map(|res| {
    ///        res.iter().map(|address| {
    ///            if let Some(input) = &address.query {
    ///                println!("INPUT ADDRESS: {:?}", input);
    ///            };
    ///            if let Some(response) = &address.response {
    ///                if let Some(results) = &response.results {
    ///                    println!("ADDRESS COMPONENTS: {:?}", results[0].address_components);
    ///                    println!("FORMATTED ADDRESS: {:?}", results[0].formatted_address);
    ///                    println!("LOCATION: {:?}", results[0].location);
    ///                    println!("ACCURACY: {:?}", results[0].accuracy);
    ///                    println!("ACCURACY TYPE: {:?}", results[0].accuracy_type);
    ///                    println!("SOURCE: {:?}", results[0].source);
    ///                }
    ///            };
    ///            println!("============================")
    ///        }).collect::<Vec<_>>()
    ///    });
    ///}
    /// ```
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
}

// ========== reverse geocode ==========
impl GeocodioProxy {
    /// Reverse geocode [`Coordinates`] to get the address and other location information.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use geocodio_lib_rust::{request::address::Coordinates, GeocodioProxy};
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let geocodio = GeocodioProxy::new().unwrap();
    ///     let coordinates = Coordinates { latitude: 40.81352, longitude: -74.074333 };
    /// 
    ///     let response = geocodio
    ///         .reverse_geocode(coordinates)
    ///         .await
    ///         .unwrap();
    ///     println!("{:?}", response);
    /// }
    /// ```
    /// 
    pub async fn reverse_geocode(&self, coordinates: Coordinates) -> Result<GeocodeReverseResponse, Error> {
        let params = format!("q={},{}", coordinates.latitude, coordinates.longitude);
        let endpoint = "reverse";
        single_fetch!(self, endpoint, params, GeocodeReverseResponse)
    }

    /// Reverse geocode a vector of [`Coordinates`] to get the addresses and other location information.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use geocodio_lib_rust::{request::address::Coordinates, GeocodioProxy};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let geocodio = GeocodioProxy::new().unwrap();
    /// 
    ///     let coordinates = vec![
    ///         Coordinates { latitude: 40.81352, longitude: -74.074333 },
    ///         Coordinates { latitude: 35.9746000, longitude: -77.9658000 },
    ///         Coordinates { latitude: 32.8793700, longitude: -96.6303900 },
    ///     ];
    /// 
    ///     let response = geocodio
    ///         .reverse_geocode_batch(coordinates)
    ///         .await
    ///         .unwrap();
    ///     println!("{:?}", response);
    /// }
    /// ```
    pub async fn reverse_geocode_batch(&self, coordinates: Vec<Coordinates>) -> Result<GeocodeBatchResponse, Error> {
        let params = coordinates.iter().map(|coords| {
                format!("{},{}", coords.latitude, coords.longitude)
            }).collect::<Vec<String>>();
        let endpoint = format!("reverse?api_key={}", &self.api_key);
        batch_fetch(self, endpoint, params).await
    }
}