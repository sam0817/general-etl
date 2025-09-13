use crate::utils::error::{EtlError, Result};
use backoff::{ExponentialBackoff, future::retry};
use reqwest::{Client, Response};
use std::time::Duration;

pub struct ApiClient {
    client: Client,
    base_url: String,
    timeout: Duration,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            timeout: Duration::from_secs(30),
        }
    }

    pub async fn fetch_json(&self, endpoint: &str) -> Result<serde_json::Value> {
        let url = format!("{}/{}", self.base_url, endpoint);

        let response = retry(ExponentialBackoff::default(), || async {
            // Ok::<Response, backoff::Error<E>>(self.client
            Ok::<Response, backoff::Error<reqwest::Error>>(self.client
                .get(&url)
                .send()
                .await
                .map_err(backoff::Error::transient)?)
        })
            .await?;

        let json = response.json().await?;
        Ok(json)
    }

    pub async fn fetch_csv(&self, endpoint: &str) -> Result<String> {
        let url = format!("{}/{}", self.base_url, endpoint);

        let response = self.client
            .get(&url)
            .send()
            .await?;

        let text = response.text().await?;
        Ok(text)
    }

    pub async fn fetch_zip(&self, endpoint: &str) -> Result<Vec<u8>> {
        let url = format!("{}/{}", self.base_url, endpoint);

        let response = self.client
            .get(&url)
            .send()
            .await?;

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
}
