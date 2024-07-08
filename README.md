# Geocodio Lib Rust

A [Geocodio API](https://www.geocod.io/docs/#introduction) client library that was originally forked from [populist-vote](https://github.com/populist-vote/geocodio)'s library. However, this Crate is substantially different. I refactored almost all of the existing code, added multiple new features, and created documentation.

# How To Use

To get started, it's recommended to create a `.env` file in your project and add the variable `GEOCODIO_API_KEY`. Once you have your API key assigned you can use `GeocodioProxy::new()` to start using the client library.

```rust
let geocodio = GeocodioProxy::new().unwrap();
```

If you're using another method to get your API key, you can use `GeocodioProxy::new_from_key()` and pass the key as a variable.

```rust
let geocodio = GeocodioProxy::new_from_key(my_api_key).unwrap();
```

Once you have `GeocodioProxy` assigned to a variable, you can do 4 things:
- [Single address geocoding](#single-address-geocode)
- [Batch geocoding](#batch-geocode)
- [Single Coordinate reverse geocoding](#single-coordinate-reverse-geocode)
- [Batch reverse geocoding](#reverse-batch-geocode)

# Geocoding

## Address Parameters

When geocoding addresses, the input address(es) are required to be the enum `AddressParams`. If you only have the address(es) as a string, you can wrap it with `AddressParams::String`. However, if you have the ability to control how the addresses are inputted, it's recommended to use `AddressParams::AddressInput` for more accurate results. The fields for AddressInput are (all fields are `Option`s):
- line_1 (Street number, name, and suffix)
- line_2
- city
- state
- country
- postal_code

## Single Address Geocode

```rust
use geocodio_lib_rust::{request::address::{AddressInput, AddressParams}, GeocodioProxy};

#[tokio::main]
async fn main() {
    let geocodio = GeocodioProxy::new().unwrap();
    let response = geocodio
        .geocode(
            AddressParams::AddressInput(AddressInput {
                line_1: Some("1500 Sugar Bowl Dr".to_string()),
                line_2: None,
                city: Some("New Orleans".to_string()),
                state: Some("LA".to_string()),
                country: Some("US".to_string()),
                postal_code: Some("70112".to_string()),
            }),
            None,
        )
        .await
        .unwrap();
    println!(
        "The coordinates for the Superdome are: {}, {}", 
        response.results[0].location.latitude, 
        response.results[0].location.longitude
    )
}
```

## Batch Geocode

```rust
use geocodio_lib_rust::{request::address::AddressParams, response::BatchResult, GeocodioProxy};

#[tokio::main]
async fn main() {
    let addresses = vec![
        AddressParams::String("1500 Sugar Bowl Dr, New Orleans, LA 70112".to_string()),
        AddressParams::String("1 MetLife Stadium Dr, East Rutherford, NJ 07073".to_string()),
        AddressParams::String("1 AT&T Way, Arlington, TX 76011".to_string())
    ];

    let geocodio = GeocodioProxy::new().unwrap();
    let response = geocodio
        .geocode_batch(addresses)
        .await
        .unwrap();

    response.results.map(|res: Vec<BatchResult>| {
        res.iter().map(|address: &BatchResult| {
            if let Some(input) = &address.query {
                println!("INPUT ADDRESS: {:?}", input);
            };
            if let Some(response) = &address.response {
                if let Some(results) = &response.results {
                    println!("ADDRESS COMPONENTS: {:?}", results[0].address_components);
                    println!("FORMATTED ADDRESS: {:?}", results[0].formatted_address);
                    println!("LOCATION: {:?}", results[0].location);
                    println!("ACCURACY: {:?}", results[0].accuracy);
                    println!("ACCURACY TYPE: {:?}", results[0].accuracy_type);
                    println!("SOURCE: {:?}", results[0].source);
                }
            };
            println!("============================")
        }).collect::<Vec<_>>()
    });
}
```

# Reverse Geocoding

## Single Coordinate Reverse Geocode

```rust
use geocodio_lib_rust::{request::address::Coordinates, GeocodioProxy};

#[tokio::main]
async fn main() {
    let geocodio = GeocodioProxy::new().unwrap();

    let coordinates = Coordinates { latitude: 40.81352, longitude: -74.074333 };

    let response = geocodio
        .reverse_geocode(coordinates)
        .await
        .unwrap();
    println!("{:?}", response);
}
```

## Reverse Batch Geocode

```rust
use geocodio_lib_rust::{request::address::Coordinates, GeocodioProxy};

#[tokio::main]
async fn main() {
    let geocodio = GeocodioProxy::new().unwrap();

    let coordinates = vec![
        Coordinates { latitude: 40.81352, longitude: -74.074333 },
        Coordinates { latitude: 35.9746000, longitude: -77.9658000 },
        Coordinates { latitude: 32.8793700, longitude: -96.6303900 },
    ];

    let response = geocodio
        .reverse_geocode_batch(coordinates)
        .await
        .unwrap();
    println!("{:?}", response);
}
```