use crate::{errors::Error, GeocodioProxy};

const GEOCODIO_BASE_URL: &str = "https://api.geocod.io/v1.7/";

pub(crate) fn proxy_new(api_key: String) -> Result<GeocodioProxy, Error> {
    let client = reqwest::Client::new();

    Ok(GeocodioProxy {
        client,
        base_url: reqwest::Url::parse(GEOCODIO_BASE_URL).unwrap(),
        api_key,
    })
}