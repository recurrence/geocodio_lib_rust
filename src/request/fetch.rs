use crate::{errors::Error, response::GeocodeBatchResponse, GeocodioProxy};

const GEOCODIO_BASE_URL: &str = "https://api.geocod.io/v1.7/";

pub(crate) fn proxy_new(api_key: String) -> Result<GeocodioProxy, Error> {
    let client = reqwest::Client::new();

    Ok(GeocodioProxy {
        client,
        base_url: reqwest::Url::parse(GEOCODIO_BASE_URL).unwrap(),
        api_key,
    })
}

pub(crate) async fn batch_fetch(data: &GeocodioProxy, endpoint: String, params: Vec<String>) -> Result<GeocodeBatchResponse, Error> {
    let res = data.request_batch(endpoint.as_str(), params).await?;
    let json = res.json::<serde_json::Value>().await?;
    let result = serde_json::from_value::<GeocodeBatchResponse>(json);
    match result {
        Ok(geocode_response) => Ok(geocode_response),
        Err(e) => Err(Error::BadAddress(e)),
    }
}