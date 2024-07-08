use crate::{errors::Error, GeocodioProxy};

pub mod fetch;
pub mod address;
pub mod utils;

impl GeocodioProxy {
    /// Helper function to wrap the request
    pub(crate) async fn request(&self, endpoint: &str, params: &str) -> Result<reqwest::Response, Error> {
        let mut url = self.base_url.join(endpoint).unwrap();
        url.set_query(Some(params));
        url.query_pairs_mut().append_pair("api_key", &self.api_key);
        // println!("{}", url);
        Ok(self.client.get(url).send().await.unwrap())
    }

    /// Request Batch
    pub(crate) async fn request_batch(&self, endpoint: &str, params: Vec<String>) -> Result<reqwest::Response, Error> {
        let url = self.base_url.join(endpoint).unwrap();
        let mut payload: Vec<String> = Vec::new();

        params.iter().enumerate().for_each(|(_i, address)| {
            payload.push(serde_json::Value::String(address.to_owned()).to_string());
        });
        let res = self.client.post(url).json(&payload).send().await?;
        Ok(res)
    }
}